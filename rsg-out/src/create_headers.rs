// Copyright (C) 2022 by GiGa infosystems
//! This here to make pseudo-default headers.
use rsg_core::enums::*;
use rsg_core::{BinHeader, TapeLabel, TraceHeader};

use crate::SegyHeaderToBytes;

pub trait CreateBinHeader: SegyHeaderToBytes {
    /// Creates an empty binary header.
    fn default() -> Self;

    /// Create a new binary header with basic information.
    fn new(
        no_traces: u16,
        sampsle_interval: u16,
        no_samples: u16,
        sample_format_code: SampleFormatCode,
    ) -> Self;

    /// Set the measurement system.
    fn set_measurement_system(&mut self, measurement_system: MeasurementSystem);

    /// Set binary direction as LE.
    fn switch_binary_flag_to_le(&mut self);
}

pub trait CreateTraceHeader {
    /// Creates an empty trace header.
    fn default() -> Self;

    /// Create a trace header using bin header and other data.
    ///
    /// NB: This function assumes that x and y coordinates are already split into
    /// ensemble and scalar.
    fn new_2d(x_ensemble: i32, y_ensemble: i32, coordinate_scalar: i16) -> Self;

    /// Create a trace header using bin header and other data.
    ///
    /// NB: This function assumes that x and y coordinates are already split into
    /// ensemble and scalar.
    fn new_3d(
        x_ensemble: i32,
        y_ensemble: i32,
        inline_no: i32,
        xline_no: i32,
        coordinate_scalar: i16,
    ) -> Self;
}

/// Creates an empty tape label.
pub fn create_tape_label() -> TapeLabel {
    TapeLabel {
        storage_unit_seq_no: [0; 4],
        segy_revision_no: [0; 5],
        storage_unit_structure: *b"RECORD",
        binding_number: *b"BXXX",
        max_block_size: u32::MAX,
        producing_organisation_code: [0; 10],
        creation_date: [0; 11],
        serial_number: [0; 12],
        external_label: [0; 12],
        recording_entity: [0; 24],
        extra: [0; 14],
    }
}

impl CreateBinHeader for BinHeader {
    fn default() -> Self {
        create_default_bin_header()
    }

    fn new(
        no_traces: u16,
        sample_interval: u16,
        no_samples: u16,
        sample_format_code: SampleFormatCode,
    ) -> Self {
        let mut header = create_default_bin_header();
        header.no_traces = no_traces;
        header.sample_interval = sample_interval;
        header.no_samples = no_samples;
        header.sample_format_code = sample_format_code;
        header
    }

    fn set_measurement_system(&mut self, measurement_system: MeasurementSystem) {
        self.measurement_system = measurement_system;
    }

    fn switch_binary_flag_to_le(&mut self) {
        self.binary_flag_direction_is_le = true;
    }
}

/// This creates a default instance of `BinHeader`
///
/// NB: This is not default in accordance with the standard.
/// This is a default, where all enums are `Unknonw`/`Unspecified` and values are 0.
fn create_default_bin_header() -> BinHeader {
    BinHeader {
        job_id: 0,
        line_number: 0,
        reel_number: 0,
        no_traces: 0,
        no_aux_traces: 0,
        sample_interval: 0,
        sample_interval_original: 0,
        no_samples: 0,
        no_samples_original: 0,
        sample_format_code: SampleFormatCode::Float32,
        ensemble_fold: 0,
        sorting_code: TraceSortingCode::Unknown,
        vertical_sum: 0,
        sweep_frequency_start: 0,
        sweep_frequency_end: 0,
        sweep_length: 0,
        sweep_type: SweepTypeCode::Unspecified,
        sweep_channel_trace_no: 0,
        sweep_taper_at_start: 0,
        sweep_taper_at_end: 0,
        taper_type: TaperType::Unspecified,
        correlated_traces: CorrelatedDataTraces::Unspecified,
        binary_gain_recovered: BinaryGainRecovered::Unspecified,
        amplitude_recovery_method: AmplitudeRecoveryMethod::Unspecified,
        measurement_system: MeasurementSystem::Unspecified,
        impulse_signal_polarity: ImpulseSignalPolarity::Unspecified,
        vibratory_polarity_code: VibratoryPolarityCode::Unspecified,
        /// Combines minor and major revision code.
        segy_revision_number: [2, 0],
        fixed_length_trace_flag: FixedLengthTraces::No,
        extended_header_count: 0,
        time_basis_code: TimeBasisCode::Unspecified,
        binary_flag_direction_is_le: false,
    }
}

