//! This library is the foundation for the `rsg-in` and `rsg-out` crates.
extern crate num;
#[macro_use]
extern crate num_derive;
extern crate ibmfloat;

#[cfg(feature = "to_json")]
extern crate serde;
#[cfg(feature = "to_json")]
extern crate serde_json;

pub mod bitconverter;
pub mod enums;
pub mod errors;
pub mod header_structs;
pub mod settings;
#[cfg(test)]
mod tests;

pub use errors::RsgError;

pub use enums::*;
pub use header_structs::*;
pub use settings::SegySettings;

pub const TAPE_LABEL_LEN: usize = 128;
pub const TEXT_HEADER_LEN: usize = 3200;
pub const BIN_HEADER_LEN: usize = 400;
pub const TRACE_HEADER_LEN: usize = 240;
pub const INLINE_BYTE_LOCATION: usize = 188;
pub const CROSSLINE_BYTE_LOCATION: usize = 192;
pub const CDPX_BYTE_LOCATION: usize = 180;
pub const CDPY_BYTE_LOCATION: usize = 184;

/// This structure represents a SEG-Y trace. The Header is parsed and stored, the rest is stored
/// as a set of indices for reading the memory map.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Trace {
    /// A parsed trace header which contains the trace metadata.
    pub(crate) trace_header: TraceHeader,
    /// Starting byte of the trace on the map.
    pub(crate) trace_start_byte: usize,
    /// Length of the trace in bytes on the map.
    pub(crate) trace_byte_len: usize,
}

/// This structure contains all of the metadata for opening a segy file.
/// different implementations of SegyMetadata can then be used.
pub struct SegyMetadata<S> {
    pub tape_label: Option<TapeLabel>,
    pub text_header: String,
    pub extended_headers: Vec<String>,
    pub bin_header: BinHeader,
    pub settings: S,
}

impl Trace {
    /// Create a new trace from data diectly extracted from the SEG-Y file.
    pub fn new(trace_header: TraceHeader, data_start: usize, data_len: usize) -> Self {
        Trace {
            trace_header,
            trace_start_byte: data_start,
            trace_byte_len: data_len,
        }
    }

    /// Get a reference to the trace header.
    pub fn get_header(&self) -> &TraceHeader {
        &self.trace_header
    }

    /// Get the starting byte of the trace data.
    pub fn get_start(&self) -> usize {
        self.trace_start_byte
    }

    /// Get the length of the data in bytes.
    pub fn len(&self) -> usize {
        self.trace_byte_len
    }

    /// Obligatory `is_empty` method.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<S> SegyMetadata<S> {
    pub fn new(
        tape_label: Option<TapeLabel>,
        text_header: String,
        extended_headers: Vec<String>,
        bin_header: BinHeader,
        settings: S,
    ) -> Self {
        Self {
            tape_label,
            text_header,
            extended_headers,
            bin_header,
            settings,
        }
    }

    /// Get the SEGY Settings.
    pub fn get_settings(&self) -> &S {
        &self.settings
    }

    /// Get the Binary header.
    pub fn get_tape_label(&self) -> &Option<TapeLabel> {
        &self.tape_label
    }

    /// Get the text header.
    pub fn get_text_header(&self) -> &str {
        &self.text_header
    }

    /// Get the extended headers.
    pub fn extended_headers_iter(&self) -> std::slice::Iter<String> {
        self.extended_headers.iter()
    }

    /// Get the extended headers.
    pub fn get_extended_headers(&self) -> &[String] {
        &self.extended_headers
    }

    /// Get the text header as collection of short substrings.
    /// NB: This is a horrifically wasteful waste of a function.
    pub fn get_text_header_lines(&self) -> Vec<String> {
        self.text_header
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
            .chunks(80)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
    }

    /// Get the binary header.
    pub fn get_bin_header(&self) -> &BinHeader {
        &self.bin_header
    }

    /// This function gets the Tape Label in a rust compatible format.
    pub fn get_readable_tape_label(&self) -> Option<ReadableTapeLabel> {
        self.tape_label.as_ref().map(|l| l.to_readable())
    }

    /// This function gets all the fields of SegyMetadata and discards the instance. Used to get all
    /// data in an efficient manner.
    /// NB: The internal mapping is discarded in the process.
    pub fn deconstruct(self) -> (Option<TapeLabel>, String, Vec<String>, BinHeader, S) {
        let SegyMetadata {
            tape_label,
            text_header,
            extended_headers,
            bin_header,
            settings,
        } = self;
        (
            tape_label,
            text_header,
            extended_headers,
            bin_header,
            settings,
        )
    }
}