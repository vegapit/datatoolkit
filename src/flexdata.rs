use std::ops::*;
use std::convert::TryFrom;
use std::iter::Sum;
use crate::helper::derive_datatype;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum FlexDataType {
    Str,
    Uint,
    Int,
    Dbl,
    Char,
    NA
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
pub enum FlexData {
    Str(String),
    Uint(u32),
    Int(i32),
    Dbl(f64),
    Char(char),
    NA
}

impl From<String> for FlexData {
    fn from(value: String) -> FlexData {
        FlexData::Str(value)
    }
}

impl From<u32> for FlexData {
    fn from(value: u32) -> FlexData {
        FlexData::Uint(value)
    }
}

impl From<i32> for FlexData {
    fn from(value: i32) -> FlexData {
        FlexData::Int(value)
    }
}

impl From<f64> for FlexData {
    fn from(value: f64) -> FlexData {
        FlexData::Dbl(value)
    }
}

impl From<char> for FlexData {
    fn from(value: char) -> FlexData {
        FlexData::Char(value)
    }
}

// Into Implementation 

impl TryFrom<&FlexData> for String {
    type Error = &'static str;
    fn try_from(value: &FlexData) -> Result<Self, Self::Error> {
        match value {
            FlexData::Str(v) => Ok(v.to_string()),
            _ => Err("Only FlexData::Str can be extracted to String")
        }
    }
}

impl TryFrom<&FlexData> for f64 {
    type Error = &'static str;
    fn try_from(value: &FlexData) -> Result<Self, Self::Error> {
        match value {
            FlexData::Dbl(v) => Ok(*v),
            _ => Err("Only FlexData::Dbl can be extracted to f64")
        }
    }
}

impl TryFrom<&FlexData> for u32 {
    type Error = &'static str;
    fn try_from(value: &FlexData) -> Result<Self, Self::Error> {
        match value {
            FlexData::Uint(v) => Ok(*v),
            _ => Err("Only FlexData::Uint can be extracted to u32")
        }
    }
}

impl TryFrom<&FlexData> for i32 {
    type Error = &'static str;
    fn try_from(value: &FlexData) -> Result<Self, Self::Error> {
        match value {
            FlexData::Int(v) => Ok(*v),
            _ => Err("Only FlexData::Int can be extracted to i32")
        }
    }
}

// Operators

impl Add for &FlexData {
    type Output = FlexData;
    fn add(self, other: &FlexData) -> Self::Output {
        match self {
            FlexData::Dbl(val) => {
                match other {
                    FlexData::Dbl(other_val) => FlexData::Dbl(val + other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Uint(val) => {
                match other {
                    FlexData::Uint(other_val) => FlexData::Uint(val + other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Int(val) => {
                match other {
                    FlexData::Int(other_val) => FlexData::Int(val + other_val),
                    _ => FlexData::NA
                }
            },
            _ => FlexData::NA
        }
    }
}

impl Sub for &FlexData {
    type Output = FlexData;
    fn sub(self, other: &FlexData) -> Self::Output {
        match self {
            FlexData::Dbl(val) => {
                match other {
                    FlexData::Dbl(other_val) => FlexData::Dbl(val - other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Uint(val) => {
                match other {
                    FlexData::Uint(other_val) => FlexData::Uint(val - other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Int(val) => {
                match other {
                    FlexData::Int(other_val) => FlexData::Int(val - other_val),
                    _ => FlexData::NA
                }
            },
            _ => FlexData::NA
        }
    }
}

impl Mul for &FlexData {
    type Output = FlexData;
    fn mul(self, other: &FlexData) -> Self::Output {
        match self {
            FlexData::Dbl(val) => {
                match other {
                    FlexData::Dbl(other_val) => FlexData::Dbl(val * other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Uint(val) => {
                match other {
                    FlexData::Uint(other_val) => FlexData::Uint(val * other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Int(val) => {
                match other {
                    FlexData::Int(other_val) => FlexData::Int(val * other_val),
                    _ => FlexData::NA
                }
            },
            _ => FlexData::NA
        }
    }
}

impl Div for &FlexData {
    type Output = FlexData;
    fn div(self, other: &FlexData) -> Self::Output {
        match self {
            FlexData::Dbl(val) => {
                match other {
                    FlexData::Dbl(other_val) => {
                        if other_val != &0f64 {
                            FlexData::Dbl(val / other_val)
                        } else {
                            FlexData::NA
                        }
                    },
                    _ => FlexData::NA
                }
            },
            FlexData::Int(val) => {
                match other {
                    FlexData::Int(other_val) => {
                        if other_val != &0 {
                            FlexData::Int(val / other_val)
                        } else {
                            FlexData::NA
                        }
                    },
                    _ => FlexData::NA
                }
            },
            FlexData::Uint(val) => {
                match other {
                    FlexData::Uint(other_val) => {
                        if other_val != &0 {
                            FlexData::Uint(val / other_val)
                        } else {
                            FlexData::NA
                        }
                    },
                    _ => FlexData::NA
                }
            },
            _ => FlexData::NA
        }
    }
}

impl AddAssign for FlexData {
    fn add_assign(&mut self, other: FlexData) {
        *self = match self {
            FlexData::Dbl(val) => {
                match other {
                    FlexData::Dbl(other_val) => FlexData::Dbl(*val + other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Uint(val) => {
                match other {
                    FlexData::Uint(other_val) => FlexData::Uint(*val + other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Int(val) => {
                match other {
                    FlexData::Int(other_val) => FlexData::Int(*val + other_val),
                    _ => FlexData::NA
                }
            },
            _ => FlexData::NA
        }
    }
}

impl SubAssign for FlexData {
    fn sub_assign(&mut self, other: FlexData) {
        *self = match self {
            FlexData::Dbl(val) => {
                match other {
                    FlexData::Dbl(other_val) => FlexData::Dbl(*val - other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Uint(val) => {
                match other {
                    FlexData::Uint(other_val) => FlexData::Uint(*val - other_val),
                    _ => FlexData::NA
                }
            },
            FlexData::Int(val) => {
                match other {
                    FlexData::Int(other_val) => FlexData::Int(*val - other_val),
                    _ => FlexData::NA
                }
            },
            _ => FlexData::NA
        }
    }
}

impl Sum<FlexData> for FlexData {
    fn sum<I>(iter: I) -> FlexData 
        where I: Iterator<Item=FlexData> {
        let mut total = FlexData::NA;
        for d in iter {
            if derive_datatype( &total ) == FlexDataType::NA {
                total = d;
            } else {
                total += d;
            }
        }
        total
    }
}