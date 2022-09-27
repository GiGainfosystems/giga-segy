// Copyright (C) 2022 by GiGa infosystems
//! This submodule exists for converting headers to bytes and writing them.
use giga_segy_core::enums::*;
use giga_segy_core::errors::*;
use giga_segy_core::{BinHeader, TapeLabel, TraceHeader};
use num::ToPrimitive;
// use rust_segy_input::{BIN_HEADER_LEN, TRACE_HEADER_LEN};

use std::fs::File;
use std::io::Write;

use crate::settings::SegyWriteSettings;
use crate::write_data;

pub trait SegyHeaderToBytes {
    fn as_bytes(&self) -> Result<Vec<u8>, RsgError>;
}

impl SegyHeaderToBytes for TapeLabel {
    fn as_bytes(&self) -> Result<Vec<u8>, RsgError> {
        let mbs_string = self.max_block_size.to_string();
        let ml = 10 - mbs_string.as_bytes().len() as i64;

        if ml < 0 {
            let msg = format!("Invalid TapeLabel: max block size too long: {}", 10 - ml);
            return Err(RsgError::InvalidHeader { msg });
        }

        let mut max_block_size = vec![0; ml as usize];
        max_block_size.extend_from_slice(mbs_string.as_bytes());

        let mut output = Vec::with_capacity(128);
        output.extend_from_slice(&self.storage_unit_seq_no);
        output.extend_from_slice(&self.segy_revision_no);
        output.extend_from_slice(&self.storage_unit_structure);
        output.extend_from_slice(&self.binding_number);
        output.extend_from_slice(&max_block_size);
        output.extend_from_slice(&self.producing_organisation_code);
        output.extend_from_slice(&self.creation_date);
        output.extend_from_slice(&self.serial_number);
        output.extend_from_slice(&self.external_label);
        output.extend_from_slice(&self.recording_entity);
        output.extend_from_slice(&self.extra);
        Ok(output)
    }
}

/// The standard text header is a string of length 3200 bytes.
///
/// NB: It is more efficient to just write it than to allocate it as a string.
pub(crate) fn write_text_header(header: &str, file: &mut File) -> Result<(), RsgError> {
    let raw_bytes = header.as_bytes();
    let bl = 3200 - raw_bytes.len() as i64;

    if bl < 0 {
        let msg = format!("Invalid TextHeader: Too long: {}", 3200 - bl);
        return Err(RsgError::InvalidHeader { msg });
    }

    file.write_all(raw_bytes)?;
    file.write_all(&vec![b' '; bl as usize])?;
    Ok(())
}

