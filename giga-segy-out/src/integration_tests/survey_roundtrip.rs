// Copyright (C) 2022 by GiGa infosystems.
//! Here we write and read a `Survey`, to test that `rust_segy_input` and
//! `rust_segy_output` are compatible.
use tempfile;

use super::*;
use giga_segy_core::SampleFormatCode as Sac;

fn create_survey(
    name: &str,
    description: &str,
    data_interval: f32,
    data_coords: Vec<(Pt, Line)>,
    trace_len: u16,
) -> Survey {
    let data = data_coords
        .into_iter()
        .map(|(pt, l)| {
            let data = (0..(trace_len as u16 * 3))
                .step_by(3)
                .map(|n| n as f32)
                .collect::<Vec<f32>>();
            DataTrace {
                coords: pt,
                line: l,
                data,
            }
        })
        .collect::<Vec<_>>();

    Survey {
        name: name.to_owned(),
        description: description.to_owned(),
        data_interval,
        data,
    }
}

fn generate_coords(x: i32, y: i32) -> Vec<(Pt, Line)> {
    let mut coords = Vec::with_capacity((x * y) as usize);
    for i in 0..x {
        for j in 0..y {
            let p = Pt {
                x: 500. * i as f32,
                y: 400. * j as f32,
                z: ((x * y * 100) as f32),
            };
            let l = Line {
                inline: i,
                xline: j,
            };
            coords.push((p, l));
        }
    }
    coords
}

fn check(read: &Survey, survey: &Survey, data_ok: bool, coords_ok: bool) {
    assert_eq!(read.name, survey.name);
    // TODO: Get text header working!
    assert_eq!(read.description.trim_end(), survey.description);
    assert_eq!(read.data_interval, survey.data_interval);
    assert_eq!(read.data.len(), survey.data.len());

    let mut all_data_equal = true;
    let mut all_lines_equal = true;
    let mut all_coords_equal = true;
    for (out, inn) in read.data.iter().zip(survey.data.iter()) {
        if out.data != inn.data {
            all_data_equal = false;
        }
        if (out.data == inn.data) != data_ok {
            println!("out:{:?}\nin:{:?}", out.data, inn.data);
        }
        if out.line != inn.line {
            all_lines_equal = false;
        }
        if (out.line == inn.line) != coords_ok {
            println!("out:{:?}\nin:{:?}", out.line, inn.line);
        }
        if out.coords != inn.coords {
            all_coords_equal = false;
        }
        if (out.coords == inn.coords) != coords_ok {
            println!("out:{:?}\nin:{:?}", out.coords, inn.coords);
        }
    }
    if data_ok != all_data_equal {
        panic!(
            "All data should be same? {}\nAll data is the same? {}",
            data_ok, all_data_equal
        );
    }
    if coords_ok && !(all_lines_equal && all_coords_equal) {
        panic!(
            "All Coords should be same but are not (lines:{}, coords:{}",
            all_lines_equal, all_coords_equal
        );
    }
    if !coords_ok && (all_coords_equal && all_lines_equal) {
        panic!("Coordinate lossiness should be observed but is not");
    }
}

fn check_headers(read: giga_segy_in::SegyFile, bin_h: BinHeader, tr_h: Vec<TraceHeader>) {
    if &bin_h != read.get_bin_header() {
        println!("new_bin_header:{:#?}", read.get_bin_header());
        println!("old_bin_header:{:#?}", bin_h);
    }
    assert_eq!(read.get_bin_header(), &bin_h, "bin headers are not equal.");
    assert_eq!(
        read.trace_count(),
        tr_h.len(),
        "trace count changed (original right)"
    );
    for (written, read) in tr_h.iter().zip(read.traces_iter()) {
        assert_eq!(written, read.get_header(), "trace headers are not equal.");
    }
}

fn test_write_survey_inner(
    survey: Survey,
    settings: SegySettings,
    data_ok: bool,
    coords_ok: bool,
    sample_format: Sac,
    multiplier: f32,
) {
    let dir = tempfile::tempdir().expect("Couldn't get tempfile.");
    {
        let path = dir.path();
        let final_name = path.join(&survey.name).with_extension("sgy");
        let path_str = final_name.to_str().expect("Couldn't string the path.");

        let res = survey.write(path, sample_format, settings.clone(), multiplier);
        assert!(res.is_ok(), "Could not write survey {:?}", survey);

        let read = Survey::read(path_str, "my_survey", settings);
        assert!(read.is_ok(), "Couldn't read SEGY survey. {:?}", read);

        let output = read.unwrap();
        check(&output, &survey, data_ok, coords_ok);
    }
}

