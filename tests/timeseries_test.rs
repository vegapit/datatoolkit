extern crate datatoolkit;
extern crate chrono;

use datatoolkit::{DataPoint,TimeSeries};
use chrono::{Utc, TimeZone};

fn build_timeseries() -> TimeSeries<DataPoint> {
    let ticker = "Dummy";
    let dps = vec![ 
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 0, 0), 122f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 1, 0), 120f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), 118f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 3, 0), 114f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 5, 0), 116f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 4, 0), 117f64)
    ];
    TimeSeries::from_vec( ticker, dps )
}

#[test]
fn getters() {
    let ts = build_timeseries();
    // Get method
    assert_eq!( ts.at(&Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), 0).unwrap().get(), 118f64 );
    assert_eq!( ts.at(&Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), -1).unwrap().get(), 120f64 );
    assert_eq!( ts.at(&Utc.ymd(2008, 1, 1).and_hms(0, 3, 0), 1).unwrap().get(), 117f64 );
    assert_eq!( ts.at(&Utc.ymd(2008, 1, 1).and_hms(0, 6, 0), 0), None );
    // Latest range method
    let res = ts.range(-3, -1);
    assert_eq!( res[0].get(), 114f64 );
    assert_eq!( res[1].get(), 117f64 );
    // Range method
    let res = ts.range_at(&Utc.ymd(2008, 1, 1).and_hms(0, 5, 0), 2, -2);
    assert_eq!( res[0].get(), 114f64 );
    assert_eq!( res[1].get(), 117f64 );
    // Index 
    assert_eq!( ts[-1].get(), 116f64 ); // Last element
    assert_eq!( ts[0].get(), 122f64 ); // First element
}

#[test]
fn iterator() {
    let ts = build_timeseries();
    let res : Vec<DataPoint> = ts.into_iter().collect();
    assert_eq!(res.len(), 6);
    assert_eq!(res[2].get(), 118f64);
}

#[test]
fn cumsum() {
    let ts = build_timeseries();
    assert_eq!( ts.cumsum()[-1].get(), 707f64);  
}

#[test]
fn insert() {
    let mut ts = build_timeseries();
    let date = Utc.ymd(2008, 1, 1).and_hms(0, 2, 0);
    ts.insert_add( DataPoint::new( ts.get_id(), date.clone() , 5f64) );
    assert_eq!( ts.at( &date, 0 ).unwrap().get(), 123f64);  
}