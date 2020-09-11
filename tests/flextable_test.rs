extern crate datatoolkit;
extern crate serde;

use datatoolkit::{FlexTable, FlexData, FlexDataType};
use datatoolkit::helper::{inverse, sum};

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
    
    assert!( table.has_na() );

    // All games where one team scored more than 3 goals
    let f = |x: &FlexData| x > &FlexData::Uint(3);
    table.filter_any(&["FTHG","FTAG"], f).print();

    // All games where no goals were scored
    let f = |x: &FlexData| x == &FlexData::Uint(0);
    table.filter_all(&["FTHG","FTAG"], f).print();

    // Create new series as function of others
    // using helper functions to condense the code
    let f = |xs: &[&FlexData]| {
        let v : Vec<FlexData> = xs.iter()
            .map(|x| inverse(x))
            .collect();
        sum(v)
    };
    let new_series = table.nary_apply(
        "B365Back",
        FlexDataType::Dbl,
        &["B365H","B365D","B365A"],
        f
    );
    new_series.print();

    table.record(24).print(); // 25th row
    println!("{:?}", table["Date"][-1]); // Last datapoint of Series["Date"]
}