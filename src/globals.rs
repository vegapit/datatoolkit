#[derive(Debug, Serialize, Deserialize, Clone)]
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