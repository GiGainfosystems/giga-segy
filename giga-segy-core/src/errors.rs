use crate::enums::SampleFormatCode;

/// Basic Error types.
#[derive(Debug)]
pub enum RsgError {
    /// TryFromSliceError from the std library.
    TryFromSlice(std::array::TryFromSliceError),
    /// TryFromSliceError from the std library.
    TryFromUtf8(std::string::FromUtf8Error),
    /// IoError from the std library.
    StdIoError(std::io::Error),
    /// A wrapped memory map error.
    /// Binary header length problems.
    BinHeaderLength { l: usize },
    /// An error in the settings of a SEG-Y.
    SEGYSettingsError { msg: String },
    /// File is too short (even shorter.)
    FileTooShort,
    /// Coordinate format cannot make this float.
    FloatConversion {
        float: f32,
        format: SampleFormatCode,
    },
    /// SEG-Y is too short for a different reason..
    IncompleteTrace,
    /// Trace not found.
    TraceNotFound { i: usize },
    /// Trace point out of bounds.
    TracePointOutOfBounds { idx: usize },
    /// SEG-Y is too short.
    SEGYTooShort,
    /// Your SEG-Y is too short.
    ShortSEGY { a: usize, b: usize },
    /// Your SEG-Y does not fit (divisibility).
    TraceDivisibility {
        a: usize,
        b: usize,
        format: SampleFormatCode,
    },
    /// Trace header length problems.
    TraceHeaderLength { l: usize },
    /// Bit converter cannot fulfil the conversion.
    BitConversionError { msg: String },
    /// An error caused by an invalid header.
    InvalidHeader { msg: String },
    /// When the data vector length exceeds 65535 data points.
    LongDataVector { l_data: usize },
    /// Thrown when the data vector length does not match that declared in headers.
    BadDataVector {
        l_data: u16,
        l_bin: u16,
        l_trace: u16,
    },
    /// Enum creation error.
    ParseEnum { f: String, code: u16 },
    /// Map file error (this is just a `std::io` error when mapping files).
    MapFile(std::io::Error),
    #[cfg(feature = "to_json")]
    /// Serialisation/Deserialisation error.
    SerdeError(serde_json::Error),
}

impl From<std::array::TryFromSliceError> for RsgError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        Self::TryFromSlice(e)
    }
}

impl From<std::io::Error> for RsgError {
    fn from(e: std::io::Error) -> Self {
        Self::StdIoError(e)
    }
}

impl std::fmt::Display for RsgError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::RsgError::*;
        match self {
            #[cfg(feature = "to_json")]
            SerdeError(x) => write!(fmt, "{}", x),
            StdIoError(x) => write!(fmt, "{}", x),
            TryFromSlice(x) => write!(fmt, "{}", x),
            TryFromUtf8(x) => write!(fmt, "{}", x),
            BinHeaderLength { l } => write!(fmt, "Binary header length should be 400 but is {}", l),
            SEGYSettingsError { msg } => write!(fmt, "Error in settings: {}", msg),
            FileTooShort => write!(fmt, "File is too short to be SEG-Y"),
            FloatConversion { float, format } => write!(fmt, "Could not convert {} to {}.", float, format),
            IncompleteTrace => write!(fmt, "Last trace incomplete: File may be corrupt."),
            TraceNotFound { i } => write!(fmt, "Trace  no. {} not found.", i),
            TracePointOutOfBounds { idx } => write!(fmt, "Error getting trace: Idx ({}) trace point is out of bounds.", idx),
            SEGYTooShort => write!(fmt, "Mapped file is too short to be a SEG-Y file, or too many Extended Text Headers are counted"),
            ShortSEGY { a, b } => write!(fmt, "Error getting trace: SEG-Y Mapping is too short (is {}-bytes, needs to be {}-bytes)", a, b),
            TraceDivisibility { a, b, format } => write!(fmt, "Error getting trace: data binary length ({}) not divisible by datum length ({}-bit ({}))", a, b, format),
            TraceHeaderLength { l } => write!(fmt, "Trace header length should be 240 but is {}", l),
            BitConversionError { msg } => write!(fmt, "Bit conversion failed: {}", msg),
            InvalidHeader { msg } => write!(fmt, "Invalid header: {}", msg),
            LongDataVector { l_data } => write!(fmt, "Data vector has {} points, but max length is 65535.", l_data),
            BadDataVector { l_data, l_bin, l_trace } => write!(fmt, "Data length is {}, but was declared as {} (binary header) or {} (trace header).", l_data, l_bin, l_trace),
            ParseEnum { f, code } => write!(fmt, "Could not parse source ({}) to {}.", code, f),
            MapFile(e) => write!(fmt, "Could not create file map: {}", e),
        }
    }
}

impl From<RsgError> for String {
    fn from(e: RsgError) -> String {
        e.to_string()
    }
}

impl std::error::Error for RsgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::RsgError::*;
        match self {
            #[cfg(feature = "to_json")]
            SerdeError(x) => x.source(),
            StdIoError(x) | MapFile(x) => x.source(),
            TryFromSlice(x) => x.source(),
            TryFromUtf8(x) => x.source(),
            _ => None,
        }
    }
}
