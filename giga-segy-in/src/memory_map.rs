//! This submodule exists to map the file as a "memory map" and then allow reading of the data.
use giga_segy_core::errors::*;
use giga_segy_core::{BinHeader, TapeLabel, TraceHeader};
use giga_segy_core::{SegySettings, Trace};
use giga_segy_core::{BIN_HEADER_LEN, TAPE_LABEL_LEN, TEXT_HEADER_LEN, TRACE_HEADER_LEN};

use super::convert_headers::{HeaderFromBytes, TraceHeaderFromBytes};

use encoding8::ebcdic::to_ascii;
use memmap2::{Mmap, MmapOptions};

/// This structure represents a memory map with an underlying SEG-Y file handle.
pub struct MappedSegY {
    pub(crate) map: Mmap,
    _file: std::fs::File,
}

impl MappedSegY {
    /// Create a mapped Seg-Y structure with a Memory map and underlying file handle.
    pub(crate) fn new(file_name: &str) -> Result<MappedSegY, RsgError> {
        // Map the file.
        let (map, file) = map_file_to_memory(file_name)?;

        // Perform sanity check to make sure file is big enough to be SEG-Y.
        let too_short_with_label = has_label(&map)?
            && (map.len() <= TEXT_HEADER_LEN + BIN_HEADER_LEN + TRACE_HEADER_LEN + TAPE_LABEL_LEN);

        let too_short_without_label =
            !has_label(&map)? && (map.len() <= TEXT_HEADER_LEN + BIN_HEADER_LEN + TRACE_HEADER_LEN);

        if too_short_with_label || too_short_without_label {
            return Err(RsgError::FileTooShort);
        }
        // Return Mapping.
        Ok(MappedSegY { map, _file: file })
    }

    /// Determine whether we have a label or not.
    fn has_label(&self) -> Result<bool, RsgError> {
        has_label(&self.map)
    }

    /// Get the bytes of a label.
    pub(crate) fn get_tape_label(
        &self,
        settings: &SegySettings,
    ) -> Result<Option<TapeLabel>, RsgError> {
        if self.has_label()? {
            let label = TapeLabel::from_bytes(&self.map[0..TAPE_LABEL_LEN], settings)?;
            Ok(Some(label))
        } else {
            Ok(None)
        }
    }

    /// This function gets the bytes corresponding to the text header and attempts to parse them
    /// into a string. It is OK for rust, but less helpful for C.
    pub(crate) fn get_text_header(&self) -> Result<String, RsgError> {
        let start = start_byte(&self.map, 0, TAPE_LABEL_LEN)?;

        // Convert to the right sort of encoding.
        let header_bytes = convert_bytes_to_ascii(&self.map, start, TEXT_HEADER_LEN);
        Ok(String::from_utf8_lossy(&header_bytes).to_string())
    }

    /// Attempts to get the bytes corresponding to the binary
    pub(crate) fn get_bin_header(&self, settings: &SegySettings) -> Result<BinHeader, RsgError> {
        let start = start_byte(&self.map, TEXT_HEADER_LEN, TAPE_LABEL_LEN)?;
        BinHeader::from_bytes(&self.map[start..(start + BIN_HEADER_LEN)], settings)
    }

    /// This attempts to get extended text headers. NB: Needs an input of how many headers there are.
    /// NB: We require a parsed binary header or foreknowledge of some kind which tells us just
    /// how many headers we have.
    /// NB2: It is possible, albeit unlikely, that this function will return an `Ok(stuff)` even
    /// if it goes past the end of the
    pub(crate) fn get_extended_text_headers(&self, count: u32) -> Result<Vec<String>, RsgError> {
        // Shortcut the process if we have nothing to give.
        if count == 0 {
            return Ok(Vec::with_capacity(0));
        }

        let default_start = TEXT_HEADER_LEN + BIN_HEADER_LEN;
        let start_byte = start_byte(&self.map, default_start, TAPE_LABEL_LEN)?;
        let count = count as usize;

        // A sanity check to shortcut us if the file is too short.
        if self.map.len() < start_byte + count * TEXT_HEADER_LEN {
            return Err(RsgError::SEGYTooShort);
        }

        let count = count as usize;
        let mut extra_headers = Vec::with_capacity(count);
        for i in 0..count {
            let start = start_byte + i * TEXT_HEADER_LEN;
            let header_bytes = convert_bytes_to_ascii(&self.map, start, TEXT_HEADER_LEN);
            let header = String::from_utf8_lossy(&header_bytes).to_string();
            extra_headers.push(header);
        }

        Ok(extra_headers)
    }

