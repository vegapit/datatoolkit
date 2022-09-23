use crate::{FlexDataType, FlexDataPoint, FlexData, FlexIndex};
use crate::helper::{convert, index_intersection};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::*;
use prettytable::{Table, Row, Cell};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexSeries {
    iter_counter: usize,
    label: String,
    datatype: FlexDataType,
    data: Vec<FlexDataPoint>,
    index_to_pos: HashMap<FlexIndex,usize>
}

impl FlexSeries {

    pub fn new(label: &str, datatype: FlexDataType) -> Self {
        Self {
            iter_counter: 0,
            label: label.to_string(),
            datatype: datatype,
            data: Vec::new(),
            index_to_pos: HashMap::new()
        }
    }

    pub fn from_vec(label: &str, datatype: FlexDataType, data: Vec<FlexDataPoint>) -> Self {
        let mod_data : Vec<FlexDataPoint> = data.into_iter()
            .map(|d| d.as_type(&datatype) )
            .collect();
        let mut index_to_pos : HashMap<FlexIndex,usize> = HashMap::new();
        for (i,fdp) in mod_data.iter().enumerate() {
            index_to_pos.insert( fdp.get_index().clone(), i);
        }
        Self {
            iter_counter: 0,
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

    pub fn at(&self, index: &FlexIndex) -> Option<&FlexDataPoint> {
        self.index_to_pos.get( index )
            .map(|&pos| &self.data[pos] )
    }

    pub fn contains(&self, index: &FlexIndex) -> bool {
        self.index_to_pos.contains_key( index )
    }

    pub fn get_subset(&self, indices: Vec<FlexIndex>) -> Self {
        let records : Vec<FlexDataPoint> = indices.into_iter()
            .filter_map(|index| self.at( &index ))
            .cloned()
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

    pub fn filter_any(&self, f: impl Fn(&FlexData) -> bool) -> Self {
        let data : Vec<FlexDataPoint> = self.data.iter()
            .filter(|d| f(d.get_data()))
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
        self.filter_any(|x: &FlexData| x == &FlexData::NA)
    }

    pub fn drop_na(&self) -> Self {
        self.filter_any(|x: &FlexData| x != &FlexData::NA)
    }

    // Statistics

    pub fn mean(&self) -> Option<f64> {
        if self.get_size() == 0 {
            None
        } else {
            let n = self.get_size() as f64;
            match self.clone().map(|dp| dp.get_data().clone()).sum() {
                FlexData::Int(val) => Some( (val as f64) / n ),
                FlexData::Uint(val) => Some( (val as f64) / n ),
                FlexData::Dbl(val) => Some( val / n ),
                _ => None
            }
        }
    }

    pub fn covariance(&self, other: &Self, is_sample: bool) -> Option<f64> {
        let intersect = index_intersection(self.get_indices(), other.get_indices());
        if intersect.len() <= 1 {
            None
        } else {
            let float_series1 = self.as_type(&FlexDataType::Dbl);
            let float_series2 = other.as_type(&FlexDataType::Dbl);
            let mut m1 = 0.0f64;
            let mut m2 = 0.0f64;
            let mut res = 0.0f64;
            let mut n = 0.0f64;
            for idx in intersect.into_iter() {
                let x1 = f64::try_from( float_series1.at(&idx).unwrap().get_data() ).unwrap();
                let x2 = f64::try_from( float_series2.at(&idx).unwrap().get_data() ).unwrap();
                n += 1.0;
                let dx1 = x1 - m1;
                m1 += dx1 / n;
                m2 += (x2 - m2) / n;
                res += dx1 * (x2 - m2);
            }
            if is_sample {
                Some( res / (n - 1.0) )
            } else {
                Some( res / n)
            }
        }
    }

    pub fn variance(&self, is_sample: bool) -> Option<f64> {
        if self.get_size() <= 1 {
            None
        } else {
            let float_series = self.as_type(&FlexDataType::Dbl);
            let mut m = 0.0f64;
            let mut res = 0.0f64;
            let mut n = 0.0f64;
            for idx in self.get_indices() {
                let x = f64::try_from( float_series.at(&idx).unwrap().get_data() ).unwrap();
                n += 1.0;
                let dx1 = x - m;
                m += dx1 / n;
                res += dx1 * (x - m);
            }
            if is_sample {
                Some( res / (n - 1.0) )
            } else {
                Some( res / n)
            }
        }
    }

    pub fn pearson_correlation(&self, other: &Self) -> Option<f64> {
        let intersect = index_intersection(self.get_indices(), other.get_indices());
        if intersect.len() <= 1 {
            None
        } else {
            let float_series1 = self.as_type(&FlexDataType::Dbl);
            let float_series2 = other.as_type(&FlexDataType::Dbl);
            let mut m1 = 0.0f64;
            let mut m2 = 0.0f64;
            let mut m12 = 0.0f64;
            let mut cov = 0.0f64;
            let mut v1 = 0.0f64;
            let mut v2 = 0.0f64;
            let mut n = 0.0f64;
            for idx in intersect.into_iter() {
                let x1 = f64::try_from( float_series1.at(&idx).unwrap().get_data() ).unwrap();
                let x2 = f64::try_from( float_series2.at(&idx).unwrap().get_data() ).unwrap();
                let dx1 = x1 - m1;
                let dx2 = x2 - m2;
                n += 1.0;
                m1 += dx1 / n;
                m2 += dx2 / n;
                m12 += (x2 - m12) / n;
                cov += dx1 * (x2 - m12);
                v1 += dx1 * (x1 - m1);
                v2 += dx2 * (x2 - m2);
            }
            Some( cov / (v1.sqrt() * v2.sqrt()) )
        }
    }

    // Sorting

    pub fn sort(&self, ascending: bool) -> Self {
        let mut data = self.data.clone();
        if ascending {
            data.sort_by(|a,b| a.partial_cmp(b).unwrap() );
        } else {
            data.sort_by(|a,b| b.partial_cmp(a).unwrap() );
        }
        FlexSeries::from_vec(self.label.as_str(), self.datatype.clone(), data)
    }

    // Pretty print

    pub fn print(&self, max_size: Option<usize>) {
        let size = max_size.map(|val| val.min(self.get_size()) ).unwrap_or( self.get_size() );
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new(""),
            Cell::new(self.label.as_str())
        ]));
        let type_cell = match self.datatype {
            FlexDataType::Dbl => Cell::new("f64"),
            FlexDataType::Uint => Cell::new("usize"),
            FlexDataType::Int => Cell::new("isize"),
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
        let mut data : Vec<FlexDataPoint> = Vec::new();
        for idx in index_intersection(self.get_indices().clone(), other.get_indices().clone()).into_iter() {
            let fdv1 = self.at( &idx ).unwrap();
            let fdv2 = other.at( &idx ).unwrap();
            let val = &convert( fdv1.get_data(), datatype ) + &convert( fdv2.get_data(), datatype );
            data.push( FlexDataPoint::new(idx, val) );
        }
        Self::from_vec(label, datatype.clone(), data)
    }

    pub fn sub(&self, label: &str, datatype: &FlexDataType, other: &Self) -> Self {
        let mut data : Vec<FlexDataPoint> = Vec::new();
        for idx in index_intersection(self.get_indices().clone(), other.get_indices().clone()).into_iter() {
            let fdv1 = self.at( &idx ).unwrap();
            let fdv2 = other.at( &idx ).unwrap();
            let val = &convert( fdv1.get_data(), datatype ) - &convert( fdv2.get_data(), datatype );
            data.push( FlexDataPoint::new(idx, val) );
        }
        Self::from_vec(label, datatype.clone(), data)
    }

    pub fn prod(&self, label: &str, datatype: &FlexDataType, other: &Self) -> Self {
        let mut data : Vec<FlexDataPoint> = Vec::new();
        for idx in index_intersection(self.get_indices().clone(), other.get_indices().clone()).into_iter() {
            let fdv1 = self.at( &idx ).unwrap();
            let fdv2 = other.at( &idx ).unwrap();
            let val = &convert( fdv1.get_data(), datatype ) * &convert( fdv2.get_data(), datatype );
            data.push( FlexDataPoint::new(idx, val) );
        }
        Self::from_vec(label, datatype.clone(), data)
    }
}

// Implement [] operator

impl Index<usize> for FlexSeries {
    type Output = FlexDataPoint;
    fn index<'a>(&'a self, index: usize) -> &'a FlexDataPoint {
        &self.data[index]
    }
}

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

impl Index<u32> for FlexSeries {
    type Output = FlexDataPoint;
    fn index<'a>(&'a self, index: u32) -> &'a FlexDataPoint {
        &self.data[index as usize]
    }
}

impl Iterator for FlexSeries {
    type Item = FlexDataPoint;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_counter < self.get_size() {
            let dv = self.data[self.iter_counter].clone();
            self.iter_counter += 1;
            Some( dv )
        } else {
            self.iter_counter = 0;
            None
        }
    }
}