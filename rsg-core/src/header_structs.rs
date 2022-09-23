//! This file contains the definitions for the binary headers of a SEGY file. These can then be
//! used for better interpreting the file in the parser.
use crate::bitconverter::ascii_bytes_to_string;
use crate::enums::*;
use crate::SegySettings;
#[cfg(feature = "to_json")]
use crate::RsgError;

use encoding8::ebcdic::to_ascii;
#[cfg(feature = "to_json")]
use serde::{Deserialize, Serialize};

/// This structure represents a parsed binary trace header for a single trace of a SEGY file..
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "to_json", derive(Serialize, Deserialize))]
pub struct TraceHeader {
    pub trace_sequence_on_line: i32,
    pub trace_sequence_in_file: i32,
    pub field_record_no: i32,
    pub trace_no: i32,
    pub energy_source_point_no: i32,
    pub ensemble_no: i32,
    pub trace_no_in_ensemble: i32,
    pub trace_identification_code: TraceIdCode,
    pub no_v_summed_traces: u16,
    pub no_h_stacked_traces: u16,
    pub data_use: DataUse,
    pub source_to_receiver_distance: i32,
    pub elevation_of_receiver_group: i32,
    pub surface_elevation_of_source: i32,
    pub source_depth: i32,
    pub datum_elevation_of_receiver_group: i32,
    pub datum_elevation_of_source: i32,
    pub water_column_height_at_source: i32,
    pub water_column_height_at_group: i32,
    pub elevation_scalar: i16,
    pub coordinate_scalar: i16,
    pub source_x: i32,
    pub source_y: i32,
    pub receiver_group_x: i32,
    pub receiver_group_y: i32,
    pub coordinate_units: CoordinateUnits,
    pub weathing_velocity: u16,
    pub sub_weathering_velocity: u16,
    pub uphole_time_at_source: u16,
    pub uphole_time_at_group: u16,
    pub source_static_correction: u16,
    pub group_static_correction: u16,
    pub total_static_applied: u16,
    pub lag_time_a: u16,
    pub lag_time_b: u16,
    pub delay_recording_time: u16,
    pub mute_time_start: u16,
    pub mute_time_end: u16,
    pub no_samples_in_trace: u16,
    pub sample_interval_of_trace: u16,
    pub gain_type: GainType,
    pub instrument_gain_constant: u16,
    pub instrument_initial_gain: u16,
    pub correlated: Correlated,
    pub sweep_frequency_at_start: u16,
    pub sweep_frequency_at_end: u16,
    pub sweep_length: u16,
    pub sweep_type: SweepType,
    pub sweep_trace_taper_length_at_start: u16,
    pub sweep_trace_taper_length_at_end: u16,
    pub taper_type: TaperType,
    pub alias_filter_frequency: u16,
    pub alias_filter_slope: u16,
    pub notch_filter_frequency: u16,
    pub notch_filter_slope: u16,
    pub low_cut_frequency: u16,
    pub high_cut_frequency: u16,
    pub low_cut_slope: u16,
    pub high_cut_slope: u16,
    pub year_recorded: u16,
    pub day_of_year: u16,
    pub hour_of_day: u16,
    pub minute_of_hour: u16,
    pub second_of_minute: u16,
    pub time_base_code: TimeBasisCode,
    pub trace_weighting_factor: u16,
    pub geophone_group_number_roll_pos1: u16,
    pub geophone_group_number_first_trace_orig_field: u16,
    pub geophone_group_number_last_trace_orig_field: u16,
    pub gap_size: u16,
    pub over_travel: OverTravel,
    // Ensemble=CDP.
    pub x_ensemble: i32,
    pub y_ensemble: i32,
    pub inline_no: i32,
    pub crossline_no: i32,
    pub shot_point_no: i32,
    pub shot_point_scalar: u16,
    pub trace_value_measurement_unit: TraceValueUnit,
    pub transduction_constant_mantissa: i32,
    pub transduction_constant_power: u16,
    pub transduction_units: TransductionUnits,
    pub trace_identifier: u16,
    pub time_scalar_trace_header: u16,
    pub source_type: SourceType,
    pub source_energy_direction_v: u16,
    pub source_energy_direction_il: u16,
    pub source_energy_direction_xl: u16,
    pub source_measurement_mantissa: i32,
    pub source_measurement_exponent: u16,
    pub source_measurement_unit: SourceMeasurementUnit,
    pub trace_name: [u8; 8],
}

