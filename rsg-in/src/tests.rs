use crate::memory_map::*;

use rsg_core::SegySettings;

#[cfg(test)]
// NB: Some tests will only work with a valid SEGY file in the right place.
const TEST_FILE: &str = "../testdata/DutchMiniHead.sgy";

#[test]
fn test_map_file_to_memory() {
    let map = MappedSegY::new(TEST_FILE);
    assert!(map.is_ok());
    let map = map.unwrap();
    println!("map len={}", map.map.len());
}

#[test]
fn test_get_tape_label_of_mapped_segy_y() {
    let s = SegySettings::default();
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let label = map.get_tape_label(&s).expect("Should be Ok but isn't.");
    assert!(label.is_none());
}

#[test]
fn test_get_bin_header() {
    let s = SegySettings::default();
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let bin_header = map.get_bin_header(&s);
    println!("{:?}", bin_header);
    assert!(bin_header.is_ok());
}

#[test]
fn test_get_text_header() {
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let text_header = map.get_text_header();
    println!("{:?}", text_header);
    assert!(text_header.is_ok());
    assert_eq!(text_header.unwrap().chars().nth(0), Some('C'));
}

#[test]
fn test_get_extended_text_headers() {
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let text_headers = map.get_extended_text_headers(0);
    println!("{:?}", text_headers);
    assert!(text_headers.is_ok());
    assert!(text_headers.unwrap().is_empty());
}

#[test]
fn test_get_trace_headers() {
    let s = SegySettings::default();
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let mut bin_header = map.get_bin_header(&s).expect("Bin header is dead.");
    let extended_headers = map
        .get_extended_text_headers(0)
        .expect("extended header is dead.");
    let trace_headers = map.get_metadata_for_traces(&mut bin_header, extended_headers.len(), &s);
    if trace_headers.is_err() {
        println!("{:?}", trace_headers);
    }

    assert!(trace_headers.is_ok());
    let trace_headers = trace_headers.unwrap();
    assert_eq!(trace_headers.len(), 2500);
    assert_eq!(trace_headers[0].get_start(), 3840);
    assert_eq!(trace_headers[0].len(), 50 * 4);
    println!("{:?}", trace_headers[2499]);
}

#[test]
fn test_get_trace_data_as_bytes_unprocessed() {
    let s = SegySettings::default();
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let mut bin_header = map.get_bin_header(&s).expect("Bin header is dead.");
    let extended_headers = map
        .get_extended_text_headers(0)
        .expect("extended header is dead.");
    let trace_headers = map
        .get_metadata_for_traces(&mut bin_header, extended_headers.len(), &s)
        .expect("Could not header the traces.");

    let data = crate::read_data::get_trace_data_as_bytes_unprocessed(
        &map,
        &trace_headers[0],
        &bin_header,
        &s,
    )
    .expect("Could not get data.");

    assert_eq!(data.len(), trace_headers[0].len());
    assert_eq!(data[0], map.map[trace_headers[0].get_start()]);
    println!("{:?}", data);
}

#[test]
fn test_get_trace_data_as_bytes_unprocessed2() {
    let s = SegySettings::default();
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let mut bin_header = map.get_bin_header(&s).expect("Bin header is dead.");
    let extended_headers = map
        .get_extended_text_headers(0)
        .expect("extended header is dead.");
    let trace_headers = map
        .get_metadata_for_traces(&mut bin_header, extended_headers.len(), &s)
        .expect("Could not header the traces.");

    let data = crate::read_data::get_trace_data_as_bytes_unprocessed(
        &map,
        &trace_headers[2499],
        &bin_header,
        &s,
    )
    .expect("Could not get data.");

    assert_eq!(data.len(), trace_headers[2499].len());
    assert_eq!(data[0], map.map[trace_headers[2499].get_start()]);
    println!("{:?}", data);
}

