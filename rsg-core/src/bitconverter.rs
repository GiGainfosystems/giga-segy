// Copyright (C) 2020 by GiGa infosystems
//! This submodule deals with reading the actual data in the file once it has been mapped to
//! memory.
use crate::enums::SampleFormatCode;
use crate::errors::*;

use ibmfloat::F32;
use std::array::TryFromSliceError;
use std::convert::TryInto;

pub type BitConverter = fn(&[u8]) -> Result<f32, TryFromSliceError>;

/// This function chooses the converter for the binary data.
/// Doing this once per trace should be more efficient than doing it dynamically.
pub fn converter_chooser(format: SampleFormatCode, le: bool) -> Result<BitConverter, RsgError> {
    let f = match format {
        SampleFormatCode::IbmFloat32 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(f32::from(F32::from_be_bytes(input.try_into()?)))
            }
            x
        }
        SampleFormatCode::Int32 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(i32::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Int32 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(i32::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Int16 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(i16::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Int16 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(i16::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Float32 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(f32::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Float32 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(f32::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Float64 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(f64::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Float64 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(f64::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        // SampleFormatCode::Int24 => 3,
        SampleFormatCode::Int8 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(i8::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        // SampleFormatCode::Int24 => 3,
        SampleFormatCode::Int8 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(i8::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Int64 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(i64::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Int64 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(i64::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::UInt32 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(u32::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::UInt32 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(u32::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::UInt16 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(u16::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::UInt16 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(u16::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::UInt64 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(u64::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::UInt64 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(u64::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        // SampleFormatCode::UInt24 => 3,
        SampleFormatCode::UInt8 if le => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(u8::from_le_bytes(input.try_into()?) as f32)
            }
            x
        }
        // SampleFormatCode::UInt24 => 3,
        SampleFormatCode::UInt8 => {
            fn x(input: &[u8]) -> Result<f32, TryFromSliceError> {
                Ok(u8::from_be_bytes(input.try_into()?) as f32)
            }
            x
        }
        SampleFormatCode::Int24 | SampleFormatCode::UInt24 => {
            return Err(RsgError::BitConversionError {
                msg: "Parsing of 24-bit integers is not implemented.".to_string(),
            });
        }
        SampleFormatCode::FixPoint32 => {
            return Err(RsgError::BitConversionError {
                msg: "FicPoint32 are obsolete.".to_string(),
            });
        }
    };
    Ok(f)
}

/// A helper function to convert ascii null terminated to string
pub(crate) fn ascii_bytes_to_string(bytes: &[u8]) -> String {
    let mut bytes = bytes.to_vec();
    let i = bytes.iter().position(|x| *x == 0).unwrap_or(bytes.len());
    bytes.truncate(i);
    String::from_utf8_lossy(&bytes).to_string()
}
