# Rust Data Analysis Toolkit

Pure Rust crate allowing the manipulation of indexed data structures like timeseries:

```rust
extern crate datatoolkit;
extern crate chrono;

use datatoolkit::{DataPoint,TimeSeries};
use chrono::{Utc, TimeZone};

let dps = vec![ 
    DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 0, 0), 122f64),
    DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 1, 0), 120f64),
    DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), 118f64),
    DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 3, 0), 114f64),
    DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 5, 0), 116f64),
    DataPoint::new(ticker, Utc.ymd(2008, 1, 1).and_hms(0, 4, 0), 117f64)
];

let ts = TimeSeries::from_vec( dps );

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
```

Please refer to the `tests` folder for more usage examples.
