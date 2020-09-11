use crate::FlexData;

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