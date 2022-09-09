mod header_structs {
    use crate::header_structs::*;
    #[test]
    fn test_c_safe_name() {
        let name0 = vec![b'T', b'e', b'a', b' ', b'b', b'a', b'g'];
        let processed = c_safe_name(&name0);
        assert_eq!(&processed, "Tea bag");
    }

    #[test]
    fn test_c_safe_nam1() {
        let name0 = vec![b'T', b'e', b'a', b'0', b'b', b'a', b'g'];
        let processed = c_safe_name(&name0);
        assert_eq!(&processed, "Tea0bag");
    }

    #[test]
    fn test_c_safe_name2() {
        let name0 = vec![b'T', b'e', b'a', 0, b'b', b'a', b'g'];
        let processed = c_safe_name(&name0);
        assert_eq!(&processed, "");
    }

    #[test]
    fn tapelabel_to_readable() {
        let l = TapeLabel {
            storage_unit_seq_no: [b'1', b'2', b'3', b'4'],
            segy_revision_no: [b'S', b'Y', b'2', b'.', b'0'],
            storage_unit_structure: [b'R', b'E', b'C', b'O', b'R', b'D'],
            binding_number: [b'B', b'9', b'9', b'9'],
            max_block_size: 1000,
            producing_organisation_code: [b'X'; 10],
            creation_date: [
                b'0', b'8', b'-', b'S', b'E', b'P', b'-', b'2', b'0', b'2', b'2',
            ],
            serial_number: [b'X'; 12],
            external_label: [b'Y'; 12],
            recording_entity: [b'Z'; 24],
            extra: [b'-'; 14],
        };

        let expected = ReadableTapeLabel {
            storage_unit_seq_no: String::from("1234"),
            segy_revision_no: String::from("SY2.0"),
            storage_unit_structure: String::from("RECORD"),
            binding_number: String::from("B999"),
            max_block_size: 1000,
            producing_organisation_code: String::from("XXXXXXXXXX"),
            creation_date: String::from("08-SEP-2022"),
            serial_number: String::from("XXXXXXXXXXXX"),
            external_label: String::from("YYYYYYYYYYYY"),
            recording_entity: String::from("ZZZZZZZZZZZZZZZZZZZZZZZZ"),
            extra: String::from("--------------"),
        };

        assert_eq!(expected, l.to_readable());
    }
}

mod enums {
    use crate::enums::*;
    #[test]
    fn sample_format_code_new() {
        assert_eq!(
            SampleFormatCode::new(1).unwrap(),
            SampleFormatCode::IbmFloat32
        );
        assert_eq!(SampleFormatCode::new(2).unwrap(), SampleFormatCode::Int32);
        assert_eq!(SampleFormatCode::new(3).unwrap(), SampleFormatCode::Int16);
        assert_eq!(
            SampleFormatCode::new(4).unwrap(),
            SampleFormatCode::FixPoint32
        );
        assert_eq!(SampleFormatCode::new(5).unwrap(), SampleFormatCode::Float32);
        assert_eq!(SampleFormatCode::new(6).unwrap(), SampleFormatCode::Float64);
        assert_eq!(SampleFormatCode::new(7).unwrap(), SampleFormatCode::Int24);
        assert_eq!(SampleFormatCode::new(8).unwrap(), SampleFormatCode::Int8);
        assert_eq!(SampleFormatCode::new(9).unwrap(), SampleFormatCode::Int64);
        assert_eq!(SampleFormatCode::new(10).unwrap(), SampleFormatCode::UInt32);
        assert_eq!(SampleFormatCode::new(11).unwrap(), SampleFormatCode::UInt16);
        assert_eq!(SampleFormatCode::new(12).unwrap(), SampleFormatCode::UInt64);
        assert!(SampleFormatCode::new(13).is_err());
        assert!(SampleFormatCode::new(14).is_err());
        assert_eq!(SampleFormatCode::new(15).unwrap(), SampleFormatCode::UInt24);
        assert_eq!(SampleFormatCode::new(16).unwrap(), SampleFormatCode::UInt8);
    }
    #[test]
    fn sample_format_code_datum_byte_length() {
        assert_eq!(SampleFormatCode::IbmFloat32.datum_byte_length(), 4);
        assert_eq!(SampleFormatCode::Int32.datum_byte_length(), 4);
        assert_eq!(SampleFormatCode::Int16.datum_byte_length(), 2);
        assert_eq!(SampleFormatCode::FixPoint32.datum_byte_length(), 4);
        assert_eq!(SampleFormatCode::Float32.datum_byte_length(), 4);
        assert_eq!(SampleFormatCode::Float64.datum_byte_length(), 8);
        assert_eq!(SampleFormatCode::Int24.datum_byte_length(), 3);
        assert_eq!(SampleFormatCode::Int8.datum_byte_length(), 1);
        assert_eq!(SampleFormatCode::Int64.datum_byte_length(), 8);
        assert_eq!(SampleFormatCode::UInt32.datum_byte_length(), 4);
        assert_eq!(SampleFormatCode::UInt16.datum_byte_length(), 2);
        assert_eq!(SampleFormatCode::UInt64.datum_byte_length(), 8);
        assert_eq!(SampleFormatCode::UInt24.datum_byte_length(), 3);
        assert_eq!(SampleFormatCode::UInt8.datum_byte_length(), 1);
    }