impl CreateTraceHeader for TraceHeader {
    fn default() -> Self {
        create_default_trace_header()
    }

    fn new_2d(x_ensemble: i32, y_ensemble: i32, coordinate_scalar: i16) -> Self {
        let mut header = create_default_trace_header();
        header.x_ensemble = x_ensemble;
        header.y_ensemble = y_ensemble;
        header.coordinate_scalar = coordinate_scalar;
        header
    }

    fn new_3d(
        x_ensemble: i32,
        y_ensemble: i32,
        inline_no: i32,
        xline_no: i32,
        coordinate_scalar: i16,
    ) -> Self {
        let mut header = create_default_trace_header();
        header.inline_no = inline_no;
        header.crossline_no = xline_no;
        header.x_ensemble = x_ensemble;
        header.y_ensemble = y_ensemble;
        header.coordinate_scalar = coordinate_scalar;
        header
    }
}

/// Creates an empty instance of `TraceHeader`.
///
/// NB: This is not default in accordance with the standard.
/// This is a default, where all enums are `Unknonw`/`Unspecified` and values are 0.
fn create_default_trace_header() -> TraceHeader {
    TraceHeader {
        trace_sequence_on_line: 0,
        trace_sequence_in_file: 0,
        field_record_no: 0,
        trace_no: 0,
        energy_source_point_no: 0,
        ensemble_no: 0,
        trace_no_in_ensemble: 0,
        trace_identification_code: TraceIdCode::Unknown,
        no_v_summed_traces: 0,
        no_h_stacked_traces: 0,
        data_use: DataUse::Unspecified,
        source_to_receiver_distance: 0,
        elevation_of_receiver_group: 0,
        surface_elevation_of_source: 0,
        source_depth: 0,
        datum_elevation_of_receiver_group: 0,
        datum_elevation_of_source: 0,
        water_column_height_at_source: 0,
        water_column_height_at_group: 0,
        elevation_scalar: 0,
        coordinate_scalar: 0,
        source_x: 0,
        source_y: 0,
        receiver_group_x: 0,
        receiver_group_y: 0,
        coordinate_units: CoordinateUnits::Unspecified,
        weathing_velocity: 0,
        sub_weathering_velocity: 0,
        uphole_time_at_source: 0,
        uphole_time_at_group: 0,
        source_static_correction: 0,
        group_static_correction: 0,
        total_static_applied: 0,
        lag_time_a: 0,
        lag_time_b: 0,
        delay_recording_time: 0,
        mute_time_start: 0,
        mute_time_end: 0,
        no_samples_in_trace: 0,
        sample_interval_of_trace: 0,
        gain_type: GainType::Unspecified,
        instrument_gain_constant: 0,
        instrument_initial_gain: 0,
        correlated: Correlated::Unspecified,
        sweep_frequency_at_start: 0,
        sweep_frequency_at_end: 0,
        sweep_length: 0,
        sweep_type: SweepType::Unspecified,
        sweep_trace_taper_length_at_start: 0,
        sweep_trace_taper_length_at_end: 0,
        taper_type: TaperType::Unspecified,
        alias_filter_frequency: 0,
        alias_filter_slope: 0,
        notch_filter_frequency: 0,
        notch_filter_slope: 0,
        low_cut_frequency: 0,
        high_cut_frequency: 0,
        low_cut_slope: 0,
        high_cut_slope: 0,
        year_recorded: 0,
        day_of_year: 0,
        hour_of_day: 0,
        minute_of_hour: 0,
        second_of_minute: 0,
        time_base_code: TimeBasisCode::Unspecified,
        trace_weighting_factor: 0,
        geophone_group_number_roll_pos1: 0,
        geophone_group_number_first_trace_orig_field: 0,
        geophone_group_number_last_trace_orig_field: 0,
        gap_size: 0,
        over_travel: OverTravel::Unspecified,
        // Ensemble=CDP.
        x_ensemble: 0,
        y_ensemble: 0,
        inline_no: 0,
        crossline_no: 0,
        shot_point_no: 0,
        shot_point_scalar: 0,
        trace_value_measurement_unit: TraceValueUnit::Unknown,
        transduction_constant_mantissa: 0,
        transduction_constant_power: 0,
        transduction_units: TransductionUnits::Unknown,
        trace_identifier: 0,
        time_scalar_trace_header: 0,
        source_type: SourceType::Unknown,
        source_energy_direction_v: 0,
        source_energy_direction_il: 0,
        source_energy_direction_xl: 0,
        source_measurement_mantissa: 0,
        source_measurement_exponent: 0,
        source_measurement_unit: SourceMeasurementUnit::Unknown,
        trace_name: [0; 8],
    }
}

