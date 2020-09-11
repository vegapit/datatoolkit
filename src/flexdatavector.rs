use std::ops::*;
use crate::{FlexIndex, FlexData, FlexDataType};
use prettytable::{Table, Row, Cell};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexDataVector {
    index: FlexIndex,
    labels: Vec<String>,
    datatypes: Vec<FlexDataType>,
    data: Vec<FlexData>
}

impl FlexDataVector {

    pub fn new(index: FlexIndex, labels: Vec<&str>, datatypes: Vec<FlexDataType>, data: Vec<FlexData>) -> Self {
        Self {
            index: index,
            labels: labels.into_iter().map(|lbl| lbl.to_string()).collect(),
            datatypes: datatypes,
            data: data
        }
    }

    pub fn get_index(&self) -> &FlexIndex {
        &self.index
    }

    pub fn set_index(&mut self, index: FlexIndex) {
        self.index = index;
    }

    pub fn get(&self, label: &str) -> Option<&FlexData> {
        self.labels.iter()
            .position(|lbl| lbl == label)
            .map(|i| &self.data[i])
    }

    pub fn set(&mut self, label:&str, data: FlexData) {
        if let Some(i) = self.labels.iter().position(|lbl| lbl == label) {
            self.data[i] = data;
        }
    }

    pub fn get_labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn get_datatypes(&self) -> &Vec<FlexDataType> {
        &self.datatypes
    }

    pub fn get_size(&self) -> usize {
        self.labels.len()
    }

    pub fn verify(&self, f: impl Fn(&FlexData) -> bool) -> bool {
        self.data.iter()
            .all(|fd| f(fd))
    }

    pub fn has_na(&self) -> bool {
        !self.verify(|x: &FlexData| x != &FlexData::NA)
    }

    pub fn contains(&self, label: &str) -> bool {
        self.labels.iter()
            .position(|lbl| lbl == label)
            .is_some()
    }

    pub fn print(&self) {
        let mut table = Table::new();
        let mut headers_cells : Vec<Cell> = self.labels.iter()
            .map(|h| Cell::new(h))
            .collect();
        headers_cells.insert(0, Cell::new(""));
        table.add_row(Row::new(headers_cells));
        let mut types_cells : Vec<Cell> = self.get_datatypes().iter()
            .map(|datatype| {
                match datatype {
                    FlexDataType::Dbl => Cell::new("f64"),
                    FlexDataType::Uint => Cell::new("u32"),
                    FlexDataType::Int => Cell::new("i64"),
                    FlexDataType::Char => Cell::new("char"),
                    FlexDataType::Str => Cell::new("str"),
                    FlexDataType::NA => Cell::new("n/a")
                }
            })
            .collect();
        types_cells.insert(0, Cell::new(""));
        table.add_row(Row::new(types_cells));
        let mut record_cells : Vec<Cell> = Vec::new();
        let index_cell = match &self.index {
            FlexIndex::Uint(val) => Cell::new( format!("{}", val).as_str() ),
            FlexIndex::Str(val) => Cell::new( val.as_str() )
        };
        record_cells.push(index_cell);
        for k in 0..self.get_size() {
            let cell = match &self.data[k] {
                FlexData::Str(val) => Cell::new( val.as_str() ),
                FlexData::Dbl(val) => Cell::new( format!("{:.5}", val).as_str() ),
                FlexData::Uint(val) => Cell::new( format!("{}", val).as_str() ),
                FlexData::Int(val) => Cell::new( format!("{}", val).as_str() ),
                FlexData::Char(val) => Cell::new( format!("{}", val).as_str() ),
                FlexData::NA => Cell::new( "N/A" )
            };
            record_cells.push(cell);
        }
        table.add_row(Row::new(record_cells));
        // Print the table to stdout
        table.printstd();
    }
}

// Implement [] operator
impl Index<usize> for FlexDataVector {
    type Output = FlexData;
    fn index<'a>(&'a self, index: usize) -> &'a FlexData {
        &self.data[index]
    }
}

impl PartialEq for FlexDataVector {
    fn eq(&self, other: &FlexDataVector) -> bool {
        self.index == other.index && self.data.iter().zip( other.data.iter() ).all(|(a,b)| a == b)
    }
}

impl Eq for FlexDataVector{}