    #[test]
    fn trace_sorting_code_new() {
        use self::TraceSortingCode::*;
        assert_eq!(Other, TraceSortingCode::new(-1));
        assert_eq!(Unknown, TraceSortingCode::new(0));
        assert_eq!(AsRec, TraceSortingCode::new(1));
        assert_eq!(CDPEnsemble, TraceSortingCode::new(2));
        assert_eq!(SingleFoldContinuous, TraceSortingCode::new(3));
        assert_eq!(HorizontalStack, TraceSortingCode::new(4));
        assert_eq!(CommonSourcePoint, TraceSortingCode::new(5));
        assert_eq!(CommonReceiverPoint, TraceSortingCode::new(6));
        assert_eq!(CommonOffsetPoint, TraceSortingCode::new(7));
        assert_eq!(CommonMidPoint, TraceSortingCode::new(8));
        assert_eq!(CommonConversionPoint, TraceSortingCode::new(9));
        assert_eq!(Invalid, TraceSortingCode::new(-50));
        assert_eq!(Invalid, TraceSortingCode::new(50));
    }

    #[test]
    fn sweep_type_code_new() {
        use self::SweepTypeCode::*;
        assert_eq!(Unspecified, SweepTypeCode::new(0));
        assert_eq!(Linear, SweepTypeCode::new(1));
        assert_eq!(Parabolic, SweepTypeCode::new(2));
        assert_eq!(Exponential, SweepTypeCode::new(3));
        assert_eq!(Other, SweepTypeCode::new(4));
        assert_eq!(Invalid, SweepTypeCode::new(50));
    }

    #[test]
    fn taper_type_new() {
        use self::TaperType::*;
        assert_eq!(Unspecified, TaperType::new(0));
        assert_eq!(Linear, TaperType::new(1));
        assert_eq!(Cosine2, TaperType::new(2));
        assert_eq!(Other, TaperType::new(3));
        assert_eq!(Invalid, TaperType::new(50));
    }

    #[test]
    fn cdt_new() {
        use self::CorrelatedDataTraces::*;
        assert_eq!(Unspecified, CorrelatedDataTraces::new(0));
        assert_eq!(No, CorrelatedDataTraces::new(1));
        assert_eq!(Yes, CorrelatedDataTraces::new(2));
        assert_eq!(Invalid, CorrelatedDataTraces::new(40));
        assert_eq!(Invalid, CorrelatedDataTraces::new(3));
        assert_eq!(Invalid, CorrelatedDataTraces::new(255));
    }

    #[test]
    fn bgr_new() {
        use self::BinaryGainRecovered::*;
        assert_eq!(Unspecified, BinaryGainRecovered::new(0));
        assert_eq!(Yes, BinaryGainRecovered::new(1));
        assert_eq!(No, BinaryGainRecovered::new(2));
        assert_eq!(Invalid, BinaryGainRecovered::new(40));
        assert_eq!(Invalid, BinaryGainRecovered::new(3));
        assert_eq!(Invalid, BinaryGainRecovered::new(255));
    }

