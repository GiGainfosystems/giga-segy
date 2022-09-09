// Copyright (C) 2020 by GiGa infosystems
//! This submodule deals with reading the actual data in the file once it has been mapped to
//! memory.
use rsg_core::bitconverter::converter_chooser;
use rsg_core::errors::*;
use rsg_core::header_structs::BinHeader;
use rsg_core::settings::SegySettings;
use rsg_core::Trace;

use crate::memory_map::MappedSegY;

/// A function to get the bytes of a SEG-Y data trace.
/// NB: This function does not process the data. It only performs a few sanity checks.
pub(crate) fn get_trace_data_reference<'a>(
    segy: &'a MappedSegY,
    trace: &Trace,
) -> Result<&'a [u8], RsgError> {
    let len = trace.len();
    let start = trace.get_start();

    // Sanity check.
    if segy.map.len() < len + start {
        return Err(RsgError::ShortSEGY {
            a: segy.map.len(),
            b: len + start,
        });
    }

    // Return the bytes as they are.
    Ok(&segy.map[start..(start + len)])
}

/// A function to get the bytes of sub-slice SEG-Y data trace.
/// NB: This function does not process the data.
/// NB2: This function does not do san-checks. You have been warned.
pub(crate) fn get_trace_data_slice_reference(
    segy: &MappedSegY,
    range: std::ops::Range<usize>,
) -> &[u8] {
    // Return the bytes as they are.
    &segy.map[range]
}

/// A function to get the bytes of a SEG-Y data trace.
/// NB: This function does not process the data. It only performs a few sanity checks.
pub(crate) fn get_trace_data_as_bytes_unprocessed(
    segy: &MappedSegY,
    trace: &Trace,
    bin_header: &BinHeader,
    settings: &SegySettings,
) -> Result<Vec<u8>, RsgError> {
    // Get the slice.
    let data = get_trace_data_reference(segy, trace)?;

    // If we are not skipping values, return the whole trace.
    if settings.get_step_by() == 1 {
        Ok(data.to_vec())
    // If we are skipping values, return only the bytes which correspond to some kind of values.
    } else {
        let datum_byte_length = if let Some(f) = settings.get_override_trace_format() {
            f.datum_byte_length()
        } else {
            bin_header.sample_format_code.datum_byte_length()
        };
        // Break data into chunks the size of the data type and then stitch together only those bits.
        let res = data
            .chunks(datum_byte_length)
            .step_by(settings.get_step_by())
            .flatten()
            .copied()
            .collect::<Vec<u8>>();
        Ok(res)
    }
}

/// This function takes the SEGY memory map and the processed metadata and returns a vector of
/// f32 result data.
pub(crate) fn get_trace_data_as_f32(
    segy: &MappedSegY,
    trace: &Trace,
    bin_header: &BinHeader,
    settings: &SegySettings,
) -> Result<Vec<f32>, RsgError> {
    // Format and byte length must be checked against overrides in the setting.
    let format = if let Some(f) = settings.get_override_trace_format() {
        f
    } else {
        bin_header.sample_format_code
    };
    let raw_data = get_trace_data_reference(segy, trace)?;

    let datum_byte_length = format.datum_byte_length();
    if raw_data.len() % datum_byte_length != 0 {
        return Err(RsgError::TraceDivisibility {
            a: raw_data.len(),
            b: datum_byte_length,
            format,
        });
    }

    // Determine whether we are dealing with LE or BE, checking for override in the `SegySettings`.
    let le = if let Some(le) = settings.get_override_to_le() {
        le
    } else {
        bin_header.binary_flag_direction_is_le
    };
    // Step size. The default is 1.
    let s = settings.get_step_by();

    // Allocate result vecor.
    let mut data = Vec::with_capacity(raw_data.len() / datum_byte_length / s);
    let converter = converter_chooser(bin_header.sample_format_code, le)?;

    for slice in raw_data.chunks(datum_byte_length).step_by(s) {
        data.push(converter(slice).map_err(RsgError::TryFromSlice)?)
    }
    Ok(data)
}

/// A function to get the bytes of a SEG-Y data trace.
/// NB: This function does not process the data. It only performs a few sanity checks.
pub(crate) fn get_trace_data_point_as_bytes_unprocessed(
    segy: &MappedSegY,
    trace: &Trace,
    bin_header: &BinHeader,
    settings: &SegySettings,
    idx: usize,
) -> Result<Vec<u8>, RsgError> {
    let datum_byte_length = if let Some(f) = settings.get_override_trace_format() {
        f.datum_byte_length()
    } else {
        bin_header.sample_format_code.datum_byte_length()
    };

    let first_byte = trace.get_start() + idx * datum_byte_length;
    let last_byte = first_byte + datum_byte_length;

    if (last_byte > trace.get_start() + trace.len()) || (last_byte > segy.map.len()) {
        return Err(RsgError::TracePointOutOfBounds { idx });
    }
    let range = first_byte..last_byte;
    let data = get_trace_data_slice_reference(segy, range).to_vec();
    Ok(data)
}

/// This function takes the SEGY memory map and the processed metadata and returns a vector of
/// f32 result data.
pub(crate) fn get_trace_data_point_as_f32(
    segy: &MappedSegY,
    trace: &Trace,
    bin_header: &BinHeader,
    settings: &SegySettings,
    idx: usize,
) -> Result<f32, RsgError> {
    // Determine byte length of a data point.
    let datum_byte_length = if let Some(f) = settings.get_override_trace_format() {
        f.datum_byte_length()
    } else {
        bin_header.sample_format_code.datum_byte_length()
    };

    // determine byte range of the data point.
    let first_byte = trace.get_start() + idx * datum_byte_length;
    let last_byte = first_byte + datum_byte_length;

    // san check.
    if (last_byte > trace.get_start() + trace.len()) || (last_byte > segy.map.len()) {
        return Err(RsgError::TracePointOutOfBounds { idx });
    }
    // Build range.
    let range = first_byte..last_byte;

    // Determine whether we are dealing with LE or BE, checking for override in the `SegySettings`.
    let le = if let Some(le) = settings.get_override_to_le() {
        le
    } else {
        bin_header.binary_flag_direction_is_le
    };

    let converter = converter_chooser(bin_header.sample_format_code, le)?;
    converter(get_trace_data_slice_reference(segy, range)).map_err(RsgError::TryFromSlice)
}
