//! This file contains the definitions for the binary headers of a SEG-Y file. These can then be
//! used for better interpreting the file in the parser.
use crate::bitconverter::ascii_bytes_to_string;
use crate::enums::*;
#[cfg(feature = "to_json")]
use crate::RsgError;
use crate::SegySettings;

use encoding8::ebcdic::to_ascii;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// This structure represents a parsed binary trace header for a single trace of a SEG-Y file..
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TraceHeader {
    /// Bytes 1 - 4 (0..4) of the trace header.
    pub trace_sequence_on_line: i32,
    /// Bytes 5 - 8 (4..8) of the trace header.
    pub trace_sequence_in_file: i32,
    /// Bytes 9 - 12 (8..12) of the trace header.
    pub field_record_no: i32,
    /// Bytes 13 - 16 (12..16) of the trace header.
    pub trace_no: i32,
    /// Bytes 17 - 20 (16..20) of the trace header.
    pub energy_source_point_no: i32,
    /// Bytes 21 - 24 (20..24) of the trace header.
    pub ensemble_no: i32,
    /// Bytes 25 - 28 (24..28) of the trace header.
    pub trace_no_in_ensemble: i32,
    /// Bytes 29 - 30 (28..30) of the trace header.
    pub trace_identification_code: TraceIdCode,
    /// Bytes 31 - 32 (30..32) of the trace header.
    pub no_v_summed_traces: u16,
    /// Bytes 33 - 34 (32..34) of the trace header.
    pub no_h_stacked_traces: u16,
    /// Bytes 35 - 36 (34..36) of the trace header.
    pub data_use: DataUse,
    /// Bytes 37 - 40 (36..40) of the trace header.
    pub source_to_receiver_distance: i32,
    /// Bytes 41 - 44 (40..44) of the trace header.
    pub elevation_of_receiver_group: i32,
    /// Bytes 45 - 48 (44..48) of the trace header.
    pub surface_elevation_of_source: i32,
    /// Bytes 49 - 52 (48..52) of the trace header.
    pub source_depth: i32,
    /// Bytes 53 - 56 (52..56) of the trace header.
    pub datum_elevation_of_receiver_group: i32,
    /// Bytes 57 - 60 (56..60) of the trace header.
    pub datum_elevation_of_source: i32,
    /// Bytes 61 - 64 (60..64) of the trace header.
    pub water_column_height_at_source: i32,
    /// Bytes 65 - 68 (64..68) of the trace header.
    pub water_column_height_at_group: i32,
    /// Bytes 69 - 70 (68..70) of the trace header.
    pub elevation_scalar: i16,
    /// Bytes 71 - 72 (70..72) of the trace header.
    pub coordinate_scalar: i16,
    /// Bytes 73 - 76 (72..76) of the trace header.
    pub source_x: i32,
    /// Bytes 77 - 80 (76..80) of the trace header.
    pub source_y: i32,
    /// Bytes 81 - 84 (80..84) of the trace header.
    pub receiver_group_x: i32,
    /// Bytes 85 - 88 (84..88) of the trace header.
    pub receiver_group_y: i32,
    /// Bytes 89 - 90 (88..90) of the trace header.
    pub coordinate_units: CoordinateUnits,
    /// Bytes 91 - 92 (90..92) of the trace header.
    pub weathing_velocity: u16,
    /// Bytes 93 - 94 (92..94) of the trace header.
    pub sub_weathering_velocity: u16,
    /// Bytes 95 - 96 (94..96) of the trace header.
    pub uphole_time_at_source: u16,
    /// Bytes 97 - 98 (96..98) of the trace header.
    pub uphole_time_at_group: u16,
    /// Bytes 99 - 100 (98..100) of the trace header.
    pub source_static_correction: u16,
    /// Bytes 101 - 102 (100..102) of the trace header.
    pub group_static_correction: u16,
    /// Bytes 103 - 104 (102..104) of the trace header.
    pub total_static_applied: u16,
    /// Bytes 105 - 106 (104..106) of the trace header.
    pub lag_time_a: u16,
    /// Bytes 107 - 108 (106..108) of the trace header.
    pub lag_time_b: u16,
    /// Bytes 109 - 110 (108..110) of the trace header.
    pub delay_recording_time: u16,
    /// Bytes 111 - 112 (110..112) of the trace header.
    pub mute_time_start: u16,
    /// Bytes 113 - 114 (112..114) of the trace header.
    pub mute_time_end: u16,
    /// Bytes 115 - 116 (114..116) of the trace header.
    pub no_samples_in_trace: u16,
    /// Bytes 117 - 118 (116..118) of the trace header.
    pub sample_interval_of_trace: u16,
    /// Bytes 119 - 120 (118..120) of the trace header.
    pub gain_type: GainType,
    /// Bytes 121 - 122 (120..122) of the trace header.
    pub instrument_gain_constant: u16,
    /// Bytes 123 - 124 (122..124) of the trace header.
    pub instrument_initial_gain: u16,
    /// Bytes 125 - 126 (124..126) of the trace header.
    pub correlated: Correlated,
    /// Bytes 127 - 128 (126..128) of the trace header.
    pub sweep_frequency_at_start: u16,
    /// Bytes 129 - 130 (128..130) of the trace header.
    pub sweep_frequency_at_end: u16,
    /// Bytes 131 - 132 (130..132) of the trace header.
    pub sweep_length: u16,
    /// Bytes 133 - 134 (132..134) of the trace header.
    pub sweep_type: SweepType,
    /// Bytes 135 - 136 (134..136) of the trace header.
    pub sweep_trace_taper_length_at_start: u16,
    /// Bytes 137 - 138 (136..138) of the trace header.
    pub sweep_trace_taper_length_at_end: u16,
    /// Bytes 139 - 140 (138..140) of the trace header.
    pub taper_type: TaperType,
    /// Bytes 141 - 142 (140..142) of the trace header.
    pub alias_filter_frequency: u16,
    /// Bytes 143 - 144 (142..144) of the trace header.
    pub alias_filter_slope: u16,
    /// Bytes 145 - 146 (144..146) of the trace header.
    pub notch_filter_frequency: u16,
    /// Bytes 147 - 148 (146..148) of the trace header.
    pub notch_filter_slope: u16,
    /// Bytes 149 - 150 (148..150) of the trace header.
    pub low_cut_frequency: u16,
    /// Bytes 151 - 152 (150..152) of the trace header.
    pub high_cut_frequency: u16,
    /// Bytes 153 - 154 (152..154) of the trace header.
    pub low_cut_slope: u16,
    /// Bytes 155 - 156 (154..156) of the trace header.
    pub high_cut_slope: u16,
    /// Bytes 157 - 158 (156..158) of the trace header.
    pub year_recorded: u16,
    /// Bytes 159 - 160 (158..160) of the trace header.
    pub day_of_year: u16,
    /// Bytes 161 - 162 (160..162) of the trace header.
    pub hour_of_day: u16,
    /// Bytes 163 - 164 (162..164) of the trace header.,
    pub minute_of_hour: u16,
    /// Bytes 165 - 166 (164..166) of the trace header.
    pub second_of_minute: u16,
    /// Bytes 167 - 168 (166..168) of the trace header.
    pub time_base_code: TimeBasisCode,
    /// Bytes 169 - 170 (158..170) of the trace header.
    pub trace_weighting_factor: u16,
    /// Bytes 171 - 172 (170..172) of the trace header.
    pub geophone_group_number_roll_pos1: u16,
    /// Bytes 173 - 174 (172..174) of the trace header.
    pub geophone_group_number_first_trace_orig_field: u16,
    /// Bytes 175 - 176 (174..176) of the trace header.
    pub geophone_group_number_last_trace_orig_field: u16,
    /// Bytes 177 - 178 (176..178) of the trace header.
    pub gap_size: u16,
    /// Bytes 179 - 180 (178..180) of the trace header.
    pub over_travel: OverTravel,
    // Ensemble=CDP.
    /// Usually bytes 181 - 184 (180..184) of the trace header.
    pub x_ensemble: i32,
    /// Usually bytes 185 - 188 (184..188) of the trace header.
    pub y_ensemble: i32,
    /// Usually, bytes 189 - 192 (188..192) of the trace header.
    pub inline_no: i32,
    /// Usually, bytes 193 - 196 (192..196) of the trace header.
    pub crossline_no: i32,
    /// Bytes 197 - 200 (196..200) of the trace header.
    pub shot_point_no: i32,
    /// Bytes 201 - 202 (200..202) of the trace header.
    pub shot_point_scalar: u16,
    /// Bytes 203 - 204 (202..204) of the trace header.
    pub trace_value_measurement_unit: TraceValueUnit,
    /// Bytes 205 - 208 (204..208) of the trace header.
    pub transduction_constant_mantissa: i32,
    /// Bytes 209 - 210 (208..210) of the trace header.
    pub transduction_constant_power: u16,
    /// Bytes 211 - 212 (210..212) of the trace header.
    pub transduction_units: TransductionUnits,
    /// Bytes 213 - 214 (212..214) of the trace header.
    pub trace_identifier: u16,
    /// Bytes 215 - 216 (214..216) of the trace header.
    pub time_scalar_trace_header: u16,
    /// Bytes 217 - 218 (216..218) of the trace header.
    pub source_type: SourceType,
    /// Bytes 219 - 220 (218..220) of the trace header.
    pub source_energy_direction_v: u16,
    /// Bytes 221 - 222 (220..222) of the trace header.
    pub source_energy_direction_il: u16,
    /// Bytes 223 - 224 (222..224) of the trace header.
    pub source_energy_direction_xl: u16,
    /// Bytes 225 - 228 (224..228) of the trace header.
    pub source_measurement_mantissa: i32,
    /// Bytes 229 - 230 (228..230) of the trace header.
    pub source_measurement_exponent: u16,
    /// Bytes 231 - 232 (230..232) of the trace header.
    pub source_measurement_unit: SourceMeasurementUnit,
    /// Bytes 233 - 230 (232..230) of the trace header.
    pub trace_name: [u8; 8],
}

