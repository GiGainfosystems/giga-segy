// Copyright (C) 2022 by GiGa infosystems
//! This contains the code for writing the data to the file.
use giga_segy_core::enums::SampleFormatCode;
use giga_segy_core::enums::SampleFormatCode::*;
use giga_segy_core::errors::*;
use giga_segy_core::BinHeader;
use num::ToPrimitive;
use std::fmt::Debug;
use std::mem;
use tinyvec::TinyVec;

/// This exists to save us a lot of typing.
pub(crate) type TVu8 = TinyVec<[u8; 8]>;

/// This trait allows a new data type to be added to `LosslessWriteableSegyData` and may be useful
/// for those who extensively use exotic data types.
/// ```
/// # use giga_segy_out::write_data::LosslessWriteableSegyData;
/// # use giga_segy_core::enums::SampleFormatCode;
/// # use num::ToPrimitive;
///
/// #[derive(Debug)]
/// /// This is my super exotic magical data type.
/// pub struct MagicalWrappedF64(f64);
///
/// impl ToPrimitive for MagicalWrappedF64 {
///     fn to_i64(&self) -> Option<i64> { self.0.to_i64() }
///     fn to_u64(&self) -> Option<u64> { self.0.to_u64() }
///     fn to_f64(&self) -> Option<f64> { Some(self.0) }
/// }
///
/// impl LosslessWriteableSegyData for MagicalWrappedF64 {
///     fn is_lossless_to(f: SampleFormatCode) -> bool {
///         matches!(f, SampleFormatCode::Float64)
///     }
/// }
/// ```
pub trait LosslessWriteableSegyData: ToPrimitive + Debug {
    /// This function exists to check whether the given type is compatible with lossless
    /// conversion with a given format. It is used as a validation check in `SegyFile::add_trace_lossless`,
    /// but it is encouraged to use it as a sanity check when writing the SEGY writer.
    /// ```
    /// # use giga_segy_out::write_data::LosslessWriteableSegyData;
    /// # use giga_segy_core::enums::SampleFormatCode;
    ///
    /// assert!(f64::is_lossless_to(SampleFormatCode::Float64));
    /// assert_eq!(f64::is_lossless_to(SampleFormatCode::Float32), false);
    /// assert!(f32::is_lossless_to(SampleFormatCode::Float32));
    /// assert_eq!(f64::is_lossless_to(SampleFormatCode::Int64), false);
    /// assert_eq!(f64::is_lossless_to(SampleFormatCode::UInt64), false);
    /// assert!(i64::is_lossless_to(SampleFormatCode::Int64));
    /// assert!(u64::is_lossless_to(SampleFormatCode::UInt64));
    /// // etc.
    /// ```
    fn is_lossless_to(f: SampleFormatCode) -> bool;
}

/// Very much like `rust_segy_input::BitConverter`, but in reverse.
pub(crate) type BitConverter<T> = fn(T) -> Result<TVu8, RsgError>;

macro_rules! make_converter {
    ($to_number:expr, $to_bytes:expr) => {{
        fn x<T: ToPrimitive + Debug>(x: T) -> Result<TVu8, RsgError> {
            let x = $to_number(&x).ok_or_else(|| RsgError::BitConversionError {
                msg: format!("Cannot convert {:?} to bytes", x),
            })?;
            Ok(TinyVec::from(&$to_bytes(x)[..]))
        }
        x
    }};
}

pub(crate) fn get_format_and_le(bh: &BinHeader) -> (SampleFormatCode, bool) {
    (bh.sample_format_code, bh.binary_flag_direction_is_le)
}

fn convert_data_inner<T: ToPrimitive + Debug>(
    data: Vec<T>,
    coord_format: SampleFormatCode,
    le: bool,
) -> Result<Vec<u8>, RsgError> {
    let converter = converter_chooser(coord_format, le)?;

    let mut output = Vec::with_capacity(data.len() * mem::size_of::<T>());
    for v in data.into_iter().map(converter) {
        output.extend_from_slice(v?.as_ref());
    }
    Ok(output)
}

pub(crate) fn convert_data<T: ToPrimitive + Debug>(
    data: Vec<T>,
    bin_header: &BinHeader,
) -> Result<Vec<u8>, RsgError> {
    let (coord_format, le) = get_format_and_le(bin_header);
    convert_data_inner(data, coord_format, le)
}

