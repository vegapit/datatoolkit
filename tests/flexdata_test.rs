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
    assert_eq!( FlexData::from( 3i64 ), &FlexData::from( -5i64 ) + &FlexData::from( 8i64 ) );
    assert_eq!( FlexData::from( 3i64 ), &FlexData::from( 8i64 ) - &FlexData::from( 5i64 ) );
    assert_eq!( FlexData::from( 15i64 ), &FlexData::from( 5i64 ) * &FlexData::from( 3i64 ) );
    assert_eq!( FlexData::from( 3i64 ), &FlexData::from( 15i64 ) / &FlexData::from( 5i64 ) );

    let mut i = FlexData::from( 1i64 );
    i += FlexData::from( 5i64 );
    assert_eq!( i, FlexData::from( 6i64 ) );

    // Uint
    assert_eq!( FlexData::from( 8u32 ), &FlexData::from( 5u32 ) + &FlexData::from( 3u32 ) );
    assert_eq!( FlexData::from( 8u32 ), &FlexData::from( 15u32 ) - &FlexData::from( 7u32 ) );
    assert_eq!( FlexData::from( 8u32 ), &FlexData::from( 2u32 ) * &FlexData::from( 4u32 ) );
    assert_eq!( FlexData::from( 8u32 ), &FlexData::from( 32u32 ) / &FlexData::from( 4u32 ) );

    let mut u = FlexData::from( 1u32 );
    u += FlexData::from( 5u32 );
    assert_eq!( u, FlexData::from( 6u32 ) );
}