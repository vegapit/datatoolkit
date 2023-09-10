use std::collections::HashMap;
use std::io::Write;
use std::ops::*;
use std::convert::TryFrom;
use std::iter::Iterator;
use rayon::prelude::*;
use prettytable::{Table, Row, Cell};

use std::sync::{Arc, Mutex};

use crate::helper::{convert, generate_flexdata_from_str, extract_csv_headers, make_index_from_data};
use crate::{FlexDataType, FlexData, FlexIndex, FlexDataPoint, FlexDataVector, FlexSeries};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexTable {
    iter_counter: usize,
    labels: Vec<String>,
    datatypes: Vec<FlexDataType>,
    data: Vec<FlexDataVector>,
    label_to_pos: HashMap<String,usize>,
    index_to_pos: HashMap<FlexIndex,usize>
}

impl FlexTable {

    // Constructors

    pub fn new( series: Vec<FlexSeries> ) -> Self {
        assert!( series.iter().map(|s| s.get_size()).min() == series.iter().map(|s| s.get_size()).max() );
        let mut data : Vec<FlexDataVector> = Vec::new();
        for i in 0..series[0].get_size() {
            let index = series[0][i].get_index().clone();
            let fds : Vec<FlexData> = series.iter()
                .map(|s| s[i].get_data().clone() )
                .collect();
            data.push( FlexDataVector::new( index, fds ) );
        }
        let mut index_to_pos : HashMap<FlexIndex,usize> = HashMap::new();
        for (i,fdp) in data.iter().enumerate() {
            index_to_pos.insert( fdp.get_index().clone(), i);
        }
        let mut label_to_pos : HashMap<String,usize> = HashMap::new();
        for (i,s) in series.iter().enumerate() {
            label_to_pos.insert( s.get_label().to_string(), i);
        }
        Self {
            iter_counter: 0,
            labels: series.iter().map(|s| s.get_label().to_string()).collect(),
            datatypes: series.iter().map(|s| s.get_datatype().clone()).collect(),
            data,
            label_to_pos,
            index_to_pos
        }
    }

    pub fn from_vecs( labels: Vec<String>, datatypes: Vec<FlexDataType>, data: Vec<FlexDataVector> ) -> Self {
        let mod_data : Vec<FlexDataVector> = data.into_iter()
            .map(|d| d.as_types(&datatypes))
            .collect();
        let mut index_to_pos : HashMap<FlexIndex,usize> = HashMap::new();
        for (i,fdp) in mod_data.iter().enumerate() {
            index_to_pos.insert( fdp.get_index().clone(), i);
        }
        let mut label_to_pos : HashMap<String,usize> = HashMap::new();
        for (i,l) in labels.iter().enumerate() {
            label_to_pos.insert( l.to_string(), i);
        }
        Self{
            iter_counter: 0,
            labels,
            datatypes,
            data: mod_data,
            label_to_pos,
            index_to_pos
        }
    }

    pub fn from_csv(text: &str, headers: Vec<String>, datatypes: Vec<FlexDataType>) -> Self {
        let mut filtered_text = text.to_string();
        filtered_text.retain(|c| c != '"');

        // Define header positions and series
        let raw_headers = extract_csv_headers(filtered_text.as_str());
        
        let header_positions : Vec<usize> = headers.iter()
            .filter_map(|header| raw_headers.iter().position(|token| token == header))
            .collect();

        let mut datavectors : Vec<FlexDataVector> = Vec::new();
        let mut counter = 0;
        for line in filtered_text.lines().skip(1) {
            let tokens : Vec<&str> = line.split(',').collect();
            let data : Vec<FlexData> = header_positions.iter()
                .enumerate()
                .map(|(i,&k)| generate_flexdata_from_str( tokens[k], &datatypes[i] ) )
                .collect();
            datavectors.push( FlexDataVector::new( FlexIndex::Uint(counter), data ) );
            counter += 1;
        }
        Self::from_vecs( headers, datatypes, datavectors )
    }

