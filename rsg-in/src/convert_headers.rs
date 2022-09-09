//! This file contains the definitions for the binary headers of a SEGY file. These can then be
//! used for better interpreting the file in the parser.
use rsg_core::bitconverter::{converter_chooser, BitConverter};
use rsg_core::enums::*;
use rsg_core::errors::*;
use rsg_core::settings::SegySettings;
use rsg_core::{BinHeader, TapeLabel, TraceHeader};

use num::FromPrimitive;

use std::convert::TryInto;

/// This trait allows a header to be created from bytes, potentially
/// using SegySettings to guide the creation process
pub trait HeaderFromBytes: Sized {
    fn from_bytes(bytes: &[u8], settings: &SegySettings) -> Result<Self, RsgError>;
}

pub trait TraceHeaderFromBytes: Sized {
    fn from_bytes(
        bytes: &[u8],
        bin_header: &BinHeader,
        settings: &SegySettings,
        idx: usize,
    ) -> Result<Self, RsgError>;
}

impl HeaderFromBytes for TapeLabel {
    /// This is always ASCII bytes so we are not too concerned with translating them.
    fn from_bytes(bytes: &[u8], _settings: &SegySettings) -> Result<Self, RsgError> {
        let max_block_size: u32 = match String::from_utf8(bytes[19..29].to_vec())
            .map_err(RsgError::TryFromUtf8)?
            .parse()
        {
            Ok(n) => n,
            Err(e) => return Err(RsgError::InvalidHeader { msg: e.to_string() }),
        };

        let res = TapeLabel {
            storage_unit_seq_no: bytes[0..4].try_into().unwrap(),
            segy_revision_no: bytes[4..9].try_into().unwrap(),
            storage_unit_structure: bytes[9..15].try_into().unwrap(),
            binding_number: bytes[15..19].try_into().unwrap(),
            max_block_size,
            producing_organisation_code: bytes[29..39].try_into().unwrap(),
            creation_date: bytes[39..50].try_into().unwrap(),
            serial_number: bytes[50..62].try_into().unwrap(),
            external_label: bytes[62..68].try_into().unwrap(),
            recording_entity: bytes[80..104].try_into().unwrap(),
            extra: bytes[104..118].try_into().unwrap(),
        };
        Ok(res)
    }
}

