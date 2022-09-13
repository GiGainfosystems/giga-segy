// Copyright (C) 2022 by GiGa infosystems
//! This contains the specific settings for writing the SEGY file.
use rsg_core::enums::*;
use rsg_core::errors::*;
use rsg_core::SegySettings;

/// This trait allows any structure that implements a few functions to be used
/// For settings. Most importantly, it allows the rust_segy_input::SegySettings
/// to be used for this purpose.
///
/// NB: We only allow the setting of:
///
/// 1) coord-format. (same for X, Y and Z)
///
/// 2) X and Y ensemble bidx.
///
/// 3) inline and crossline no bidx
///
/// Other settings cannot be overridden when creating a segy file, because
/// the API user has direct access to the `BinHeader` and `TraceHeader` fields
/// when creating them, thus over-riding them seems counterintuitive (why override
/// something that you had the chance to set yourself a few lines earlier?).
///
/// NB2: Changing the settings for byte indices will the TraceHeader in ways that
/// Will override other settings without compensating for the changes.
pub trait SegyWriteSettings {
    fn get_override_coordinate_format(&self) -> Option<SampleFormatCode>;

    /// Get the byte index of the inline number.
    fn get_inline_no_bidx(&self) -> usize;

    /// Get the byte index of the crossline number.
    fn get_crossline_no_bidx(&self) -> usize;

    /// Get the byte index of the x-ensemble (x-CDP) number.
    fn get_x_ensemble_bidx(&self) -> usize;

    /// Gets the y-ensemble (y-CDP) byte index.
    fn get_y_ensemble_bidx(&self) -> usize;

    /// Sets the inline number byte index as overridden by the value.
    fn set_inline_no_bidx(&mut self, bidx: usize) -> Result<(), RsgError>;

    /// Sets the crossline number byte index as overridden by the value.
    fn set_crossline_no_bidx(&mut self, bidx: usize) -> Result<(), RsgError>;

    /// Sets the x-ensemble (x-CDP) as overridden by the input value.
    fn set_x_ensemble_bidx(&mut self, bidx: usize) -> Result<(), RsgError>;

    /// Sets the y-ensemble (y-CDP) as overridden by the input value.
    fn set_y_ensemble_bidx(&mut self, bidx: usize) -> Result<(), RsgError>;

    /// Sets the trace format to the input. NB: This may return an error if the format code is for
    /// a format which is not four bytes long (because that would raise more questions than it answers).
    fn set_override_coordinate_format(&mut self, format: SampleFormatCode) -> Result<(), RsgError>;
}

impl SegyWriteSettings for SegySettings {
    fn get_override_coordinate_format(&self) -> Option<SampleFormatCode> {
        self.get_override_coordinate_format()
    }

    fn get_inline_no_bidx(&self) -> usize {
        self.get_inline_no_bidx()
    }

    fn get_crossline_no_bidx(&self) -> usize {
        self.get_crossline_no_bidx()
    }

    fn get_x_ensemble_bidx(&self) -> usize {
        self.get_x_ensemble_bidx()
    }

    fn get_y_ensemble_bidx(&self) -> usize {
        self.get_y_ensemble_bidx()
    }

    fn set_inline_no_bidx(&mut self, bidx: usize) -> Result<(), RsgError> {
        self.set_inline_no_bidx(bidx)
    }

    fn set_crossline_no_bidx(&mut self, bidx: usize) -> Result<(), RsgError> {
        self.set_crossline_no_bidx(bidx)
    }

    fn set_x_ensemble_bidx(&mut self, bidx: usize) -> Result<(), RsgError> {
        self.set_x_ensemble_bidx(bidx)
    }

    fn set_y_ensemble_bidx(&mut self, bidx: usize) -> Result<(), RsgError> {
        self.set_y_ensemble_bidx(bidx)
    }

    fn set_override_coordinate_format(&mut self, format: SampleFormatCode) -> Result<(), RsgError> {
        self.set_override_coordinate_format(format)
    }
}