/// This structure represents a parsed binary header for a SEGY file.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "to_json", derive(Serialize, Deserialize))]
pub struct BinHeader {
    pub job_id: u32,
    pub line_number: u32,
    pub reel_number: u32,
    pub no_traces: u16,
    pub no_aux_traces: u16,
    pub sample_interval: u16,
    pub sample_interval_original: u16,
    pub no_samples: u16,
    pub no_samples_original: u16,
    pub sample_format_code: SampleFormatCode,
    pub ensemble_fold: u16,
    pub sorting_code: TraceSortingCode,
    pub vertical_sum: u16,
    pub sweep_frequency_start: u16,
    pub sweep_frequency_end: u16,
    pub sweep_length: u16,
    pub sweep_type: SweepTypeCode,
    pub sweep_channel_trace_no: u16,
    pub sweep_taper_at_start: u16,
    pub sweep_taper_at_end: u16,
    pub taper_type: TaperType,
    pub correlated_traces: CorrelatedDataTraces,
    pub binary_gain_recovered: BinaryGainRecovered,
    pub amplitude_recovery_method: AmplitudeRecoveryMethod,
    pub measurement_system: MeasurementSystem,
    pub impulse_signal_polarity: ImpulseSignalPolarity,
    pub vibratory_polarity_code: VibratoryPolarityCode,
    /// Combines minor and major revision code.
    pub segy_revision_number: [u8; 2],
    pub fixed_length_trace_flag: FixedLengthTraces,
    pub extended_header_count: u32,
    pub time_basis_code: TimeBasisCode,
    pub binary_flag_direction_is_le: bool,
}

/// This structure represents the 128-byte SEG-Y tape label which is largely optional.
/// It appears that this is stored mostly as character bytes (u8).
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TapeLabel {
    /// Bytes 1-4 (0..4)
    pub storage_unit_seq_no: [u8; 4],
    /// Bytes 5-9 (4..9)
    pub segy_revision_no: [u8; 5],
    /// Storage Unit Type 1-15 (9..15) b"RECORD"
    pub storage_unit_structure: [u8; 6],
    /// Binding Edition 16-19 (15..19) (b"BXXX")
    pub binding_number: [u8; 4],
    /// Max Block Size: 20-29 (19..29) Stored as character bytes. Needs to be parsed.
    pub max_block_size: u32,
    /// Producing Organisation code: 29-39. Stored as character bytes.
    pub producing_organisation_code: [u8; 10],
    /// Creation Date: 40-50 (39..50).
    pub creation_date: [u8; 11],
    /// Serial Number: 51-62 (50..62).
    pub serial_number: [u8; 12],
    /// Reserved: 63-68 (62..68).?
    /// Storage Set Identifier:
    /// External Label Name: 69-80 (68..80)
    pub external_label: [u8; 12],
    /// Recording Entity Name (81-104 (80..104).
    pub recording_entity: [u8; 24],
    /// User Defined: 105-118 (104..118)
    pub extra: [u8; 14],
    // The last ten bytes are reserved.
}

/// This is a rust readable version of the `TapeLabel` structure, which can be generated after the file
/// has been read, but is not stored.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "to_json", derive(Serialize, Deserialize))]
pub struct ReadableTapeLabel {
    pub storage_unit_seq_no: String,
    pub segy_revision_no: String,
    pub storage_unit_structure: String,
    pub binding_number: String,
    pub max_block_size: u32,
    pub producing_organisation_code: String,
    pub creation_date: String,
    pub serial_number: String,
    pub external_label: String,
    pub recording_entity: String,
    pub extra: String,
}

impl TapeLabel {
    /// Converts the C compatible `TapeLabel` to a rust compatible `ReadableTapeLabel`
    pub fn to_readable(&self) -> ReadableTapeLabel {
        ReadableTapeLabel {
            storage_unit_seq_no: ascii_bytes_to_string(&self.storage_unit_seq_no),
            segy_revision_no: ascii_bytes_to_string(&self.segy_revision_no),
            storage_unit_structure: ascii_bytes_to_string(&self.storage_unit_structure),
            binding_number: ascii_bytes_to_string(&self.binding_number),
            max_block_size: self.max_block_size,
            producing_organisation_code: ascii_bytes_to_string(&self.producing_organisation_code),
            creation_date: ascii_bytes_to_string(&self.creation_date),
            serial_number: ascii_bytes_to_string(&self.serial_number),
            external_label: ascii_bytes_to_string(&self.external_label),
            recording_entity: ascii_bytes_to_string(&self.recording_entity),
            extra: ascii_bytes_to_string(&self.extra),
        }
    }

