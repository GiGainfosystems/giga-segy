//! This is a simplified library for reading SEGY files into rust. It is designed for efficient
//! reading of SEG-Y headers and data without holding potentially very large SEG-Y files in memory.
//!
//! The library was designed to follow the SEG Technial Standards Committee's
//! SEG-Y_r2.0 standard (from January 2017).
//!
//! This library is not designed for editing of SEG-Y files, although it can theoretically be accomplished
//! with the clever use of `giga_segy_in` and `giga_segy_out`, we do not recommend this.
extern crate encoding8;
extern crate memmap2;
extern crate num;

pub mod convert_headers;
pub mod memory_map;
pub mod read_data;
#[cfg(test)]
mod tests;

use memory_map::MappedSegY;
use std::collections::HashMap;

pub use giga_segy_core::enums;
pub use giga_segy_core::errors::*;
pub use giga_segy_core::header_structs::*;
pub use giga_segy_core::{SegyMetadata, SegySettings, Trace};

use giga_segy_core::*;

/// A structure which represents a mapped SEG-Y file.
///
/// The structure contains:
///
/// - The memory map of the SEG-Y file.
///
/// - The metadata associated with the settings needed to open the file.
///
/// - A list of traces (consisting of the headers and "coordinates" that are used to access data) 
///
/// - A lookup that speeds up access to individual traces.
///
/// NB: The trace headers are parsed, but the data is contained in the memory map.
pub struct SegyFile {
    pub(crate) metadata: SegyMetadata<SegySettings>,
    pub(crate) traces: Vec<Trace>,
    /// This is here to speed up the lookup of traces. The crossline and inline
    /// numbers act as the key, and the index of the trace is retrieved.
    pub(crate) lookup: HashMap<[i32; 2], usize>,
    data: MappedSegY,
}

impl SegyFile {
    /// Attempts to open a SEG-Y file stored in a given location, with a given set of settings.
    /// This creates an instance of [`SegyFile`] which can then be used as a handle to get headers
    /// and data.
    ///
    /// ```
    /// use std::env::var;
    /// use std::path::PathBuf;
    /// use giga_segy_in::SegyFile;
    /// 
    /// let mut root = var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    /// root.pop();
    /// let name = root.join("testdata").join("DutchMiniHead.sgy");
    /// let file = SegyFile::open(name.to_str().unwrap(), Default::default());
    /// assert!(file.is_ok());
    /// ```
    pub fn open(file_name: &str, settings: SegySettings) -> Result<Self, RsgError> {
        let data = MappedSegY::new(file_name)?;
        let tape_label = data.get_tape_label(&settings)?;
        let text_header = data.get_text_header()?;
        let mut bin_header = data.get_bin_header(&settings)?;
        let extended_headers = data.get_extended_text_headers(bin_header.extended_header_count)?;
        let traces =
            data.get_metadata_for_traces(&mut bin_header, extended_headers.len(), &settings)?;
        let lookup = traces
            .iter()
            .enumerate()
            .map(|(i, trace)| {
                let header = trace.get_header();
                ([header.crossline_no, header.inline_no], i)
            })
            .collect::<HashMap<[i32; 2], usize>>();

        let metadata = SegyMetadata::new(
            tape_label,
            text_header,
            extended_headers,
            bin_header,
            settings,
        );

        let file = SegyFile {
            metadata,
            traces,
            lookup,
            data,
        };
        Ok(file)
    }
    
    /// Get a reference to the [`SegySettings`] which were used when opening the file in this
    /// instance of [`SegyFile`].
    ///
    /// ```
    /// use std::env::var;
    /// use std::path::PathBuf;
    /// use giga_segy_in::SegyFile;
    /// 
    /// let mut root = var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    /// root.pop();
    /// let name = root.join("testdata").join("DutchMiniHead.sgy");
    ///
    /// let file = SegyFile::open(name.to_str().unwrap(), Default::default()).unwrap();
    ///
    /// let retrieved_settings = file.get_settings();
    /// assert_eq!(retrieved_settings, &Default::default());
    /// ```
    pub fn get_settings(&self) -> &SegySettings {
        self.metadata.get_settings()
    }

    /// Get a reference to the tape label from file if it has one.
    ///
    /// ```
    /// use std::env::var;
    /// use std::path::PathBuf;
    /// use giga_segy_in::SegyFile;
    /// 
    /// let mut root = var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    /// root.pop();
    /// let name = root.join("testdata").join("DutchMiniHead.sgy");
    ///
    /// let file = SegyFile::open(name.to_str().unwrap(), Default::default()).unwrap();
    ///
    /// let retrieved_settings = file.get_tape_label();
    /// assert_eq!(retrieved_settings, &None);
    /// ```
    pub fn get_tape_label(&self) -> &Option<TapeLabel> {
        self.metadata.get_tape_label()
    }

