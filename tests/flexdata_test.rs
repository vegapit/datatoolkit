extern crate datatoolkit;

use datatoolkit::FlexData;
use std::convert::TryFrom;

#[test]
fn into_from() {
    let flexdata_string = FlexData::from( String::from("datatoolkit") );
    let flexdata_float = FlexData::from( 1f64 );

    let s = String::try_from( &flexdata_string ).unwrap();
    let d = f64::try_from( &flexdata_float ).unwrap();

    assert_eq!( d, 1f64);
    assert_eq!( s, String::from("datatoolkit"));
}

#[test]
fn operators() {
    // Int
    assert_eq!( FlexData::from( 3 ), &FlexData::from( -5 ) + &FlexData::from( 8 ) );
    assert_eq!( FlexData::from( 3 ), &FlexData::from( 8 ) - &FlexData::from( 5 ) );
    assert_eq!( FlexData::from( 15 ), &FlexData::from( 5 ) * &FlexData::from( 3 ) );
    assert_eq!( FlexData::from( 3 ), &FlexData::from( 15 ) / &FlexData::from( 5 ) );

    let mut i = FlexData::from( 1 );
    i += FlexData::from( 5 );
    assert_eq!( i, FlexData::from( 6 ) );

    // Uint
    assert_eq!( FlexData::from( 8 ), &FlexData::from( 5 ) + &FlexData::from( 3 ) );
    assert_eq!( FlexData::from( 8 ), &FlexData::from( 15 ) - &FlexData::from( 7 ) );
    assert_eq!( FlexData::from( 8 ), &FlexData::from( 2 ) * &FlexData::from( 4 ) );
    assert_eq!( FlexData::from( 8 ), &FlexData::from( 32 ) / &FlexData::from( 4 ) );

    let mut u = FlexData::from( 1 );
    u += FlexData::from( 5 );
    assert_eq!( u, FlexData::from( 6 ) );
}