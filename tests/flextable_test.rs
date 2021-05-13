extern crate datatoolkit;
extern crate serde;

use std::convert::TryFrom;
use datatoolkit::{FlexTable, FlexData, FlexDataType};

fn create_table() -> FlexTable {
    // Pandas Equivalent:
    // df = pd.read_csv('./tests/E3.csv')
    // df = df[["Div","Date","Time","HomeTeam","AwayTeam","FTHG","FTAG","B365H","B365D","B365A"]]

    let headers = vec!["Div","Date","Time","HomeTeam","AwayTeam","FTHG","FTAG","B365H","B365D","B365A"];
    let datatypes = vec![
        FlexDataType::Str,
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
    FlexTable::from_csv("./tests/E3.csv", headers, datatypes)
}

#[test]
fn csv_import() {

    let mut table = create_table();
    assert!( table.has_na() );

    // All games where one team scored more than 3 goals
    // Pandas equivalent: df.where((df['FTHG'] > 3) | (df['FTAG'] > 3))
    let f = |x: &FlexData| x > &FlexData::Uint(3);
    table.filter_any(&["FTHG","FTAG"], f).print( Some(20) );

    // All games where no goals were scored
    // Pandas equivalent: df.where((df['FTHG'] == 0) & (df['FTAG'] == 0))
    let f = |x: &FlexData| x == &FlexData::Uint(0);
    table.filter_all(&["FTHG","FTAG"], f).print( Some(20) );

    // Create new series as function of others
    // using helper functions to condense the code
    // Pandas equivalent: df['GoalDiff'] = df['FTHG'] - df['FTAG']
    let gd_series = table["FTHG"].sub( "GoalDiff", &FlexDataType::Int, &table["FTAG"] );
    table.add_series( gd_series );
    
    // Pandas equivalent: print( df.head(10) )
    table.print( Some(10) ); // print first 10 records only

    // Pandas equivalent: print( df.iloc[24,:] )
    table.record(24).print(); // 25th row

    // Group by Hometeams
    for (k,v) in table.group_by("HomeTeam") {
        println!("{}", k);
        v.print( Some(5) );
    }

    let filtered_table = table.drop_na();
    assert!( filtered_table.has_na() == false );

    let corr = f64::try_from( &filtered_table.pearson_correlation("B365H", "B365A", true) ).unwrap();
    assert!( corr < 0.0 );

    //table.to_csv("test.csv");
}