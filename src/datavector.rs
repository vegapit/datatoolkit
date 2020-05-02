use std::cmp::Ordering;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::HistoricalData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataVector {
    pub id: String,
    pub timesignature: DateTime<Utc>,
    pub data: HashMap<String,f64>
}

impl DataVector {

    pub fn new(id: &str, timesignature: DateTime<Utc>, data: HashMap<String,f64>) -> DataVector {
        DataVector{
            id: id.to_string(),
            timesignature: timesignature,
            data: data.clone()
        }
    }

    pub fn get(&self, label: &str) -> Option<f64> {
        self.data.get(label).cloned()
    }

    pub fn set(&mut self, label: &str, value: f64) {
        self.data.insert(label.to_string(), value);
    }

    pub fn labels(&self) -> Vec<String> {
        self.data.keys().cloned().collect::<Vec<String>>()
    }

    pub fn contains(&self, label: &str) -> bool {
        self.data.contains_key(label)
    }

}

impl HistoricalData for DataVector{
    fn id(&self) -> String{
        self.id.clone()
    }
    fn timesignature(&self) -> DateTime<Utc> {
        self.timesignature
    }
}

impl Ord for DataVector {
    fn cmp(&self, other: &DataVector) -> Ordering {
        self.timesignature.cmp( &other.timesignature )
    }
}

impl PartialOrd for DataVector {
    fn partial_cmp(&self, other: &DataVector) -> Option<Ordering> {
        if self.id == other.id {
            Some( self.timesignature.cmp( &other.timesignature ) )
        } else {
            None
        }
    }
}

impl PartialEq for DataVector {
    fn eq(&self, other: &DataVector) -> bool {
        for (label, value) in &self.data {
            if other.contains( label.as_str() ) {
                return false;
            }
            if other.get( label.as_str() ).as_ref() != Some( value ) {
                return false;
            }
        }
        self.id == other.id && self.timesignature == other.timesignature
    }
}

impl Eq for DataVector {}