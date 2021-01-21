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