use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{FlexData, FlexDataType};

pub fn csv_header(filepath: &str) -> Option<Vec<String>> {
    let file = File::open(filepath).expect("File not found");
    for opt_line in BufReader::new(file).lines() {
        if let Ok( line ) = opt_line {
            let tokens : Vec<&str> = line.as_str().split(',').collect();
            return Some( tokens.iter().map(|s| s.to_string()).collect::<Vec<String>>() );
        }
    }
    None
}

pub fn convert(x: &FlexData, datatype: &FlexDataType) -> FlexData {
    match x {
        FlexData::Dbl( val ) => {
            match datatype {
                FlexDataType::Str => FlexData::Str( format!("{}", val) ),
                FlexDataType::Dbl => FlexData::Dbl( *val as f64 ),
                FlexDataType::Int => FlexData::Int( *val as i64 ),
                FlexDataType::Uint => FlexData::Uint( *val as u32 ),
                _ => FlexData::NA
            }
        },
        FlexData::Uint( val ) => {
            match datatype {
                FlexDataType::Str => FlexData::Str( format!("{}", val) ),
                FlexDataType::Dbl => FlexData::Dbl( *val as f64 ),
                FlexDataType::Int => FlexData::Int( *val as i64 ),
                FlexDataType::Uint => FlexData::Uint( *val as u32 ),
                _ => FlexData::NA
            }
        },
        FlexData::Int( val ) => {
            match datatype {
                FlexDataType::Str => FlexData::Str( format!("{}", val) ),
                FlexDataType::Dbl => FlexData::Dbl( *val as f64 ),
                FlexDataType::Int => FlexData::Int( *val as i64 ),
                FlexDataType::Uint => FlexData::Uint( *val as u32 ),
                _ => FlexData::NA
            }
        },
        FlexData::Str( val ) => {
            match datatype {
                FlexDataType::Str => FlexData::Str(val.to_string()),
                _ => FlexData::NA
            }
        },
        FlexData::Char( val ) => {
            match datatype {
                FlexDataType::Str => FlexData::Str( format!("{}", val) ),
                FlexDataType::Char => FlexData::Char( *val ),
                _ => FlexData::NA
            }
        },
        _ => FlexData::NA
    }
}

pub fn inverse(x: &FlexData) -> FlexData {
    match x {
        FlexData::Dbl(val) => {
            if val != &0f64 {
                FlexData::Dbl(1.0 / val)
            } else {
                FlexData::NA
            }
        },
        _ => FlexData::NA
    }
}

pub fn ln(x: &FlexData) -> FlexData {
    match x {
        FlexData::Dbl(val) => {
            if val > &0f64 {
                FlexData::Dbl( val.ln() )
            } else {
                FlexData::NA
            }
        },
        _ => FlexData::NA
    }
}

pub fn exp(x: &FlexData) -> FlexData {
    match x {
        FlexData::Dbl(val) => FlexData::Dbl( val.exp() ),
        _ => FlexData::NA
    }
}

pub fn sum(v: Vec<FlexData>) -> FlexData {
    let mut total = v[0].clone();
    for i in 1..v.len() {
        total += v[i].clone();
    }
    total
}