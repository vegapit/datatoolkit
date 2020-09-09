use chrono::{DateTime, Utc};

pub trait HistoricalData{
    fn get_id(&self) -> &str;
    fn get_time_signature(&self) -> &DateTime<Utc>;
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(tag="t",content="c")]
pub enum Frequency{
    Days(u32),
    Hours(u32),
    Minutes(u32)
}