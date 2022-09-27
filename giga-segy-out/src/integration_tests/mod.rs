// Copyright (C) 2022 by GiGa infosystems
//! This module uses `rust_segy_input` and `rust_segy_output` to make roundtrip
//! tests and observe whether the crates work in a compatible manner. They
//! perform tests with two simplified geometry types, a trace based `Survey`
//! type and 3D voxet type.
use crate::create_headers::CreateBinHeader;
use crate::create_headers::CreateTraceHeader;
use crate::utils::CoordinateScalar;
use crate::SegyFile;

use giga_segy_core::enums::*;
use giga_segy_core::errors::*;
use giga_segy_core::{BinHeader, SegySettings, TraceHeader};
use num::ToPrimitive;
use std::fmt::Debug;
use std::path::Path;

mod survey_roundtrip;

/// An XYZ point.
#[derive(Clone, Debug, PartialEq)]
struct Pt {
    x: f32,
    y: f32,
    z: f32,
}

/// Crossline/inline.
#[derive(Clone, Debug, PartialEq)]
struct Line {
    inline: i32,
    xline: i32,
}

/// The data conserved in a temporal trace, or a vertical borehole.
#[derive(Clone, Debug, PartialEq)]
struct DataTrace {
    coords: Pt,
    line: Line,
    data: Vec<f32>,
}

/// A package that contains allt he data in a survey.
#[derive(Clone, Debug, PartialEq)]
struct Survey {
    name: String,
    description: String,
    data_interval: f32,
    data: Vec<DataTrace>,
}

impl Survey {
    fn write(
        &self,
        path: &Path,
        written_sample_format: SampleFormatCode,
        settings: SegySettings,
        scale_multiplier: f32,
    ) -> Result<SegyFile<SegySettings>, RsgError> {
        let final_name = path.join(&self.name).with_extension("sgy");

        let longest_trace = self
            .data
            .iter()
            .map(|d| d.data.len())
            .max()
            .expect("Empty survey");

        let mut bin_header = BinHeader::new(
            self.data.len() as u16,
            self.data_interval as u16,
            longest_trace as u16,
            written_sample_format,
        );
        bin_header.set_measurement_system(MeasurementSystem::Meters);

        let mut segy_file = SegyFile::create_file(
            final_name,
            settings,
            self.description.to_string(),
            bin_header,
            None,
        )?;

        for (i, t) in self.data.iter().enumerate() {
            // Convert ensemble into a SEGY writeable format.
            let scale_converter =
                CoordinateScalar::from_multiplier(scale_multiplier).ok_or_else(|| {
                    RsgError::InvalidHeader {
                        msg: format!("Bad scale multiplier {}", scale_multiplier),
                    }
                })?;
            let x_ensemble = scale_converter.scale_to_i32(t.coords.x).ok_or_else(|| {
                RsgError::SEGYSettingsError {
                    msg: format!("Bad conversion {}", t.coords.x),
                }
            })?;
            let y_ensemble = scale_converter.scale_to_i32(t.coords.y).ok_or_else(|| {
                RsgError::SEGYSettingsError {
                    msg: format!("Bad conversion {}", t.coords.y),
                }
            })?;
            let elevation = scale_converter.scale_to_i32(t.coords.z).ok_or_else(|| {
                RsgError::SEGYSettingsError {
                    msg: format!("Bad conversion {}", t.coords.z),
                }
            })?;
            let coord_scalar = scale_converter.writeable_scalar();

            let mut trace_header = TraceHeader::new_2d(x_ensemble, y_ensemble, coord_scalar);

            trace_header.trace_sequence_in_file = i as i32;
            trace_header.trace_no = i as i32;
            trace_header.datum_elevation_of_source = elevation;
            trace_header.elevation_scalar = coord_scalar;
            trace_header.no_samples_in_trace = t.data.len() as u16;
            trace_header.inline_no = t.line.inline;
            trace_header.crossline_no = t.line.xline;

            // Write data to file.
            segy_file.add_trace(trace_header, None, t.data.to_owned())?;
        }
        Ok(segy_file)
    }