impl SegyHeaderToBytes for BinHeader {
    fn as_bytes(&self) -> Result<Vec<u8>, RsgError> {
        let le = self.binary_flag_direction_is_le;

        let u16_to_b = if le {
            u16::to_le_bytes
        } else {
            u16::to_be_bytes
        };
        let i16_to_b = if le {
            i16::to_le_bytes
        } else {
            i16::to_be_bytes
        };
        let u32_to_b = if le {
            u32::to_le_bytes
        } else {
            u32::to_be_bytes
        };

        let sample_format_code = self.sample_format_code.to_u16().unwrap();
        let sorting_code = self.sorting_code.to_i16().unwrap();
        let sweep_type = self.sweep_type.to_u16().unwrap();
        let taper_type = self.taper_type.to_u16().unwrap();
        let correlated_traces = self.correlated_traces.to_u16().unwrap();
        let binary_gain_recovered = self.binary_gain_recovered.to_u16().unwrap();
        let amplitude_recovery_method = self.amplitude_recovery_method.to_u16().unwrap();
        let measurement_system = self.measurement_system.to_u16().unwrap();
        let impulse_signal_polarity = self.impulse_signal_polarity.to_u16().unwrap();
        let vibratory_polarity_code = self.vibratory_polarity_code.to_u16().unwrap();
        let fixed_length_trace_flag = self.fixed_length_trace_flag.to_u16().unwrap();
        let time_basis_code = self.time_basis_code.to_u16().unwrap();
        let binary_flag_direction_is_le = if le { [1, 2, 3, 4] } else { [4, 3, 2, 1] };

        let mut output = Vec::with_capacity(400);
        output.extend_from_slice(&u32_to_b(self.job_id)); // 1-4
        output.extend_from_slice(&u32_to_b(self.line_number)); // 5-8
        output.extend_from_slice(&u32_to_b(self.reel_number)); // 9-12
        output.extend_from_slice(&u16_to_b(self.no_traces)); // 13-14
        output.extend_from_slice(&u16_to_b(self.no_aux_traces)); // 15-16
        output.extend_from_slice(&u16_to_b(self.sample_interval)); // 17-18
        output.extend_from_slice(&u16_to_b(self.sample_interval_original));
        output.extend_from_slice(&u16_to_b(self.no_samples)); // 21-22
        output.extend_from_slice(&u16_to_b(self.no_samples_original));
        output.extend_from_slice(&u16_to_b(sample_format_code)); // 25-26
        output.extend_from_slice(&u16_to_b(self.ensemble_fold)); // 27-28
        output.extend_from_slice(&i16_to_b(sorting_code)); // 29-30 !!NB: i16!!
        output.extend_from_slice(&u16_to_b(self.vertical_sum)); // 31-32
        output.extend_from_slice(&u16_to_b(self.sweep_frequency_start));
        output.extend_from_slice(&u16_to_b(self.sweep_frequency_start));
        output.extend_from_slice(&u16_to_b(self.sweep_length)); // 37-38
        output.extend_from_slice(&u16_to_b(sweep_type)); // 39-40
        output.extend_from_slice(&u16_to_b(self.sweep_channel_trace_no));
        output.extend_from_slice(&u16_to_b(self.sweep_taper_at_start));
        output.extend_from_slice(&u16_to_b(self.sweep_taper_at_end));
        output.extend_from_slice(&u16_to_b(taper_type)); // 47-48
        output.extend_from_slice(&u16_to_b(correlated_traces)); // 49-50
        output.extend_from_slice(&u16_to_b(binary_gain_recovered)); // 51-52
        output.extend_from_slice(&u16_to_b(amplitude_recovery_method));
        output.extend_from_slice(&u16_to_b(measurement_system)); // 55-56
        output.extend_from_slice(&u16_to_b(impulse_signal_polarity));
        output.extend_from_slice(&u16_to_b(vibratory_polarity_code));
        // NB: We use only the first 60 bytes and:
        // [96..100] for the directionality (97-100)
        // [300-312] for stuff (301-312)
        // So we must write 36 empty bytes, and then our 4, and then another 200.
        output.extend_from_slice(&[0; 36]); // 97-100
        output.extend_from_slice(&binary_flag_direction_is_le);

        output.extend_from_slice(&[0; 200]); // 97-100
        debug_assert_eq!(output.len(), 300);

        output.extend_from_slice(&self.segy_revision_number); // 301-302
        output.extend_from_slice(&u16_to_b(fixed_length_trace_flag));
        output.extend_from_slice(&[0, 0]); // 305-306
        output.extend_from_slice(&u32_to_b(self.extended_header_count));
        output.extend_from_slice(&u16_to_b(time_basis_code)); // 311-312
        debug_assert_eq!(output.len(), 312);

        output.extend_from_slice(&[0; 88]);
        debug_assert_eq!(output.len(), 400);
        Ok(output)
    }
}