impl HeaderFromBytes for BinHeader {
    fn from_bytes(bytes: &[u8], settings: &SegySettings) -> Result<Self, RsgError> {
        // Binary header should be 400 bytes long. If it is not, we Houston has a problem.
        if bytes.len() != crate::BIN_HEADER_LEN {
            return Err(RsgError::BinHeaderLength { l: bytes.len() });
        }

        // Flag direction will determine how all other data is to be interpreted.
        let le = if let Some(le) = settings.get_override_to_le() {
            le
        } else {
            bytes[96..100] == [1, 2, 3, 4]
        };

        // Use the `bonary_flag_direction` to determine how to interpret bytes.
        let u16_from_bytes = if le {
            u16::from_le_bytes
        } else {
            u16::from_be_bytes
        };

        let i16_from_bytes = if le {
            i16::from_le_bytes
        } else {
            i16::from_be_bytes
        };

        let u32_from_bytes = if le {
            u32::from_le_bytes
        } else {
            u32::from_be_bytes
        };

        // Set sample format code, using override if one is set.
        let sample_format_code = if let Some(code) = settings.get_override_trace_format() {
            code
        } else {
            SampleFormatCode::new(u16_from_bytes(bytes[24..26].try_into().unwrap()))?
        };

        let time_basis_code =
            TimeBasisCode::new(u16_from_bytes(bytes[310..312].try_into().unwrap()));

        let vibratory_polarity_code =
            VibratoryPolarityCode::new(u16_from_bytes(bytes[58..60].try_into().unwrap()));

        let impulse_signal_polarity =
            ImpulseSignalPolarity::new(u16_from_bytes(bytes[56..58].try_into().unwrap()));

        let measurement_system = if let Some(units) = settings.get_override_coordinate_units() {
            units
        } else {
            MeasurementSystem::new(u16_from_bytes(bytes[54..56].try_into().unwrap()))
        };

        let amplitude_recovery_method =
            AmplitudeRecoveryMethod::new(u16_from_bytes(bytes[52..54].try_into().unwrap()));

        let binary_gain_recovered =
            BinaryGainRecovered::new(u16_from_bytes(bytes[50..52].try_into().unwrap()));

        let correlated_traces =
            CorrelatedDataTraces::new(u16_from_bytes(bytes[48..50].try_into().unwrap()));

        let fixed_length_trace_flag =
            FixedLengthTraces::new(u16_from_bytes(bytes[302..304].try_into().unwrap()))?;

        let header = BinHeader {
            job_id: u32_from_bytes(bytes[0..4].try_into().unwrap()),
            line_number: u32_from_bytes(bytes[4..8].try_into().unwrap()),
            reel_number: u32_from_bytes(bytes[8..12].try_into().unwrap()),
            no_traces: u16_from_bytes(bytes[12..14].try_into().unwrap()),
            no_aux_traces: u16_from_bytes(bytes[14..16].try_into().unwrap()),
            sample_interval: u16_from_bytes(bytes[16..18].try_into().unwrap()),
            sample_interval_original: u16_from_bytes(bytes[18..20].try_into().unwrap()),
            no_samples: u16_from_bytes(bytes[20..22].try_into().unwrap()),
            no_samples_original: u16_from_bytes(bytes[22..24].try_into().unwrap()),
            sample_format_code,
            ensemble_fold: u16_from_bytes(bytes[26..28].try_into().unwrap()),
            sorting_code: TraceSortingCode::new(i16_from_bytes(bytes[28..30].try_into().unwrap())),
            vertical_sum: u16_from_bytes(bytes[30..32].try_into().unwrap()),
            sweep_frequency_start: u16_from_bytes(bytes[32..34].try_into().unwrap()),
            sweep_frequency_end: u16_from_bytes(bytes[34..36].try_into().unwrap()),
            sweep_length: u16_from_bytes(bytes[36..38].try_into().unwrap()),
            sweep_type: SweepTypeCode::new(u16_from_bytes(bytes[38..40].try_into().unwrap())),
            sweep_channel_trace_no: u16_from_bytes(bytes[40..42].try_into().unwrap()),
            sweep_taper_at_start: u16_from_bytes(bytes[42..44].try_into().unwrap()),
            sweep_taper_at_end: u16_from_bytes(bytes[44..46].try_into().unwrap()),
            taper_type: TaperType::new(u16_from_bytes(bytes[46..48].try_into().unwrap())),
            correlated_traces,
            binary_gain_recovered,
            amplitude_recovery_method,
            measurement_system,
            impulse_signal_polarity,
            vibratory_polarity_code,
            segy_revision_number: [bytes[300], bytes[301]],
            fixed_length_trace_flag,
            extended_header_count: u32_from_bytes(bytes[306..310].try_into().unwrap()),
            time_basis_code,
            binary_flag_direction_is_le: le,
        };

        Ok(header)
    }
}

