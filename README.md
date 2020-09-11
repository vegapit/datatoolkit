# Rust Data Analysis Toolkit

Pure Rust crate allowing the manipulation of indexed data structures like timeseries:

```rust
extern crate datatoolkit;
extern crate chrono;

use datatoolkit::{DataPoint,TimeSeries};
use chrono::{Utc, TimeZone};

let dps = vec![ 
    DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 0, 0), 122),
    DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 1, 0), 120),
    DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 2, 0), 118),
    DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 3, 0), 114),
    DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 5, 0), 116),
    DataPoint::new(Utc.ymd(2008, 1, 1).and_hms(0, 4, 0), 117)
];
Series::from_vec( "Test", dps )

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
assert_eq!( ts[-1].get(), &116 ); // Last element
assert_eq!( ts[0].get(), &122 ); // First element
```

Similarly to Pandas in Python, it also handles data from multiple types thanks to flexible data structures like `FlexTable`:

```rust
let headers = vec!["Div","Date","HomeTeam","AwayTeam","FTHG","FTAG","B365H","B365D","B365A"];
let datatypes = vec![
    FlexDataType::Str,
    FlexDataType::Str,
    FlexDataType::Str,
    FlexDataType::Str,
    FlexDataType::Uint,
    FlexDataType::Uint,
    FlexDataType::Dbl,
    FlexDataType::Dbl,
    FlexDataType::Dbl
];
let table = FlexTable::from_csv("./tests/E3.csv", headers, datatypes);
println!("{}", table);
```

All data missing or not fitting the type requirements are assigned a type of `FlexDataType:NA`. Please refer to the `tests` folder for more usage examples.

Bear in mind that this library is in early development so the interface could vary over time.
