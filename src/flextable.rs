use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::ops::*;
use crate::{FlexDataType, FlexData, FlexIndex, FlexDataPoint, FlexDataVector, FlexSeries};

pub struct FlexTable{
    iter_counter: usize,
    series: Vec<FlexSeries>
}

impl FlexTable {

    pub fn new( series: Vec<FlexSeries> ) -> Self {
        assert!( series.iter().map(|s| s.get_size()).min() == series.iter().map(|s| s.get_size()).max() );
        Self {
            iter_counter: 0,
            series: series
        }
    }

    pub fn from_records( records: Vec<FlexDataVector> ) -> Self {
        let headers : Vec<&str> = records[0].get_labels().iter().map(|s| s.as_str()).collect();
        let datatypes : Vec<FlexDataType> = records[0].get_datatypes().iter().cloned().collect();
        let mut series : Vec<FlexSeries> = headers.iter().zip( datatypes.into_iter() )
            .map(|(h,d)| FlexSeries::new(h, d))
            .collect();
        for counter in 0..records.len() {
            for k in 0..headers.len() {
                series[k].insert_update( FlexDataPoint::new( FlexIndex::Uint(counter), records[counter][k].clone()) );
            }
        }
        Self::new( series )
    }

    pub fn from_csv(filepath: &str, headers: Vec<&str>, datatypes: Vec<FlexDataType>) -> Self {
        let file = File::open(filepath).expect("File not found");
        let mut headers_processed = false;
        let mut column_positions : HashMap<String, usize> = HashMap::new();
        let mut series : Vec<FlexSeries> = Vec::new();
        let mut counter : usize = 0;
        for opt_line in BufReader::new(file).lines() {
            if let Ok( line ) = opt_line {
                let tokens : Vec<&str> = line.as_str().split(',').collect();
                if !headers_processed {
                    for (header, datatype) in headers.iter().zip( datatypes.iter() ) {
                        if let Some(pos) = tokens.iter().position(|token| token == header) {
                            column_positions.insert(header.to_string(), pos);
                            series.push( FlexSeries::new(header, datatype.clone()) );
                        }
                    }
                    headers_processed = true;
                } else {
                    for s in series.iter_mut() {
                        let i = column_positions.get( s.get_label() ).cloned().unwrap();
                        let str_value = tokens[i].to_string();
                        let fdata = match s.get_datatype() {
                            FlexDataType::Dbl => {
                                if let Some( value ) = str_value.parse::<f64>().ok() {
                                    FlexData::Dbl(value)
                                } else {
                                    FlexData::NA
                                }
                            },
                            FlexDataType::Int => {
                                if let Some( value ) = str_value.parse::<i64>().ok() {
                                    FlexData::Int(value)
                                } else {
                                    FlexData::NA
                                }
                            },
                            FlexDataType::Uint => {
                                if let Some( value ) = str_value.parse::<u32>().ok() {
                                    FlexData::Uint(value)
                                } else {
                                    FlexData::NA
                                }
                            },
                            FlexDataType::Char => {
                                if let Some( value ) = str_value.parse::<char>().ok() {
                                    FlexData::Char(value)
                                } else {
                                    FlexData::NA
                                }
                            },
                            _ => FlexData::Str(str_value)
                        };
                        s.insert_update( FlexDataPoint::new( FlexIndex::Uint(counter), fdata ) );
                    }
                    counter += 1;
                }
            }
        }
        Self::new( series )
    }

    pub fn get_headers(&self) -> Vec<&str> {
        self.series.iter()
            .map(|s| s.get_label())
            .collect()
    }

    pub fn get_datatypes(&self) -> Vec<&FlexDataType> {
        self.series.iter()
            .map(|s| s.get_datatype())
            .collect()
    }

    pub fn num_records(&self) -> usize {
        self.series[0].get_size()
    }

    pub fn num_series(&self) -> usize {
        self.series.len()
    }