/// This function saves us a lot of code lines, as it is basically the same as
/// `convert_data`, but with an extra check for compatibility.
///
pub(crate) fn convert_data_losslessly<T>(
    data: Vec<T>,
    bin_header: &BinHeader,
) -> Result<Vec<u8>, RsgError>
where
    T: LosslessWriteableSegyData + ToPrimitive + Debug,
{
    let (format, le) = get_format_and_le(bin_header);
    match T::is_lossless_to(format) {
        true => convert_data_inner(data, format, le),
        false => Err(RsgError::BitConversionError {
            msg: format!(
                "Data of type '{}' cannot be written losslessly as '{:?}'",
                std::any::type_name::<T>(),
                format,
            ),
        }),
    }
}

/// This function chooses the converter for the binary data.
/// Doing this once per trace should be more efficient than doing it dynamically.
pub(crate) fn converter_chooser<T: ToPrimitive + Debug>(
    f: SampleFormatCode,
    le: bool,
) -> Result<BitConverter<T>, RsgError> {
    let f = match f {
        SampleFormatCode::Int32 if le => make_converter!(ToPrimitive::to_i32, i32::to_le_bytes),
        SampleFormatCode::Int32 => make_converter!(ToPrimitive::to_i32, i32::to_be_bytes),
        SampleFormatCode::Int16 if le => make_converter!(ToPrimitive::to_i16, i16::to_le_bytes),
        SampleFormatCode::Int16 => make_converter!(ToPrimitive::to_i16, i16::to_be_bytes),
        SampleFormatCode::Float32 if le => make_converter!(ToPrimitive::to_f32, f32::to_le_bytes),
        SampleFormatCode::Float32 => make_converter!(ToPrimitive::to_f32, f32::to_be_bytes),
        SampleFormatCode::Float64 if le => make_converter!(ToPrimitive::to_f64, f64::to_le_bytes),
        SampleFormatCode::Float64 => make_converter!(ToPrimitive::to_f64, f64::to_be_bytes),
        // SampleFormatCode::Int24 => 3,
        SampleFormatCode::Int8 if le => make_converter!(ToPrimitive::to_i8, i8::to_le_bytes),
        // SampleFormatCode::Int24 => 3,
        SampleFormatCode::Int8 => make_converter!(ToPrimitive::to_i8, i8::to_be_bytes),
        SampleFormatCode::Int64 if le => make_converter!(ToPrimitive::to_i64, i64::to_le_bytes),
        SampleFormatCode::Int64 => make_converter!(ToPrimitive::to_i64, i64::to_be_bytes),
        SampleFormatCode::UInt32 if le => make_converter!(ToPrimitive::to_u32, u32::to_le_bytes),
        SampleFormatCode::UInt32 => make_converter!(ToPrimitive::to_u32, u32::to_be_bytes),
        SampleFormatCode::UInt16 if le => make_converter!(ToPrimitive::to_u16, u16::to_le_bytes),
        SampleFormatCode::UInt16 => make_converter!(ToPrimitive::to_u16, u16::to_be_bytes),
        SampleFormatCode::UInt64 if le => make_converter!(ToPrimitive::to_u64, u64::to_le_bytes),
        SampleFormatCode::UInt64 => make_converter!(ToPrimitive::to_u64, u64::to_be_bytes),
        // SampleFormatCode::UInt24 => 3,
        SampleFormatCode::UInt8 if le => make_converter!(ToPrimitive::to_u8, u8::to_le_bytes),
        // SampleFormatCode::UInt24 => 3,
        SampleFormatCode::UInt8 => make_converter!(ToPrimitive::to_u8, u8::to_be_bytes),
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
        SampleFormatCode::IbmFloat32 => {
            return Err(RsgError::BitConversionError {
                msg: "IbmFloats cannot be written from IEEE values.".to_string(),
            });
        }
    };

    Ok(f)
}

impl LosslessWriteableSegyData for f32 {
    /// Returns true for `Float64` and `Float32`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, Float32 | Float64)
    }
}

impl LosslessWriteableSegyData for f64 {
    /// Returns true for `Float64`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, Float64)
    }
}

impl LosslessWriteableSegyData for i64 {
    /// Returns true for `Int64`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, Int64)
    }
}

impl LosslessWriteableSegyData for i32 {
    /// Returns true for `Int64`, `Int32`, and `Float64`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, Int64 | Int32 | Float64)
    }
}

impl LosslessWriteableSegyData for i16 {
    /// Returns true for `Int64`, `Int32`, `Int16`, `Float64` and `Float32`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, Int64 | Int32 | Int16 | Float64 | Float32)
    }
}

