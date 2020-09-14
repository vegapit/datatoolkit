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
