extern crate datatoolkit;

use datatoolkit::{DataPoint,Series};

fn build_series() -> Series<usize, char> {
    let dps = vec![ 
        DataPoint::new(0, 'v'),
        DataPoint::new(1, 'e'),
        DataPoint::new(2, 'g'),
        DataPoint::new(3, 'a'),
        DataPoint::new(4, 'p'),
        DataPoint::new(5, 'i'),
        DataPoint::new(6, 't')
    ];
    Series::from_vec( "Test", dps )
}

#[test]
fn getters() {
    let ts = build_series();
    // Get method
    assert_eq!( ts.at(&2, 0).unwrap().get(), &'g' );
    assert_eq!( ts.at(&2, -1).unwrap().get(), &'e' );
    assert_eq!( ts.at(&3, 1).unwrap().get(), &'p' );
    assert_eq!( ts.at(&7, 0), None );
    // Latest range method
    let res = ts.range(-3, -1);
    assert_eq!( res[0].get(), &'p' );
    assert_eq!( res[1].get(), &'i' );
    assert_eq!( res[2].get(), &'t' );
    // Range method
    let res = ts.range_at(&5, 2, -2);
    assert_eq!( res[0].get(), &'a' );
    assert_eq!( res[1].get(), &'p' );
    // Index 
    assert_eq!( ts[-1i32].get(), &'t' ); // Last element
    assert_eq!( ts[0i32].get(), &'v' ); // First element
}

#[test]
fn iterator() {
    let ts = build_series();
    let res : Vec<DataPoint<usize,char>> = ts.into_iter().collect();
    assert_eq!(res.len(), 7);
    assert_eq!(res[2].get(), &'g');
}