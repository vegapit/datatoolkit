extern crate datatoolkit;

use datatoolkit::{FlexDataType, FlexSeries, FlexDataPoint, FlexData, FlexIndex};
use std::convert::TryFrom;

fn make_double_series1() -> FlexSeries {
    let datapoints = vec![
        FlexDataPoint::new(FlexIndex::Uint(1), FlexData::Dbl(2.5)),
        FlexDataPoint::new(FlexIndex::Uint(2), FlexData::Dbl(1.2)),
        FlexDataPoint::new(FlexIndex::Uint(3), FlexData::Dbl(3.6)),
        FlexDataPoint::new(FlexIndex::Uint(4), FlexData::Dbl(0.1)),
        FlexDataPoint::new(FlexIndex::Uint(5), FlexData::Dbl(0.7)),
        FlexDataPoint::new(FlexIndex::Uint(6), FlexData::Dbl(1.8)),
        FlexDataPoint::new(FlexIndex::Uint(7), FlexData::Dbl(2.7)),
        FlexDataPoint::new(FlexIndex::Uint(8), FlexData::Dbl(2.9)),
        FlexDataPoint::new(FlexIndex::Uint(9), FlexData::Dbl(1.4)),
        FlexDataPoint::new(FlexIndex::Uint(10), FlexData::Dbl(0.3))
    ];
    FlexSeries::from_vec("dummy1", FlexDataType::Dbl, datapoints)
}

fn make_double_series2() -> FlexSeries {
    let datapoints = vec![
        FlexDataPoint::new(FlexIndex::Uint(1), FlexData::Dbl(1.5)),
        FlexDataPoint::new(FlexIndex::Uint(2), FlexData::Dbl(2.2)),
        FlexDataPoint::new(FlexIndex::Uint(3), FlexData::Dbl(0.6)),
        FlexDataPoint::new(FlexIndex::Uint(4), FlexData::Dbl(3.1)),
        FlexDataPoint::new(FlexIndex::Uint(5), FlexData::Dbl(3.7)),
        FlexDataPoint::new(FlexIndex::Uint(6), FlexData::Dbl(2.8)),
        FlexDataPoint::new(FlexIndex::Uint(7), FlexData::Dbl(1.7)),
        FlexDataPoint::new(FlexIndex::Uint(8), FlexData::Dbl(1.9)),
        FlexDataPoint::new(FlexIndex::Uint(9), FlexData::Dbl(2.4)),
        FlexDataPoint::new(FlexIndex::Uint(10), FlexData::Dbl(3.3))
    ];
    FlexSeries::from_vec("dummy2", FlexDataType::Dbl, datapoints)
}

#[test]
fn stats() {
    let series1 = make_double_series1();
    let series2 = make_double_series2();

    let m1 = f64::try_from( &series1.mean() ).unwrap();
    assert!( (1.72f64 - m1).abs() < 1e-5 );

    let v1 = f64::try_from( &series1.variance(true) ).unwrap();
    assert!( (1.3951f64 - v1).abs() < 1e-4 );

    let m2 = f64::try_from( &series2.mean() ).unwrap();
    assert!( (2.32f64 - m2).abs() < 1e-5 );

    let v2 = f64::try_from( &series2.variance(true) ).unwrap();
    assert!( (0.8795f64 - v2).abs() < 1e-4 );

    let cov = f64::try_from( &series1.covariance( &series2, true ) ).unwrap();
    assert!( (-0.996f64 - cov).abs() < 1e-4 );

    let corr = f64::try_from( &series1.pearson_correlation( &series2, true ) ).unwrap();
    println!("{:?}", corr);
    assert!( (-0.8991f64 - corr).abs() < 1e-4 );
}