impl TraceHeaderFromBytes for TraceHeader {
    /// When making a `TraceHeader` we use the data from the `BinHeader` to determine whether or
    /// not
    fn from_bytes(
        bytes: &[u8],
        bin_header: &BinHeader,
        settings: &SegySettings,
        idx: usize,
    ) -> Result<Self, RsgError> {
        // Binary header should be 400 bytes long. If it is not, we Houston has a problem.
        if bytes.len() != crate::TRACE_HEADER_LEN {
            return Err(RsgError::TraceHeaderLength { l: bytes.len() });
        }

        let use_le = bin_header.binary_flag_direction_is_le;

        let inline_no_rng = settings.get_inline_no_bidx()..(4 + settings.get_inline_no_bidx());
        let xline_no_rng = settings.get_crossline_no_bidx()..(4 + settings.get_crossline_no_bidx());
        let x_ensemble_rng = settings.get_x_ensemble_bidx()..(4 + settings.get_x_ensemble_bidx());
        let y_ensemble_rng = settings.get_y_ensemble_bidx()..(4 + settings.get_y_ensemble_bidx());

        let u16_from_bytes = if use_le {
            u16::from_le_bytes
        } else {
            u16::from_be_bytes
        };

        let i16_from_bytes = if use_le {
            i16::from_le_bytes
        } else {
            i16::from_be_bytes
        };

        let trace_name = if use_le {
            bytes[232..240].try_into().unwrap()
        } else {
            bytes[232..240].iter().cloned().rev().collect::<Vec<_>>()[..]
                .try_into()
                .unwrap()
        };

        let i32_from_bytes = if use_le {
            i32::from_le_bytes
        } else {
            i32::from_be_bytes
        };

        // A little bit convoluted because of types.
        let coordinate_format =
            if let Some(coord_override) = settings.get_override_coordinate_format() {
                coord_override
            } else {
                // Default coordinate format is Int32.
                SampleFormatCode::Int32
            };

        let coordinate_parser: BitConverter = converter_chooser(coordinate_format, use_le)?;

        let coord_parser = |x: [u8; 4]| {
            let float: f32 = coordinate_parser(&x)?;
            i32::from_f32(float).ok_or(RsgError::FloatConversion {
                float,
                format: coordinate_format,
            })
        };

        // Make coordinate scalar, using override if one is set.
        let coordinate_scalar = if let Some(scaling) = settings.get_override_coordinate_scaling() {
            scaling as i16 // This is valid because `set_override_coordinate_scaling` is checked.
        } else {
            i16_from_bytes(bytes[70..72].try_into().unwrap())
        };

        let source_measurement_unit =
            SourceMeasurementUnit::new(i16_from_bytes(bytes[230..232].try_into().unwrap()));
        let source_type = SourceType::new(i16_from_bytes(bytes[216..218].try_into().unwrap()));
        let trace_value_measurement_unit =
            TraceValueUnit::new(i16_from_bytes(bytes[202..204].try_into().unwrap()));
        let transduction_units =
            TransductionUnits::new(i16_from_bytes(bytes[210..212].try_into().unwrap()));
        let over_travel = OverTravel::new(u16_from_bytes(bytes[178..180].try_into().unwrap()));
        let time_base_code =
            TimeBasisCode::new(u16_from_bytes(bytes[166..168].try_into().unwrap()));
        let taper_type = TaperType::new(u16_from_bytes(bytes[138..140].try_into().unwrap()));
        let sweep_type = SweepType::new(u16_from_bytes(bytes[132..134].try_into().unwrap()));
        let correlated = Correlated::new(u16_from_bytes(bytes[124..126].try_into().unwrap()));
        let gain_type = GainType::new(u16_from_bytes(bytes[118..120].try_into().unwrap()));
        let coordinate_units =
            CoordinateUnits::new(u16_from_bytes(bytes[88..90].try_into().unwrap()));
        let data_use = DataUse::new(u16_from_bytes(bytes[34..36].try_into().unwrap()));
        let trace_identification_code = if let Some(id) = settings.get_override_trace_id_code() {
            id
        } else {
            TraceIdCode::new(i16_from_bytes(bytes[28..30].try_into().unwrap()))
        };

        let trace_sequence_on_line = i32_from_bytes(bytes[0..4].try_into().unwrap());
        let trace_sequence_in_file = i32_from_bytes(bytes[4..8].try_into().unwrap());
        let field_record_no = i32_from_bytes(bytes[8..12].try_into().unwrap());
        let trace_no = i32_from_bytes(bytes[12..16].try_into().unwrap());
        let trace_no_in_ensemble = i32_from_bytes(bytes[24..28].try_into().unwrap());

        let idx = match settings.get_order_trace_by() {
            OrderTraceBy::Default => idx,
            OrderTraceBy::TraceSequenceOnLine => trace_sequence_on_line as usize,
            OrderTraceBy::TraceSequenceInFile => trace_sequence_in_file as usize,
            OrderTraceBy::FieldRecordNo => field_record_no as usize,
            OrderTraceBy::TraceNo => trace_no as usize,
            OrderTraceBy::TraceNoInEnsemble => trace_no_in_ensemble as usize,
        };

        // If dimensions are customised, set them manually, otherwise, read them from the header.
        // NB, setting customised x on its own doesn't make much sense and can lead to strange
        // results. However, sometimes it may be required.
        let (inline_no, crossline_no) =
            match (settings.get_override_dim_x(), settings.get_override_dim_y()) {
                (Some(x), _) => {
                    let i_no = (idx / x as usize) as i32;
                    let x_no = (idx % x as usize) as i32;
                    (i_no, x_no)
                }
                (None, _) => {
                    let i_no = i32_from_bytes(bytes[inline_no_rng].try_into().unwrap());
                    let x_no = i32_from_bytes(bytes[xline_no_rng].try_into().unwrap());
                    (i_no, x_no)
                }
            };

        let traceheader = TraceHeader {
            trace_sequence_on_line,
            trace_sequence_in_file,
            field_record_no,
            trace_no,
            energy_source_point_no: i32_from_bytes(bytes[16..20].try_into().unwrap()),
            ensemble_no: i32_from_bytes(bytes[20..24].try_into().unwrap()),
            trace_no_in_ensemble,
            trace_identification_code,
            no_v_summed_traces: u16_from_bytes(bytes[30..32].try_into().unwrap()),
            no_h_stacked_traces: u16_from_bytes(bytes[32..34].try_into().unwrap()),
            data_use,
            source_to_receiver_distance: coord_parser(bytes[36..40].try_into().unwrap())?,
            elevation_of_receiver_group: coord_parser(bytes[40..44].try_into().unwrap())?,
            surface_elevation_of_source: coord_parser(bytes[44..48].try_into().unwrap())?,
            source_depth: coord_parser(bytes[48..52].try_into().unwrap())?,
            datum_elevation_of_receiver_group: coord_parser(bytes[52..56].try_into().unwrap())?,
            datum_elevation_of_source: coord_parser(bytes[56..60].try_into().unwrap())?,
            water_column_height_at_source: coord_parser(bytes[60..64].try_into().unwrap())?,
            water_column_height_at_group: coord_parser(bytes[64..68].try_into().unwrap())?,
            elevation_scalar: i16_from_bytes(bytes[68..70].try_into().unwrap()),
            coordinate_scalar,
            source_x: coord_parser(bytes[72..76].try_into().unwrap())?,
            source_y: coord_parser(bytes[76..80].try_into().unwrap())?,
            receiver_group_x: coord_parser(bytes[80..84].try_into().unwrap())?,
            receiver_group_y: coord_parser(bytes[84..88].try_into().unwrap())?,
            coordinate_units,
            weathing_velocity: u16_from_bytes(bytes[90..92].try_into().unwrap()),
            sub_weathering_velocity: u16_from_bytes(bytes[92..94].try_into().unwrap()),
            uphole_time_at_source: u16_from_bytes(bytes[94..96].try_into().unwrap()),
            uphole_time_at_group: u16_from_bytes(bytes[96..98].try_into().unwrap()),
            source_static_correction: u16_from_bytes(bytes[98..100].try_into().unwrap()),
            group_static_correction: u16_from_bytes(bytes[100..102].try_into().unwrap()),
            total_static_applied: u16_from_bytes(bytes[102..104].try_into().unwrap()),
            lag_time_a: u16_from_bytes(bytes[104..106].try_into().unwrap()),
            lag_time_b: u16_from_bytes(bytes[106..108].try_into().unwrap()),
            delay_recording_time: u16_from_bytes(bytes[108..110].try_into().unwrap()),
            mute_time_start: u16_from_bytes(bytes[110..112].try_into().unwrap()),
            mute_time_end: u16_from_bytes(bytes[112..114].try_into().unwrap()),
            no_samples_in_trace: u16_from_bytes(bytes[114..116].try_into().unwrap()),
            sample_interval_of_trace: u16_from_bytes(bytes[116..118].try_into().unwrap()),
            gain_type,
            instrument_gain_constant: u16_from_bytes(bytes[120..122].try_into().unwrap()),
            instrument_initial_gain: u16_from_bytes(bytes[122..124].try_into().unwrap()),
            correlated,
            sweep_frequency_at_start: u16_from_bytes(bytes[126..128].try_into().unwrap()),
            sweep_frequency_at_end: u16_from_bytes(bytes[128..130].try_into().unwrap()),
            sweep_length: u16_from_bytes(bytes[130..132].try_into().unwrap()),
            sweep_type,
            sweep_trace_taper_length_at_start: u16_from_bytes(bytes[134..136].try_into().unwrap()),
            sweep_trace_taper_length_at_end: u16_from_bytes(bytes[136..138].try_into().unwrap()),
            taper_type,
            alias_filter_frequency: u16_from_bytes(bytes[140..142].try_into().unwrap()),
            alias_filter_slope: u16_from_bytes(bytes[142..144].try_into().unwrap()),
            notch_filter_frequency: u16_from_bytes(bytes[144..146].try_into().unwrap()),
            notch_filter_slope: u16_from_bytes(bytes[146..148].try_into().unwrap()),
            low_cut_frequency: u16_from_bytes(bytes[148..150].try_into().unwrap()),
            high_cut_frequency: u16_from_bytes(bytes[150..152].try_into().unwrap()),
            low_cut_slope: u16_from_bytes(bytes[152..154].try_into().unwrap()),
            high_cut_slope: u16_from_bytes(bytes[154..156].try_into().unwrap()),
            year_recorded: u16_from_bytes(bytes[156..158].try_into().unwrap()),
            day_of_year: u16_from_bytes(bytes[158..160].try_into().unwrap()),
            hour_of_day: u16_from_bytes(bytes[160..162].try_into().unwrap()),
            minute_of_hour: u16_from_bytes(bytes[162..164].try_into().unwrap()),
            second_of_minute: u16_from_bytes(bytes[164..166].try_into().unwrap()),
            time_base_code,
            trace_weighting_factor: u16_from_bytes(bytes[168..170].try_into().unwrap()),
            geophone_group_number_roll_pos1: u16_from_bytes(bytes[170..172].try_into().unwrap()),
            geophone_group_number_first_trace_orig_field: u16_from_bytes(
                bytes[172..174].try_into().unwrap(),
            ),
            geophone_group_number_last_trace_orig_field: u16_from_bytes(
                bytes[174..176].try_into().unwrap(),
            ),
            gap_size: u16_from_bytes(bytes[176..178].try_into().unwrap()),
            over_travel,
            x_ensemble: coord_parser(bytes[x_ensemble_rng].try_into().unwrap())?,
            y_ensemble: coord_parser(bytes[y_ensemble_rng].try_into().unwrap())?,
            inline_no,
            crossline_no,
            shot_point_no: i32_from_bytes(bytes[196..200].try_into().unwrap()),
            shot_point_scalar: u16_from_bytes(bytes[200..202].try_into().unwrap()),
            trace_value_measurement_unit,
            transduction_constant_mantissa: i32_from_bytes(bytes[204..208].try_into().unwrap()),
            transduction_constant_power: u16_from_bytes(bytes[208..210].try_into().unwrap()),
            transduction_units,
            trace_identifier: u16_from_bytes(bytes[212..214].try_into().unwrap()),
            time_scalar_trace_header: u16_from_bytes(bytes[214..216].try_into().unwrap()),
            source_type,
            source_energy_direction_v: u16_from_bytes(bytes[218..220].try_into().unwrap()),
            source_energy_direction_il: u16_from_bytes(bytes[220..222].try_into().unwrap()),
            source_energy_direction_xl: u16_from_bytes(bytes[222..224].try_into().unwrap()),
            source_measurement_mantissa: i32_from_bytes(bytes[224..228].try_into().unwrap()),
            source_measurement_exponent: u16_from_bytes(bytes[228..230].try_into().unwrap()),
            source_measurement_unit,
            trace_name,
        };

        Ok(traceheader)
    }
}