    pub fn to_csv(&self, filepath: &str) {
        let mut file = std::fs::File::create(filepath).expect("File creation failed");
        file.write_all(",".as_bytes()).expect("Writing failed");
        file.write_all(self.labels.join(",").as_bytes()).expect("Writing failed");
        file.write_all("\n".to_string().as_bytes()).expect("Writing failed");
        for i in 0..self.num_records() {
            let mut row : Vec<String> = Vec::new();
            let cell = match self.data[i].get_index() {
                FlexIndex::Uint(val) => format!("{}", val),
                FlexIndex::Str(val) => val.clone()
            };
            row.push(cell);
            for d in self.data[i].get_data() {
                let cell = match d {
                    FlexData::Str(val) => val.clone(),
                    FlexData::Dbl(val) => format!("{:.5}", val),
                    FlexData::Uint(val) => format!("{}", val),
                    FlexData::Int(val) => format!("{}", val),
                    FlexData::Char(val) => format!("{}", val),
                    FlexData::NA => "N/A".to_string()
                };
                row.push(cell);
            }
            file.write_all(row.join(",").as_bytes()).expect("Writing failed");
            file.write_all("\n".to_string().as_bytes()).expect("Writing failed");
        }
    }

    // Getters

    pub fn get_labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn get_datatypes(&self) -> &Vec<FlexDataType> {
        &self.datatypes
    }

    pub fn get_indices(&self) -> Vec<FlexIndex> {
        self.index_to_pos.keys().cloned().collect()
    }

    pub fn num_records(&self) -> usize {
        self.data.len()
    }

    pub fn num_series(&self) -> usize {
        self.datatypes.len()
    }

    // Selecting

    pub fn at(&self, index: &FlexIndex) -> Option<FlexDataVector> {
        self.index_to_pos.get( index ).map(|pos| self.data[*pos].clone())
    }

    pub fn contains(&self, index: &FlexIndex) -> bool {
        self.index_to_pos.contains_key( index )
    }

    pub fn get_subset(&self, indices: Vec<FlexIndex>) -> Self {
        let records : Vec<FlexDataVector> = indices.into_iter()
            .filter_map(|index| self.at(&index))
            .collect();
        Self::from_vecs( self.labels.clone(), self.datatypes.clone(),  records )
    }

    pub fn extract_series(&self, labels: &[&str]) -> Vec<FlexSeries> {
        let res : Vec<FlexSeries> = labels.iter()
            .map(|&label| {
                let pos = self.label_to_pos.get(label).unwrap();
                let data : Vec<FlexDataPoint> = self.data.par_iter()
                    .map(|v| {
                        FlexDataPoint::new( v.get_index().clone(), v.get_data()[*pos].clone() )
                    })
                    .collect();
                FlexSeries::from_vec(label, self.datatypes[*pos].clone(), data)
            })
            .collect();
        res
    }

    pub fn extract_all_series(&self) -> HashMap<String,FlexSeries> {
        let mut res : HashMap<String,FlexSeries> = HashMap::new();
        self.labels.iter()
            .for_each(|label| {
                let pos = self.label_to_pos.get(label).unwrap();
                let data : Vec<FlexDataPoint> = self.data.par_iter()
                    .map(|v| FlexDataPoint::new( v.get_index().clone(), v.get_data()[*pos].clone() ))
                    .collect();
                res.insert( label.clone(), FlexSeries::from_vec(label, self.datatypes[*pos].clone(), data) );
            });
        res
    }

    // Modifiers

    pub fn add_series(&mut self, series: FlexSeries) {
        let adj_series = series.align_to( &self.get_indices() );
        self.labels.push( adj_series.get_label().to_string() );
        self.label_to_pos.insert( adj_series.get_label().to_string(), self.labels.len() - 1);
        self.datatypes.push( adj_series.get_datatype().clone() );
        let mod_data : Vec<FlexDataVector> = self.data.iter()
            .zip(adj_series.get_data())
            .map(|(dv,dp)| {
                let mut v = dv.get_data().clone();
                v.push( dp.clone() );
                FlexDataVector::new(dv.get_index().clone(), v)
            })
            .collect();
        self.data = mod_data;
    }