    /// This function gets the Tape Label in a rust compatible format if a tape label is present
    /// on the geometry.
    pub fn get_readable_tape_label(&self) -> Option<ReadableTapeLabel> {
        self.metadata.get_readable_tape_label()
    }

    /// Get the text header of the file:
    ///
    /// ```
    /// use std::env::var;
    /// use std::path::PathBuf;
    /// use giga_segy_in::SegyFile;
    /// 
    /// let mut root = var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    /// root.pop();
    /// let name = root.join("testdata").join("DutchMiniHead.sgy");
    ///
    /// let file = SegyFile::open(name.to_str().unwrap(), Default::default()).unwrap();
    ///
    /// let text_header: &str = file.get_text_header();
    /// // SEG-Y text headers should always have 3200 character.
    /// assert_eq!(text_header.len(), 3200);
    /// assert_eq!(&text_header[0..14], "C 1 EPSG:31469");
    /// assert_eq!(&text_header[80..112], "C 2 Geometry name: DutchMiniHead");
    /// ```
    pub fn get_text_header(&self) -> &str {
        self.metadata.get_text_header()
    }

    /// Get an iterator over the extended headers.
    pub fn extended_headers_iter(&self) -> std::slice::Iter<String> {
        self.metadata.extended_headers_iter()
    }

    /// Get a reference to the extended headers.
    pub fn get_extended_headers(&self) -> &[String] {
        self.metadata.get_extended_headers()
    }

    /// Get the text header as a collection of short substrings.
    ///
    /// NB: This function splits the header into 80 character substrings
    /// which are then copied to owned strings. If the highest performance is
    /// required, [`Self::get_text_header`] is recommended instead.
    ///
    /// ```
    /// use std::env::var;
    /// use std::path::PathBuf;
    /// use giga_segy_in::SegyFile;
    /// 
    /// let mut root = var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    /// root.pop();
    /// let name = root.join("testdata").join("DutchMiniHead.sgy");
    ///
    /// let file = SegyFile::open(name.to_str().unwrap(), Default::default()).unwrap();
    ///
    /// let text_headers = file.get_text_header_lines();
    /// // SEG-Y text headers should always have 40 lines.
    /// assert_eq!(text_headers.len(), 40);
    /// assert_eq!(text_headers[39].len(), 80);
    /// assert_eq!(&text_headers[0][0..14], "C 1 EPSG:31469");
    /// assert_eq!(&text_headers[1][0..32], "C 2 Geometry name: DutchMiniHead");
    /// ```
    pub fn get_text_header_lines(&self) -> Vec<String> {
        self.metadata.get_text_header_lines()
    }

    /// Get the binary header.
    pub fn get_bin_header(&self) -> &BinHeader {
        self.metadata.get_bin_header()
    }