    fn read(path: &str, name: &str, settings: SegySettings) -> Result<Self, RsgError> {
        let segy_file = giga_segy_in::SegyFile::open(path, settings)?;

        let bin_header = segy_file.get_bin_header();
        let mut data = Vec::with_capacity(segy_file.trace_count());
        for trace in segy_file.traces_iter() {
            // Data and header.
            let trace_data = segy_file.get_trace_data_as_f32_from_trace(trace)?;
            let header = trace.get_header();

            // Converter from ensemble, height and coordinates. Our measurement system
            // is always meters.

            let e_scalar = header.elevation_scalar;
            let scalar = header.coordinate_scalar;
            let converter = |s: i16| match s.cmp(&0) {
                std::cmp::Ordering::Greater => s as f32,
                std::cmp::Ordering::Less => -1. / s as f32,
                std::cmp::Ordering::Equal => 1.,
            };

            let coords = Pt {
                x: converter(scalar) * header.x_ensemble as f32,
                y: converter(scalar) * header.y_ensemble as f32,
                z: converter(e_scalar) * header.datum_elevation_of_source as f32,
            };

            let line = Line {
                inline: header.inline_no,
                xline: header.crossline_no,
            };

            data.push(DataTrace {
                coords,
                line,
                data: trace_data,
            });
        }

        Ok(Survey {
            name: name.to_owned(),
            description: segy_file.get_text_header().to_owned(),
            data_interval: bin_header.sample_interval as f32,
            data,
        })
    }
}

/// The data contained in a single XY layer of a Voxet dataset.
/// X is the fast axis and Z is the slow axis.
type LayerData<T> = Vec<T>;

struct Size {
    x: i32,
    y: i32,
    z: i32,
}

/// The structure represent a simple SEGY compatible voxet.
#[allow(dead_code)]
struct Voxet<T: ToPrimitive + Debug> {
    name: String,
    description: String,
    size: Size,
    start_pt: Pt,
    end_pt: Pt,
    data: Vec<LayerData<T>>,
}

#[allow(dead_code)]
impl<T: ToPrimitive + Debug + Copy> Voxet<T> {
    fn write(&self, path: &Path, written_sample_format: SampleFormatCode) -> Result<(), RsgError> {
        let final_name = path.join(&self.name).with_extension("sgy");

        let no_traces = (self.size.x * self.size.y) as u16;
        let no_samples = (self.size.z) as u16;
        let sample_interval = ((self.end_pt.z - self.start_pt.z) / no_samples as f32) as u16;
        let mut bin_header = BinHeader::new(
            no_traces,
            sample_interval,
            no_samples,
            written_sample_format,
        );
        bin_header.set_measurement_system(MeasurementSystem::Meters);

        let mut segy_file = SegyFile::create_file(
            final_name,
            SegySettings::default(),
            self.description.to_string(),
            bin_header,
            None,
        )?;

        let theoretical_size = (self.size.x * self.size.y) as usize;

        for i in 0..theoretical_size {
            if self.data[i].len() != theoretical_size {
                return Err(RsgError::SEGYSettingsError {
                    msg: "Corrupt Voxet with different sized layers.".to_string(),
                });
            }

            // Get data coordinates.
            let x_row = i as i32 % self.size.x;
            let y_row = i as i32 / self.size.x;
            let x_ensemble =
                self.start_pt.x + x_row as f32 * (self.end_pt.x - self.end_pt.x) as f32;
            let y_ensemble =
                self.start_pt.y + y_row as f32 * (self.end_pt.y - self.end_pt.y) as f32;

            // Convert ensemble into a SEGY writeable format.
            let scale_converter = CoordinateScalar::from_multiplier(100.).unwrap();
            let x_ensemble = scale_converter.scale_to_i32(x_ensemble).unwrap();
            let y_ensemble = scale_converter.scale_to_i32(y_ensemble).unwrap();
            let elevation = scale_converter.scale_to_i32(self.start_pt.z).unwrap();
            let coord_scalar = scale_converter.writeable_scalar();

            // Create trace header. (This is a cat)
            let mut trace_header =
                TraceHeader::new_3d(x_ensemble, y_ensemble, x_row, y_row, coord_scalar);
            trace_header.trace_sequence_in_file = i as i32;
            trace_header.trace_no = i as i32;
            trace_header.trace_sequence_on_line = x_row;
            trace_header.datum_elevation_of_source = elevation;
            trace_header.elevation_scalar = coord_scalar;

            let trace_data = self.data.iter().map(|v| v[i]).collect::<Vec<_>>();

            // Write data to file.
            segy_file.add_trace(trace_header, None, trace_data)?;
        }
        Ok(())
    }

    fn read(_path: &Path) -> Result<Self, RsgError> {
        unimplemented!();
    }
}
