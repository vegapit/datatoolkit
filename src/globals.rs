use std::ops::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum FlexData {
    Str(String),
    Uint(u32),
    Int(i64),
    Dbl(f64),
    Char(char),
    NA
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FlexDataType {
    Str,
    Uint,
    Int,
    Dbl,
    Char,
    NA
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlexIndex {
    Str(String),
    Uint(usize)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FlexIndexType {
    Str,
    Uint
}

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