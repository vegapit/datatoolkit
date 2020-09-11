extern crate datatoolkit;
extern crate serde;

use datatoolkit::{FlexTable, FlexData, FlexDataType};

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
    println!("{}", table.filter_any(&["FTHG","FTAG"], f));

    // All games where no goals were scored
    let f = |x: &FlexData| x == &FlexData::Uint(0);
    println!("{}", table.filter_all(&["FTHG","FTAG"], f));

    // Create new series as function of others
    let f = |xs: &[&FlexData]| {
        let mut total = 0f64;
        for x in xs.iter() {
            match x {
                FlexData::Dbl(val) => {
                    if val != &0f64 {
                        total += 1.0 / val;
                    } else {
                        return FlexData::NA;
                    }
                },
                _ => { return FlexData::NA; }
            }
        }
        FlexData::Dbl( total )
    };
    let mut new_series = table.nary_apply(
        "B365Back",
        FlexDataType::Dbl,
        &["B365H","B365D","B365A"],
        f
    );
    println!("{}", new_series);

    println!("{}", table.record(24)); // 25th row
    println!("{:?}", table["Date"][-1]); // Last datapoint of Series["Date"]
    
}