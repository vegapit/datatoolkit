extern crate datatoolkit;
extern crate chrono;

use datatoolkit::{DataPoint};
use chrono::{Utc, TimeZone};

#[test]
fn datapoint_operations() {
    let dt1 = Utc.ymd(2007, 3, 31).and_hms(23, 59, 59);
    let dt2 = Utc.ymd(2007, 4, 1).and_hms(0, 0, 0);
    let ticker = "Dummy";
    
    let r1 = DataPoint::new(ticker, dt1, 10f64);
    let r2 = DataPoint::new(ticker, dt1, 5f64);
    let r3 = DataPoint::new(ticker, dt2, 5f64);

    assert!(r1 != r2);
    assert!(r2 != r3);
}