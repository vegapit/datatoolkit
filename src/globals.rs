use chrono::{DateTime, Utc};

pub trait HistoricalData{
    fn id(&self) -> String;
    fn timesignature(&self) -> DateTime<Utc>;
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(tag="t",content="c")]
pub enum Frequency{
    Days(u32),
    Hours(u32),
    Minutes(u32)
}