    #[cfg(feature = "to_json")]
    pub fn to_json(&self) -> Result<String, RsgError> {
        serde_json::to_string(&self.to_readable()).map_err(RsgError::SerdeError)
    }
}

impl BinHeader {
    pub fn adjust_sample_count(&mut self, settings: &SegySettings) {
        if let Some(dim_z) = settings.override_dim_z {
            self.no_samples = dim_z as u16;
            self.fixed_length_trace_flag = FixedLengthTraces::Yes;
        }
    }

    #[cfg(feature = "to_json")]
    pub fn to_json(&self) -> Result<String, RsgError> {
        serde_json::to_string(&self).map_err(RsgError::SerdeError)
    }
}

impl TraceHeader {
    pub fn adjust_sample_count(&mut self, settings: &SegySettings) {
        if let Some(dim_z) = settings.override_dim_z {
            self.no_samples_in_trace = dim_z as u16;
        }
    }

    /// This gets the trace name as a String.
    pub fn get_trace_name(&self) -> String {
        // Trace name should start with "SEG", or just be blank.
        let is_ascii = self.trace_name[0] == b'S';
        if is_ascii {
            return ascii_bytes_to_string(&self.trace_name);
        }
        let name = self
            .trace_name
            .iter()
            .map(|c| to_ascii(*c))
            .collect::<Vec<_>>();
        ascii_bytes_to_string(&name)
    }

    #[cfg(feature = "to_json")]
    pub fn to_json(&self) -> Result<String, RsgError> {
        serde_json::to_string(&self).map_err(RsgError::SerdeError)
    }
}

impl std::fmt::Display for BinHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let b = &self;
        write!(
            f,
            "amplitude recovery method: {:?}
binary flag direction is le: {}
binary gain recovered: {:?}
correlated traces: {:?}
ensemble fold: {}
extended header count: {}
fixed length trace flag: {:?}
impulse signal polarity: {:?}
job id: {:?}
line number: {:?}
measurement system:{:?}
no aux traces: {}
no samples original: {}
no samples: {}
no traces: {}
reel number: {}
sample format code: {:?}
sample interval original: {}
sample interval: {}
segy revision number: {}.{}
sorting code: {:?}
sweep channel trace no: {:?}
sweep frequency end: {}
sweep frequency start: {}
sweep length: {}
sweep taper at end: {:?}
sweep taper at start: {:?}
sweep type: {:?}
taper type: {:?}
time basis code: {:?}
vertical sum: {}
vibratory polarity code: {:?}",
            b.amplitude_recovery_method,
            b.binary_flag_direction_is_le,
            b.binary_gain_recovered,
            b.correlated_traces,
            b.ensemble_fold,
            b.extended_header_count,
            b.fixed_length_trace_flag,
            b.impulse_signal_polarity,
            b.job_id,
            b.line_number,
            b.measurement_system,
            b.no_aux_traces,
            b.no_samples_original,
            b.no_samples,
            b.no_traces,
            b.reel_number,
            b.sample_format_code,
            b.sample_interval_original,
            b.sample_interval,
            b.segy_revision_number[0],
            b.segy_revision_number[1],
            b.sorting_code,
            b.sweep_channel_trace_no,
            b.sweep_frequency_end,
            b.sweep_frequency_start,
            b.sweep_length,
            b.sweep_taper_at_end,
            b.sweep_taper_at_start,
            b.sweep_type,
            b.taper_type,
            b.time_basis_code,
            b.vertical_sum,
            b.vibratory_polarity_code,
        )
    }
}

