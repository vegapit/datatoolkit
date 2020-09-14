use crate::{FlexData, FlexDataType};

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