    pub fn record(&self, k: usize) -> FlexDataVector {
        let data : Vec<FlexData> = self.series.iter()
            .map(|s| s[k].get() )
            .cloned()
            .collect();
        let index = self.series[0][k].get_index().clone();
        let datatypes : Vec<FlexDataType> = self.get_datatypes().iter().cloned().cloned().collect();
        FlexDataVector::new( index, self.get_headers().clone(), datatypes, data)
    }

    pub fn remove_record(&mut self, k: usize) {
        for series in self.series.iter_mut() {
            series.remove(k);
        }
    }

    pub fn remove_record_at(&mut self, index: &FlexIndex) {
        for series in self.series.iter_mut() {
            series.remove_at(index);
        }
    }

    // Filtering

    pub fn filter_all(&self, headers: &[&str], f: impl Fn(&FlexData) -> bool) -> Self {
        let mut records : Vec<FlexDataVector> = Vec::new();
        for k in 0..self.num_records() {
            let rec = self.record(k);
            if headers.iter().all(|lbl| f( rec.get(lbl).unwrap() )) {
                records.push(rec);
            }
        }
        Self::from_records( records )
    }

    pub fn filter_any(&self, headers: &[&str], f: impl Fn(&FlexData) -> bool) -> Self {
        let mut records : Vec<FlexDataVector> = Vec::new();
        for k in 0..self.num_records() {
            let rec = self.record(k);
            if headers.iter().any(|lbl| f( rec.get(lbl).unwrap() )) {
                records.push(rec);
            }
        }
        Self::from_records( records )
    }

    // NA Management

    pub fn has_na(&self) -> bool {
        self.series.iter()
            .any(|s| s.has_na())
    }

    pub fn get_na(&self) -> Self {
        self.filter_any( self.get_headers().as_slice(), |x: &FlexData| x == &FlexData::NA )
    }

    pub fn drop_na(&self) -> Self {
        self.filter_all( self.get_headers().as_slice(), |x: &FlexData| x != &FlexData::NA )
    }

    // n-ary operation

    pub fn nary_apply(&self, label: &str, datatype: FlexDataType, headers: &[&str], f: impl Fn(&[&FlexData]) -> FlexData) -> FlexSeries {
        let mut data : Vec<FlexDataPoint> = Vec::new();
        for k in 0..self.num_records() {
            let rec = self.record(k);
            let inputs : Vec<&FlexData> = headers.iter()
                .map(|h| rec.get(h).unwrap())
                .collect();
            data.push( FlexDataPoint::new(rec.get_index().clone(), f( inputs.as_slice() ) ) );
        }
        FlexSeries::from_vec(label, datatype, data)
    }
}

//Implement [] operator
impl Index<&str> for FlexTable {
    type Output = FlexSeries;
    fn index<'a>(&'a self, index: &str) -> &'a FlexSeries {
        self.series.iter().find(|s| s.get_label() == index).expect("Label not found")
    }
}

impl Iterator for FlexTable {
    type Item = FlexDataVector;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_counter < self.num_records() {
            self.iter_counter += 1;
            let data : Vec<FlexData> = self.series.iter()
                .map(|s| s[self.iter_counter].get() )
                .cloned()
                .collect();
            let index = self.series[0][self.iter_counter].get_index().clone();
            let datatypes : Vec<FlexDataType> = self.get_datatypes().iter().cloned().cloned().collect();
            let dv = FlexDataVector::new( index, self.get_headers().clone(), datatypes, data);
            Some( dv )
        } else {
            self.iter_counter = 0;
            None
        }
    }
}

impl std::fmt::Display for FlexTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output;
        output = format!("{:>width$}", " ", width=5);
        for header in self.get_headers().iter() {
            output = format!("{}{:>width$}", output, header, width=14);
        }
        output.push_str("\n");
        output = format!("{}{:>width$}", output, " ", width=5);
        for datatype in self.get_datatypes().iter() {
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
        for i in 0..self.num_records() {
            for j in 0..self.num_series() {
                if j == 0 {
                    match self.series[0][i].get_index() {
                        FlexIndex::Uint(val) => { output = format!("{}{:>width$}", output, val, width=5); },
                        FlexIndex::Str(val) => { output = format!("{}{:>width$}", output, val, width=5); }
                    }
                }
                match self.series[j][i].get() {
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
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}