    #[test]
    fn arm_new() {
        use self::AmplitudeRecoveryMethod::*;
        assert_eq!(Unspecified, AmplitudeRecoveryMethod::new(0));
        assert_eq!(None, AmplitudeRecoveryMethod::new(1));
        assert_eq!(SphericalDivergence, AmplitudeRecoveryMethod::new(2));
        assert_eq!(Agc, AmplitudeRecoveryMethod::new(3));
        assert_eq!(Other, AmplitudeRecoveryMethod::new(4));
        assert_eq!(Invalid, AmplitudeRecoveryMethod::new(9));
    }

    #[test]
    fn measurement_system_new() {
        use self::MeasurementSystem::*;
        assert_eq!(Unspecified, MeasurementSystem::new(0));
        assert_eq!(Meters, MeasurementSystem::new(1));
        assert_eq!(Feet, MeasurementSystem::new(2));
        assert_eq!(Invalid, MeasurementSystem::new(6));
    }

    #[test]
    fn isp_new() {
        use self::ImpulseSignalPolarity::*;
        assert_eq!(Unspecified, ImpulseSignalPolarity::new(0));
        assert_eq!(IncreasePressureMinus, ImpulseSignalPolarity::new(1));
        assert_eq!(IncreasePressurePlus, ImpulseSignalPolarity::new(2));
        assert_eq!(Invalid, ImpulseSignalPolarity::new(50));
    }

    #[test]
    fn vpc_new() {
        use self::VibratoryPolarityCode::*;
        assert_eq!(Unspecified, VibratoryPolarityCode::new(0));
        assert_eq!(From338, VibratoryPolarityCode::new(1));
        assert_eq!(From23, VibratoryPolarityCode::new(2));
        assert_eq!(From68, VibratoryPolarityCode::new(3));
        assert_eq!(From113, VibratoryPolarityCode::new(4));
        assert_eq!(From158, VibratoryPolarityCode::new(5));
        assert_eq!(From203, VibratoryPolarityCode::new(6));
        assert_eq!(From248, VibratoryPolarityCode::new(7));
        assert_eq!(From293, VibratoryPolarityCode::new(8));
        assert_eq!(Invalid, VibratoryPolarityCode::new(99));
        assert_eq!(Invalid, VibratoryPolarityCode::new(9));
    }

    #[test]
    fn fixed_length_traces() {
        use self::FixedLengthTraces::*;
        assert_eq!(Yes, FixedLengthTraces::new(1).unwrap());
        assert_eq!(No, FixedLengthTraces::new(0).unwrap());
        assert!(FixedLengthTraces::new(55).is_err());
        assert!(FixedLengthTraces::new(2).is_err());
        assert!(FixedLengthTraces::new(255).is_err());
        assert!(!FixedLengthTraces::No.yes());
        assert!(FixedLengthTraces::No.no());
        assert!(!FixedLengthTraces::Yes.no());
        assert!(FixedLengthTraces::Yes.yes());
    }

    #[test]
    fn time_basis_new() {
        use self::TimeBasisCode::*;
        assert_eq!(Unspecified, TimeBasisCode::new(0));
        assert_eq!(Local, TimeBasisCode::new(1));
        assert_eq!(GreenwichGMT, TimeBasisCode::new(2));
        assert_eq!(Other, TimeBasisCode::new(3));
        assert_eq!(CoordinatedUTC, TimeBasisCode::new(4));
        assert_eq!(GlobalGPS, TimeBasisCode::new(5));
        assert_eq!(Invalid, TimeBasisCode::new(6));
        assert_eq!(Invalid, TimeBasisCode::new(66));
    }