    /// This function retrieves the metadata for the headers, which includes the trace headers
    /// and the start and end point of each trace in the form of a `Trace` instance.
    /// The `extended_header_count` should come from the actual extended headers.
    pub(crate) fn get_metadata_for_traces(
        &self,
        bin_header: &mut BinHeader,
        extended_header_count: usize,
        settings: &SegySettings,
    ) -> Result<Vec<Trace>, RsgError> {
        // If all traces have the same length, our task is quite easy. In theory.
        let datum_size = bin_header.sample_format_code.datum_byte_length();
        let default_start = TEXT_HEADER_LEN * (extended_header_count + 1) + BIN_HEADER_LEN;
        let start_byte = start_byte(&self.map, default_start, TAPE_LABEL_LEN)?;

        let mut traces: Vec<Trace> = Vec::new();
        let mut last_header_err = None;
        let max_trace_length = settings.get_max_trace_length_by_override_dimensions();
        let max_trace_count = settings.get_max_trace_count_by_override_dimensions();
        // If the traces have the same length then this is fairly easy and we just iterate through
        // blocks.
        if bin_header.fixed_length_trace_flag.yes() {
            // Get the length of each block.
            let block_byte_length = datum_size * bin_header.no_samples as usize + TRACE_HEADER_LEN;
            let trace_byte_length = datum_size * bin_header.no_samples as usize;
            // This is needed to set a "fake" byte length used purely for reading the data,
            // if we wish to truncate all traces.
            let trace_apparent_byte_length = if let Some(l) = max_trace_length {
                datum_size * l
            } else {
                trace_byte_length
            };

            'regular: for (i, ch) in self.map[start_byte..].chunks(block_byte_length).enumerate() {
                // Preliminary san check.
                if (ch.len() < block_byte_length) || (i >= max_trace_count) {
                    break 'regular;
                }

                let start = start_byte + i * block_byte_length;
                let b_range = start..(start + TRACE_HEADER_LEN);
                // It is possible that we already have a collection of valid traces followed by
                // something else. In this case we may get an error here, instead of an "end of data"
                // clause. Thus if traces are not empty, an invalid header is interpreted as an
                // end of trace data statement. Otherwise, it'sjust an error.
                match TraceHeader::from_bytes(&self.map[b_range], bin_header, settings, i) {
                    Ok(mut t) => {
                        // If sample count is not adjusted, we will not truncate the record.
                        t.adjust_sample_count(settings);
                        // check the inline and crossline number, and if they're outside of our
                        // optional range, discard them.
                        if settings.trace_in_bounds(t.inline_no, t.crossline_no) {
                            let trace =
                                Trace::new(t, start + TRACE_HEADER_LEN, trace_apparent_byte_length);
                            traces.push(trace);
                        }
                    }
                    Err(e) if (i == 0) || last_header_err.is_some() => return Err(e),
                    Err(e) => last_header_err = Some(e),
                };
            }
        // If the trace length is variable, we must parse one header at a time, then grab some
        // bytes for the trace. Then rinse and repeat.
        } else {
            // TODO: This branch.
            let mut pointer = start_byte;
            let mut i = 0;
            'irregular: while i < max_trace_count {
                // Check that we have enough space to read header. Finish if we're too close to the end.
                // If the last header was invalid, but we still have space, we have probably got to
                // the end of the data and have some extended headers or corrupt records. Either
                // way, we're done.
                if self.map.len() < pointer + TRACE_HEADER_LEN || last_header_err.is_some() {
                    break;
                }
                // Get the trace headers
                let b_range = pointer..(pointer + TRACE_HEADER_LEN);
                match TraceHeader::from_bytes(&self.map[b_range], bin_header, settings, i) {
                    Ok(mut t) => {
                        let trace_byte_length = datum_size * t.no_samples_in_trace as usize;

                        // Sample count can be adjusted here. NB: Must be done after `trace_byte_length`
                        // is calculated or the byte lengths will be wrong.
                        t.adjust_sample_count(settings);

                        // If the file is shorter than the total block length, it means we have
                        // a good header, but an incomplete trace, so the file is corrupt. For
                        // Now this is probably best producing an error.
                        if self.map.len() < TRACE_HEADER_LEN + trace_byte_length {
                            return Err(RsgError::IncompleteTrace);
                        }
                        // check the inline and crossline number, and if they're outside of our
                        // optional range, discard them.
                        // NB: The pointer MUST be incremented, even if the trace is discarded,
                        // otherwise we would never proceed to the next trace.
                        if settings.trace_in_bounds(t.inline_no, t.crossline_no) {
                            // This makes a "fake" byte length if we are truncating all traces for
                            // reading.
                            let trace_apparent_byte_length = if let Some(l) = max_trace_length {
                                if l * datum_size < trace_byte_length {
                                    l * datum_size
                                } else {
                                    trace_byte_length
                                }
                            } else {
                                trace_byte_length
                            };
                            // make the trace.
                            let trace = Trace::new(
                                t,
                                pointer + TRACE_HEADER_LEN,
                                trace_apparent_byte_length,
                            );
                            traces.push(trace);
                        }
                        // Increment pointer.
                        pointer += TRACE_HEADER_LEN + trace_byte_length;
                        i += 1;
                    }
                    // Since length is derived from a header, if one header is invalid, then the
                    // recording is over. A "corrupt" header at this stage is not considered
                    // an error. This is because we have no way of knowing if we have got to the end
                    // of the data or not. (ie it is for the user to decide.)
                    Err(_e) => break 'irregular,
                };
            }
        }
        // The sample count in the binary header can now be adjusted.
        bin_header.adjust_sample_count(settings);
        Ok(traces)
    }
}

