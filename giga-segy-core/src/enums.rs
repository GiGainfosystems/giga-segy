//! This contains all the enums that are used in the trace and binary headers.
//!
//! The SEG-Y format uses binary and trace headers (technically both headers are binary), which contain
//! general metadata and other data which convey how the trace data is to be interpreted. In a lot of cases
//! this information may only have certain values, which lends itself well to being represented by enums.
//! (More information on this can be found in the
//! [SEG-Y_r2.0 document](<https://seg.org/Portals/0/SEG/News%20and%20Resources/Technical%20Standards/seg_y_rev2_0-mar2017.pdf>)
//! (January 2017) tables 2 and 3).
//!
//! As a general rule, enums that are found only in the binary header have fixed numerical values
//! and return an error when an invalid value is found. Enums from the trace header can also
//! return an undefined `Invalid` variant. This is needed because custom byte indices can be set for
//! some values in the header, which would mean that the placement of others is unknown. Hence there
//! is a need to be able to return a "non value" without crashing (but also ideally without too many)
//! layers of complexity.
use num::FromPrimitive;
#[cfg(any(feature = "to_json", feature = "serde"))]
use serde::{Deserialize, Serialize};

use crate::errors::*;

/// Choose which of the header lines to count traces by.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum OrderTraceBy {
    Default = 1,
    TraceSequenceOnLine = 2,
    TraceSequenceInFile = 3,
    FieldRecordNo = 4,
    TraceNo = 5,
    TraceNoInEnsemble = 6,
}

/// From bytes 3225-3226  (25-26) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum SampleFormatCode {
    IbmFloat32 = 1,
    Int32 = 2,
    Int16 = 3,
    FixPoint32 = 4, //Obsolete.
    Float32 = 5,
    Float64 = 6,
    Int24 = 7,
    Int8 = 8,
    Int64 = 9,
    UInt32 = 10,
    UInt16 = 11,
    UInt64 = 12,
    UInt24 = 15,
    UInt8 = 16,
}

impl std::fmt::Display for SampleFormatCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl SampleFormatCode {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Result<Self, RsgError> {
        SampleFormatCode::from_u16(source).ok_or_else(|| RsgError::ParseEnum {
            f: "SampleFormatCode".to_string(),
            code: source,
        })
    }

    /// The byte length of a datum is important when guestimating the length of a trace.
    pub fn datum_byte_length(self) -> usize {
        match self {
            Self::IbmFloat32 => 4,
            Self::Int32 => 4,
            Self::Int16 => 2,
            Self::FixPoint32 => 4, //Obsolete.
            Self::Float32 => 4,
            Self::Float64 => 8,
            Self::Int24 => 3,
            Self::Int8 => 1,
            Self::Int64 => 8,
            Self::UInt32 => 4,
            Self::UInt16 => 2,
            Self::UInt64 => 8,
            Self::UInt24 => 3,
            Self::UInt8 => 1,
        }
    }
}

/// From bytes 3229-3230 (29-30) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum TraceSortingCode {
    Other = -1,
    Unknown = 0,
    AsRec = 1,
    CDPEnsemble = 2,
    SingleFoldContinuous = 3,
    HorizontalStack = 4,
    CommonSourcePoint = 5,
    CommonReceiverPoint = 6,
    CommonOffsetPoint = 7,
    CommonMidPoint = 8,
    CommonConversionPoint = 9,
    Invalid,
}

impl TraceSortingCode {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: i16) -> Self {
        Self::from_i16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 3239-3240 (39-40) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum SweepTypeCode {
    Unspecified = 0,
    Linear = 1,
    Parabolic = 2,
    Exponential = 3,
    Other = 4,
    Invalid,
}

impl SweepTypeCode {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 3247-3248 (47-48) of the binary header.
/// Also in bytes 139-140 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum TaperType {
    Unspecified = 0,
    Linear = 1,
    Cosine2 = 2,
    Other = 3,
    Invalid,
}

impl TaperType {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// 3249-3250 (49-50) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum CorrelatedDataTraces {
    Unspecified = 0,
    No = 1,
    Yes = 2,
    Invalid,
}

impl CorrelatedDataTraces {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 3251-3252 (51-52) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum BinaryGainRecovered {
    Unspecified = 0,
    Yes = 1,
    No = 2,
    Invalid,
}

impl BinaryGainRecovered {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 3253-3254 (53-54) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum AmplitudeRecoveryMethod {
    Unspecified = 0,
    None = 1,
    SphericalDivergence = 2,
    Agc = 3,
    Other = 4,
    Invalid,
}

impl AmplitudeRecoveryMethod {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 3255-3256 (55-56) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum MeasurementSystem {
    Unspecified = 0,
    Meters = 1,
    Feet = 2,
    Invalid,
}

impl MeasurementSystem {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 3257-3258 (57-58) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum ImpulseSignalPolarity {
    Unspecified = 0,
    IncreasePressureMinus = 1,
    IncreasePressurePlus = 2,
    Invalid,
}

impl ImpulseSignalPolarity {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 3259-3260 (59-60) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum VibratoryPolarityCode {
    Unspecified = 0,
    From338 = 1,
    From23 = 2,
    From68 = 3,
    From113 = 4,
    From158 = 5,
    From203 = 6,
    From248 = 7,
    From293 = 8,
    Invalid,
}

impl VibratoryPolarityCode {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 3503-3504 (303-304) of the binary header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum FixedLengthTraces {
    Yes = 1,
    No = 0,
}

impl FixedLengthTraces {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Result<Self, RsgError> {
        Self::from_u16(source).ok_or_else(|| RsgError::ParseEnum {
            f: "FixedLengthTraces".to_string(),
            code: source,
        })
    }