#[test]
fn read_write_survey_1() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        100,
    );
    let settings = SegySettings::default();
    test_write_survey_inner(survey, settings, true, true, Sac::Float32, 100.);
}

#[test]
fn read_write_survey_2() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let settings = SegySettings::default();
    test_write_survey_inner(survey, settings, true, true, Sac::Float32, 1.);
}

#[test]
fn read_write_survey_scale_multiplier_too_high_precision_loss() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let settings = SegySettings::default();
    test_write_survey_inner(survey, settings, true, false, Sac::Float32, 1000.);
}

#[test]
fn read_write_survey_scale_multiplier_too_low_overflow() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let settings = SegySettings::default();

    let dir = tempfile::tempdir().expect("Couldn't get tempfile.");
    {
        let path = dir.path();

        let res = survey.write(path, Sac::Float32, settings, 0.000_000_01);
        assert!(
            res.is_err(),
            "Overflow should occur but does not {:?}",
            survey
        );
    }
}

#[test]
fn read_write_survey_u8_i8_sample_format_fails_for_big_floats() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let settings = SegySettings::default();

    let dir = tempfile::tempdir().expect("Couldn't get tempfile.");
    {
        let path = dir.path();

        let res = survey.write(path, Sac::UInt8, settings.clone(), 1.);
        assert!(
            res.is_err(),
            "Overflow should occur but does not {:?}",
            survey
        );

        let res = survey.write(path, Sac::Int8, settings, 1.);
        assert!(
            res.is_err(),
            "Overflow should occur but does not {:?}",
            survey
        );
    }
}

#[test]
fn read_write_survey_u32_sample_format_okish() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let settings = SegySettings::default();
    test_write_survey_inner(survey, settings, true, true, Sac::UInt32, 1.);
}

#[test]
fn read_write_survey_i32_sample_format_okish() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let settings = SegySettings::default();
    test_write_survey_inner(survey, settings, true, true, Sac::Int32, 1.);
}

#[test]
fn read_write_survey_coord_floats_1() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let mut settings = SegySettings::default();
    settings
        .set_override_coordinate_format(Sac::Float32)
        .expect("32-bit is ok.");
    test_write_survey_inner(survey, settings, true, true, Sac::Float32, 1.);
}

#[test]
fn read_write_survey_coord_floats_2() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let mut settings = SegySettings::default();
    settings
        .set_override_coordinate_format(Sac::Float32)
        .expect("32-bit is ok.");
    test_write_survey_inner(survey, settings, true, true, Sac::Float32, 100.);
}

#[test]
fn read_write_survey_line_bidx_shifts() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let mut settings = SegySettings::default();
    settings.set_inline_no_bidx(100).expect("It's fine.");
    settings.set_crossline_no_bidx(104).expect("It's fine.");

    test_write_survey_inner(survey, settings, true, true, Sac::Float32, 100.);
}

#[test]
fn read_write_survey_ensemble_bidx_shifts() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let mut settings = SegySettings::default();
    settings.set_x_ensemble_bidx(100).expect("It's fine.");
    settings.set_y_ensemble_bidx(104).expect("It's fine.");

    test_write_survey_inner(survey, settings, true, true, Sac::Float32, 100.);
}