/// Convert the trace header to bytes.
///
/// NB: We cannot use the above trait as we also need values from the binary header.
///
/// NB2: Because of how variable byte positions work, this can get a little bit silly.
pub fn th_as_bytes_with_settings<S: SegyWriteSettings>(
    trace_header: &TraceHeader,
    settings: &S,
    bin_header: &BinHeader,
) -> Result<Vec<u8>, RsgError> {
    use std::convert::TryInto;

    let mut output = vec![0; 240];

    let le = bin_header.binary_flag_direction_is_le;

    let u16_to_b = if le {
        u16::to_le_bytes
    } else {
        u16::to_be_bytes
    };
    let i16_to_b = if le {
        i16::to_le_bytes
    } else {
        i16::to_be_bytes
    };
    let i32_to_b = if le {
        i32::to_le_bytes
    } else {
        i32::to_be_bytes
    };

    // Get variables.

    // // A little bit convoluted because of types. Default = Int32.
    let coordinate_format = match settings.get_override_coordinate_format() {
        Some(coord_override) => coord_override,
        None => SampleFormatCode::Int32,
    };
    let cb_inner = write_data::converter_chooser(coordinate_format, le)?;
    let coord_byter = |x: i32| -> Result<[u8; 4], RsgError> {
        let x = cb_inner(x)?;
        if x.len() != 4 {
            let m = format!("Header coords should give 4 byte values, but give {:?}", x);
            return Err(RsgError::BitConversionError { msg: m });
        }
        Ok([x[0], x[1], x[2], x[3]])
    };

    let b0_4 = i32_to_b(trace_header.trace_sequence_on_line.to_i32().unwrap());
    let b4_8 = i32_to_b(trace_header.trace_sequence_in_file.to_i32().unwrap());
    let b8_12 = i32_to_b(trace_header.field_record_no.to_i32().unwrap());
    let b12_16 = i32_to_b(trace_header.trace_no.to_i32().unwrap());
    let b16_20 = i32_to_b(trace_header.energy_source_point_no.to_i32().unwrap());
    let b20_24 = i32_to_b(trace_header.ensemble_no.to_i32().unwrap());
    let b24_28 = i32_to_b(trace_header.trace_no_in_ensemble.to_i32().unwrap());
    let b28_30 = i16_to_b(trace_header.trace_identification_code.to_i16().unwrap());
    let b30_32 = u16_to_b(trace_header.no_v_summed_traces.to_u16().unwrap());
    let b32_34 = u16_to_b(trace_header.no_h_stacked_traces.to_u16().unwrap());
    let b34_36 = u16_to_b(trace_header.data_use.to_u16().unwrap());

    //// These are coordinates. They use the `coord_byter`.
    let b36_40 = coord_byter(trace_header.source_to_receiver_distance.to_i32().unwrap())?;
    let b40_44 = coord_byter(trace_header.elevation_of_receiver_group.to_i32().unwrap())?;
    let b44_48 = coord_byter(trace_header.surface_elevation_of_source.to_i32().unwrap())?;
    let b48_52 = coord_byter(trace_header.source_depth.to_i32().unwrap())?;
    let b52_56 = coord_byter(
        trace_header
            .datum_elevation_of_receiver_group
            .to_i32()
            .unwrap(),
    )?;
    let b56_60 = coord_byter(trace_header.datum_elevation_of_source.to_i32().unwrap())?;
    let b60_64 = coord_byter(trace_header.water_column_height_at_source.to_i32().unwrap())?;
    let b64_68 = coord_byter(trace_header.water_column_height_at_group.to_i32().unwrap())?;
    //////////////////////////////////////

    let b68_70 = i16_to_b(trace_header.elevation_scalar.to_i16().unwrap());
    let b70_72 = i16_to_b(trace_header.coordinate_scalar);
    //// These are coordinates. They use the `coord_byter`.
    let b72_76 = coord_byter(trace_header.source_x.to_i32().unwrap())?;
    let b76_80 = coord_byter(trace_header.source_y.to_i32().unwrap())?;
    let b80_84 = coord_byter(trace_header.receiver_group_x.to_i32().unwrap())?;
    let b84_88 = coord_byter(trace_header.receiver_group_y.to_i32().unwrap())?;
    //////////////////////////////////
    let b88_90 = u16_to_b(trace_header.coordinate_units.to_u16().unwrap());
    let b90_92 = u16_to_b(trace_header.weathing_velocity.to_u16().unwrap());
    let b92_94 = u16_to_b(trace_header.sub_weathering_velocity.to_u16().unwrap());
    let b94_96 = u16_to_b(trace_header.uphole_time_at_source.to_u16().unwrap());
    let b96_98 = u16_to_b(trace_header.uphole_time_at_group.to_u16().unwrap());
    let b98_100 = u16_to_b(trace_header.source_static_correction.to_u16().unwrap());

    let b100_102 = u16_to_b(trace_header.group_static_correction.to_u16().unwrap());
    let b102_104 = u16_to_b(trace_header.total_static_applied.to_u16().unwrap());
    let b104_106 = u16_to_b(trace_header.lag_time_a.to_u16().unwrap());
    let b106_108 = u16_to_b(trace_header.lag_time_b.to_u16().unwrap());
    let b108_110 = u16_to_b(trace_header.delay_recording_time.to_u16().unwrap());
    let b110_112 = u16_to_b(trace_header.mute_time_start.to_u16().unwrap());
    let b112_114 = u16_to_b(trace_header.mute_time_end.to_u16().unwrap());
    let b114_116 = u16_to_b(trace_header.no_samples_in_trace.to_u16().unwrap());
    let b116_118 = u16_to_b(trace_header.sample_interval_of_trace.to_u16().unwrap());
    let b118_120 = u16_to_b(trace_header.gain_type.to_u16().unwrap());

    let b120_122 = u16_to_b(trace_header.instrument_gain_constant.to_u16().unwrap());
    let b122_124 = u16_to_b(trace_header.instrument_initial_gain.to_u16().unwrap());
    let b124_126 = u16_to_b(trace_header.correlated.to_u16().unwrap());
    let b126_128 = u16_to_b(trace_header.sweep_frequency_at_start.to_u16().unwrap());
    let b128_130 = u16_to_b(trace_header.sweep_frequency_at_end.to_u16().unwrap());
    let b130_132 = u16_to_b(trace_header.sweep_length.to_u16().unwrap());
    let b132_134 = u16_to_b(trace_header.sweep_type.to_u16().unwrap());
    let b134_136 = u16_to_b(
        trace_header
            .sweep_trace_taper_length_at_start
            .to_u16()
            .unwrap(),
    );
    let b136_138 = u16_to_b(
        trace_header
            .sweep_trace_taper_length_at_end
            .to_u16()
            .unwrap(),
    );
    let b138_140 = u16_to_b(trace_header.taper_type.to_u16().unwrap());

    let b140_142 = u16_to_b(trace_header.alias_filter_frequency.to_u16().unwrap());
    let b142_144 = u16_to_b(trace_header.alias_filter_slope.to_u16().unwrap());
    let b144_146 = u16_to_b(trace_header.notch_filter_frequency.to_u16().unwrap());
    let b146_148 = u16_to_b(trace_header.notch_filter_slope.to_u16().unwrap());
    let b148_150 = u16_to_b(trace_header.low_cut_frequency.to_u16().unwrap());
    let b150_152 = u16_to_b(trace_header.high_cut_frequency.to_u16().unwrap());
    let b152_154 = u16_to_b(trace_header.low_cut_slope.to_u16().unwrap());
    let b154_156 = u16_to_b(trace_header.low_cut_frequency.to_u16().unwrap());
    let b156_158 = u16_to_b(trace_header.year_recorded.to_u16().unwrap());
    let b158_160 = u16_to_b(trace_header.day_of_year.to_u16().unwrap());

    let b160_162 = u16_to_b(trace_header.hour_of_day.to_u16().unwrap());
    let b162_164 = u16_to_b(trace_header.minute_of_hour.to_u16().unwrap());
    let b164_166 = u16_to_b(trace_header.second_of_minute.to_u16().unwrap());
    let b166_168 = u16_to_b(trace_header.time_base_code.to_u16().unwrap());
    let b168_170 = u16_to_b(trace_header.trace_weighting_factor.to_u16().unwrap());
    let b170_172 = u16_to_b(
        trace_header
            .geophone_group_number_roll_pos1
            .to_u16()
            .unwrap(),
    );
    let b172_174 = u16_to_b(
        trace_header
            .geophone_group_number_first_trace_orig_field
            .to_u16()
            .unwrap(),
    );
    let b174_176 = u16_to_b(
        trace_header
            .geophone_group_number_last_trace_orig_field
            .to_u16()
            .unwrap(),
    );
    let b176_178 = u16_to_b(trace_header.gap_size.to_u16().unwrap());
    let b178_180 = u16_to_b(trace_header.over_travel.to_u16().unwrap());

    // REDO WITH COORDINATE PARSER!!!!!!!
    let x_ensemble_bytes = coord_byter(trace_header.x_ensemble)?;
    let y_ensemble_bytes = coord_byter(trace_header.y_ensemble)?;
    //////////////////////////////////////////////

    let inline_no_bytes = i32_to_b(trace_header.inline_no.to_i32().unwrap());
    let xline_no_bytes = i32_to_b(trace_header.crossline_no.to_i32().unwrap());

    let b196_200 = i32_to_b(trace_header.shot_point_no.to_i32().unwrap());
    let b200_202 = u16_to_b(trace_header.shot_point_scalar.to_u16().unwrap());
    let b202_204 = i16_to_b(trace_header.trace_value_measurement_unit.to_i16().unwrap());
    let b204_208 = i32_to_b(
        trace_header
            .transduction_constant_mantissa
            .to_i32()
            .unwrap(),
    );
    let b208_210 = u16_to_b(trace_header.transduction_constant_power.to_u16().unwrap());
    let b210_212 = i16_to_b(trace_header.transduction_units.to_i16().unwrap());
    let b212_214 = u16_to_b(trace_header.trace_identifier.to_u16().unwrap());
    let b214_216 = u16_to_b(trace_header.time_scalar_trace_header.to_u16().unwrap());
    let b216_218 = i16_to_b(trace_header.source_type.to_i16().unwrap());
    let b218_220 = u16_to_b(trace_header.source_energy_direction_v.to_u16().unwrap());

    let b220_222 = u16_to_b(trace_header.source_energy_direction_il.to_u16().unwrap());
    let b222_224 = u16_to_b(trace_header.source_energy_direction_xl.to_u16().unwrap());
    let b224_228 = i32_to_b(trace_header.source_measurement_mantissa.to_i32().unwrap());
    let b228_230 = u16_to_b(trace_header.transduction_constant_power.to_u16().unwrap());
    let b230_232 = i16_to_b(trace_header.source_measurement_unit.to_i16().unwrap());
    let b232_240 = if le {
        trace_header.trace_name
    } else {
        trace_header
            .trace_name
            .iter()
            .copied()
            .rev()
            .collect::<Vec<u8>>()
            .try_into()
            .expect("The arrays are the same length")
    };

    array_cpy(&mut output, &b0_4, 0);
    array_cpy(&mut output, &b4_8, 4);
    array_cpy(&mut output, &b8_12, 8);
    array_cpy(&mut output, &b12_16, 12);
    array_cpy(&mut output, &b16_20, 16);
    array_cpy(&mut output, &b20_24, 20);
    array_cpy(&mut output, &b24_28, 24);
    array_cpy(&mut output, &b28_30, 28);
    array_cpy(&mut output, &b30_32, 30);
    array_cpy(&mut output, &b32_34, 32);
    array_cpy(&mut output, &b34_36, 34);
    array_cpy(&mut output, &b36_40, 36);
    array_cpy(&mut output, &b40_44, 40);
    array_cpy(&mut output, &b44_48, 44);

    array_cpy(&mut output, &b48_52, 48);
    array_cpy(&mut output, &b52_56, 52);
    array_cpy(&mut output, &b56_60, 56);
    array_cpy(&mut output, &b60_64, 60);
    array_cpy(&mut output, &b64_68, 64);
    array_cpy(&mut output, &b68_70, 68);
    array_cpy(&mut output, &b70_72, 70);

    array_cpy(&mut output, &b72_76, 72);
    array_cpy(&mut output, &b76_80, 76);
    array_cpy(&mut output, &b80_84, 80);
    array_cpy(&mut output, &b84_88, 84);
    array_cpy(&mut output, &b88_90, 88);
    array_cpy(&mut output, &b90_92, 90);
    array_cpy(&mut output, &b92_94, 92);
    array_cpy(&mut output, &b94_96, 94);
    array_cpy(&mut output, &b96_98, 96);
    array_cpy(&mut output, &b98_100, 98);

    array_cpy(&mut output, &b100_102, 100);
    array_cpy(&mut output, &b102_104, 102);
    array_cpy(&mut output, &b104_106, 104);
    array_cpy(&mut output, &b106_108, 106);
    array_cpy(&mut output, &b108_110, 108);
    array_cpy(&mut output, &b110_112, 110);
    array_cpy(&mut output, &b112_114, 112);
    array_cpy(&mut output, &b114_116, 114);
    array_cpy(&mut output, &b116_118, 116);
    array_cpy(&mut output, &b118_120, 118);

    array_cpy(&mut output, &b120_122, 120);
    array_cpy(&mut output, &b122_124, 122);
    array_cpy(&mut output, &b124_126, 124);
    array_cpy(&mut output, &b126_128, 126);
    array_cpy(&mut output, &b128_130, 128);
    array_cpy(&mut output, &b130_132, 130);
    array_cpy(&mut output, &b132_134, 132);
    array_cpy(&mut output, &b134_136, 134);
    array_cpy(&mut output, &b136_138, 136);
    array_cpy(&mut output, &b138_140, 138);

    array_cpy(&mut output, &b140_142, 140);
    array_cpy(&mut output, &b142_144, 142);
    array_cpy(&mut output, &b144_146, 144);
    array_cpy(&mut output, &b146_148, 146);
    array_cpy(&mut output, &b148_150, 148);
    array_cpy(&mut output, &b150_152, 150);
    array_cpy(&mut output, &b152_154, 152);
    array_cpy(&mut output, &b154_156, 154);
    array_cpy(&mut output, &b156_158, 156);
    array_cpy(&mut output, &b158_160, 150);
    array_cpy(&mut output, &b160_162, 160);

    array_cpy(&mut output, &b162_164, 162);
    array_cpy(&mut output, &b164_166, 164);
    array_cpy(&mut output, &b166_168, 166);
    array_cpy(&mut output, &b168_170, 168);
    array_cpy(&mut output, &b170_172, 170);
    array_cpy(&mut output, &b172_174, 172);
    array_cpy(&mut output, &b174_176, 174);
    array_cpy(&mut output, &b176_178, 176);
    array_cpy(&mut output, &b178_180, 178);

    array_cpy(&mut output, &b196_200, 196);
    array_cpy(&mut output, &b200_202, 200);
    array_cpy(&mut output, &b202_204, 202);

    array_cpy(&mut output, &b204_208, 204);
    array_cpy(&mut output, &b208_210, 208);
    array_cpy(&mut output, &b210_212, 210);
    array_cpy(&mut output, &b212_214, 212);
    array_cpy(&mut output, &b214_216, 214);
    array_cpy(&mut output, &b216_218, 216);
    array_cpy(&mut output, &b218_220, 218);
    array_cpy(&mut output, &b220_222, 220);
    array_cpy(&mut output, &b222_224, 222);
    array_cpy(&mut output, &b224_228, 224);
    array_cpy(&mut output, &b228_230, 228);

    array_cpy(&mut output, &b230_232, 230);
    array_cpy(&mut output, &b232_240, 232);

    // NB: These parameters will simply over-write whatever is already there. This system
    // does nothing to compensate for the lost data.
    array_cpy(
        &mut output,
        &x_ensemble_bytes,
        settings.get_x_ensemble_bidx(),
    );
    array_cpy(
        &mut output,
        &y_ensemble_bytes,
        settings.get_y_ensemble_bidx(),
    );
    array_cpy(&mut output, &inline_no_bytes, settings.get_inline_no_bidx());
    array_cpy(
        &mut output,
        &xline_no_bytes,
        settings.get_crossline_no_bidx(),
    );
    debug_assert_eq!(output.len(), 240);
    Ok(output)
}

fn array_cpy(dest: &mut [u8], src: &[u8], idx: usize) {
    for (i, v) in src.iter().enumerate() {
        dest[idx + i] = *v;
    }
}
