use crate::FlexData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexDataPoint<T> {
    id: String,
    index: T,
    data: FlexData
}

impl<T> FlexDataPoint<T> {

    pub fn new(id: &str, index: T, data: FlexData) -> Self {
        Self {
            id: id.to_string(),
            index: index,
            data: data
        }
    }

    pub fn get(&self) -> &FlexData {
        &self.data
    }

    pub fn set(&mut self, data: FlexData) {
        self.data = data
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
}