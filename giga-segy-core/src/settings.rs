//! This module contains the [`SegySettings`] structure which can be used to customise the SEG-Y
//! parsing.
//!
//! NB: It should be noted that since few files are in keeping with the proper SEG-Y format, this
//! is necessary. On the other hand, using this functionality can easily cause incorrect writing
//! or parsing of SEG-Y files and should therefore be done with care.
use crate::enums::{MeasurementSystem, OrderTraceBy, SampleFormatCode, TraceIdCode};
use crate::errors::*;
use crate::{
    CDPX_BYTE_LOCATION, CDPY_BYTE_LOCATION, CROSSLINE_BYTE_LOCATION, INLINE_BYTE_LOCATION,
    TRACE_HEADER_LEN,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// This structure holds a list of various settings to be imported for the custom reading of
/// byte locations of various variables in the headers and other things when interpreting a SEG-Y file.
///
/// This structure does not allow direct manipulation of fields as in several cases not all possible
/// values for that field are valid (eg byte indices are [`usize`], but a trace header is only 240 bytes long),
/// and in other cases the value of one field may influence the value of another field.
/// Therefore using setter and getter functions is generally safer.
pub struct SegySettings {
    /// An enum which determines what traces are ordered by.
    pub(crate) order_trace_by: OrderTraceBy,
    /// Should the endianness be overwridden to LE?
    pub(crate) override_to_le: Option<bool>,
    /// Should trace format be overwridden?
    pub(crate) override_trace_format: Option<SampleFormatCode>,
    /// Reads trace header coordinates as f32 instead of i32.
    pub(crate) override_coordinate_format: Option<SampleFormatCode>,
    /// A chance to override the z_axis domain by changing the id code of the traces.
    pub(crate) override_trace_id_code: Option<TraceIdCode>,
    /// A chance to override z-axis unit ONLY. NB: Not used in crate. Provided for consuming
    /// libraries and applications.
    pub(crate) override_trace_depth_units: Option<MeasurementSystem>,
    /// A chance to override xy-axis units ONLY.
    pub(crate) override_coordinate_units: Option<MeasurementSystem>,
    /// Should coordinate scaling be overridden?
    pub(crate) override_coordinate_scaling: Option<i16>,
    /// The byte index of the inline_no on the trace headers.
    pub(crate) inline_no_bidx: usize,
    /// The byte index of the crossline_no on the trace headers.
    pub(crate) crossline_no_bidx: usize,
    /// The byte index of the x_ensemble (CDP) in the trace header.
    pub(crate) x_ensemble_bidx: usize,
    /// The byte index of the y_ensemble (CDP) in the trace header.
    pub(crate) y_ensemble_bidx: usize,
    /// This is the strp-by number for moving down a trae (usually it is equal to 1 (ie nothing is skipped)).
    pub(crate) step_by: usize,
    /// Custom minimum and maximum inline number. Is `None` by default. (Probably not important here)
    pub(crate) inline_min_max: Option<[i32; 2]>,
    /// The minimum and maximum crossline number. Is `None` by default. (Probably not important here)
    pub(crate) crossline_min_max: Option<[i32; 2]>,
    /// This is the origin of the geometry.
    pub(crate) origin: Option<[f64; 3]>,
    /// Sets a set of custom dimensions on the grid as inline.
    pub(crate) override_dim_x: Option<i32>,
    /// Sets custom crossline count.
    pub(crate) override_dim_y: Option<i32>,
    /// Sets custom depth.
    pub(crate) override_dim_z: Option<i32>,
    /// Sets a custom u unit vector.
    pub(crate) override_u: Option<[f64; 3]>,
    /// Sets a custom v unit vector.
    pub(crate) override_v: Option<[f64; 3]>,
    /// Sets a custom w unit vector.
    pub(crate) override_sample_interval: Option<f64>,
}

impl Default for SegySettings {
    /// Creates the default instance, where nothing is overridden and defualt settings are used
    /// for everything.
    fn default() -> Self {
        SegySettings {
            override_to_le: None,
            override_trace_format: None,
            override_coordinate_format: None,
            override_coordinate_scaling: None,
            override_trace_id_code: None,
            override_trace_depth_units: None,
            override_coordinate_units: None,
            inline_no_bidx: INLINE_BYTE_LOCATION,
            crossline_no_bidx: CROSSLINE_BYTE_LOCATION,
            x_ensemble_bidx: CDPX_BYTE_LOCATION,
            y_ensemble_bidx: CDPY_BYTE_LOCATION,
            step_by: 1,
            inline_min_max: None,
            crossline_min_max: None,
            origin: None,
            override_dim_x: None,
            override_dim_y: None,
            override_dim_z: None,
            override_u: None,
            override_v: None,
            override_sample_interval: None,
            order_trace_by: OrderTraceBy::Default,
        }
    }
}

impl SegySettings {
    #[cfg(feature = "to_json")]
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self).map_err(|e| e.to_string())
    }

    /// A function to set the order_trace_by`
    pub fn set_order_trace_by(&mut self, order: OrderTraceBy) {
        self.order_trace_by = order;
    }

    /// Sets the endianness to LE if true and BE if false
    pub fn set_override_to_le(&mut self, le: bool) {
        self.override_to_le = Some(le);
    }

    /// Sets the trace format to the input.
    pub fn set_override_trace_format(&mut self, format: SampleFormatCode) {
        self.override_trace_format = Some(format);
    }

    /// Sets the trace format to the input.
    pub fn set_override_trace_id_code(&mut self, domain: TraceIdCode) {
        self.override_trace_id_code = Some(domain);
    }

    /// Sets the trace format to the input.
    /// NB: Not used in crate. Provided for consuming libraries and applications.
    pub fn set_override_trace_depth_units(&mut self, units: MeasurementSystem) {
        self.override_trace_depth_units = Some(units);
    }

    /// Sets the trace format to the input.
    pub fn set_override_coordinate_units(&mut self, units: MeasurementSystem) {
        self.override_coordinate_units = Some(units);
    }

    /// Sets the trace format to the input.
    ///
    /// This function will return an error if the format code is for a format which is not four bytes long.
    /// This convention is maintained because the fields of the trace header used to store coordinate
    /// values are 4 bytes long and therefore it should not usually be possible for SEG-Y files that
    /// even pretend to follow the standard to store coordinates in a 2, 3 or 8 byte format.
    /// ```
    /// # use giga_segy_core::settings::*;
    /// # use giga_segy_core::enums::SampleFormatCode;
    /// let mut settings = SegySettings::default();
    /// assert!(settings.get_override_coordinate_format().is_none());
    ///
    /// settings
    ///     .set_override_coordinate_format(SampleFormatCode::IbmFloat32)
    ///     .unwrap();
    /// assert_eq!(
    ///     settings.get_override_coordinate_format(),
    ///     Some(SampleFormatCode::IbmFloat32)
    /// );
    ///
    /// let res = settings.set_override_coordinate_format(SampleFormatCode::UInt64);
    /// assert!(res.is_err());
    /// ```
    pub fn set_override_coordinate_format(
        &mut self,
        format: SampleFormatCode,
    ) -> Result<(), RsgError> {
        use SampleFormatCode::*;
        match format {
            IbmFloat32 | Float32 | UInt32 | Int32 => self.override_coordinate_format = Some(format),
            _ => {
                return Err(RsgError::BitConversionError {
                    msg: format!("Coordinate format must be 4-byte. {:?} is not", format),
                })
            }
        }
        Ok(())
    }

    /// Sets the coordinate scaling as overridden by the value.
    ///
    /// Since scaling in the trace headers is stored essentially as an [`i16`] value,
    /// if the value given overflows this data type the function will return an error.
    ///
    /// Furthermore, the value given must already be in the format used by SEG-Y (see the
    /// [SEG-Y_r2.0 standard](<https://seg.org/Portals/0/SEG/News%20and%20Resources/Technical%20Standards/seg_y_rev2_0-mar2017.pdf>)
    /// (January 2017), page 17 for more details).
    /// ```
    /// # use giga_segy_core::settings::*;
    /// let mut settings = SegySettings::default();
    /// assert!(settings.get_override_coordinate_scaling().is_none());
    ///
    /// settings
    ///     .set_override_coordinate_scaling(-100.0f64)
    ///     .unwrap();
    /// assert_eq!(
    ///     settings.get_override_coordinate_scaling(),
    ///     Some(-100.)
    /// );
    ///
    /// let res = settings.set_override_coordinate_scaling(i16::MAX as f64 + 1.0);
    /// assert!(res.is_err());
    /// ```
    pub fn set_override_coordinate_scaling(&mut self, scaling: f64) -> Result<(), RsgError> {
        use num::FromPrimitive;

        let scaling = i16::from_f64(scaling).ok_or_else(|| RsgError::BitConversionError {
            msg: format!("{} is outside of the scaling range.", scaling),
        })?;
        self.override_coordinate_scaling = Some(scaling);
        Ok(())
    }

    /// Sets the inline number byte index as overridden by the value.
    ///
    /// If the byte index given would lead to reading past the end of the trace header an error is returned.
    /// ```
    /// # use giga_segy_core::settings::*;
    /// let mut settings = SegySettings::default();
    /// // NB: Rust uses zero indexing, the SEG-Y standard does not.
    /// assert_eq!(settings.get_inline_no_bidx(), 188);
    ///
    /// // Trace header length is 240, so the last valid range is 236..239.
    /// settings.set_inline_no_bidx(236).unwrap();
    /// assert_eq!(settings.get_inline_no_bidx(), 236);
    ///
    /// let res = settings.set_inline_no_bidx(237);
    /// assert!(res.is_err());
    /// ```
    pub fn set_inline_no_bidx(&mut self, bidx: usize) -> Result<(), RsgError> {
        if bidx > TRACE_HEADER_LEN - 4 {
            return Err(RsgError::SEGYSettingsError {
                msg: "Maximum permitted index value for trace header".to_string(),
            });
        }
        self.inline_no_bidx = bidx;
        Ok(())
    }

    /// Sets the crossline number byte index as overridden by the value.
    pub fn set_crossline_no_bidx(&mut self, bidx: usize) -> Result<(), RsgError> {
        if bidx > TRACE_HEADER_LEN - 4 {
            return Err(RsgError::SEGYSettingsError {
                msg: "Maximum permitted index value for trace header".to_string(),
            });
        }
        self.crossline_no_bidx = bidx;
        Ok(())
    }

    /// Sets the x-ensemble (x-CDP) as overridden by the input value.
    pub fn set_x_ensemble_bidx(&mut self, bidx: usize) -> Result<(), RsgError> {
        if bidx > TRACE_HEADER_LEN - 4 {
            return Err(RsgError::SEGYSettingsError {
                msg: "Maximum permitted index value for trace header".to_string(),
            });
        }
        self.x_ensemble_bidx = bidx;
        Ok(())
    }

    /// Sets the y-ensemble (y-CDP) as overridden by the input value.
    pub fn set_y_ensemble_bidx(&mut self, bidx: usize) -> Result<(), RsgError> {
        if bidx > TRACE_HEADER_LEN - 4 {
            return Err(RsgError::SEGYSettingsError {
                msg: "Maximum permitted index value for trace header".to_string(),
            });
        }
        self.y_ensemble_bidx = bidx;
        Ok(())
    }

    /// Sets the trace data step by (skip-1) value to the input value.
    pub fn set_step_by(&mut self, step: usize) {
        self.step_by = step;
    }

    /// Sets the mininmum and maximum inline value to the input values.
    pub fn set_inlne_min_max(&mut self, min_max: [i32; 2]) {
        self.inline_min_max = Some(min_max);
    }

    /// Sets the minimum and maximum crossline value to the input value.
    pub fn set_crossline_min_max(&mut self, min_max: [i32; 2]) {
        self.crossline_min_max = Some(min_max);
    }

    /// Sets the geometry origin to the given value.
    pub fn set_origin(&mut self, origin: [f64; 3]) {
        self.origin = Some(origin);
    }

    /// Sets the override for inline count.
    ///
    /// While this function takes an [`i32`] value as an argument, it will throw an error if the
    /// value is negative. Furthermore, both [`Self::set_override_dim_x`] and [`Self::set_override_dim_y`]
    /// also set `crossline_min_max` and `inline_min_max` respectively. Since the minimum
    /// is set to zero in this case, it is strongly recommended that the user check and override this value
    /// manually with [`Self::set_inlne_min_max`] or [`Self::set_crossline_min_max`] as appropriate.
    /// ```
    /// # use giga_segy_core::settings::*;
    /// let mut settings = SegySettings::default();
    /// // NB: Rust uses zero indexing, the SEG-Y standard does not.
    /// assert!(settings.get_override_dim_x().is_none());
    /// assert!(settings.get_crossline_min_max().is_none());
    /// assert!(settings.set_override_dim_x(-1).is_err());
    ///
    /// settings.set_override_dim_x(50).unwrap();
    /// assert_eq!(settings.get_override_dim_x(), Some(50));
    /// // This could be troublesome, so it should be overridden manually
    /// // once the actual min-max is known.
    /// assert_eq!(settings.get_crossline_min_max(), Some([0, 49]));
    /// ```
    pub fn set_override_dim_x(&mut self, dim_x: i32) -> Result<(), RsgError> {
        if dim_x < 0 {
            Err(RsgError::SEGYSettingsError {
                msg: "Custom x-dimension passed to `SegySettings` must be positive.".to_string(),
            })
        } else {
            self.override_dim_x = Some(dim_x);
            // These must be overriden, or limiters will use the original inline/crossline numbers
            // for cutoffs of out of bounds traces.
            self.crossline_min_max = Some([0, dim_x - 1]);
            Ok(())
        }
    }

    /// Sets the override for crossline count. As with [`Self::set_override_dim_x`], this function
    /// should be used with care.
    /// ```
    /// # use giga_segy_core::settings::*;
    /// let mut settings = SegySettings::default();
    /// // NB: Rust uses zero indexing, the SEG-Y standard does not.
    /// assert!(settings.get_override_dim_y().is_none());
    /// assert!(settings.get_inlne_min_max().is_none());
    /// assert!(settings.set_override_dim_y(-1).is_err());
    ///
    /// settings.set_override_dim_y(50).unwrap();
    /// assert_eq!(settings.get_override_dim_y(), Some(50));
    /// // This could be troublesome, so it should be overridden manually
    /// // once the actual min-max is known.
    /// assert_eq!(settings.get_inlne_min_max(), Some([0, 49]));
    /// ```
    pub fn set_override_dim_y(&mut self, dim_y: i32) -> Result<(), RsgError> {
        if dim_y < 0 {
            Err(RsgError::SEGYSettingsError {
                msg: "Custom x-dimension passed to `SegySettings` must be positive.".to_string(),
            })
        } else {
            self.override_dim_y = Some(dim_y);
            // These must be overriden, or limiters will use the original inline/crossline numbers
            // for cutoffs of out of bounds traces.
            self.inline_min_max = Some([0, dim_y - 1]);
            Ok(())
        }
    }

    /// Sets the override for sample count.
    pub fn set_override_dim_z(&mut self, dim_z: i32) -> Result<(), RsgError> {
        if dim_z < 0 {
            Err(RsgError::SEGYSettingsError {
                msg: "Custom x-dimension passed to `SegySettings` must be positive.".to_string(),
            })
        } else {
            self.override_dim_z = Some(dim_z);
            Ok(())
        }
    }

    /// Set the physical `u` vector to something other than what is found in headers.
    pub fn set_override_u(&mut self, u: [f64; 3]) {
        self.override_u = Some(u)
    }

    /// Set the physical `v` vector to something other than what is found in headers.
    pub fn set_override_v(&mut self, v: [f64; 3]) {
        self.override_v = Some(v);
    }

    /// Set an override for the physical sample interval (NB: one dimensionsal by definition.)
    pub fn set_override_sample_interval(&mut self, t: f64) {
        self.override_sample_interval = Some(t);
    }

    /// Get override endianness if any.
    pub fn get_override_to_le(&self) -> Option<bool> {
        self.override_to_le
    }

    /// Get the trace format override if any.
    pub fn get_override_trace_format(&self) -> Option<SampleFormatCode> {
        self.override_trace_format
    }

    /// Gets the coordinate format if any.
    pub fn get_override_coordinate_format(&self) -> Option<SampleFormatCode> {
        self.override_coordinate_format
    }

    /// Gets the trace format to the input.
    pub fn get_override_trace_id_code(&self) -> Option<TraceIdCode> {
        self.override_trace_id_code
    }

    /// Gets the depth units override value.
    /// NB: Not used in crate. Provided for consuming libraries and applications.
    pub fn get_override_trace_depth_units(&self) -> Option<MeasurementSystem> {
        self.override_trace_depth_units
    }

    /// Gets the measurment system of the coordinate unit override.
    pub fn get_override_coordinate_units(&self) -> Option<MeasurementSystem> {
        self.override_coordinate_units
    }

    /// Gets the coordinate scaling override if any.
    pub fn get_override_coordinate_scaling(&self) -> Option<f64> {
        self.override_coordinate_scaling.map(|x| x as f64)
    }

    /// Get the byte index of the inline number.
    pub fn get_inline_no_bidx(&self) -> usize {
        self.inline_no_bidx
    }

    /// Get the byte index of the crossline number.
    pub fn get_crossline_no_bidx(&self) -> usize {
        self.crossline_no_bidx
    }

    /// Get the byte index of the x-ensemble (x-CDP) number.
    pub fn get_x_ensemble_bidx(&self) -> usize {
        self.x_ensemble_bidx
    }

    /// Gets the y-ensemble (y-CDP) byte index.
    pub fn get_y_ensemble_bidx(&self) -> usize {
        self.y_ensemble_bidx
    }

    /// Gets the trace data step by (skip-1) value.
    pub fn get_step_by(&self) -> usize {
        self.step_by
    }

    /// Get the mininmum and maximum inline value, if any.
    pub fn get_inlne_min_max(&self) -> Option<[i32; 2]> {
        self.inline_min_max
    }

    /// Get the minimum and maximum crossline value, if any.
    pub fn get_crossline_min_max(&self) -> Option<[i32; 2]> {
        self.crossline_min_max
    }

    /// Get the origin, as set in the override, if set.
    pub fn get_origin(&self) -> Option<[f64; 3]> {
        self.origin
    }

    /// Get the custom iinline count of a voxet
    pub fn get_override_dim_x(&self) -> Option<i32> {
        self.override_dim_x
    }

    /// Get the custom crossline count of a voxet
    pub fn get_override_dim_y(&self) -> Option<i32> {
        self.override_dim_y
    }

    /// Get the custom depth (sample count) of a voxet
    pub fn get_override_dim_z(&self) -> Option<i32> {
        self.override_dim_z
    }

    /// Gets the maximum number of traces in the geometry: If `override_dimensions` is
    /// not set, we have `usize::MAX`, otherwise we have the grid size.
    pub fn get_max_trace_count_by_override_dimensions(&self) -> usize {
        match (self.override_dim_x, self.override_dim_y) {
            (Some(x), Some(y)) => x as usize * y as usize,
            _ => std::usize::MAX,
        }
    }

    /// Gets the maximum length of traces in the geometry: If `override_dimensions` is
    /// not set, we have `usize::MAX`, otherwise we have the grid size.
    pub fn get_max_trace_length_by_override_dimensions(&self) -> Option<usize> {
        self.override_dim_z.map(|count| count as usize)
    }

    /// Get the physical `u` vector.
    pub fn get_override_u(&self) -> Option<[f64; 3]> {
        self.override_u
    }

    /// Get the physical `v` vector.
    pub fn get_override_v(&self) -> Option<[f64; 3]> {
        self.override_v
    }

    /// Get the physcial sample interval (NB: one dimensionsal by definition.)
    pub fn get_override_sample_interval(&self) -> Option<f64> {
        self.override_sample_interval
    }

    /// A function to get the order_trace_by`
    pub fn get_order_trace_by(&self) -> OrderTraceBy {
        self.order_trace_by
    }

    /// Check whether a given inline and crossline number will be in bounds
    /// according to the options. If no inline/crossline min-max is set, the
    /// return is [`true`].
    /// ```
    /// # use giga_segy_core::settings::SegySettings;
    /// let mut settings = SegySettings::default();
    /// assert!(settings.trace_in_bounds(99999, -99999));
    ///
    /// settings.set_inlne_min_max([50, 2000]);
    /// assert_ne!(settings.trace_in_bounds(99999, -99999), true);
    /// assert!(settings.trace_in_bounds(100, -99999));
    ///
    /// settings.set_crossline_min_max([50, 2000]);
    /// assert_ne!(settings.trace_in_bounds(100, -99999), true);
    /// assert!(settings.trace_in_bounds(100, 100));
    /// ```
    pub fn trace_in_bounds(&self, inline: i32, crossline: i32) -> bool {
        let inline_ok = if let Some([min, max]) = self.inline_min_max {
            inline <= max && inline >= min
        } else {
            true
        };
        let crossline_ok = if let Some([min, max]) = self.crossline_min_max {
            crossline <= max && crossline >= min
        } else {
            true
        };
        inline_ok && crossline_ok
    }
}
