use crate::{FlexDataType, FlexDataPoint, FlexData, FlexIndex};
use crate::helper::convert;
use std::ops::*;
use prettytable::{Table, Row, Cell};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn get_indices(&self) -> Vec<&FlexIndex> {
        self.data.iter()
            .map(|fdp| fdp.get_index())
            .collect()
    }

    // Selecting

    pub fn at(&self, index: &FlexIndex) -> Option<&FlexDataPoint> {
        self.data.iter()
            .position(|fdp| fdp.get_index() == index)
            .map(|i| &self.data[i] )
    }

    pub fn contains(&self, index: &FlexIndex) -> bool {
        self.data.iter()
            .position(|fdp| fdp.get_index() == index)
            .is_some()
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

    pub fn as_type(&mut self, datatype: &FlexDataType) {
        for fdp in self.data.iter_mut() {
            fdp.as_type( datatype );
        }
    }

    // Transformation

    pub fn align_to(&self, indices: Vec<&FlexIndex>) -> Self {
        let mut series = self.clone();
        for index in indices.iter() {
            if !series.contains( index ) {
                series.insert_update( FlexDataPoint::new((*index).clone(), FlexData::NA) );
            }
        }
        series
    }

    pub fn apply(&mut self, f: impl Fn(&FlexData) -> FlexData) {
        for fdp in self.data.iter_mut() {
            fdp.apply(&f);
        }
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

    // pretty print
    pub fn print(&self, max_size: Option<usize>) {
        let size = max_size.map(|val| val.min(self.get_size()) ).unwrap_or( self.get_size() );
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new(""),
            Cell::new(self.label.as_str())
        ]));
        let type_cell = match self.datatype {
            FlexDataType::Dbl => Cell::new("f64"),
            FlexDataType::Uint => Cell::new("u32"),
            FlexDataType::Int => Cell::new("i64"),
            FlexDataType::Char => Cell::new("char"),
            FlexDataType::Str => Cell::new("str"),
            FlexDataType::NA => Cell::new("n/a")
        };
        table.add_row(Row::new(vec![Cell::new(""), type_cell]));
        for i in 0..size {
            let index_cell = match self[i].get_index() {
                FlexIndex::Uint(val) => Cell::new( format!("{}", val).as_str() ),
                FlexIndex::Str(val) => Cell::new( val.as_str() )
            };
            let data_cell = match self[i].get() {
                FlexData::Str(val) => Cell::new( val.as_str() ),
                FlexData::Dbl(val) => Cell::new( format!("{:.5}", val).as_str() ),
                FlexData::Uint(val) => Cell::new( format!("{}", val).as_str() ),
                FlexData::Int(val) => Cell::new( format!("{}", val).as_str() ),
                FlexData::Char(val) => Cell::new( format!("{}", val).as_str() ),
                FlexData::NA => Cell::new( "N/A" )
            };
            table.add_row(Row::new(vec![index_cell,data_cell]));
        }
        // Print the table to stdout
        table.printstd();
    }

    // Operations

    pub fn add(&self, label: &str, datatype: &FlexDataType, other: &FlexSeries) -> Self {
        let mut data = self.data.clone();
        for fdp in data.iter_mut() {
            if let Some( other_fdp ) = other.at( fdp.get_index() ) {
                let val = &convert( fdp.get(), datatype ) + &convert( other_fdp.get(), datatype );
                fdp.set( val );
            }
        }
        FlexSeries::from_vec(label, datatype.clone(), data)
    }

    pub fn sub(&self, label: &str, datatype: &FlexDataType, other: &FlexSeries) -> Self {
        let mut data = self.data.clone();
        for fdp in data.iter_mut() {
            if let Some( other_fdp ) = other.at( fdp.get_index() ) {
                let val = &convert( fdp.get(), datatype ) - &convert( other_fdp.get(), datatype );
                fdp.set( val );
            }
        }
        FlexSeries::from_vec(label, datatype.clone(), data)
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