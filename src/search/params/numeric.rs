/// Numeric enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Numeric {
    /// Represents the type of u8, u16, u32, u64
    U64(u64),
    /// Represents the type of i8, i16, i32, i64
    I64(i64),
    /// Represents the type of f32
    F32(f32),
    /// Represents the type of f64
    F64(f64),
}

impl From<u8> for Numeric {
    fn from(value: u8) -> Self {
        Self::U64(value as u64)
    }
}

impl From<u16> for Numeric {
    fn from(value: u16) -> Self {
        Self::U64(value as u64)
    }
}

impl From<u32> for Numeric {
    fn from(value: u32) -> Self {
        Self::U64(value as u64)
    }
}

impl From<u64> for Numeric {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<i8> for Numeric {
    fn from(value: i8) -> Self {
        Self::I64(value as i64)
    }
}

impl From<i16> for Numeric {
    fn from(value: i16) -> Self {
        Self::I64(value as i64)
    }
}

impl From<i32> for Numeric {
    fn from(value: i32) -> Self {
        Self::I64(value as i64)
    }
}

impl From<i64> for Numeric {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<f32> for Numeric {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for Numeric {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}
