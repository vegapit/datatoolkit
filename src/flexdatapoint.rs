use crate::{FlexData, FlexDataType, FlexIndex};
use crate::helper::convert;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexDataPoint {
    index: FlexIndex,
    data: FlexData
}

impl FlexDataPoint {

    pub fn new(index: FlexIndex, data: FlexData) -> Self {
        Self {
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

    pub fn get_index(&self) -> &FlexIndex {
        &self.index
    }

    pub fn set_index(&mut self, index: FlexIndex) {
        self.index = index;
    }

    pub fn apply(&mut self, f: impl Fn(&FlexData) -> FlexData) {
        self.data = f(&self.data)
    }

    pub fn as_type(&mut self, datatype: &FlexDataType) {
        self.data = convert(&self.data, datatype);
    }

}

impl PartialEq for FlexDataPoint {
    fn eq(&self, other: &FlexDataPoint) -> bool {
        self.index == other.index && self.data == other.data
    }
}

impl Eq for FlexDataPoint{}

impl PartialOrd for FlexDataPoint {
    fn partial_cmp(&self, other: &FlexDataPoint) -> Option<std::cmp::Ordering> {
        if self.index == other.index {
            Some( self.index.cmp(&other.index) )
        } else {
            None
        }
    }
}

impl Ord for FlexDataPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}