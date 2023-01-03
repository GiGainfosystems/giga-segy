// Copyright (C) 2022 by GiGa infosystems
//! This module contains the [`CoordinateScalar`] structure which deals with the somewhat unusual
//! way that SEG-Y uses to express scaling factor of coordinates.
//!
//! See the SEGY-Y_r2.0 standard (january 2017), page 17 for more details.
use num::{One, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Div;

/// This structure deals with scaling of coordinates.
///
/// Since coordinates in  SEGY are stored as [`i32`], by default,
/// this means that decimal points cannot be stored. A scalar
/// corrects this flaw. Scalars are somewhat unintuitive, so this
/// utility is provided to convert normal multipliers to scalars.
///
/// See the SEGY-Y_r2.0 standard (january 2017), page 17 for more details.
///
/// NB: The multiplier should be of the same type as the coordinates
/// that are being converted.
///
/// NB2: SEGY stores scalars as i16, so initial multipliers of a high
/// magnitude, or those from non-integer floats will be handled lossily.
///
/// ```
/// # use giga_segy_out::utils::CoordinateScalar;
/// // Our coordinate value is 52.55. We want to keep the decimal places,
/// // So we multiply by x100, which means the final value, 5255 must be multiplied
/// // by x0.01 for it to be returned to 52.55. So the input here 0.01.
/// let s = CoordinateScalar::from_multiplier(0.01f64).unwrap();
///
/// // `output_a`, which is what will be inserted into the trace header, should be
/// // about 5255.
/// let output_a = s.scale(52.55);
/// // `output_b`, which is what will be inserted into the trace header, should be
/// // about 5200.
/// let output_b = s.scale_to_i32(52.).unwrap();
/// assert_eq!(output_a, 5255.0);
/// assert_eq!(output_b, 5200);
///
/// assert_eq!(s.multiplier(), 0.01);
/// // The `writeable_scalar` is what must be written in the trace header to get the
/// // correct final conversion when reading the SEG-Y.
/// assert_eq!(s.writeable_scalar(), -100);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct CoordinateScalar<T>
where
    T: ToPrimitive + Debug + Clone + PartialOrd + One + Zero + Div<Output = T>,
{
    original_multiplier: T,
    writeable_scalar: i16,
}

impl<T> CoordinateScalar<T>
where
    T: ToPrimitive + Debug + Clone + PartialOrd + One + Zero + Div<Output = T>,
{
    /// Create a new scalar.
    ///
    /// NB: If the value provided is outside of the range, `None` is returned.
    ///
    /// NB2: A multiplier MUST be a non-negative value.
    ///
    /// NB3: If the multiplier is 100, the scalar is -100.
    ///
    /// ```
    /// # use giga_segy_out::utils::CoordinateScalar;
    /// assert!(CoordinateScalar::from_multiplier(-1i16).is_none());
    /// assert!(CoordinateScalar::from_multiplier(i16::MAX).is_some());
    /// assert!(CoordinateScalar::from_multiplier(i16::MAX as i32 + 1).is_none());
    ///
    /// let s = CoordinateScalar::from_multiplier(1000.0f32).unwrap();
    /// assert_eq!(s.multiplier(), 1000.);
    /// assert_eq!(s.writeable_scalar(), 1000);
    ///
    /// let s = CoordinateScalar::from_multiplier(0.001f32).unwrap();
    /// assert_eq!(s.multiplier(), 0.001);
    /// assert_eq!(s.writeable_scalar(), -1000);
    /// ```
    pub fn from_multiplier(multiplier: T) -> Option<Self> {
        if let Some(Ordering::Less) = multiplier.partial_cmp(&T::zero()) {
            return None;
        }

        match multiplier.partial_cmp(&T::one()) {
            Some(Ordering::Greater) => multiplier.to_i16(),
            Some(Ordering::Less) => multiplier
                .to_f64()
                // This needs to be rounded otherwise silly things happen.
                .and_then(|m| (1. / m).round().to_i16().map(|i| -i)),
            Some(Ordering::Equal) => Some(0),
            None => None,
        }
        .map(|x| Self {
            original_multiplier: multiplier,
            writeable_scalar: x,
        })
    }

    /// Operate on a value converting to [`i32`] directly.
    ///
    /// NB: This is useful if all coordinates are stored directly as [`i32`].
    ///
    /// NB2: If the value cannot be converted to [`i32`] directly, [`None`] is returned.
    ///
    /// ```
    /// # use giga_segy_out::utils::CoordinateScalar;
    /// let s = CoordinateScalar::from_multiplier(10.0f32).unwrap();
    /// assert_eq!(s.scale_to_i32(990i16 as f32), Some(99));
    /// assert_eq!(s.scale_to_i32(i32::MAX as f32 * 11.), None);
    /// ```
    pub fn scale_to_i32(&self, x: T) -> Option<i32> {
        (x / self.original_multiplier.clone()).to_i32()
    }

    /// Operate on a value.
    ///
    /// ```
    /// # use giga_segy_out::utils::CoordinateScalar;
    /// let s = CoordinateScalar::from_multiplier(10.0f32).unwrap();
    /// assert_eq!(s.scale(990i16 as f32), 99.);
    /// assert_eq!(s.scale(i32::MAX as f32 * 11.), i32::MAX as f32 * 11./10.);
    /// ```
    pub fn scale(&self, x: T) -> T {
        x / self.original_multiplier.clone()
    }

    /// Get the multiplier.
    pub fn multiplier(&self) -> T {
        self.original_multiplier.clone()
    }

    /// Get the scaler as the final i16 value.
    pub fn writeable_scalar(&self) -> i16 {
        self.writeable_scalar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::integer::Roots;

    #[test]
    fn create_cs1() {
        let scalar = CoordinateScalar::from_multiplier(0.01f64);
        let expected = CoordinateScalar {
            original_multiplier: 0.01f64,
            writeable_scalar: -100,
        };

        // A few creation tests.
        assert_eq!(scalar, Some(expected));
        let scalar = scalar.unwrap();
        assert_eq!(scalar.writeable_scalar(), -100);
        assert_eq!(scalar.multiplier(), 0.01);

        // A few scaling tests. (FP comparisons FTL)
        assert!(scalar.scale(62.) - 6200. < (0.000000000000001f64).abs());
        assert_eq!(scalar.scale(0.), 0.);
        assert_eq!(scalar.scale(1.), 100.);

        assert_eq!(scalar.scale_to_i32(f64::MAX.sqrt()), None);
        assert_eq!(scalar.scale_to_i32(0.), Some(0));
        assert_eq!(scalar.scale_to_i32(1.), Some(100));
        assert_eq!(scalar.scale_to_i32(360000.), Some(36000000));
    }

    #[test]
    fn create_cs2() {
        let scalar = CoordinateScalar::from_multiplier(10000u64);
        let expected = CoordinateScalar {
            original_multiplier: 10000u64,
            writeable_scalar: 10000,
        };

        // A few creation tests.
        assert_eq!(scalar, Some(expected));
        let scalar = scalar.unwrap();
        assert_eq!(scalar.writeable_scalar(), 10000);
        assert_eq!(scalar.multiplier(), 10000);

        // A few scaling tests.
        assert_eq!(scalar.scale(620000), 62);
        assert_eq!(scalar.scale(0), 0);
        assert_eq!(scalar.scale(10000), 1);

        assert_eq!(scalar.scale_to_i32(u64::MAX.sqrt() * 1000000), None);
        assert_eq!(scalar.scale_to_i32(0), Some(0));
        assert_eq!(scalar.scale_to_i32(12000), Some(1));
        assert_eq!(scalar.scale_to_i32(360000), Some(36));
    }

    #[test]
    fn create_cs3() {
        let scalar = CoordinateScalar::from_multiplier(1i128);
        let expected = CoordinateScalar {
            original_multiplier: 1i128,
            writeable_scalar: 0,
        };

        // A few creation tests.
        assert_eq!(scalar, Some(expected));
        let scalar = scalar.unwrap();
        assert_eq!(scalar.writeable_scalar(), 0);
        assert_eq!(scalar.multiplier(), 1);

        // A few scaling tests.
        assert_eq!(scalar.scale(62), 62);
        assert_eq!(scalar.scale(0), 0);
        assert_eq!(scalar.scale(1), 1);

        assert_eq!(scalar.scale_to_i32(i128::MAX.sqrt()), None);
        assert_eq!(scalar.scale_to_i32(0), Some(0));
        assert_eq!(scalar.scale_to_i32(1), Some(1));
        assert_eq!(scalar.scale_to_i32(36), Some(36));
    }

    #[test]
    fn create_cs_fail() {
        let scalar = CoordinateScalar::from_multiplier(u64::MAX);
        assert_eq!(scalar, None);
    }

    #[test]
    fn create_cs_fail2() {
        let scalar = CoordinateScalar::from_multiplier(-100i64);
        assert_eq!(scalar, None);
    }
}
