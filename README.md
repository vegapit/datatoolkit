# Rust Data Analysis Toolkit

This crate allows the manipulation of indexed data structures like time series. Cretaing a TimeSeries struct is easy:

```rust
extern crate datatoolkit;
extern crate chrono;

use datatoolkit::{DataPoint,TimeSeries};
use chrono::{Utc, TimeZone};

let mut ts = TimeSeries::new( "Dummy", None );
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
```

Please refer to the `test` folder for more usage examples.