    pub fn remove_record(&mut self, k: usize) {
        self.index_to_pos.remove( self.data[k].get_index() );
        self.data.remove(k);
    }

    pub fn remove_record_at(&mut self, index: &FlexIndex) {
        if let Some( &i ) = self.index_to_pos.get( index ) {
            self.index_to_pos.remove( index );
            self.data.remove(i);
        }
    }

    pub fn set_index(&mut self, label: &str) {
        let pos = self.label_to_pos.get(label).unwrap();
        let mod_data : Vec<FlexDataVector> = self.data.iter()
            .map(|v| {
                let mut data = v.get_data().clone();
                let index = make_index_from_data( &data[*pos] );
                data.remove( *pos );
                FlexDataVector::new(index, data)
            })
            .collect();
        self.data = mod_data;
        self.label_to_pos.remove(label);
    }

    // Filtering

    pub fn filter_all(&self, labels: &[&str], f: impl Fn(&FlexData) -> bool) -> Self {
        let mut records : Vec<FlexDataVector> = Vec::new();
        for k in 0..self.num_records() {
            if labels.iter()
                .all(|&l| {
                    if let Some( &pos ) = self.label_to_pos.get( l ) {
                        f( &self.data[k].get_data()[pos] )
                    } else {
                        false
                    }
                }) {
                records.push( self.data[k].clone() );
            }
        }
        Self::from_vecs( self.labels.clone(), self.datatypes.clone(),  records )
    }

    pub fn filter_any(&self, labels: &[&str], f: impl Fn(&FlexData) -> bool) -> Self {
        let mut records : Vec<FlexDataVector> = Vec::new();
        for k in 0..self.num_records() {
            if labels.iter().any(|&l| {
                if let Some( &pos ) = self.label_to_pos.get( l ) {
                    f( &self.data[k].get_data()[pos] )
                } else {
                    false
                }
            }) {
                records.push( self.data[k].clone() );
            }
        }
        Self::from_vecs( self.labels.clone(), self.datatypes.clone(),  records )
    }

    // NA Management

    pub fn has_na(&self) -> bool {
        self.data.iter()
            .any(|s| s.has_na())
    }

    pub fn get_na(&self) -> Self {
        let labels : Vec<&str> = self.get_labels().iter()
            .map(|s| s.as_str())
            .collect();
        self.filter_any( labels.as_slice(), |x: &FlexData| x == &FlexData::NA )
    }

    pub fn drop_na(&self) -> Self {
        let labels : Vec<&str> = self.get_labels().iter()
            .map(|s| s.as_str())
            .collect();
        self.filter_all( labels.as_slice(), |x: &FlexData| x != &FlexData::NA )
    }

    // n-ary operation

    pub fn nary_apply(&self, label: &str, datatype: FlexDataType, labels: &[&str], f: impl Fn(&[&FlexData]) -> FlexData) -> FlexSeries {
        let mut data : Vec<FlexDataPoint> = Vec::new();
        for k in 0..self.num_records() {
            let inputs : Vec<&FlexData> = labels.iter()
                .map(|&l| {
                    let pos = self.label_to_pos.get(l).expect("Label not found");
                    &self.data[k][*pos]
                })
                .collect();
            data.push( FlexDataPoint::new( self.data[k].get_index().clone(), f( inputs.as_slice() ) ) );
        }
        FlexSeries::from_vec(label, datatype, data)
    }

    pub fn sort(&self, label: &str, ascending: bool) -> Self {
        let pos = self.label_to_pos.get(label).unwrap();
        let mut data = self.data.clone();
        if ascending {
            data.sort_by(|a,b| a[*pos].partial_cmp(&b[*pos]).unwrap() );
        } else {
            data.sort_by(|a,b| b[*pos].partial_cmp(&a[*pos]).unwrap() );
        }
        FlexTable::from_vecs(self.labels.clone(), self.datatypes.clone(), data)
    }

    // grouping 

