use std::cmp::Ordering;
use std::ops;
use chrono::{DateTime, Utc};
use crate::HistoricalData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPoint {
    pub id: String,
    pub timesignature: DateTime<Utc>,
    pub data: f64
}

impl DataPoint {
    pub fn new(id: &str, timesignature: DateTime<Utc>, data: f64) -> DataPoint {
        DataPoint{
            id: id.to_string(),
            data: data, 
            timesignature: timesignature
        }
    }
}

impl HistoricalData for DataPoint{
    fn id(&self) -> String {
        self.id.clone()
    }
    fn timesignature(&self) -> DateTime<Utc> {
        self.timesignature
    }
}

impl Ord for DataPoint {
    fn cmp(&self, other: &DataPoint) -> Ordering {
        self.timesignature.cmp(&other.timesignature)
    }
}

impl PartialOrd for DataPoint {
    fn partial_cmp(&self, other: &DataPoint) -> Option<Ordering> {
        if self.id == other.id {
            Some( self.timesignature.cmp(&other.timesignature) )
        } else {
            None
        }
    }
}

impl PartialEq for DataPoint {
    fn eq(&self, other: &DataPoint) -> bool {
        self.id == other.id && self.timesignature == other.timesignature && self.data == other.data
    }
}

impl Eq for DataPoint {}

impl ops::Add<DataPoint> for DataPoint {
    type Output = Option<DataPoint>;

    fn add(self, other: DataPoint) -> Self::Output {
        let mut dp = self.clone();
        if dp.timesignature == other.timesignature && self.id == other.id {
            dp.data += other.data;
            Some( dp )
        } else {
            None
        }
    }
}