    #[test]
    fn trace_id_code_new() {
        use self::TraceIdCode::*;
        assert_eq!(Other, TraceIdCode::new(-1));
        assert_eq!(Unknown, TraceIdCode::new(0));
        assert_eq!(TimeDomainSeismic, TraceIdCode::new(1));
        assert_eq!(Dead, TraceIdCode::new(2));
        assert_eq!(Dummy, TraceIdCode::new(3));
        assert_eq!(TimeBreak, TraceIdCode::new(4));
        assert_eq!(Uphole, TraceIdCode::new(5));
        assert_eq!(Sweep, TraceIdCode::new(6));
        assert_eq!(Timing, TraceIdCode::new(7));
        assert_eq!(Waterbreak, TraceIdCode::new(8));
        assert_eq!(NearFieldGunSig, TraceIdCode::new(9));
        assert_eq!(FarFieldGunSig, TraceIdCode::new(10));
        assert_eq!(SeismicPressureSensor, TraceIdCode::new(11));
        assert_eq!(MulticomponentVertical, TraceIdCode::new(12));
        assert_eq!(MulticomponentCrossLine, TraceIdCode::new(13));
        assert_eq!(MulticomponentInLine, TraceIdCode::new(14));
        assert_eq!(RotatedVertical, TraceIdCode::new(15));
        assert_eq!(RotatedTransverse, TraceIdCode::new(16));
        assert_eq!(RotatedRadial, TraceIdCode::new(17));
        assert_eq!(VibratorReactionMass, TraceIdCode::new(18));
        assert_eq!(VibratorBaseplate, TraceIdCode::new(19));
        assert_eq!(VibratorEstimatedGroundForce, TraceIdCode::new(20));
        assert_eq!(VibratorReference, TraceIdCode::new(21));
        assert_eq!(TimeVelocityPairs, TraceIdCode::new(22));
        assert_eq!(TimeDepthPairs, TraceIdCode::new(23));
        assert_eq!(DepthVelocityPairs, TraceIdCode::new(24));
        assert_eq!(DepthDomainSeismic, TraceIdCode::new(25));
        assert_eq!(GravityPotential, TraceIdCode::new(26));
        assert_eq!(EFVertical, TraceIdCode::new(27));
        assert_eq!(EFCrossLine, TraceIdCode::new(28));
        assert_eq!(EFInLine, TraceIdCode::new(29));
        assert_eq!(RotatedEFVertical, TraceIdCode::new(30));
        assert_eq!(RotatedEFTransverse, TraceIdCode::new(31));
        assert_eq!(RotatedEFRadial, TraceIdCode::new(32));
        assert_eq!(MFVertical, TraceIdCode::new(33));
        assert_eq!(MFCrossLine, TraceIdCode::new(34));
        assert_eq!(MFInLine, TraceIdCode::new(35));
        assert_eq!(RotatedMFVertical, TraceIdCode::new(36));
        assert_eq!(RotatedMFTransverse, TraceIdCode::new(37));
        assert_eq!(RotatedMFRadial, TraceIdCode::new(38));
        assert_eq!(RotatedSensorPitch, TraceIdCode::new(39));
        assert_eq!(RotatedSensorRoll, TraceIdCode::new(40));
        assert_eq!(RotatedSensorYaw, TraceIdCode::new(41));
        assert_eq!(Invalid, TraceIdCode::new(42));
        assert_eq!(Invalid, TraceIdCode::new(43));
        assert_eq!(Invalid, TraceIdCode::new(255));
        assert_eq!(Invalid, TraceIdCode::new(-255));
        assert_eq!(Invalid, TraceIdCode::new(-2));
    }

    #[test]
    fn data_use_new() {
        use self::DataUse::*;
        assert_eq!(Unspecified, DataUse::new(0));
        assert_eq!(Production, DataUse::new(1));
        assert_eq!(Test, DataUse::new(2));
        assert_eq!(Invalid, DataUse::new(3));
        assert_eq!(Invalid, DataUse::new(40));
    }

    #[test]
    fn coordinate_units_new() {
        use self::CoordinateUnits::*;
        assert_eq!(Unspecified, CoordinateUnits::new(0));
        assert_eq!(Length, CoordinateUnits::new(1));
        assert_eq!(SecondsOfArc, CoordinateUnits::new(2));
        assert_eq!(DegreesDecimal, CoordinateUnits::new(3));
        assert_eq!(DegreesMinutesSeconds, CoordinateUnits::new(4));
        assert_eq!(Invalid, CoordinateUnits::new(5));
        assert_eq!(Invalid, CoordinateUnits::new(255));
        assert_eq!(Invalid, CoordinateUnits::new(55));
    }

