extern crate datatoolkit;
extern crate chrono;

use datatoolkit::DataPoint;
use chrono::{Utc, TimeZone};

#[test]
fn datapoint_operations() {
    let dt1 = Utc.with_ymd_and_hms(2007, 3, 31, 23, 59, 59).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2007, 4, 1, 0, 0, 0).unwrap();
    
    let r1 = DataPoint::new(dt1, 10f64);
    let r2 = DataPoint::new(dt1, 5f64);
    let mut r3 = DataPoint::new(dt2, 5f64);

    assert!(r1 != r2);
    assert!(r2 != r3);

    assert_eq!( (&r1 + &r2).unwrap().get(), &15f64 );
    assert_eq!( (&r1 - &r2).unwrap().get(), &5f64 );
    assert_eq!( (&r1 * &r2).unwrap().get(), &50f64 );
    assert_eq!( (&r1 / &r2).unwrap().get(), &2f64 );
    assert_eq!( &r1 + &r3, None );

    r3.apply(|&x: &f64| x.powf(2f64));
    assert_eq!( r3.get(), &25f64 );
}