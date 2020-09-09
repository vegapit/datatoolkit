use std::cmp::Ordering;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::HistoricalData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataVector {
    id: String,
    time_signature: DateTime<Utc>,
    data: HashMap<String,f64>
}

impl DataVector {

    pub fn new(id: &str, time_signature: DateTime<Utc>, data: HashMap<String,f64>) -> DataVector {
        DataVector{
            id: id.to_string(),
            time_signature: time_signature,
            data: data
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
    fn get_id(&self) -> &str {
        self.id.as_str()
    }
    fn get_time_signature(&self) -> &DateTime<Utc> {
        &self.time_signature
    }
}

impl Ord for DataVector {
    fn cmp(&self, other: &DataVector) -> Ordering {
        self.time_signature.cmp( &other.time_signature )
    }
}

impl PartialOrd for DataVector {
    fn partial_cmp(&self, other: &DataVector) -> Option<Ordering> {
        if self.id == other.id {
            Some( self.time_signature.cmp( &other.time_signature ) )
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
        self.id == other.id && self.time_signature == other.time_signature
    }
}

impl Eq for DataVector {}