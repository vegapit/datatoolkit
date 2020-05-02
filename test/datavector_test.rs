extern crate datatoolkit;
extern crate chrono;

use std::collections::HashMap;
use chrono::{Utc, TimeZone};
use datatoolkit::DataVector;

fn build_datavector() -> DataVector {
    let timesig = Utc.ymd(2019,7,24).and_hms(12,0,0);
    let mut data : HashMap<String,f64> = HashMap::new();
    data.insert("price".to_string(), 100f64);
    data.insert("volume".to_string(), 10f64);
    DataVector::new("test", timesig, data)
}

#[test]
fn methods() {
    let mut dv = build_datavector();
    assert_eq!( dv.get("close"), None);
    assert_eq!( dv.get("price"), Some(100f64) );
    assert!( dv.contains("price") );
    assert!( dv.contains("volume") );
    dv.set("price", 102f64);
    assert_eq!( dv.get("price"), Some(102f64) );
    println!("{}", serde_json::to_string( &dv ).unwrap() );
}