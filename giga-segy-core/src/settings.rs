//! This module contains the `SegySettings` structure which can be used to customise the SEG-Y
//! parsing.
//! NB: It should be noted that since few files are in keeping with the proper SEG-Y format, this
//! is necessary. On the other hand, using this functionality can easily break things.
use crate::enums::{MeasurementSystem, OrderTraceBy, SampleFormatCode, TraceIdCode};
use crate::errors::*;
use crate::{
    CDPX_BYTE_LOCATION, CDPY_BYTE_LOCATION, CROSSLINE_BYTE_LOCATION, INLINE_BYTE_LOCATION,
    TRACE_HEADER_LEN,
};
#[cfg(feature = "to_json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "to_json", derive(Serialize, Deserialize))]
/// This structure holds a list of various settings to be imported, for the custom reading of
/// byte locations of various variables in the headers and other things when interpreting a SEGY file.
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

    /// Sets the endiannss to LE if true and BE if false
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

    /// Sets the trace format to the input. NB: This may return an error if the format code is for
    /// a format which is not four bytes long (because that would raise more questions than it answers).
    pub fn set_override_coordinate_format(
        &mut self,
        format: SampleFormatCode,
    ) -> Result<(), RsgError> {
        use SampleFormatCode::*;
        match format {
            IbmFloat32 | Float32 | UInt32 | Int32 => self.override_coordinate_format = Some(format),
            _ => {
                return Err(RsgError::BitConversionError {
                    msg: format!("Coordinate format must be 32-byte. {:?} is not", format),
                })
            }
        }
        Ok(())
    }

    /// Sets the coordinate scaling as overridden by the value.
    pub fn set_override_coordinate_scaling(&mut self, scaling: f64) -> Result<(), RsgError> {
        use num::FromPrimitive;

        let scaling = i16::from_f64(scaling).ok_or_else(|| RsgError::BitConversionError {
            msg: format!("{} is outside of the scaling range.", scaling),
        })?;
        self.override_coordinate_scaling = Some(scaling);
        Ok(())
    }

    /// Sets the inline number byte index as overridden by the value.
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

    /// Sets the override for crossline count.
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

    /// Get the physical `u` vector.
    pub fn set_override_u(&mut self, u: [f64; 3]) {
        self.override_u = Some(u)
    }

    /// Get the physical `v` vector.
    pub fn set_override_v(&mut self, v: [f64; 3]) {
        self.override_v = Some(v);
    }

    /// Get the physcial sample interval (NB: one dimensionsal by definition.)
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