extern crate datatoolkit;
extern crate serde;

use datatoolkit::{FlexTable, FlexDataType};

#[test]
fn csv_import() {
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
}