    #[test]
    fn correlated_new() {
        use self::Correlated::*;
        assert_eq!(Unspecified, Correlated::new(0));
        assert_eq!(No, Correlated::new(1));
        assert_eq!(Yes, Correlated::new(2));
        assert_eq!(Invalid, Correlated::new(9));
        assert_eq!(Invalid, Correlated::new(3));
        assert_eq!(Invalid, Correlated::new(29));
    }

    #[test]
    fn sweep_type_new() {
        use self::SweepType::*;
        assert_eq!(Unspecified, SweepType::new(0));
        assert_eq!(Linear, SweepType::new(1));
        assert_eq!(Parabolic, SweepType::new(2));
        assert_eq!(Exponential, SweepType::new(3));
        assert_eq!(Other, SweepType::new(4));
        assert_eq!(Invalid, SweepType::new(5));
        assert_eq!(Invalid, SweepType::new(55));
        assert_eq!(Invalid, SweepType::new(255));
    }

    #[test]
    fn overtravel_new() {
        use self::OverTravel::*;
        assert_eq!(Unspecified, OverTravel::new(0));
        assert_eq!(Up, OverTravel::new(1));
        assert_eq!(Down, OverTravel::new(2));
        assert_eq!(Invalid, OverTravel::new(3));
        assert_eq!(Invalid, OverTravel::new(33));
        assert_eq!(Invalid, OverTravel::new(233));
    }

    #[test]
    fn trace_value_unit_new() {
        use self::TraceValueUnit::*;
        assert_eq!(Other, TraceValueUnit::new(-1));
        assert_eq!(Unknown, TraceValueUnit::new(0));
        assert_eq!(Pascal, TraceValueUnit::new(1));
        assert_eq!(Volts, TraceValueUnit::new(2));
        assert_eq!(Millivolts, TraceValueUnit::new(3));
        assert_eq!(Amperes, TraceValueUnit::new(4));
        assert_eq!(Meters, TraceValueUnit::new(5));
        assert_eq!(MetersPerSecond, TraceValueUnit::new(6));
        assert_eq!(MetersPerSecond2, TraceValueUnit::new(7));
        assert_eq!(Newton, TraceValueUnit::new(8));
        assert_eq!(Watt, TraceValueUnit::new(9));
        assert_eq!(Invalid, TraceValueUnit::new(10));
        assert_eq!(Invalid, TraceValueUnit::new(100));
        assert_eq!(Invalid, TraceValueUnit::new(255));
    }

    #[test]
    fn transduction_unit_new() {
        use self::TransductionUnits::*;
        assert_eq!(Other, TransductionUnits::new(-1));
        assert_eq!(Unknown, TransductionUnits::new(0));
        assert_eq!(Pascal, TransductionUnits::new(1));
        assert_eq!(Volts, TransductionUnits::new(2));
        assert_eq!(Millivolts, TransductionUnits::new(3));
        assert_eq!(Amperes, TransductionUnits::new(4));
        assert_eq!(Meters, TransductionUnits::new(5));
        assert_eq!(MetersPerSecond, TransductionUnits::new(6));
        assert_eq!(MetersPerSecond2, TransductionUnits::new(7));
        assert_eq!(Newton, TransductionUnits::new(8));
        assert_eq!(Watt, TransductionUnits::new(9));
        assert_eq!(Invalid, TransductionUnits::new(10));
        assert_eq!(Invalid, TransductionUnits::new(100));
        assert_eq!(Invalid, TransductionUnits::new(255));
    }

    #[test]
    fn source_type_new() {
        use self::SourceType::*;
        assert_eq!(Unknown, SourceType::new(0));
        assert_eq!(VibratoryVertical, SourceType::new(1));
        assert_eq!(VibratoryCrossLine, SourceType::new(2));
        assert_eq!(VibratoryInLine, SourceType::new(3));
        assert_eq!(ImpulsiveVertical, SourceType::new(4));
        assert_eq!(ImpulsiveCrossLine, SourceType::new(5));
        assert_eq!(ImpulsiveInLine, SourceType::new(6));
        assert_eq!(DistributedImpulsiveVertical, SourceType::new(7));
        assert_eq!(DistributedImpulsiveCrossLine, SourceType::new(8));
        assert_eq!(DistributedImpulsiveInLine, SourceType::new(9));
        assert_eq!(Invalid, SourceType::new(10));
        assert_eq!(Invalid, SourceType::new(100));
        assert_eq!(Invalid, SourceType::new(-100));
    }

