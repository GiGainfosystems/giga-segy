// Copyright (C) 2022 by GiGa infosystems
//! This is a simplified library for reading SEGY files into rust.
extern crate fnv;
extern crate num;
extern crate rsg_core;
#[macro_use]
extern crate tinyvec;
#[cfg(test)]
extern crate rsg_in;
#[cfg(test)]
extern crate tempfile;

pub mod create_headers;
#[cfg(test)]
mod integration_tests;
mod settings;
pub mod utils;
mod write_data;
pub mod write_headers;

use rsg_core::errors::*;
use rsg_core::header_structs::*;
use rsg_core::{SegyMetadata, Trace};

use num::ToPrimitive;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::settings::SegyWriteSettings;
use crate::write_data::LosslessWriteableSegyData;
use crate::write_headers::SegyHeaderToBytes;

// NB: Change this to pub(crate) later.
/// A structure that gives several different ways of looking at trace coordinates.
pub struct TraceCoordinates {
    pub idx: usize,
    pub start_byte: usize,
    pub data_start_byte: usize,
    /// NB; Overall length with all headers.
    pub byte_len: usize,
}

impl TraceCoordinates {
    fn new(idx: usize, start: usize, ds: usize, overall_len: usize) -> Self {
        Self {
            idx,
            start_byte: start,
            data_start_byte: ds,
            byte_len: overall_len,
        }
    }
}

/// A structure which represents a mapped SEG-Y file. This represents a writeable SEGY.
pub struct SegyFile<S: SegyWriteSettings> {
    pub metadata: SegyMetadata<S>,
    pub traces: Vec<Trace>,
    /// This is here to speed up the lookup of traces. NB: (start_byte,index).
    /// NB2: I suspect that the lookup will not be useful.
    pub lookup: fnv::FnvHashMap<usize, TraceCoordinates>,
    pub file: File,
}

impl<S: SegyWriteSettings> SegyFile<S> {
    /// Create a file and return the Writeable file.
    pub fn create_file<T: AsRef<Path>>(
        file_name: T,
        settings: S,
        text_header: String,
        bin_header: BinHeader,
        tape_label: Option<TapeLabel>,
    ) -> Result<Self, RsgError> {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open(file_name)?;

        if let Some(ref tl) = tape_label {
            file.write_all(&tl.as_bytes()?)?;
        }

        crate::write_headers::write_text_header(&text_header, &mut file)?;

        file.write_all(&bin_header.as_bytes()?)?;

        Ok(SegyFile {
            metadata: SegyMetadata::new(tape_label, text_header, vec![], bin_header, settings),
            traces: Vec::new(),
            /// This is here to speed up the lookup of traces. NB: ([xline,inline],index)
            lookup: fnv::FnvHashMap::default(),
            file,
        })
    }

    /// This function will add a trace to the file being written. It will try to convert
    /// The data to the desired `CoordinateFormat`, which can result in loss of precision.
    ///
    /// NB
    pub fn add_trace<T: ToPrimitive + Debug>(
        &mut self,
        trace_header: TraceHeader,
        extended_header: Option<String>,
        data: Vec<T>,
    ) -> Result<&Trace, RsgError> {
        // Get some parameters for construction of byte coordinates.
        write_trace_internal(
            self,
            trace_header,
            extended_header,
            data,
            write_data::convert_data,
        )
    }

    #[allow(unused_variables)]
    /// This function tries to guarantee that types are not converted freely,
    /// but instead uses a trait that makes sure only appropriate data types
    /// can be written. For example, data in `f64` cannot be written as `f32`,
    /// data as `i64` cannot be written as `u16`, etc.
    ///
    /// NB: For this it uses the `LosslessWriteableSegyData` trait. In theory
    /// the `LosslessWriteableSegyData` trait can be implemented for type
    /// conversions that are not lossless. The out of the box implementation
    /// should not do this however.
    pub fn add_trace_lossless<T: LosslessWriteableSegyData>(
        &mut self,
        trace_header: TraceHeader,
        extended_header: Option<String>,
        data: Vec<T>,
    ) -> Result<&Trace, RsgError> {
        // This is a sanity check to ensure we throw an error in case the
        // requested format does not support lossless writing.
        let (format, _) = write_data::get_format_and_le(self.metadata.get_bin_header());
        if !T::is_lossless_to(format) {
            return Err(RsgError::BitConversionError {
                msg: format!(
                    "Data of type '{}' cannot be written losslessly as '{:?}'",
                    core::any::type_name::<T>(),
                    format,
                ),
            });
        }
        write_trace_internal(
            self,
            trace_header,
            extended_header,
            data,
            write_data::convert_data_losslessly,
        )
    }
}

fn write_trace_internal<T, S>(
    segy: &mut SegyFile<S>,
    trace_header: TraceHeader,
    extended_header: Option<String>,
    data: Vec<T>,
    write_fn: fn(Vec<T>, &BinHeader) -> Result<Vec<u8>, RsgError>,
) -> Result<&Trace, RsgError>
where
    T: ToPrimitive + Debug,
    S: SegyWriteSettings,
{
    // Get some parameters for construction of byte coordinates.
    let idx = segy.traces.len();
    let start = segy.traces.last().map(|t| t.get_start()).unwrap_or(0);
    let length = segy.lookup.get(&start).map(|g| g.byte_len).unwrap_or(0);
    let new_start = start + length;

    // A sanity check to make sure that if we are given the wrong number of data, we return an error.
    let data_len = data.len();
    let bin_header = segy.metadata.get_bin_header();
    let bin_sample_count = bin_header.no_samples;
    let th_sample_count = trace_header.no_samples_in_trace;

    if data.len() > u16::MAX as usize {
        return Err(RsgError::LongDataVector { l_data: data.len() });
    } else if (data_len as u16 != bin_sample_count) && (data_len as u16 != th_sample_count) {
        return Err(RsgError::BadDataVector {
            l_data: data_len as u16,
            l_bin: bin_sample_count,
            l_trace: th_sample_count,
        });
    }

    let header_bytes = write_headers::th_as_bytes_with_settings(
        &trace_header,
        segy.metadata.get_settings(),
        bin_header,
    )?;
    let mut length = header_bytes.len();
    segy.file.write_all(&header_bytes)?;

    if let Some(extra) = extended_header {
        let bytes = extra.as_bytes();
        length += bytes.len();
        segy.file.write_all(bytes)?;
    }

    let data = write_fn(data, bin_header)?;
    length += data_len;
    segy.file.write_all(&data)?;

    let new_coordinates = TraceCoordinates::new(idx, new_start, data_len, length);
    segy.traces
        .push(Trace::new(trace_header, new_start, data_len));
    segy.lookup.insert(idx, new_coordinates);

    Ok(segy.traces.last().expect("Just added."))
}