    /// Convert to bool.
    pub fn yes(self) -> bool {
        self == Self::Yes
    }
    /// Convert to bool
    pub fn no(self) -> bool {
        self == Self::No
    }
}

/// From bytes 3511-3512 (311-312) of the binary header.
/// Alternatively bytes 167-168 of a standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum TimeBasisCode {
    Unspecified = 0,
    Local = 1,
    GreenwichGMT = 2,
    Other = 3,
    CoordinatedUTC = 4,
    GlobalGPS = 5,
    Invalid,
}

impl TimeBasisCode {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 29-30 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum TraceIdCode {
    Other = -1,
    Unknown = 0,
    TimeDomainSeismic = 1,
    Dead = 2,
    Dummy = 3,
    TimeBreak = 4,
    Uphole = 5,
    Sweep = 6,
    Timing = 7,
    Waterbreak = 8,
    NearFieldGunSig = 9,
    FarFieldGunSig = 10,
    SeismicPressureSensor = 11,
    MulticomponentVertical = 12,
    MulticomponentCrossLine = 13,
    MulticomponentInLine = 14,
    RotatedVertical = 15,
    RotatedTransverse = 16,
    RotatedRadial = 17,
    VibratorReactionMass = 18,
    VibratorBaseplate = 19,
    VibratorEstimatedGroundForce = 20,
    VibratorReference = 21,
    TimeVelocityPairs = 22,
    TimeDepthPairs = 23,
    DepthVelocityPairs = 24,
    DepthDomainSeismic = 25,
    GravityPotential = 26,
    EFVertical = 27,
    EFCrossLine = 28,
    EFInLine = 29,
    RotatedEFVertical = 30,
    RotatedEFTransverse = 31,
    RotatedEFRadial = 32,
    MFVertical = 33,
    MFCrossLine = 34,
    MFInLine = 35,
    RotatedMFVertical = 36,
    RotatedMFTransverse = 37,
    RotatedMFRadial = 38,
    RotatedSensorPitch = 39,
    RotatedSensorRoll = 40,
    RotatedSensorYaw = 41,
    Invalid,
}

impl TraceIdCode {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: i16) -> Self {
        Self::from_i16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 35-36 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum DataUse {
    Unspecified = 0,
    Production = 1,
    Test = 2,
    Invalid,
}

impl DataUse {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 89-90 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum CoordinateUnits {
    Unspecified = 0,
    Length = 1,
    SecondsOfArc = 2,
    DegreesDecimal = 3,
    DegreesMinutesSeconds = 4,
    Invalid,
}

impl CoordinateUnits {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

///From bytes 119-120 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum GainType {
    Unspecified = 0,
    Fixed = 1,
    Binary = 2,
    FloatingPoint = 3,
    Invalid,
}

impl GainType {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

// From bytes 125-126 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum Correlated {
    Unspecified = 0,
    No = 1,
    Yes = 2,
    Invalid,
}

impl Correlated {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// From bytes 133-134 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum SweepType {
    Unspecified = 0,
    Linear = 1,
    Parabolic = 2,
    Exponential = 3,
    Other = 4,
    Invalid,
}

impl SweepType {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

// Taper type is also found in bytes 139-140 of the standard trace header.

// Time Basis code is covered above. (but is found in bytes 157-158 of the STH)

/// Found in bytes 179-180 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum OverTravel {
    Unspecified = 0,
    Up = 1,
    Down = 2,
    Invalid,
}

impl OverTravel {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: u16) -> Self {
        Self::from_u16(source).unwrap_or(Self::Invalid)
    }
}

/// Found in bytes 203-204 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum TraceValueUnit {
    Other = -1,
    Unknown = 0,
    Pascal = 1,
    Volts = 2,
    Millivolts = 3,
    Amperes = 4,
    Meters = 5,
    MetersPerSecond = 6,
    MetersPerSecond2 = 7,
    Newton = 8,
    Watt = 9,
    Invalid,
}

impl TraceValueUnit {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: i16) -> Self {
        Self::from_i16(source).unwrap_or(Self::Invalid)
    }
}

/// Found in bytes 211-212 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum TransductionUnits {
    Other = -1,
    Unknown = 0,
    Pascal = 1,
    Volts = 2,
    Millivolts = 3,
    Amperes = 4,
    Meters = 5,
    MetersPerSecond = 6,
    MetersPerSecond2 = 7,
    Newton = 8,
    Watt = 9,
    Invalid,
}

impl TransductionUnits {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: i16) -> Self {
        Self::from_i16(source).unwrap_or(Self::Invalid)
    }
}

/// Found in bytes 217-218 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum SourceType {
    Unknown = 0,
    VibratoryVertical = 1,
    VibratoryCrossLine = 2,
    VibratoryInLine = 3,
    ImpulsiveVertical = 4,
    ImpulsiveCrossLine = 5,
    ImpulsiveInLine = 6,
    DistributedImpulsiveVertical = 7,
    DistributedImpulsiveCrossLine = 8,
    DistributedImpulsiveInLine = 9,
    Invalid,
}

impl SourceType {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: i16) -> Self {
        Self::from_i16(source).unwrap_or(Self::Invalid)
    }
}

/// Found in bytes 231-232 of the standard trace header.
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
#[cfg_attr(
    any(feature = "to_json", feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum SourceMeasurementUnit {
    Other = -1,
    Unknown = 0,
    Joule = 1,
    KiloWatt = 2,
    Pascal = 3,
    Bar = 4,
    BarMeter = 5,
    Newton = 6,
    Kilograms = 7,
    Invalid,
}

impl SourceMeasurementUnit {
    /// NB: We give a result here to make life simpler for ourselves down the line.
    pub fn new(source: i16) -> Self {
        Self::from_i16(source).unwrap_or(Self::Invalid)
    }
}