    #[test]
    fn scu_new() {
        use self::SourceMeasurementUnit::*;
        assert_eq!(Other, SourceMeasurementUnit::new(-1));
        assert_eq!(Unknown, SourceMeasurementUnit::new(0));
        assert_eq!(Joule, SourceMeasurementUnit::new(1));
        assert_eq!(KiloWatt, SourceMeasurementUnit::new(2));
        assert_eq!(Pascal, SourceMeasurementUnit::new(3));
        assert_eq!(Bar, SourceMeasurementUnit::new(4));
        assert_eq!(BarMeter, SourceMeasurementUnit::new(5));
        assert_eq!(Newton, SourceMeasurementUnit::new(6));
        assert_eq!(Kilograms, SourceMeasurementUnit::new(7));
        assert_eq!(Invalid, SourceMeasurementUnit::new(8));
        assert_eq!(Invalid, SourceMeasurementUnit::new(80));
        assert_eq!(Invalid, SourceMeasurementUnit::new(-8));
        assert_eq!(Invalid, SourceMeasurementUnit::new(80));
    }
}

mod bitconverter {
    use crate::bitconverter::*;

    #[test]
    fn test_ascii_bytes_to_string() {
        let name0 = vec![b'T', b'e', b'a', b' ', b'b', b'a', b'g'];
        let processed = ascii_bytes_to_string(&name0);
        assert_eq!(&processed, "Tea bag");
    }

    #[test]
    fn test_ascii_bytes_to_string1() {
        let name0 = vec![b'T', b'e', b'a', b'0', b'b', b'a', b'g'];
        let processed = ascii_bytes_to_string(&name0);
        assert_eq!(&processed, "Tea0bag");
    }

    #[test]
    fn test_ascii_bytes_to_string2() {
        let name0 = vec![b'T', b'e', b'a', 0, b'b', b'a', b'g'];
        let processed = ascii_bytes_to_string(&name0);
        assert_eq!(&processed, "Tea");
    }

