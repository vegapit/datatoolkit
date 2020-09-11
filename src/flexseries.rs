use crate::{FlexDataType, FlexDataPoint, FlexData, FlexIndex};
use std::ops::*;

pub struct FlexSeries {
    label: String,
    datatype: FlexDataType,
    data: Vec<FlexDataPoint>
}

impl FlexSeries {

    pub fn new(label: &str, datatype: FlexDataType) -> Self {
        Self {
            label: label.to_string(),
            datatype: datatype,
            data: Vec::new()
        }
    }

    pub fn from_vec(label: &str, datatype: FlexDataType, data: Vec<FlexDataPoint>) -> Self {
        Self {
            label: label.to_string(),
            datatype: datatype,
            data: data
        }
    }

    // Getters and setters

    pub fn get_label(&self) -> &str {
        self.label.as_str()
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    pub fn get_datatype(&self) -> &FlexDataType {
        &self.datatype
    }

    pub fn set_datatype(&mut self, datatype: FlexDataType) {
        self.datatype = datatype;
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn at(&self, index: &FlexIndex) -> Option<&FlexDataPoint> {
        self.data.iter()
            .position(|fdp| fdp.get_index() == index)
            .map(|i| &self.data[i] )
    }

    // Data operations

    pub fn update(&mut self, data: FlexDataPoint) {
        if let Some(i) = self.data.iter().position(|fdp| fdp.get_index() == data.get_index()) {
            self.data[i] = data;
        }
    }

    pub fn insert_update(&mut self, item: FlexDataPoint) {
        if let Some(k) = self.data.iter().position(|x| x.get_index() == item.get_index() ) {
            self.data[k] = item;
        } else {
            self.data.push(item);
            self.data.sort();
        }
    }

    pub fn remove(&mut self, k: usize) {
        self.data.remove(k);
    }

    pub fn remove_at(&mut self, index: &FlexIndex) {
        if let Some(i) = self.data.iter().position(|fdp| fdp.get_index() == index) {
            self.data.remove(i);
        }
    }

    // Transformation

    pub fn apply(&self, f: impl Fn(&FlexData) -> FlexData) -> Self {
        let data : Vec<FlexDataPoint> = self.data.iter()
            .map(|fdp| fdp.apply(&f))
            .collect();
        Self::from_vec(self.label.as_str(), self.datatype.clone(), data)
    }

    // Filtering

    pub fn filter(&self, f: impl Fn(&FlexDataPoint) -> bool) -> Self {
        let data : Vec<FlexDataPoint> = self.data.iter()
            .filter(|d| f(d))
            .cloned()
            .collect();
        Self::from_vec(self.label.as_str(), self.datatype.clone(), data)
    }

    // NA management

    pub fn has_na(&self) -> bool {
        self.data.iter()
            .any(|fdp| fdp.get() == &FlexData::NA )
    }

    pub fn get_na(&self) -> FlexSeries {
        self.filter(|x: &FlexDataPoint| x.get() == &FlexData::NA)
    }

    pub fn drop_na(&self) -> FlexSeries {
        self.filter(|x: &FlexDataPoint| x.get() != &FlexData::NA)
    }

}

// Implement [] operator
impl Index<i32> for FlexSeries {
    type Output = FlexDataPoint;
    fn index<'a>(&'a self, index: i32) -> &'a FlexDataPoint {
        if index >= 0 {
            &self.data[index as usize]
        } else {
            &self.data[self.data.len() - (-index as usize)]
        }
    }
}

impl Index<usize> for FlexSeries {
    type Output = FlexDataPoint;
    fn index<'a>(&'a self, index: usize) -> &'a FlexDataPoint {
        &self.data[index as usize]
    }
}

// Console display
impl std::fmt::Display for FlexSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output;
        output = format!("{:>width$}", " ", width=5);
        output = format!("{}{:>width$}", output, self.label, width=14);
        output.push_str("\n");
        output = format!("{}{:>width$}", output, " ", width=5);
        match self.datatype {
            FlexDataType::Dbl => { output = format!("{}{:>width$}", output, "f64", width=14) },
            FlexDataType::Uint => { output = format!("{}{:>width$}", output, "u32", width=14) },
            FlexDataType::Int => { output = format!("{}{:>width$}", output, "i64", width=14) },
            FlexDataType::Char => { output = format!("{}{:>width$}", output, "char", width=14) },
            FlexDataType::Str => { output = format!("{}{:>width$}", output, "str", width=14) },
            FlexDataType::NA => { output = format!("{}{:>width$}", output, "n/a", width=14) }
        }
        output.push_str("\n");
        for i in 0..self.get_size() {
            match self.data[i].get_index() {
                FlexIndex::Uint(val) => { output = format!("{}{:>width$}", output, val, width=5); },
                FlexIndex::Str(val) => { output = format!("{}{:>width$}", output, val, width=5); }
            }
            match self.data[i].get() {
                FlexData::Str(val) => {
                    if val.len() >= 12 {
                        output = format!("{}{:>width$}", output, format!("{}..", &val[..10]), width=14);
                    } else {
                        output = format!("{}{:>width$}", output, val, width=14);
                    }
                },
                FlexData::Dbl(val) => { output = format!("{}{:>width$.5}", output, val, width=14); },
                FlexData::Uint(val) => { output = format!("{}{:>width$}", output, val, width=14); },
                FlexData::Int(val) => { output = format!("{}{:>width$}", output, val, width=14); },
                FlexData::Char(val) => { output = format!("{}{:>width$}", output, val, width=14); }
                FlexData::NA => { output = format!("{}{:>width$}", output, "N/A", width=14); }
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}