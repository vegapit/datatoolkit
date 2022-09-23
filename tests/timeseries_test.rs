extern crate datatoolkit;
extern crate chrono;

use datatoolkit::{DataPoint,Series};
use chrono::{DateTime, Utc, TimeZone};

fn build_series() -> Series<DateTime<Utc>, usize> {
    let dps = vec![ 
        DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 0, 0), 122),
        DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 1, 0), 120),
        DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), 118),
        DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 3, 0), 114),
        DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 5, 0), 116),
        DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 4, 0), 117)
    ];
    Series::from_vec( "Test", dps )
}

#[test]
fn getters() {
    let ts = build_series();
    // Get method
    assert_eq!( ts.at(&Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), 0).unwrap().get(), &118 );
    assert_eq!( ts.at(&Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), -1).unwrap().get(), &120 );
    assert_eq!( ts.at(&Utc.ymd(2008, 1, 1).and_hms(0, 3, 0), 1).unwrap().get(), &117 );
    assert_eq!( ts.at(&Utc.ymd(2008, 1, 1).and_hms(0, 6, 0), 0), None );
    // Latest range method
    let res = ts.range(-3, -1);
    assert_eq!( res[0].get(), &114 );
    assert_eq!( res[1].get(), &117 );
    assert_eq!( res[2].get(), &116 );
    // Range method
    let res = ts.range_at(&Utc.ymd(2008, 1, 1).and_hms(0, 5, 0), 2, -2);
    assert_eq!( res[0].get(), &114 );
    assert_eq!( res[1].get(), &117 );
    // Index 
    assert_eq!( ts[-1i32].get(), &116 ); // Last element
    assert_eq!( ts[0i32].get(), &122 ); // First element
}

#[test]
fn iterator() {
    let ts = build_series();
    let res : Vec<DataPoint<DateTime<Utc>,usize>> = ts.into_iter().collect();
    assert_eq!(res.len(), 6);
    assert_eq!(res[2].get(), &118);
}

#[test]
fn cumsum() {
    let mut ts = build_series();
    ts = ts.cumsum();
    assert_eq!( ts[-1i32].get(), &707);
}

#[test]
fn insert() {
    let mut ts = build_series();
    let date = Utc.ymd(2008, 1, 1).and_hms(0, 2, 0);
    ts.insert_add( DataPoint::new( date.clone() , 5) );
    assert_eq!( ts.at( &date, 0 ).unwrap().get(), &123);  
}