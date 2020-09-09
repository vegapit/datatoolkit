use std::cmp::Ordering;
use std::ops;
use chrono::{DateTime, Utc};
use crate::HistoricalData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPoint {
    id: String,
    time_signature: DateTime<Utc>,
    data: f64
}

impl DataPoint {
    pub fn new(id: &str, time_signature: DateTime<Utc>, data: f64) -> DataPoint {
        DataPoint{
            id: id.to_string(),
            data: data, 
            time_signature: time_signature
        }
    }

    pub fn get(&self) -> f64 {
        self.data
    }

    pub fn set(&mut self, value: f64) {
        self.data = value;
    }

    pub fn apply(&mut self, f: impl Fn(f64) -> f64) {
        self.data = f(self.data);
    }
}

impl HistoricalData for DataPoint{
    fn get_id(&self) -> &str {
        self.id.as_str()
    }
    fn get_time_signature(&self) -> &DateTime<Utc> {
        &self.time_signature
    }
}

impl Ord for DataPoint {
    fn cmp(&self, other: &DataPoint) -> Ordering {
        self.time_signature.cmp(&other.time_signature)
    }
}

impl PartialOrd for DataPoint {
    fn partial_cmp(&self, other: &DataPoint) -> Option<Ordering> {
        if self.id == other.id {
            Some( self.time_signature.cmp(&other.time_signature) )
        } else {
            None
        }
    }
}

impl PartialEq for DataPoint {
    fn eq(&self, other: &DataPoint) -> bool {
        self.id == other.id && self.time_signature == other.time_signature && self.data == other.data
    }
}

impl Eq for DataPoint {}

impl ops::Add<&DataPoint> for &DataPoint {
    type Output = Option<DataPoint>;

    fn add(self, other: &DataPoint) -> Self::Output {
        let mut dp = self.clone();
        if dp.time_signature == other.time_signature && self.id == other.id {
            dp.data += other.data;
            Some( dp )
        } else {
            None
        }
    }
}

impl ops::Sub<&DataPoint> for &DataPoint {
    type Output = Option<DataPoint>;

    fn sub(self, other: &DataPoint) -> Self::Output {
        let mut dp = self.clone();
        if dp.time_signature == other.time_signature && self.id == other.id {
            dp.data -= other.data;
            Some( dp )
        } else {
            None
        }
    }
}

impl ops::Mul<&DataPoint> for &DataPoint {
    type Output = Option<DataPoint>;

    fn mul(self, other: &DataPoint) -> Self::Output {
        let mut dp = self.clone();
        if dp.time_signature == other.time_signature && self.id == other.id {
            dp.data *= other.data;
            Some( dp )
        } else {
            None
        }
    }
}

impl ops::Div<&DataPoint> for &DataPoint {
    type Output = Option<DataPoint>;

    fn div(self, other: &DataPoint) -> Self::Output {
        let mut dp = self.clone();
        if dp.time_signature == other.time_signature && self.id == other.id && other.data != 0f64 {
            dp.data /= other.data;
            Some( dp )
        } else {
            None
        }
    }
}