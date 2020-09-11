use std::ops::*;
use crate::{FlexIndex, FlexData, FlexDataType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexDataVector {
    index: FlexIndex,
    labels: Vec<String>,
    datatypes: Vec<FlexDataType>,
    data: Vec<FlexData>
}

impl FlexDataVector {

    pub fn new(index: FlexIndex, labels: Vec<&str>, datatypes: Vec<FlexDataType>, data: Vec<FlexData>) -> Self {
        Self {
            index: index,
            labels: labels.into_iter().map(|lbl| lbl.to_string()).collect(),
            datatypes: datatypes,
            data: data
        }
    }

    pub fn get_index(&self) -> &FlexIndex {
        &self.index
    }

    pub fn set_index(&mut self, index: FlexIndex) {
        self.index = index;
    }

    pub fn get(&self, label: &str) -> Option<&FlexData> {
        self.labels.iter()
            .position(|lbl| lbl == label)
            .map(|i| &self.data[i])
    }

    pub fn set(&mut self, label:&str, data: FlexData) {
        if let Some(i) = self.labels.iter().position(|lbl| lbl == label) {
            self.data[i] = data;
        }
    }

    pub fn get_labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn get_datatypes(&self) -> &Vec<FlexDataType> {
        &self.datatypes
    }

    pub fn get_size(&self) -> usize {
        self.labels.len()
    }

    pub fn verify(&self, f: impl Fn(&FlexData) -> bool) -> bool {
        self.data.iter()
            .all(|fd| f(fd))
    }

    pub fn has_na(&self) -> bool {
        !self.verify(|x: &FlexData| x != &FlexData::NA)
    }

    pub fn contains(&self, label: &str) -> bool {
        self.labels.iter()
            .position(|lbl| lbl == label)
            .is_some()
    }

}

// Implement [] operator
impl Index<usize> for FlexDataVector {
    type Output = FlexData;
    fn index<'a>(&'a self, index: usize) -> &'a FlexData {
        &self.data[index]
    }
}

impl PartialEq for FlexDataVector {
    fn eq(&self, other: &FlexDataVector) -> bool {
        self.index == other.index && self.data.iter().zip( other.data.iter() ).all(|(a,b)| a == b)
    }
}

impl Eq for FlexDataVector{}

impl std::fmt::Display for FlexDataVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output;
        output = format!("{:>width$}", " ", width=5);
        for header in self.labels.iter() {
            output = format!("{}{:>width$}", output, header, width=14);
        }
        output.push_str("\n");
        output = format!("{}{:>width$}", output, " ", width=5);
        for datatype in self.datatypes.iter() {
            match datatype {
                FlexDataType::Dbl => { output = format!("{}{:>width$}", output, "f64", width=14) },
                FlexDataType::Uint => { output = format!("{}{:>width$}", output, "u32", width=14) },
                FlexDataType::Int => { output = format!("{}{:>width$}", output, "i64", width=14) },
                FlexDataType::Char => { output = format!("{}{:>width$}", output, "char", width=14) },
                FlexDataType::Str => { output = format!("{}{:>width$}", output, "str", width=14) },
                FlexDataType::NA => { output = format!("{}{:>width$}", output, "n/a", width=14) }
            }
        }
        output.push_str("\n");
        for i in 0..self.data.len() {
            if i == 0 {
                match &self.index {
                    FlexIndex::Uint(val) => { output = format!("{}{:>width$}", output, val, width=5); },
                    FlexIndex::Str(val) => { output = format!("{}{:>width$}", output, val, width=5); }
                }
            }
            match &self.data[i] {
                FlexData::Str(val) => {
                    if val.len() >= 12 {
                        output = format!("{}{:>width$}", output, format!("{}..", &val[..10]), width=14);
                    } else {
                        output = format!("{}{:>width$}", output, val, width=14);
                    }
                },
                FlexData::Dbl(val) => { output = format!("{}{:>width$}", output, val, width=14); },
                FlexData::Uint(val) => { output = format!("{}{:>width$}", output, val, width=14); },
                FlexData::Int(val) => { output = format!("{}{:>width$}", output, val, width=14); },
                FlexData::Char(val) => { output = format!("{}{:>width$}", output, val, width=14); }
                FlexData::NA => { output = format!("{}{:>width$}", output, "N/A", width=14); }
            }
        }
        output.push_str("\n");
        write!(f, "{}", output)
    }
}