    /// Get the number of readable traces in the SEG-Y file.
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }

    /// Get the reference to a certain trace by the order in which it appears in the
    /// SEG-Y file. An out of bounds index returns a `None`.
    pub fn get_trace(&self, i: usize) -> Option<&Trace> {
        self.traces.get(i)
    }

    /// Get the reference to a certain trace by inline and crossline number using the lookup.
    /// This function should be used rather than iterating over traces retrieved by [`Self::get_trace`].
    pub fn get_trace_by_xline_inline(&self, xline: i32, inline: i32) -> Option<&Trace> {
        if let Some(index) = self.lookup.get(&[xline, inline]) {
            self.get_trace(*index)
        } else {
            None
        }
    }

    /// Gets the trace data for a trace with a given index as a `Vec<f32>`. In this case, if
    /// the index is out of bounds, an error is returned to reflect that there was a failure
    /// to retrieve the data. If the data is not in an `f32` format, it is converted to `f32`
    /// with potential loss of precision.
    ///
    /// ```
    /// use std::env::var;
    /// use std::path::PathBuf;
    /// use giga_segy_in::SegyFile;
    /// use giga_segy_core::errors::RsgError;
    /// 
    /// let mut root = var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    /// root.pop();
    /// let name = root.join("testdata").join("DutchMiniHead.sgy");
    ///
    /// let file = SegyFile::open(name.to_str().unwrap(), Default::default()).unwrap();
    ///
    /// let expected_sample_count = file.get_bin_header().no_samples as usize;
    /// let data_vec_4 = file.get_trace_data_as_f32(3).unwrap();
    /// assert_eq!(data_vec_4.len(), expected_sample_count);
    ///
    /// let no_this_fails = file.get_trace_data_as_f32(999_999_999).unwrap_err();
    /// assert!(matches!(no_this_fails, RsgError::TraceNotFound { i } if i == 999_999_999));
    /// ```
    pub fn get_trace_data_as_f32(&self, i: usize) -> Result<Vec<f32>, RsgError> {
        let trace = &self.get_trace(i).ok_or(RsgError::TraceNotFound { i })?;

        crate::read_data::get_trace_data_as_f32(
            &self.data,
            trace,
            self.get_bin_header(),
            self.get_settings(),
        )
    }

    /// Get the trace for a given index as a `Vec<u8>`. This is useful if there
    /// is concern for precision loss, or the file contains an unusual data type (eg paired data).
    ///
    /// As with [`Self::get_trace_data_as_f32`], if the trace is not found, an error is returned.
    pub fn get_trace_data_as_bytes(&self, i: usize) -> Result<Vec<u8>, RsgError> {
        let trace = &self.get_trace(i).ok_or(RsgError::TraceNotFound { i })?;

        crate::read_data::get_trace_data_as_bytes_unprocessed(
            &self.data,
            trace,
            self.get_bin_header(),
            self.get_settings(),
        )
    }

    /// Retrives the trace data for a given [`Trace`] from the same [`SegyFile`] as a `Vec<f32>`.
    /// If one is already holding a reference to a trace, this function should be preferred
    /// over [`Self::get_trace_data_as_f32`].
    pub fn get_trace_data_as_f32_from_trace(&self, t: &Trace) -> Result<Vec<f32>, RsgError> {
        crate::read_data::get_trace_data_as_f32(
            &self.data,
            t,
            self.get_bin_header(),
            self.get_settings(),
        )
    }

    /// Retrives the trace data for a given [`Trace`] from the same [`SegyFile`] as a `Vec<u8>`.
    /// If one is already holding a reference to a trace, this function should be preferred
    /// over [`Self::get_trace_data_as_bytes`].
    pub fn get_trace_data_as_bytes_from_trace(&self, t: &Trace) -> Result<Vec<u8>, RsgError> {
        crate::read_data::get_trace_data_as_bytes_unprocessed(
            &self.data,
            t,
            self.get_bin_header(),
            self.get_settings(),
        )
    }

    /// This function tries to get a data point at a particular index in the trace recording
    /// from a particular trace. Returns it as an unprocessed byte slice as an owned `Vec<u8>`.
    /// This function is less efficient than [`Self::get_trace_data_as_bytes_from_trace`], so if multiple
    /// data points are needed, then usually that function should be preferred.  
    pub fn get_trace_data_point_as_bytes_from_trace(
        &self,
        t: &Trace,
        idx: usize,
    ) -> Result<Vec<u8>, RsgError> {
        crate::read_data::get_trace_data_point_as_bytes_unprocessed(
            &self.data,
            t,
            self.get_bin_header(),
            self.get_settings(),
            idx,
        )
    }

    /// This function tries to get a data point at a particular index in the trace recording
    /// from a particular trace, returning it as a `f32` value.
    /// This function is less efficient than [`Self::get_trace_data_as_f32_from_trace`], so if multiple
    /// data points are needed, then usually that function should be preferred. 
    /// ```
    /// use std::env::var;
    /// use std::path::PathBuf;
    /// use giga_segy_in::SegyFile;
    /// use giga_segy_core::errors::RsgError;
    /// 
    /// let mut root = var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    /// root.pop();
    /// let name = root.join("testdata").join("DutchMiniHead.sgy");
    ///
    /// let file = SegyFile::open(name.to_str().unwrap(), Default::default()).unwrap();
    ///
    /// let third_trace = file.get_trace(3).unwrap();
    /// let data_vec_4 = file.get_trace_data_as_f32(3).unwrap();
    /// let third_value = file.get_trace_data_point_as_f32_from_trace(third_trace, 2).unwrap();
    /// assert_eq!(third_value, data_vec_4[2]);
    ///
    /// let no_this_fails = file.get_trace_data_point_as_f32_from_trace(third_trace, 999_999_999);
    /// assert!(matches!(
    ///     no_this_fails.unwrap_err(),
    ///     RsgError::TracePointOutOfBounds { idx } if idx == 999_999_999
    /// ));
    /// ```
    pub fn get_trace_data_point_as_f32_from_trace(
        &self,
        t: &Trace,
        idx: usize,
    ) -> Result<f32, RsgError> {
        crate::read_data::get_trace_data_point_as_f32(
            &self.data,
            t,
            self.get_bin_header(),
            self.get_settings(),
            idx,
        )
    }

    /// Iterate through the traces.
    pub fn traces_iter(&self) -> std::slice::Iter<Trace> {
        self.traces.iter()
    }

    /// Get the indices for the traces with the minimum and maximum values for the crossline
    /// number. The traces can then be retrieved with [`Self::get_trace`].
    /// ```
    /// use std::env::var;
    /// use std::path::PathBuf;
    /// use giga_segy_in::SegyFile;
    /// 
    /// let mut root = var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    /// root.pop();
    /// let name = root.join("testdata").join("DutchMiniHead.sgy");
    ///
    /// let file = SegyFile::open(name.to_str().unwrap(), Default::default()).unwrap();
    ///
    /// let minmax = file.get_trace_idx_for_crossline_min_max().unwrap();
    /// let min_trace = file.get_trace(minmax[0]).unwrap();
    /// let max_trace = file.get_trace(minmax[1]).unwrap();
    /// let xline_min = min_trace.get_header().crossline_no;
    /// let xline_max = max_trace.get_header().crossline_no;
    /// assert!(xline_max > xline_min);
    /// ```
    pub fn get_trace_idx_for_crossline_min_max(&self) -> Option<[usize; 2]> {
        let min = self
            .lookup
            .iter()
            .min_by_key(|(t, _i)| t[0])
            .map(|(_t, i)| i);
        let max = self
            .lookup
            .iter()
            .max_by_key(|(t, _i)| t[0])
            .map(|(_t, i)| i);
        match (min, max) {
            (Some(min), Some(max)) => Some([*min, *max]),
            _ => None,
        }
    }

    /// Get the indices for the traces with the minimum and maximum values for the inline
    /// number. The traces can then be retrieved with [`Self::get_trace`].
    pub fn get_trace_idx_for_inline_min_max(&self) -> Option<[usize; 2]> {
        let min = self
            .lookup
            .iter()
            .min_by_key(|(t, _i)| t[1])
            .map(|(_t, i)| i);
        let max = self
            .lookup
            .iter()
            .max_by_key(|(t, _i)| t[1])
            .map(|(_t, i)| i);
        match (min, max) {
            (Some(min), Some(max)) => Some([*min, *max]),
            _ => None,
        }
    }


    /// Get the indices for the traces with the minimum and maximum values for the x ensemble
    /// number. The traces can then be retrieved with [`Self::get_trace`].
    pub fn get_trace_idx_for_x_ensemble_min_max(&self) -> Option<[usize; 2]> {
        let min = self
            .traces
            .iter()
            .enumerate()
            .min_by_key(|(_i, t)| t.get_header().x_ensemble)
            .map(|(i, _t)| i);
        let max = self
            .traces
            .iter()
            .enumerate()
            .max_by_key(|(_i, t)| t.get_header().x_ensemble)
            .map(|(i, _t)| i);
        match (min, max) {
            (Some(min), Some(max)) => Some([min, max]),
            _ => None,
        }
    }


    /// Get the indices for the traces with the minimum and maximum values for the y ensemble
    /// number. The traces can then be retrieved with [`Self::get_trace`].
    pub fn get_trace_idx_for_y_ensemble_min_max(&self) -> Option<[usize; 2]> {
        let min = self
            .traces
            .iter()
            .enumerate()
            .min_by_key(|(_i, t)| t.get_header().y_ensemble)
            .map(|(i, _t)| i);
        let max = self
            .traces
            .iter()
            .enumerate()
            .max_by_key(|(_i, t)| t.get_header().y_ensemble)
            .map(|(i, _t)| i);
        match (min, max) {
            (Some(min), Some(max)) => Some([min, max]),
            _ => None,
        }
    }

    /// This function consumes the instance of [`SegyFile`] returning all
    /// metadata and header data in an efficient manner.
    ///
    /// NB: The internal mapping is discarded in the process, so once this function is called,
    /// internal trace data can no longer be accessed.
    pub fn deconstruct(
        self,
    ) -> (
        Option<TapeLabel>,
        String,
        Vec<String>,
        BinHeader,
        Vec<Trace>,
    ) {
        let SegyFile {
            metadata, traces, ..
        } = self;
        let (tape_label, text_header, extended_headers, bin_header, _) = metadata.deconstruct();
        (
            tape_label,
            text_header,
            extended_headers,
            bin_header,
            traces,
        )
    }
}