#[test]
fn header_test_bidx_shifts() {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );
    let mut settings = SegySettings::default();
    // NB: We must be very careful what we do here, or we will corrupt the file.
    // Eg, byte 114-116 are the trace length. If we over-write that, we are screwed.
    settings.set_x_ensemble_bidx(96).expect("It's fine.");
    settings.set_y_ensemble_bidx(100).expect("It's fine.");
    settings.set_inline_no_bidx(104).expect("It's fine.");
    settings.set_crossline_no_bidx(108).expect("It's fine.");

    let dir = tempfile::tempdir().expect("Couldn't get tempfile.");
    {
        let path = dir.path();
        let final_name = path.join(&survey.name).with_extension("sgy");
        let path_str = final_name.to_str().expect("Couldn't string the path.");

        let res = survey.write(path, Sac::Float32, settings.clone(), 1.);
        let res = res.unwrap();
        let original_bin_header = res.metadata.get_bin_header().clone();
        let old_trace_headers = res
            .traces
            .iter()
            .map(|t| t.get_header())
            .cloned()
            .collect::<Vec<TraceHeader>>();

        // First to demonstrate that we have indeed shifted because trying to open
        // the file with default settings will give us super invalid headers.
        let read = giga_segy_in::SegyFile::open(path_str, SegySettings::default())
            .expect("Could not reopen");

        assert_eq!(&original_bin_header, read.get_bin_header());
        // There are some traces where inline_no and xline_no are zero anyway.
        assert!(!old_trace_headers.iter().all(|t| t.inline_no == 0));
        assert!(!old_trace_headers.iter().all(|t| t.crossline_no == 0));
        assert!(!old_trace_headers.iter().all(|t| t.x_ensemble == 0));
        assert!(!old_trace_headers.iter().all(|t| t.y_ensemble == 0));

        assert!(read.traces_iter().all(|t| t.get_header().x_ensemble == 0));
        assert!(read.traces_iter().all(|t| t.get_header().y_ensemble == 0));
        assert!(read.traces_iter().all(|t| t.get_header().inline_no == 0));
        assert!(read.traces_iter().all(|t| t.get_header().crossline_no == 0));

        // But if we open the file with the right settings, magic happens.
        let read = giga_segy_in::SegyFile::open(path_str, settings).expect("Could not reopen");

        assert_eq!(&original_bin_header, read.get_bin_header());
        for (w, r) in old_trace_headers.iter().zip(read.traces_iter()) {
            assert_eq!(w.inline_no, r.get_header().inline_no, "inline no :(");
            assert_eq!(w.crossline_no, r.get_header().crossline_no, "xline no :(");
            assert_eq!(w.x_ensemble, r.get_header().x_ensemble, "x_ensemble :(");
            assert_eq!(w.y_ensemble, r.get_header().y_ensemble, "y_ensemble :(");
        }
    }
}

fn header_tests_inner(settings: SegySettings, sample_format: SampleFormatCode) {
    let survey = create_survey(
        "my_survey",
        "I like surveys, and this one is pretty nifty I think.",
        5.,
        generate_coords(10, 10),
        200,
    );

    let dir = tempfile::tempdir().expect("Couldn't get tempfile.");
    {
        let path = dir.path();
        let final_name = path.join(&survey.name).with_extension("sgy");
        let path_str = final_name.to_str().expect("Couldn't string the path.");

        let res = survey.write(path, sample_format, settings.clone(), 1.);
        let res = res.unwrap();
        let original_bin_header = res.metadata.get_bin_header().clone();
        let original_trace_headers = res
            .traces
            .iter()
            .map(|t| t.get_header())
            .cloned()
            .collect::<Vec<_>>();

        let segy_file = giga_segy_in::SegyFile::open(path_str, settings).expect("Could not reopen");
        check_headers(segy_file, original_bin_header, original_trace_headers);
    }
}

#[test]
fn read_write_survey_check_headers_default() {
    let settings = SegySettings::default();
    let sample_format = Sac::Float32;
    header_tests_inner(settings, sample_format);
}

#[test]
fn read_write_survey_check_headers_default_f64() {
    let settings = SegySettings::default();
    let sample_format = Sac::Float64;
    header_tests_inner(settings, sample_format);
}

#[test]
fn read_write_survey_check_headers_default_i64() {
    let settings = SegySettings::default();
    let sample_format = Sac::Int64;
    header_tests_inner(settings, sample_format);
}

#[test]
fn read_write_survey_check_headers_default_u32() {
    let settings = SegySettings::default();
    let sample_format = Sac::UInt32;
    header_tests_inner(settings, sample_format);
}

#[test]
fn read_write_survey_check_headers_default_i16() {
    let settings = SegySettings::default();
    let sample_format = Sac::Int16;
    header_tests_inner(settings, sample_format);
}

#[test]
fn read_write_survey_check_headers_default_u16() {
    let settings = SegySettings::default();
    let sample_format = Sac::UInt16;
    header_tests_inner(settings, sample_format);
}