    #[test]
    /// This tests whether the converter correctly converts bytes back to the correct number.
    fn test_converter_chooser() {
        use crate::enums::SampleFormatCode::*;

        for i in i8::MIN..i8::MAX {
            assert_eq!(
                converter_chooser(Int8, false).unwrap()(&(i as i8).to_be_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(Int8, true).unwrap()(&(i as i8).to_le_bytes()).unwrap(),
                i as f32
            );
            let i = i as u8;
            assert_eq!(
                converter_chooser(UInt8, false).unwrap()(&(i as u8).to_be_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(UInt8, true).unwrap()(&(i as u8).to_le_bytes()).unwrap(),
                i as f32
            );
        }

        for i in i16::MIN..i16::MAX {
            assert_eq!(
                converter_chooser(Int16, false).unwrap()(&(i as i16).to_be_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(Int16, true).unwrap()(&(i as i16).to_le_bytes()).unwrap(),
                i as f32
            );

            let i = i as u16;
            assert_eq!(
                converter_chooser(UInt16, false).unwrap()(&(i as u16).to_be_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(UInt16, true).unwrap()(&(i as u16).to_le_bytes()).unwrap(),
                i as f32
            );
        }

        for i in (i32::MIN..i32::MAX).step_by(10_000) {
            assert_eq!(
                converter_chooser(Int32, false).unwrap()(&i.to_be_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(Int64, false).unwrap()(&(i as i64).to_be_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(Float32, true).unwrap()(&(i as f32).to_le_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(Float64, true).unwrap()(&(i as f64).to_le_bytes()).unwrap(),
                i as f32
            );
            // Now for the LE side of things.
            assert_eq!(
                converter_chooser(Int32, true).unwrap()(&i.to_le_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(Int64, true).unwrap()(&(i as i64).to_le_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(Float32, true).unwrap()(&(i as f32).to_le_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(Float64, true).unwrap()(&(i as f64).to_le_bytes()).unwrap(),
                i as f32
            );

            let i = i as u32;
            assert_eq!(
                converter_chooser(UInt32, false).unwrap()(&(i as u32).to_be_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(UInt64, false).unwrap()(&(i as u64).to_be_bytes()).unwrap(),
                i as f32
            );
            // Now for the LE side of things
            assert_eq!(
                converter_chooser(UInt32, true).unwrap()(&(i as u32).to_le_bytes()).unwrap(),
                i as f32
            );
            assert_eq!(
                converter_chooser(UInt64, true).unwrap()(&(i as u64).to_le_bytes()).unwrap(),
                i as f32
            );
        }
    }
}

mod settings {
    use crate::enums::*;
    use crate::settings::*;
    use crate::TRACE_HEADER_LEN;
    use crate::{CDPX_BYTE_LOCATION, CDPY_BYTE_LOCATION};
    use crate::{CROSSLINE_BYTE_LOCATION, INLINE_BYTE_LOCATION};

    macro_rules! test_set_get {
        ($setter:ident, $getter:ident, $field:ident, $in_val:expr, $out_val:expr) => {
            let mut default = SegySettings::default();
            default.$setter($in_val.clone());
            assert_eq!(default.$field, $out_val);
            assert_eq!(default.$getter(), $out_val);
        };
    }

    #[test]
    fn test_default() {
        let expected = SegySettings {
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
        };
        assert_eq!(SegySettings::default(), expected);
    }

    #[test]
    fn test_order_trace_by() {
        test_set_get!(
            set_order_trace_by,
            get_order_trace_by,
            order_trace_by,
            OrderTraceBy::TraceNo,
            OrderTraceBy::TraceNo
        );
    }

    #[test]
    fn test_override_to_le1() {
        test_set_get!(
            set_override_to_le,
            get_override_to_le,
            override_to_le,
            true,
            Some(true)
        );
    }

    #[test]
    fn test_override_to_le2() {
        test_set_get!(
            set_override_to_le,
            get_override_to_le,
            override_to_le,
            false,
            Some(false)
        );
    }

    #[test]
    fn test_override_sample_format() {
        test_set_get!(
            set_override_trace_format,
            get_override_trace_format,
            override_trace_format,
            SampleFormatCode::UInt32,
            Some(SampleFormatCode::UInt32)
        );
    }

    #[test]
    fn test_override_trace_id_code() {
        test_set_get!(
            set_override_trace_id_code,
            get_override_trace_id_code,
            override_trace_id_code,
            TraceIdCode::VibratorBaseplate,
            Some(TraceIdCode::VibratorBaseplate)
        );
    }

    #[test]
    fn test_override_trace_depth_units() {
        test_set_get!(
            set_override_trace_depth_units,
            get_override_trace_depth_units,
            override_trace_depth_units,
            MeasurementSystem::Meters,
            Some(MeasurementSystem::Meters)
        );
    }

    #[test]
    fn test_override_coordinate_units() {
        test_set_get!(
            set_override_coordinate_units,
            get_override_coordinate_units,
            override_coordinate_units,
            MeasurementSystem::Meters,
            Some(MeasurementSystem::Meters)
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_override_coordinate_format() {
        test_set_get!(
            set_override_coordinate_format,
            get_override_coordinate_format,
            override_coordinate_format,
            SampleFormatCode::IbmFloat32,
            Some(SampleFormatCode::IbmFloat32)
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_set_override_coordinate_scaling() {
        let mut default = SegySettings::default();
        default.set_override_coordinate_scaling(42.).expect("Valid");
        assert!(default.set_override_coordinate_scaling(42000.).is_err());
        assert_eq!(default.override_coordinate_scaling, Some(42));
        assert_eq!(default.get_override_coordinate_scaling(), Some(42.));
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_inline_no_bidx() {
        test_set_get!(
            set_inline_no_bidx,
            get_inline_no_bidx,
            inline_no_bidx,
            34,
            34
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_crossline_no_bidx() {
        test_set_get!(
            set_crossline_no_bidx,
            get_crossline_no_bidx,
            crossline_no_bidx,
            34,
            34
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_y_ensemble_bidx() {
        test_set_get!(
            set_y_ensemble_bidx,
            get_y_ensemble_bidx,
            y_ensemble_bidx,
            34,
            34
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_x_ensemble_bidx() {
        test_set_get!(
            set_x_ensemble_bidx,
            get_x_ensemble_bidx,
            x_ensemble_bidx,
            34,
            34
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_inline_no_bidx_fail() {
        let mut default = SegySettings::default();
        assert!(default.set_inline_no_bidx(TRACE_HEADER_LEN - 3).is_err());
        assert!(default.set_inline_no_bidx(TRACE_HEADER_LEN + 3).is_err());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_crossline_no_bidx_fail() {
        let mut default = SegySettings::default();
        assert!(default.set_crossline_no_bidx(TRACE_HEADER_LEN - 3).is_err());
        assert!(default.set_crossline_no_bidx(TRACE_HEADER_LEN + 3).is_err());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_y_ensemble_bidx_fail() {
        let mut default = SegySettings::default();
        assert!(default.set_y_ensemble_bidx(TRACE_HEADER_LEN - 3).is_err());
        assert!(default.set_y_ensemble_bidx(TRACE_HEADER_LEN + 3).is_err());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_x_ensemble_bidx_fail() {
        let mut default = SegySettings::default();
        assert!(default.set_x_ensemble_bidx(TRACE_HEADER_LEN - 3).is_err());
        assert!(default.set_x_ensemble_bidx(TRACE_HEADER_LEN + 3).is_err());
    }

    #[test]
    fn test_step_by() {
        test_set_get!(set_step_by, get_step_by, step_by, 34, 34);
    }

    #[test]
    fn test_inlne_min_max() {
        test_set_get!(
            set_inlne_min_max,
            get_inlne_min_max,
            inline_min_max,
            [-100, 1000],
            Some([-100, 1000])
        );
    }

    #[test]
    fn test_crossline_min_max() {
        test_set_get!(
            set_crossline_min_max,
            get_crossline_min_max,
            crossline_min_max,
            [-100, 1000],
            Some([-100, 1000])
        );
    }

    #[test]
    fn test_origin() {
        test_set_get!(
            set_origin,
            get_origin,
            origin,
            [999., -999., 0.1234],
            Some([999., -999., 0.1234])
        );
    }

    #[test]
    fn test_override_dim_x() {
        let mut default = SegySettings::default();
        default.set_override_dim_x(44).expect("Valid");
        assert_eq!(default.override_dim_x, Some(44));
        assert_eq!(default.get_override_dim_x(), Some(44));
        assert_eq!(default.crossline_min_max, Some([0, 43]))
    }

    #[test]
    fn test_override_dim_y() {
        let mut default = SegySettings::default();
        default.set_override_dim_y(44).expect("Valid");
        assert_eq!(default.override_dim_y, Some(44));
        assert_eq!(default.get_override_dim_y(), Some(44));
        assert_eq!(default.inline_min_max, Some([0, 43]))
    }

    #[test]
    fn test_override_dim_z() {
        let mut default = SegySettings::default();
        default.set_override_dim_z(44).expect("Valid");
        assert_eq!(default.override_dim_z, Some(44));
        assert_eq!(default.get_override_dim_z(), Some(44));
        assert_eq!(default.inline_min_max, None)
    }

    #[test]
    fn test_override_dim_x_fail() {
        let mut default = SegySettings::default();
        assert!(default.set_override_dim_x(-44).is_err());
    }

    #[test]
    fn test_override_dim_y_fail() {
        let mut default = SegySettings::default();
        assert!(default.set_override_dim_y(-44).is_err());
    }

    #[test]
    fn test_override_dim_z_fail() {
        let mut default = SegySettings::default();
        assert!(default.set_override_dim_z(-44).is_err());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_override_u() {
        test_set_get!(
            set_override_u,
            get_override_u,
            override_u,
            [340., 430., -999.],
            Some([340., 430., -999.])
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_override_v() {
        test_set_get!(
            set_override_u,
            get_override_u,
            override_u,
            [340., 430., -999.],
            Some([340., 430., -999.])
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_override_sample_interval() {
        test_set_get!(
            set_override_sample_interval,
            get_override_sample_interval,
            override_sample_interval,
            -999.,
            Some(-999.)
        );
    }
}
