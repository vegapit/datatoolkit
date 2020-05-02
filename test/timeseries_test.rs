extern crate datatoolkit;
extern crate chrono;

use datatoolkit::{DataPoint,TimeSeries};
use chrono::{Utc, TimeZone};

fn build_timeseries() -> TimeSeries<DataPoint> {
    let ticker = "Dummy";
    let mut ts = TimeSeries::new( ticker, None );
    let dps = vec![ 
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 0, 0), 122f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 1, 0), 120f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), 118f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 3, 0), 114f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 5, 0), 116f64),
        DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 4, 0), 117f64)
    ];
    for dp in dps {
        ts.insert_update( dp );
    }
    ts
}

#[test]
fn getters() {
    let ts = build_timeseries();
    // Last method
    assert_eq!( ts.previous(0).unwrap().data, 116f64);
    // Get method
    assert_eq!( ts.get(&Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), 0).unwrap().data, 118f64 );
    assert_eq!( ts.get(&Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), -1).unwrap().data, 120f64 );
    assert_eq!( ts.get(&Utc.ymd(2008, 1, 1).and_hms(0, 3, 0), 1).unwrap().data, 117f64 );
    assert_eq!( ts.get(&Utc.ymd(2008, 1, 1).and_hms(0, 6, 0), 0), None );
    // Latest range method
    let res = ts.previous_range(2, 1).unwrap();
    assert_eq!( res[0].data, 114f64 );
    assert_eq!( res[1].data, 117f64 );
    // Range method
    let res = ts.range(&Utc.ymd(2008, 1, 1).and_hms(0, 5, 0), 2, -2).unwrap();
    assert_eq!( res[0].data, 114f64 );
    assert_eq!( res[1].data, 117f64 );
}

#[test]
fn iterator() {
    let ts = build_timeseries();
    let res : Vec<DataPoint> = ts.into_iter().collect();
    assert_eq!(res.len(), 6);
    assert_eq!(res[2].data, 118f64);
}

#[test]
fn cumsum() {
    let mut ts = build_timeseries();
    ts = ts.cumsum();
    assert_eq!( ts.last().unwrap().data, 707f64);  
}

#[test]
fn insert() {
    let mut ts = build_timeseries();
    let date = Utc.ymd(2008, 1, 1).and_hms(0, 2, 0);
    ts.insert_add( DataPoint::new( ts.id.as_str(), date.clone() , 5f64) );
    assert_eq!( ts.get( &date, 0 ).unwrap().data, 123f64);  
}