impl std::fmt::Display for TraceHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let t = &self;
        write!(
            f,
            "trace sequence on line: {}
trace sequence in file: {}
field record no: {}
trace no: {}
energy source point no: {}
ensemble no: {}
trace no in ensemble: {}
trace identification code: {:?},
no v summed traces: {}
no h stacked traces: {}
data use: {:?}
source to receiver distance: {}
elevation of receiver group: {}
surface elevation of source: {}
source depth: {}
datum elevation of receiver group: {}
datum elevation of source: {}
water column height at source: {}
water column height at group: {}
elevation scalar: {},
coordinate scalar: {},
source x: {}
source y: {}
receiver group x: {}
receiver group y: {}
coordinate units: {:?}
weathing velocity: {}
sub weathering velocity: {}
uphole time at source: {}
uphole time at group: {}
source static correction: {}
group static correction: {}
total static applied: {}
lag time a: {}
lag time b: {}
delay recording time: {}
mute time start: {}
mute time end: {}
no samples in trace: {}
sample interval of trace: {}
gain type: {:?}
instrument gain constant: {}
instrument initial gain: {}
correlated: {:?}
sweep frequency at start: {}
sweep frequency at end: {}
sweep length: {}
sweep type: {:?}
sweep trace taper length at start: {}
sweep trace taper length at end: {}
taper type: {:?}
alias filter frequency: {}
alias filter slope: {}
notch filter frequency: {}
notch filter slope: {}
low cut frequency: {}
high cut frequency: {}
low cut slope: {}
high cut slope: {}
year recorded: {}
day of year: {}
hour of day: {}
minute of hour: {}
second of minute: {}
time base code: {:?}
trace weighting factor: {}
geophone group number roll pos1: {}
geophone group number first trace orig field: {}
geophone group number last trace orig field: {}
gap size: {}
over travel: {:?}
x ensemble: {}
y ensemble: {}
inline no: {}
crossline no: {}
shot point no: {}
shot point scalar: {}
trace value measurement unit: {:?}
transduction constant mantissa: {}
transduction constant power: {}
transduction units: {:?}
trace identifier: {}
time scalar trace header: {}
source type: {:?}
source energy direction v: {}
source energy direction il: {}
source energy direction xl: {}
source measurement mantissa: {}
source measurement exponent: {}
source measurement unit: {:?}
trace name: {}",
            t.trace_sequence_on_line,
            t.trace_sequence_in_file,
            t.field_record_no,
            t.trace_no,
            t.energy_source_point_no,
            t.ensemble_no,
            t.trace_no_in_ensemble,
            t.trace_identification_code,
            t.no_v_summed_traces,
            t.no_h_stacked_traces,
            t.data_use,
            t.source_to_receiver_distance,
            t.elevation_of_receiver_group,
            t.surface_elevation_of_source,
            t.source_depth,
            t.datum_elevation_of_receiver_group,
            t.datum_elevation_of_source,
            t.water_column_height_at_source,
            t.water_column_height_at_group,
            t.elevation_scalar,
            t.coordinate_scalar,
            t.source_x,
            t.source_y,
            t.receiver_group_x,
            t.receiver_group_y,
            t.coordinate_units,
            t.weathing_velocity,
            t.sub_weathering_velocity,
            t.uphole_time_at_source,
            t.uphole_time_at_group,
            t.source_static_correction,
            t.group_static_correction,
            t.total_static_applied,
            t.lag_time_a,
            t.lag_time_b,
            t.delay_recording_time,
            t.mute_time_start,
            t.mute_time_end,
            t.no_samples_in_trace,
            t.sample_interval_of_trace,
            t.gain_type,
            t.instrument_gain_constant,
            t.instrument_initial_gain,
            t.correlated,
            t.sweep_frequency_at_start,
            t.sweep_frequency_at_end,
            t.sweep_length,
            t.sweep_type,
            t.sweep_trace_taper_length_at_start,
            t.sweep_trace_taper_length_at_end,
            t.taper_type,
            t.alias_filter_frequency,
            t.alias_filter_slope,
            t.notch_filter_frequency,
            t.notch_filter_slope,
            t.low_cut_frequency,
            t.high_cut_frequency,
            t.low_cut_slope,
            t.high_cut_slope,
            t.year_recorded,
            t.day_of_year,
            t.hour_of_day,
            t.minute_of_hour,
            t.second_of_minute,
            t.time_base_code,
            t.trace_weighting_factor,
            t.geophone_group_number_roll_pos1,
            t.geophone_group_number_first_trace_orig_field,
            t.geophone_group_number_last_trace_orig_field,
            t.gap_size,
            t.over_travel,
            t.x_ensemble,
            t.y_ensemble,
            t.inline_no,
            t.crossline_no,
            t.shot_point_no,
            t.shot_point_scalar,
            t.trace_value_measurement_unit,
            t.transduction_constant_mantissa,
            t.transduction_constant_power,
            t.transduction_units,
            t.trace_identifier,
            t.time_scalar_trace_header,
            t.source_type,
            t.source_energy_direction_v,
            t.source_energy_direction_il,
            t.source_energy_direction_xl,
            t.source_measurement_mantissa,
            t.source_measurement_exponent,
            t.source_measurement_unit,
            c_safe_name(&t.trace_name),
        )
    }
}

pub(crate) fn c_safe_name(name: &[u8]) -> String {
    if name.iter().any(|c| *c == 0) {
        "".to_owned()
    } else {
        String::from_utf8(name.to_vec()).unwrap_or_else(|_| "".to_owned())
    }
}