impl LosslessWriteableSegyData for i8 {
    /// Returns true for `Int64`, `Int32`, `Int16`, `Int8`, `Float64` and `Float32`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, Int64 | Int32 | Int16 | Int8 | Float64 | Float32)
    }
}

impl LosslessWriteableSegyData for u64 {
    /// Returns true for `UInt64`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, UInt64)
    }
}

impl LosslessWriteableSegyData for u32 {
    /// Returns true for `UInt64`, `UInt32` and `Int64`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, UInt32 | UInt64 | Int64)
    }
}

impl LosslessWriteableSegyData for u16 {
    /// Returns true for `UInt64`, `UInt32`, `UInt16`, `Int64` and `Float64`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(f, UInt16 | UInt32 | UInt64 | Int32 | Int64 | Float64)
    }
}

impl LosslessWriteableSegyData for u8 {
    /// Returns true for `UInt64`, `UInt32`, `UInt16`, `UInt8`, `Int64`, `Int32`, `Int16`, `Float64` and `Float32`.
    fn is_lossless_to(f: SampleFormatCode) -> bool {
        matches!(
            f,
            UInt8 | UInt16 | UInt32 | UInt64 | Int16 | Int32 | Int64 | Float32 | Float64
        )
    }
}

#[cfg(test)]
/// NB: for now, due to the lare number of combos we mostly don't test conversions
/// outside of 32 and 64 bit types.
mod tests {
    use giga_segy_core::enums::SampleFormatCode;
    use giga_segy_core::enums::SampleFormatCode::*;
    use giga_segy_core::errors::*;
    use giga_segy_core::BinHeader;

    use super::*;
    use crate::create_headers::CreateBinHeader;

    use num::{FromPrimitive, ToPrimitive};
    use std::fmt::Debug;

    fn make_bit_converter<T: ToPrimitive + Debug>() {
        for b in [true, false].iter() {
            for f in [
                Int32, Int16, Float32, Float64, Int8, Int64, UInt32, UInt16, UInt64, UInt8,
            ]
            .iter()
            {
                let choice: Result<BitConverter<T>, RsgError> = converter_chooser(*f, *b);
                assert!(
                    choice.is_ok(),
                    "`converter_chooser` should work for le={}, SampleFormatCode::{:?}",
                    b,
                    f
                );
            }
            for f in [IbmFloat32, FixPoint32, Int24, UInt24].iter() {
                let choice: Result<BitConverter<T>, RsgError> = converter_chooser(*f, *b);
                assert!(
                    choice.is_err(),
                    "`converter_chooser` should fail for le={}, SampleFormatCode::{:?}",
                    b,
                    f
                );
            }
        }
    }

    fn convert_data_setup<T: ToPrimitive + Debug, U: FromPrimitive + Debug>(
        header_format: SampleFormatCode,
        le: bool,
        to: fn(&T) -> Option<U>,
        data: Vec<T>,
    ) -> (Vec<U>, BitConverter<T>, BinHeader) {
        let mut header = BinHeader::default();
        header.sample_format_code = header_format;
        header.binary_flag_direction_is_le = le;

        let data = data
            .iter()
            .map(|x| to(x).expect("Can't convert."))
            .collect::<Vec<U>>();
        let converter = converter_chooser(header_format, le).expect("works");

        (data, converter, header)
    }

    macro_rules! make_convert_test {
        ($fr_type:ident, $to_type:ident, $le:expr, $format:expr, $conversion:ident, $is_ok:ident) => {{
            let original: Vec<$fr_type> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
                .into_iter()
                .map(|x| x as $fr_type)
                .collect::<Vec<_>>();
            let (expected_data, converter, header) = convert_data_setup::<$fr_type, $to_type>(
                $format,
                $le,
                $conversion,
                original.to_owned(),
            );

            let lossless_res = convert_data_losslessly(original.clone(), &header);
            assert!($is_ok(&lossless_res));
            let res = convert_data(original.clone(), &header).expect("Is ok.");

            let expected_bytes = expected_data
                .into_iter()
                .map(|x| {
                    if !$le {
                        x.to_be_bytes().iter().map(|x| *x).collect::<Vec<u8>>()
                    } else {
                        x.to_le_bytes().iter().map(|x| *x).collect::<Vec<u8>>()
                    }
                })
                .flatten()
                .collect::<Vec<u8>>();

            let expected_bytes_2 = original
                .into_iter()
                .map(|x| converter(x).expect("Should work"))
                .flatten()
                .collect::<Vec<u8>>();

            assert_eq!(res, expected_bytes, "`convert_data` failed.");
            assert_eq!(res, expected_bytes_2, "`converter` failed.");
        }};
    }