/// This function creates a memory map from a file.
pub(crate) fn map_file_to_memory(file_name: &str) -> Result<(Mmap, std::fs::File), RsgError> {
    let segy = std::fs::File::open(file_name).map_err(RsgError::MapFile)?;
    let map = unsafe { MmapOptions::new().map(&segy).map_err(RsgError::MapFile)? };
    Ok((map, segy))
}

/// A function to determine whether we need to ascify the text.
fn is_ascii(map: &Mmap, start: usize) -> bool {
    map[start..(start + TEXT_HEADER_LEN)]
        .iter()
        .all(|c| c.is_ascii() && !c.is_ascii_control())
}

/// A way to save LOC when getting the start byte.
fn start_byte(map: &Mmap, default: usize, extra: usize) -> Result<usize, RsgError> {
    if has_label(map)? {
        Ok(default + extra)
    } else {
        Ok(default)
    }
}

/// The inner `has_label` function which can be applied to a map before it is turned into `MappedSegY`
fn has_label(map: &Mmap) -> Result<bool, RsgError> {
    // Sanity check.
    if map.len() <= TAPE_LABEL_LEN {
        return Err(RsgError::FileTooShort);
    }

    let c1 = map[0];
    let c2 = map[TAPE_LABEL_LEN];
    let starts_with_text_header =
        (u8::from_le(c1) == b'C') || (u8::from_be(c1) == b'C') || (to_ascii(c1) == b'C');
    let text_header_at_128 =
        (u8::from_le(c2) == b'C') || (u8::from_be(c2) == b'C') || (to_ascii(c2) == b'C');

    if !starts_with_text_header && text_header_at_128 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// This is a helper function that converts a potential char vector to ASCII from EBCDIC.
/// NB: This is a copy function.
/// NB2: Bounds checking is not performed.
/// NB3: String conversion is not performed here.
fn convert_bytes_to_ascii(map: &Mmap, start: usize, len: usize) -> Vec<u8> {
    let mut header_bytes = if is_ascii(map, start) {
        map[start..(start + len)].to_vec()
    } else {
        map[start..(start + len)]
            .iter()
            .map(|c| to_ascii(*c))
            .collect::<Vec<_>>()
    };
    // Truncate to valid string.
    let i = header_bytes
        .iter()
        .position(|x| *x == 0)
        .unwrap_or(header_bytes.len());
    header_bytes.truncate(i);
    header_bytes
}