    pub fn group_by(table: &Self, label: &str) -> HashMap<String, Self> {
        
        let groups : Arc<Mutex<HashMap<String, Self>>> = Arc::new( Mutex::new( HashMap::new() ) );
        let mut thread_handles : Vec<_> = Vec::new();

        let series = table.extract_series( &[label] );
        if series.len() == 1 {
            // Define value set
            let mut value_set : HashMap<String, Vec<FlexIndex>> = HashMap::new();
            for k in 0..series[0].get_size() {
                let fdp = series[0][k].clone();
                let val : String = String::try_from( &convert(fdp.get_data(), &FlexDataType::Str) )
                    .expect("Value not convertible to String");
                if let Some( v ) = value_set.get_mut( &val ) {
                    v.push( fdp.get_index().clone() );
                } else {
                    value_set.insert( val, Vec::<FlexIndex>::new() );
                }
            }

            // Build subsets
            let arc_table = Arc::new( table.clone() );
            for (k,v) in value_set.into_iter() {
                let cloned_groups = groups.clone();
                let cloned_table = arc_table.clone();
                let handle = std::thread::spawn(move || {
                    let subset = cloned_table.get_subset(v);
                    let mut local_groups = cloned_groups.lock().unwrap();
                    local_groups.insert(k, subset);
                });
                thread_handles.push( handle );
            }
        }

        if !thread_handles.is_empty() {
            thread_handles.into_iter()
                .for_each(|handle| { let _ = handle.join(); });
        }

        let res = groups.lock().unwrap().clone();
        res
    }

    // pretty print

    pub fn print(&self, max_size: Option<usize>) {
        let size = max_size.map(|val| val.min(self.num_records()) ).unwrap_or( self.num_records() );
        let mut table = Table::new();
        let mut headers_cells : Vec<Cell> = self.labels.iter()
            .map(|h| Cell::new(h))
            .collect();
        headers_cells.insert(0, Cell::new(""));
        table.add_row(Row::new(headers_cells));
        let mut types_cells : Vec<Cell> = self.datatypes.iter()
            .map(|datatype| {
                match datatype {
                    FlexDataType::Dbl => Cell::new("f64"),
                    FlexDataType::Uint => Cell::new("usize"),
                    FlexDataType::Int => Cell::new("isize"),
                    FlexDataType::Char => Cell::new("char"),
                    FlexDataType::Str => Cell::new("str"),
                    FlexDataType::NA => Cell::new("n/a")
                }
            })
            .collect();
        types_cells.insert(0, Cell::new(""));
        table.add_row(Row::new(types_cells));
        for i in 0..size {
            let mut record_cells : Vec<Cell> = Vec::new();
            for j in 0..self.num_series() {
                if j == 0 {
                    let cell = match self.data[i].get_index() {
                        FlexIndex::Uint(val) => Cell::new( format!("{}", val).as_str() ),
                        FlexIndex::Str(val) => Cell::new( val.as_str() )
                    };
                    record_cells.push(cell);
                }
                let cell = match &self.data[i].get_data()[j] {
                    FlexData::Str(val) => Cell::new( val.as_str() ),
                    FlexData::Dbl(val) => Cell::new( format!("{:.5}", val).as_str() ),
                    FlexData::Uint(val) => Cell::new( format!("{}", val).as_str() ),
                    FlexData::Int(val) => Cell::new( format!("{}", val).as_str() ),
                    FlexData::Char(val) => Cell::new( format!("{}", val).as_str() ),
                    FlexData::NA => Cell::new( "N/A" )
                };
                record_cells.push(cell);
            }
            table.add_row(Row::new(record_cells));
        }
        // Print the table to stdout
        table.printstd();
    }
}

//Implement [] operator

impl Index<usize> for FlexTable {
    type Output = FlexDataVector;
    fn index(&self, index: usize) -> &FlexDataVector {
        &self.data[index]
    }
}

impl Iterator for FlexTable {
    type Item = FlexDataVector;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_counter < self.num_records() {
            let dv = self.data[self.iter_counter].clone();
            self.iter_counter += 1;
            Some( dv )
        } else {
            self.iter_counter = 0;
            None
        }
    }
}