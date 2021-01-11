extern crate datatoolkit;
extern crate serde;

use datatoolkit::{FlexTable, FlexData, FlexDataType};

#[test]
fn csv_import() {

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
    let mut table = FlexTable::from_csv("./tests/E3.csv", headers, datatypes);
    
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

    //table.to_csv("test.csv");
}