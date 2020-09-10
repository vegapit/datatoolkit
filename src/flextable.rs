use std::fs::File;
use std::fmt;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use crate::{FlexDataVector, FlexDataPoint, FlexDataType, FlexData};

pub struct FlexTable{
    headers: Vec<String>,
    datatypes: Vec<FlexDataType>,
    data: Vec<FlexDataVector<usize>>
}

impl FlexTable {

    pub fn from_csv(filepath: &str, headers: Vec<&str>, datatypes: Vec<FlexDataType>) -> Self {
        let file = File::open(filepath).expect("File not found");
        let mut headers_processed = false;
        let mut column_positions : HashMap<String, usize> = HashMap::new();
        let mut data : Vec<FlexDataVector<usize>> = Vec::new();
        let mut counter : usize = 0;
        for opt_line in BufReader::new(file).lines() {
            if let Ok( line ) = opt_line {
                let tokens : Vec<&str> = line.as_str().split(',').collect();
                if !headers_processed {
                    for header in headers.iter() {
                        if let Some(pos) = tokens.iter().position(|token| token == header) {
                            column_positions.insert(header.to_string(), pos);
                        }
                    }
                    headers_processed = true;
                } else {
                    let mut datapoints : Vec<FlexDataPoint<usize>> = Vec::new();
                    for (header, datatype) in headers.iter().zip( datatypes.iter() ) {
                        let i = column_positions.get( &header.to_string() ).cloned().unwrap();
                        let str_value = tokens[i].to_string();
                        let fdata = match datatype {
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
                        datapoints.push( FlexDataPoint::<usize>::new( header, counter, fdata ) );
                    }
                    data.push( FlexDataVector::new(counter, datapoints) );
                    counter += 1;
                }
            }
        }
        Self {
            headers: headers.into_iter().map(|x| x.to_string()).collect(),
            datatypes: datatypes,
            data: data
        }
    }

    pub fn headers(&self) -> Vec<String> {
        self.headers.clone()
    }

}

impl fmt::Display for FlexTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output;
        output = format!("{:>width$}", " ", width=5);
        for header in self.headers.iter() {
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
        for dv in self.data.iter() {
            output = format!("{}{:>width$}", output, dv.get_index(), width=5);
            for header in self.headers.iter() {
                match dv.get(header).unwrap().get() {
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
        }
        write!(f, "{}", output)
    }
}