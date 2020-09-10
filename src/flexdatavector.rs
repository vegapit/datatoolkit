use std::cmp::Ordering;
use std::cmp::Ord;
use crate::FlexDataPoint;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexDataVector<T> {
    index: T,
    data: Vec<FlexDataPoint<T>>
}

impl<T: Clone> FlexDataVector<T> {

    pub fn new(index: T, data: Vec<FlexDataPoint<T>>) -> Self {
        Self {
            index: index,
            data: data
        }
    }

    pub fn get_index(&self) -> &T {
        &self.index
    }

    pub fn get(&self, id: &str) -> Option<&FlexDataPoint<T>> {
        self.data.iter().find(|&fdp| fdp.get_id() == id)
    }

    pub fn set(&mut self, dp: FlexDataPoint<T>) {
        self.data.push(dp);
    }

    pub fn labels(&self) -> Vec<String> {
        self.data.iter().map(|fdp| fdp.get_id().to_string()).collect::<Vec<String>>()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.data.iter().position(|fdp| fdp.get_id() == id).is_some()
    }

}

impl<T: PartialEq> PartialEq for FlexDataVector<T> {
    fn eq(&self, other: &FlexDataVector<T>) -> bool {
        self.index == other.index
    }
}

impl<T: Ord> PartialOrd for FlexDataVector<T> {
    fn partial_cmp(&self, other: &FlexDataVector<T>) -> Option<Ordering> {
        Some( self.index.cmp(&other.index) )
    }
}