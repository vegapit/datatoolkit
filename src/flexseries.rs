use crate::{FlexDataType, FlexDataPoint, FlexIndex};
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

    pub fn update(&mut self, data: FlexDataPoint) {
        if let Some(i) = self.data.iter().position(|fdp| fdp.get_index() == data.get_index()) {
            self.data[i] = data;
        }
    }

    // Data insertion
    pub fn insert_update(&mut self, item: FlexDataPoint) {
        if let Some(k) = self.data.iter().position(|x| x.get_index() == item.get_index() ) {
            self.data[k] = item;
        } else {
            self.data.push(item);
            self.data.sort();
        }
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