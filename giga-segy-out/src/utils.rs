// Copyright (C) 2022 by GiGa infosystems
//! This file is here for miscellenious helper functions which have nowhere else to go.
use num::{One, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Div;

/// This structure deals with scaling of coordinates.
///
/// Since coordinates in  SEGY are stored as `i32`, by default,
/// this means that decimal points cannot be stored. A scalar
/// corrects this flaw. Scalars are somewhat unintuitive, so this
/// utility is provided to convert normal ultipliers to scalaras.
///
/// NB: The multiplier should be of the same type as the coordinates
/// that are being converted.
///
/// NB2: SEGY stores scalars as i16, so initial multipliers of a high
/// magnitude, or those from non-integer floats will be handled lossily.
///
/// ```
/// # use giga_segy_out::utils::CoordinateScalar;
/// let s = CoordinateScalar::from_multiplier(0.1f64).unwrap();
/// let a = s.scale(52.55);
/// let b = s.scale_to_i32(52.).unwrap();
/// assert!(a - 525.5 < (0.000000000000001f64).abs());
/// assert_eq!(b, 520);
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
    pub fn from_multiplier(multiplier: T) -> Option<Self> {
        if let Some(Ordering::Less) = multiplier.partial_cmp(&T::zero()) {
            return None;
        }

        match multiplier.partial_cmp(&T::one()) {
            Some(Ordering::Greater) => multiplier.to_i16(),
            Some(Ordering::Less) => multiplier
                .to_f64()
                .and_then(|m| (1.0 / m).to_i16().map(|i| -i)),
            Some(Ordering::Equal) => Some(0),
            None => None,
        }
        .map(|x| Self {
            original_multiplier: multiplier,
            writeable_scalar: x,
        })
    }

    /// Operate on a value converting to i32 directly.
    ///
    /// NB: This is useful if all coordinates are stored directly as i32.
    pub fn scale_to_i32(&self, x: T) -> Option<i32> {
        (x / self.original_multiplier.clone()).to_i32()
    }

    /// Operate on a value.
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