/// This structure represents a parsed binary header for a SEG-Y file.
///
/// It should be noted that while the binary header of a SEG-Y file is 400 bytes long
/// and contains approximately 45 fields, this structure uses about 29 of these fields
/// and rearranges then into 32 fields for convenience.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BinHeader {
    /// Bytes 3201 - 3204 of the SEG-Y file, (0..4) of the binary header.
    pub job_id: u32,
    /// Bytes 3205 - 3208 of the SEG-Y file, (4..8) of the binary header.
    pub line_number: u32,
    /// Bytes 3209 - 3212 of the SEG-Y file, (8..12) of the binary header.
    pub reel_number: u32,
    /// Bytes 3213 - 3214 of the SEG-Y file, (12..14) of the binary header.
    pub no_traces: u16,
    /// Bytes 3215 - 3216 of the SEG-Y file, (14..16) of the binary header.
    pub no_aux_traces: u16,
    /// Bytes 3217 - 3218 of the SEG-Y file, (16..18) of the binary header.
    pub sample_interval: u16,
    /// Bytes 3219 - 3220 of the SEG-Y file, (18..20) of the binary header.
    pub sample_interval_original: u16,
    /// Bytes 3221 - 3222 of the SEG-Y file, (20..22) of the binary header.
    pub no_samples: u16,
    /// Bytes 3223 - 3224 of the SEG-Y file, (22..24) of the binary header.
    pub no_samples_original: u16,
    /// Bytes 3225 - 3226 of the SEG-Y file, (24..26) of the binary header.
    pub sample_format_code: SampleFormatCode,
    /// Bytes 3227 - 3228 of the SEG-Y file, (26..28) of the binary header.
    pub ensemble_fold: u16,
    /// Bytes 3229 - 3230 of the SEG-Y file, (28..30) of the binary header.
    pub sorting_code: TraceSortingCode,
    /// Bytes 3231 - 3232 of the SEG-Y file, (30..32) of the binary header.
    pub vertical_sum: u16,
    /// Bytes 3233 - 3234 of the SEG-Y file, (32..34) of the binary header.
    pub sweep_frequency_start: u16,
    /// Bytes 3235 - 3236 of the SEG-Y file, (34..36) of the binary header.
    pub sweep_frequency_end: u16,
    /// Bytes 3237 - 3238 of the SEG-Y file, (36..38) of the binary header.
    pub sweep_length: u16,
    /// Bytes 3239 - 3240 of the SEG-Y file, (38..40) of the binary header.
    pub sweep_type: SweepTypeCode,
    /// Bytes 3241 - 3242 of the SEG-Y file, (40..42) of the binary header.
    pub sweep_channel_trace_no: u16,
    /// Bytes 3243 - 3244 of the SEG-Y file, (42..44) of the binary header.
    pub sweep_taper_at_start: u16,
    /// Bytes 3245 - 3246 of the SEG-Y file, (44..46) of the binary header.
    pub sweep_taper_at_end: u16,
    /// Bytes 3247 - 3248 of the SEG-Y file, (46..48) of the binary header.
    pub taper_type: TaperType,
    /// Bytes 3249 - 3250 of the SEG-Y file, (48..50) of the binary header.
    pub correlated_traces: CorrelatedDataTraces,
    /// Bytes 3251 - 3252 of the SEG-Y file, (50..52) of the binary header.
    pub binary_gain_recovered: BinaryGainRecovered,
    /// Bytes 3253 - 3254 of the SEG-Y file, (52..54) of the binary header.
    pub amplitude_recovery_method: AmplitudeRecoveryMethod,
    /// Bytes 3255 - 3256 of the SEG-Y file, (54..56) of the binary header.
    pub measurement_system: MeasurementSystem,
    /// Bytes 3257 - 3258 of the SEG-Y file, (56..58) of the binary header.
    pub impulse_signal_polarity: ImpulseSignalPolarity,
    /// Bytes 3259 - 3260 of the SEG-Y file, (58..60) of the binary header.
    pub vibratory_polarity_code: VibratoryPolarityCode,
    /// Bytes 3501 - 3502 of the SEG-Y file, (300..302) of the binary header.
    /// Combines minor and major revision code.
    pub segy_revision_number: [u8; 2],
    /// Bytes 3503 - 3504 of the SEG-Y file, (302..304) of the binary header.
    pub fixed_length_trace_flag: FixedLengthTraces,
    /// Bytes 3505 - 3506 of the SEG-Y file, (304..306) of the binary header.
    pub extended_header_count: u32,
    /// Bytes 3511 - 3512 of the SEG-Y file, (310..312) of the binary header.
    pub time_basis_code: TimeBasisCode,
    /// Determined from bytes 3297 - 3300 of the SEG-Y file, (96..100) of the binary header.
    /// This library does not support all variants, only complete LE or complete BE.
    ///
    /// See the [SEG-Y_r2.0 standard](<https://seg.org/Portals/0/SEG/News%20and%20Resources/Technical%20Standards/seg_y_rev2_0-mar2017.pdf>)
    /// (January 2017), page 9, for more details.
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

/// This is a rust readable version of the [`TapeLabel`] structure, which can be generated after the file
/// has been read, but is not stored.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// Converts the C compatible [`TapeLabel`] to a rust compatible [`ReadableTapeLabel`]
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
