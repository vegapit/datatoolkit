use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use crate::{FlexData, FlexIndex, FlexDataType};

pub fn extract_csv_headers(filepath: &str) -> Option<Vec<String>> {
    let file = File::open(filepath).expect("File not found");
    BufReader::new(file).lines().flatten()
        .map(|line| {
            let tokens : Vec<String> = line.split(',')
                .map(|s| s.to_string())
                .collect();
            tokens
        })
        .next()
}

pub fn derive_datatype(data: &FlexData) -> FlexDataType {
    match data {
        FlexData::Uint(_) => FlexDataType::Uint,
        FlexData::Int(_) => FlexDataType::Int,
        FlexData::Dbl(_) => FlexDataType::Dbl,
        FlexData::Str(_) => FlexDataType::Str,
        FlexData::Char(_) => FlexDataType::Char,
        _ => FlexDataType::NA
    }
}

pub fn generate_flexdata_from_str(token: &str, datatype: &FlexDataType) -> FlexData {
    match datatype {
        FlexDataType::Dbl => token.parse::<f64>().map_or(FlexData::NA, FlexData::Dbl),
        FlexDataType::Int => token.parse::<i32>().map_or(FlexData::NA, FlexData::Int),
        FlexDataType::Uint => token.parse::<u32>().map_or(FlexData::NA, FlexData::Uint),
        FlexDataType::Char => token.parse::<char>().map_or(FlexData::NA, FlexData::Char),
        _ => FlexData::Str( token.to_string() )
    }
}

pub fn make_data_from_index(index: &FlexIndex) -> FlexData {
    match index {
        FlexIndex::Uint(val) => FlexData::Uint(*val as u32),
        FlexIndex::Str(val) => FlexData::Str(val.to_string())
    }
}

pub fn make_index_from_data(data: &FlexData) -> FlexIndex {
    match data {
        FlexData::Uint(val) => FlexIndex::Uint(*val as usize),
        FlexData::Int(val) => FlexIndex::Uint(*val as usize),
        FlexData::Char(val) => FlexIndex::Str(format!("{}", val)),
        FlexData::Str(val) => FlexIndex::Str(val.to_string()),
        _ => panic!("FlexData::NA and FlexData::Dbl can not be indices")
    }
}

pub fn index_intersection(first: Vec<&FlexIndex>, other: Vec<&FlexIndex>) -> Vec<FlexIndex> {
    let set1 : HashSet<FlexIndex> = first.into_iter().cloned().collect();
    let set2 : HashSet<FlexIndex> = other.into_iter().cloned().collect();
    set1.intersection(&set2).cloned().collect()
}

pub fn convert(x: &FlexData, datatype: &FlexDataType) -> FlexData {
    match x {
        FlexData::Dbl( val ) => {
            match datatype {
                FlexDataType::Str => FlexData::Str( format!("{}", val) ),
                FlexDataType::Dbl => FlexData::Dbl( *val ),
                FlexDataType::Int => FlexData::Int( *val as i32 ),
                FlexDataType::Uint => FlexData::Uint( *val as u32 ),
                _ => FlexData::NA
            }
        },
        FlexData::Uint( val ) => {
            match datatype {
                FlexDataType::Str => FlexData::Str( format!("{}", val) ),
                FlexDataType::Dbl => FlexData::Dbl( *val as f64 ),
                FlexDataType::Int => FlexData::Int( *val as i32 ),
                FlexDataType::Uint => FlexData::Uint( *val ),
                _ => FlexData::NA
            }
        },
        FlexData::Int( val ) => {
            match datatype {
                FlexDataType::Str => FlexData::Str( format!("{}", val) ),
                FlexDataType::Dbl => FlexData::Dbl( *val as f64 ),
                FlexDataType::Int => FlexData::Int( *val ),
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
    for elt in v.iter().skip(1) {
        total += elt.clone();
    }
    total
}