#[cfg(test)]
mod tests {
    use crate::create_headers::*;
    #[test]
    fn create_bin_header_1() {
        let header = BinHeader::new(1600, 5, 500, SampleFormatCode::Float32);
        assert_eq!(header.no_traces, 1600);
        assert_eq!(header.sample_interval, 5);
        assert_eq!(header.no_samples, 500);
        assert_eq!(header.sample_format_code, SampleFormatCode::Float32);
    }

    #[test]
    fn create_bin_header_2() {
        let header = BinHeader::new(123, 4, 567, SampleFormatCode::Float64);
        assert_eq!(header.no_traces, 123);
        assert_eq!(header.sample_interval, 4);
        assert_eq!(header.no_samples, 567);
        assert_eq!(header.sample_format_code, SampleFormatCode::Float64);
    }

    #[test]
    fn create_bin_header_default() {
        let header = BinHeader::default();
        assert_eq!(header.no_traces, 0);
        assert_eq!(header.sample_interval, 0);
        assert_eq!(header.no_samples, 0);
        assert_eq!(header.sample_format_code, SampleFormatCode::Float32);
    }

    #[test]
    fn create_trace_header_2d_1() {
        let header = TraceHeader::new_2d(889000, 623000, 2);
        assert_eq!(header.x_ensemble, 889000);
        assert_eq!(header.y_ensemble, 623000);
        assert_eq!(header.coordinate_scalar, 2);
    }

    #[test]
    fn create_trace_header_2d_2() {
        let header = TraceHeader::new_2d(45, 23, -5);
        assert_eq!(header.x_ensemble, 45);
        assert_eq!(header.y_ensemble, 23);
        assert_eq!(header.coordinate_scalar, -5);
    }

    #[test]
    fn create_trace_header_3d_1() {
        let header = TraceHeader::new_3d(889000, 623000, 2300, 1898, 2);
        assert_eq!(header.x_ensemble, 889000);
        assert_eq!(header.y_ensemble, 623000);
        assert_eq!(header.inline_no, 2300);
        assert_eq!(header.crossline_no, 1898);
        assert_eq!(header.coordinate_scalar, 2);
    }

    #[test]
    fn create_trace_header_3d_2() {
        let header = TraceHeader::new_3d(45, 23, 999, -5000, -5);
        assert_eq!(header.x_ensemble, 45);
        assert_eq!(header.y_ensemble, 23);
        assert_eq!(header.inline_no, 999);
        assert_eq!(header.crossline_no, -5000);
        assert_eq!(header.coordinate_scalar, -5);
    }

    #[test]
    fn create_trace_header_default() {
        let header = TraceHeader::default();
        assert_eq!(header.x_ensemble, 0);
        assert_eq!(header.y_ensemble, 0);
        assert_eq!(header.coordinate_scalar, 0);
    }
}
