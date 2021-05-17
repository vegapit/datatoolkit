use crate::{FlexDataType, FlexDataPoint, FlexData, FlexIndex};
use crate::helper::convert;
use std::collections::HashMap;
use std::ops::*;
use std::convert::TryFrom;
use prettytable::{Table, Row, Cell};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexSeries {
    label: String,
    datatype: FlexDataType,
    data: Vec<FlexDataPoint>,
    index_to_pos: HashMap<FlexIndex,usize>
}

impl FlexSeries {

    pub fn new(label: &str, datatype: FlexDataType) -> Self {
        Self {
            label: label.to_string(),
            datatype: datatype,
            data: Vec::new(),
            index_to_pos: HashMap::new()
        }
    }

    pub fn from_vec(label: &str, datatype: FlexDataType, data: Vec<FlexDataPoint>) -> Self {
        let mod_data : Vec<FlexDataPoint> = data.clone().iter_mut()
            .map(|d| d.as_type(&datatype) )
            .collect();
        let mut index_to_pos : HashMap<FlexIndex,usize> = HashMap::new();
        for (i,fdp) in mod_data.iter().enumerate() {
            index_to_pos.insert( fdp.get_index().clone(), i);
        }
        Self {
            label: label.to_string(),
            datatype: datatype,
            data: mod_data,
            index_to_pos: index_to_pos
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

    pub fn get_data(&self) -> Vec<&FlexData> {
        self.data.iter()
            .map(|fdp| fdp.get_data())
            .collect()
    }

    // Selecting

    pub fn at(&self, index: &FlexIndex) -> Option<FlexDataPoint> {
        self.index_to_pos.get( index )
            .map(|&pos| self.data[pos].clone() )
    }

    pub fn contains(&self, index: &FlexIndex) -> bool {
        self.index_to_pos.contains_key( index )
    }

    pub fn get_subset(&self, indices: Vec<FlexIndex>) -> Self {
        let records : Vec<FlexDataPoint> = indices.into_iter()
            .filter_map(|index| self.at( &index ))
            .collect();
        Self::from_vec( self.get_label(), self.get_datatype().clone(), records )
    }

    // Data operations

    pub fn update(&mut self, data: FlexDataPoint) {
        if let Some( &i ) = self.index_to_pos.get( data.get_index() ) {
            self.data[i] = data.as_type(&self.datatype);
        }
    }

    pub fn insert(&mut self, data: FlexDataPoint) {
        self.index_to_pos.insert(data.get_index().clone(), self.data.len());
        self.data.push(data.as_type(&self.datatype));
    }

    pub fn insert_update(&mut self, data: FlexDataPoint) {
        if let Some( &i ) = self.index_to_pos.get( data.get_index() ) {
            self.data[i] = data.as_type(&self.datatype);
        } else {
            self.index_to_pos.insert(data.get_index().clone(), self.data.len());
            self.data.push(data.as_type(&self.datatype));
        }
    }

    pub fn remove(&mut self, k: usize) {
        self.index_to_pos.remove( self.data[k].get_index() );
        self.data.remove(k);
    }

    pub fn remove_at(&mut self, index: &FlexIndex) {
        if let Some( &i ) = self.index_to_pos.get( index ) {
            self.index_to_pos.remove( index );
            self.data.remove(i);
        }
    }

    // Transformation

    pub fn as_type(&self, datatype: &FlexDataType) -> Self {
        let data : Vec<FlexDataPoint> = self.data.iter()
            .map(|d| d.as_type(datatype))
            .collect();
        Self::from_vec(self.label.as_str(), self.datatype.clone(), data)
    }

    pub fn align_to(&self, indices: &Vec<FlexIndex>) -> Self {
        let mut series = self.clone();
        for index in indices.iter() {
            if !series.contains( index ) {
                series.insert( FlexDataPoint::new((*index).clone(), FlexData::NA) );
            }
        }
        series
    }

    pub fn apply(&self, f: impl Fn(&FlexData) -> FlexData) -> Self {
        let data = self.data.iter()
            .map(|dp| dp.apply(&f))
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
            .any(|fdp| fdp.get_data() == &FlexData::NA )
    }

    pub fn get_na(&self) -> Self {
        self.filter(|x: &FlexDataPoint| x.get_data() == &FlexData::NA)
    }

    pub fn drop_na(&self) -> Self {
        self.filter(|x: &FlexDataPoint| x.get_data() != &FlexData::NA)
    }

    // Statistics

    pub fn sum(&self) -> FlexData {
        match self.datatype {
            FlexDataType::Int => self.data.iter().fold( FlexData::Int(0), |acc, fdp| &acc + fdp.get_data()),
            FlexDataType::Uint => self.data.iter().fold( FlexData::Uint(0), |acc, fdp| &acc + fdp.get_data()),
            FlexDataType::Dbl => self.data.iter().fold( FlexData::Dbl(0f64), |acc, fdp| &acc + fdp.get_data()),
            _ => FlexData::NA 
        }
    }

    pub fn mean(&self) -> FlexData {
        if self.get_size() == 0 {
            FlexData::NA
        } else {
            match self.sum() {
                FlexData::Int(val) => FlexData::Dbl( (val as f64) / (self.get_size() as f64) ),
                FlexData::Uint(val) => FlexData::Dbl( (val as f64) / (self.get_size() as f64) ),
                FlexData::Dbl(val) => FlexData::Dbl( val / (self.get_size() as f64) ),
                _ => FlexData::NA
            }
        }
    }

    pub fn covariance(&self, other: &Self, is_sample: bool) -> FlexData {
        if self.get_size() == 0 || other.get_size() == 0 || self.get_size() != other.get_size() {
            FlexData::NA
        } else {
            let float_series1 = self.as_type(&FlexDataType::Dbl);
            let m1 = f64::try_from( &float_series1.mean() ).unwrap();
            let float_series2 = other.as_type(&FlexDataType::Dbl);
            let m2 = f64::try_from( &float_series2.mean() ).unwrap();
            let prod_float_series = float_series1.prod("product", &FlexDataType::Dbl, &float_series2);
            let n = prod_float_series.get_size() as f64;
            if is_sample {
                match prod_float_series.sum() {
                    FlexData::Dbl(val) => FlexData::Dbl( (val - n * m1 * m2) / (n - 1f64) ),
                    _ => FlexData::NA
                }
            } else {
                match prod_float_series.sum() {
                    FlexData::Dbl(val) => FlexData::Dbl( val / n - m1 * m2 ),
                    _ => FlexData::NA
                }
            }
        }
    }

    pub fn variance(&self, is_sample: bool) -> FlexData {
        if self.get_size() == 0 {
            FlexData::NA
        } else {
            let float_series = self.as_type(&FlexDataType::Dbl);
            let m = f64::try_from( &float_series.mean() ).unwrap();
            let squared_float_series = float_series.prod("squared", &FlexDataType::Dbl, &float_series);
            let n = float_series.get_size() as f64;
            if is_sample {
                match squared_float_series.sum() {
                    FlexData::Dbl(val) => FlexData::Dbl( (val - n * m.powf(2f64)) / (n - 1f64) ),
                    _ => FlexData::NA
                }
            } else {
                match squared_float_series.sum() {
                    FlexData::Dbl(val) => FlexData::Dbl( val / n - m.powf(2f64) ),
                    _ => FlexData::NA
                }
            }
        }
    }

    pub fn pearson_correlation(&self, other: &Self) -> FlexData {
        let float_series1 = self.as_type(&FlexDataType::Dbl);
        let m1 = f64::try_from( &float_series1.mean() ).unwrap();
        let float_series2 = other.as_type(&FlexDataType::Dbl);
        let m2 = f64::try_from( &float_series2.mean() ).unwrap();
        let prod_float_series = float_series1.prod("product", &FlexDataType::Dbl, &float_series2);
        let squared_float_series1 = float_series1.prod("squared1", &FlexDataType::Dbl, &float_series1);
        let squared_float_series2 = float_series2.prod("squared2", &FlexDataType::Dbl, &float_series2);
        let n = prod_float_series.get_size() as f64;
        match (prod_float_series.sum(),squared_float_series1.sum(),squared_float_series2.sum())  {
            (FlexData::Dbl(val12),FlexData::Dbl(val1),FlexData::Dbl(val2)) => FlexData::Dbl( (val12 - n * m1 * m2) / ((val1 - n * m1.powf(2f64)).sqrt() * (val2 - n * m2.powf(2f64)).sqrt()) ),
            _ => FlexData::NA
        }
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
            let data_cell = match self[i].get_data() {
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

    pub fn add(&self, label: &str, datatype: &FlexDataType, other: &Self) -> Self {
        let mut data = self.data.clone();
        for fdp in data.iter_mut() {
            if let Some( other_fdp ) = other.at( fdp.get_index() ) {
                let val = &convert( fdp.get_data(), datatype ) + &convert( other_fdp.get_data(), datatype );
                fdp.set_data( val );
            }
        }
        Self::from_vec(label, datatype.clone(), data)
    }

    pub fn sub(&self, label: &str, datatype: &FlexDataType, other: &Self) -> Self {
        let mut data = self.data.clone();
        for fdp in data.iter_mut() {
            if let Some( other_fdp ) = other.at( fdp.get_index() ) {
                let val = &convert( fdp.get_data(), datatype ) - &convert( other_fdp.get_data(), datatype );
                fdp.set_data( val );
            }
        }
        Self::from_vec(label, datatype.clone(), data)
    }

    pub fn prod(&self, label: &str, datatype: &FlexDataType, other: &Self) -> Self {
        let mut data = self.data.clone();
        for fdp in data.iter_mut() {
            if let Some( other_fdp ) = other.at( fdp.get_index() ) {
                let val = &convert( fdp.get_data(), datatype ) * &convert( other_fdp.get_data(), datatype );
                fdp.set_data( val );
            }
        }
        Self::from_vec(label, datatype.clone(), data)
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