#[test]
fn test_get_trace_data_point_as_bytes_unprocessed() {
    let s = SegySettings::default();
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let mut bin_header = map.get_bin_header(&s).expect("Bin header is dead.");
    let extended_headers = map
        .get_extended_text_headers(0)
        .expect("extended header is dead.");
    let trace_headers = map
        .get_metadata_for_traces(&mut bin_header, extended_headers.len(), &s)
        .expect("Could not header the traces.");

    let data = crate::read_data::get_trace_data_as_bytes_unprocessed(
        &map,
        &trace_headers[0],
        &bin_header,
        &s,
    )
    .expect("Could not get data.");
    let data_point = crate::read_data::get_trace_data_point_as_bytes_unprocessed(
        &map,
        &trace_headers[0],
        &bin_header,
        &s,
        0,
    )
    .expect("Could not get data.");

    assert_eq!(
        &data_point[..],
        &data[0..bin_header.sample_format_code.datum_byte_length()]
    );
}

#[test]
fn test_get_trace_data_as_f32() {
    let s = SegySettings::default();
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let mut bin_header = map.get_bin_header(&s).expect("Bin header is dead.");
    let extended_headers = map
        .get_extended_text_headers(0)
        .expect("extended header is dead.");
    let trace_headers = map
        .get_metadata_for_traces(&mut bin_header, extended_headers.len(), &s)
        .expect("Could not header the traces.");

    let data = crate::read_data::get_trace_data_as_f32(&map, &trace_headers[0], &bin_header, &s)
        .expect("Could not get data.");

    assert_eq!(
        data.len(),
        trace_headers[0].len() / bin_header.sample_format_code.datum_byte_length()
    );
    // assert_eq!(data[0], map.map[trace_headers[0].trace_start_byte]);
    println!("{:?}", data);

    for h in trace_headers.into_iter().take(100) {
        let data = crate::read_data::get_trace_data_as_f32(&map, &h, &bin_header, &s)
            .expect("Could not get data.");
        println!("{:?}", data);
        assert_eq!(
            data.len(),
            h.len() / bin_header.sample_format_code.datum_byte_length()
        );
    }
}

#[test]
fn test_get_trace_data_point_as_f32() {
    let s = SegySettings::default();
    let map = MappedSegY::new(TEST_FILE).expect("Couldn't map SEG-Y");
    let mut bin_header = map.get_bin_header(&s).expect("Bin header is dead.");
    let extended_headers = map
        .get_extended_text_headers(0)
        .expect("extended header is dead.");
    let trace_headers = map
        .get_metadata_for_traces(&mut bin_header, extended_headers.len(), &s)
        .expect("Could not header the traces.");

    for h in trace_headers.into_iter().take(100) {
        let data = crate::read_data::get_trace_data_as_f32(&map, &h, &bin_header, &s)
            .expect("Could not get data.");
        let data_point =
            crate::read_data::get_trace_data_point_as_f32(&map, &h, &bin_header, &s, 0)
                .expect("Could not get data point.");

        assert_eq!(data_point, data[0]);
    }
}

#[test]
fn test_open_file() {
    let s = SegySettings::default();
    let segy = crate::SegyFile::open(TEST_FILE, s);
    assert!(segy.is_ok());
}

#[test]
fn test_open_file_get_get_trace_data_as_f32() {
    let s = SegySettings::default();
    let segy = crate::SegyFile::open(TEST_FILE, s).unwrap();

    for h in segy.traces_iter().take(1000) {
        let data = segy
            .get_trace_data_as_f32_from_trace(&h)
            .expect("Could not get f32 data.");
        println!("{:?}", data);
        assert_eq!(
            data.len(),
            h.len() / segy.get_bin_header().sample_format_code.datum_byte_length()
        );
    }
}

#[test]
fn test_open_file_get_get_trace_data_as_bytes_unprocessed() {
    let s = SegySettings::default();
    let segy = crate::SegyFile::open(TEST_FILE, s).unwrap();

    for h in segy.traces_iter().take(1000) {
        let data = segy
            .get_trace_data_as_bytes_from_trace(&h)
            .expect("Could not get u8 data.");
        println!("{:?}", data);
        assert_eq!(data.len(), h.len());
    }
}

#[test]
fn test_get_trace_name() {
    let s = SegySettings::default();
    let segy = crate::SegyFile::open(TEST_FILE, s).unwrap();

    let mut th = segy.traces_iter().nth(0).unwrap().get_header().to_owned();
    th.trace_name = [b'S', b'E', b'G', b'0', b'1', b'2', b'3', b'4'];
    let string = th
        .get_trace_name();
    assert_eq!(&string, "SEG01234");
}