    #[test]
    fn converter_chooser_formats_ux() {
        make_bit_converter::<u8>();
        make_bit_converter::<u16>();
        make_bit_converter::<u32>();
        make_bit_converter::<u64>();
    }

    #[test]
    fn converter_chooser_formats_ix() {
        make_bit_converter::<i8>();
        make_bit_converter::<i16>();
        make_bit_converter::<i32>();
        make_bit_converter::<i64>();
    }

    #[test]
    fn converter_chooser_formats_fx() {
        make_bit_converter::<f32>();
        make_bit_converter::<f64>();
    }

    #[test]
    fn convert_u64_to_u32_ok() {
        let con = u64::to_u32;
        let is_err = Result::is_err;
        make_convert_test!(u64, u32, false, UInt32, con, is_err);
        make_convert_test!(u64, u32, true, UInt32, con, is_err);
    }

    #[test]
    fn convert_u32_to_u64_ok() {
        let con = u32::to_u64;
        let is_err = Result::is_ok;
        make_convert_test!(u32, u64, false, UInt64, con, is_err);
        make_convert_test!(u32, u64, true, UInt64, con, is_err);
    }

    #[test]
    fn convert_i64_to_i32_ok() {
        let con = i64::to_i32;
        let is_err = Result::is_err;
        make_convert_test!(i64, i32, false, Int32, con, is_err);
        make_convert_test!(i64, i32, true, Int32, con, is_err);
    }

    #[test]
    fn convert_i32_to_i64_ok() {
        let con = i32::to_i64;
        let is_err = Result::is_ok;
        make_convert_test!(i32, i64, false, Int64, con, is_err);
        make_convert_test!(i32, i64, true, Int64, con, is_err);
    }

    #[test]
    fn convert_f64_to_f32_ok() {
        let con = f64::to_f32;
        let is_err = Result::is_err;
        make_convert_test!(f64, f32, false, Float32, con, is_err);
        make_convert_test!(f64, f32, true, Float32, con, is_err);
    }

    #[test]
    fn convert_f32_to_f64_ok() {
        let con = f32::to_f64;
        let is_err = Result::is_ok;
        make_convert_test!(f32, f64, true, Float64, con, is_err);
        make_convert_test!(f32, f64, false, Float64, con, is_err);
    }

    #[test]
    fn convert_i32_to_f32_ok() {
        let con = i32::to_f32;
        let is_err = Result::is_err;
        make_convert_test!(i32, f32, false, Float32, con, is_err);
        make_convert_test!(i32, f32, true, Float32, con, is_err);
    }

    #[test]
    fn convert_u32_to_f32_ok() {
        let con = u32::to_f32;
        let is_err = Result::is_err;
        make_convert_test!(u32, f32, false, Float32, con, is_err);
        make_convert_test!(u32, f32, true, Float32, con, is_err);
    }

    #[test]
    fn convert_u64_to_f32_ok() {
        let con = u64::to_f32;
        let is_err = Result::is_err;
        make_convert_test!(u64, f32, false, Float32, con, is_err);
        make_convert_test!(u64, f32, true, Float32, con, is_err);
    }

    #[test]
    fn convert_i64_to_f32_ok() {
        let con = i64::to_f32;
        let is_err = Result::is_err;
        make_convert_test!(i64, f32, false, Float32, con, is_err);
        make_convert_test!(i64, f32, true, Float32, con, is_err);
    }

    #[test]
    fn convert_i32_to_f64_ok() {
        let con = i32::to_f64;
        let is_err = Result::is_ok;
        make_convert_test!(i32, f64, false, Float64, con, is_err);
        make_convert_test!(i32, f64, true, Float64, con, is_err);
    }

    #[test]
    fn convert_u32_to_f64_ok() {
        let con = u32::to_f64;
        let is_err = Result::is_err;
        make_convert_test!(u32, f64, false, Float64, con, is_err);
        make_convert_test!(u32, f64, true, Float64, con, is_err);
    }

    #[test]
    fn convert_u64_to_f64_ok() {
        let con = u64::to_f64;
        let is_err = Result::is_err;
        make_convert_test!(u64, f64, false, Float64, con, is_err);
        make_convert_test!(u64, f64, true, Float64, con, is_err);
    }

    #[test]
    fn convert_i64_to_f64_ok() {
        let con = i64::to_f64;
        let is_err = Result::is_err;
        make_convert_test!(i64, f64, false, Float64, con, is_err);
        make_convert_test!(i64, f64, true, Float64, con, is_err);
    }
}
