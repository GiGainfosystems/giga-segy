//! This is a simplified library for reading SEGY files into rust.
extern crate encoding8;
extern crate memmap;
extern crate num;

pub mod convert_headers;
pub mod memory_map;
pub mod read_data;
#[cfg(test)]
mod tests;

use memory_map::MappedSegY;
use std::collections::HashMap;

pub use rsg_core::errors::*;
pub use rsg_core::header_structs::*;
pub use rsg_core::settings::SegySettings;
use rsg_core::*;

/// A structure which represents a mapped SEG-Y file. The headers are parsed, but the traces
/// themselves are stored as a header and the coordinates of the data in the memory map.
pub struct SegyFile {
    pub(crate) metadata: SegyMetadata<SegySettings>,
    pub(crate) traces: Vec<Trace>,
    /// This is here to speed up the lookup of traces. NB: ([xline,inline],index).
    pub(crate) lookup: HashMap<[i32; 2], usize>,
    data: MappedSegY,
}

impl SegyFile {
    /// Create a new instance of `SegyFile` from a mapped instance of SEG-Y data.
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

    /// Get the SEGY Settings.
    pub fn get_settings(&self) -> &SegySettings {
        self.metadata.get_settings()
    }

    /// Get the Binary header.
    pub fn get_tape_label(&self) -> &Option<TapeLabel> {
        self.metadata.get_tape_label()
    }

    /// This function gets the Tape Label in a rust compatible format.
    pub fn get_readable_tape_label(&self) -> Option<ReadableTapeLabel> {
        self.metadata.get_readable_tape_label()
    }

    /// Get the text header.
    pub fn get_text_header(&self) -> &str {
        self.metadata.get_text_header()
    }

    /// Get the extended headers.
    pub fn extended_headers_iter(&self) -> std::slice::Iter<String> {
        self.metadata.extended_headers_iter()
    }

    /// Get the extended headers.
    pub fn get_extended_headers(&self) -> &[String] {
        self.metadata.get_extended_headers()
    }

    /// Get the text header as collection of short substrings.
    /// NB: This is a horrifically wasteful waste of a function.
    pub fn get_text_header_lines(&self) -> Vec<String> {
        self.metadata.get_text_header_lines()
    }

    /// Get the binary header.
    pub fn get_bin_header(&self) -> &BinHeader {
        self.metadata.get_bin_header()
    }

    /// Get the number of parsed traces.
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }

    /// Get the reference to a certain trace.
    pub fn get_trace(&self, i: usize) -> Option<&Trace> {
        self.traces.get(i)
    }

    /// Get the reference to a certain trace.
    pub fn get_trace_by_xline_inline(&self, xline: i32, inline: i32) -> Option<&Trace> {
        if let Some(index) = self.lookup.get(&[xline, inline]) {
            self.get_trace(*index)
        } else {
            None
        }
    }

    /// Get the reference to a certain trace.
    pub fn get_trace_data_as_f32(&self, i: usize) -> Result<Vec<f32>, RsgError> {
        let trace = &self.get_trace(i).ok_or(RsgError::TraceNotFound { i })?;

        crate::read_data::get_trace_data_as_f32(
            &self.data,
            trace,
            self.get_bin_header(),
            self.get_settings(),
        )
    }

    /// Get the reference to a certain trace.
    pub fn get_trace_data_as_bytes(&self, i: usize) -> Result<Vec<u8>, RsgError> {
        let trace = &self.get_trace(i).ok_or(RsgError::TraceNotFound { i })?;

        crate::read_data::get_trace_data_as_bytes_unprocessed(
            &self.data,
            trace,
            self.get_bin_header(),
            self.get_settings(),
        )
    }

    /// Get the reference to a certain trace.
    pub fn get_trace_data_as_f32_from_trace(&self, t: &Trace) -> Result<Vec<f32>, RsgError> {
        crate::read_data::get_trace_data_as_f32(
            &self.data,
            t,
            self.get_bin_header(),
            self.get_settings(),
        )
    }

    /// Get the reference to a certain trace.
    pub fn get_trace_data_as_bytes_from_trace(&self, t: &Trace) -> Result<Vec<u8>, RsgError> {
        crate::read_data::get_trace_data_as_bytes_unprocessed(
            &self.data,
            t,
            self.get_bin_header(),
            self.get_settings(),
        )
    }

    /// This function tries to get a data point at a particular index in the trace recording
    /// from a particular trace. Returns it as an unprocessed byte slice.
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
    /// from a particular trace. Returns it as an unprocessed byte slice.
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

    /// Iterate through the traces
    pub fn traces_iter(&self) -> std::slice::Iter<Trace> {
        self.traces.iter()
    }

    /// Get the minimum and maximum crossline number from trace headers
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

    /// Get the minimum and maximum inline number from trace headers
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

    /// Get the minimum and maximum crossline number from trace headers
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

    /// Get the minimum and maximum crossline number from trace headers
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

    /// This function gets all the fields of SegyFile and discards the instance. Used to get all
    /// data in an efficient manner.
    /// NB: The internal mapping is discarded in the process.
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
