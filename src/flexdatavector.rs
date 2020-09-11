use crate::{FlexIndex, FlexData};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlexDataVector {
    index: FlexIndex,
    labels: Vec<String>,
    data: Vec<FlexData>
}

impl FlexDataVector {

    pub fn new(index: FlexIndex, labels: Vec<&str>, data: Vec<FlexData>) -> Self {
        Self {
            index: index,
            labels: labels.into_iter().map(|lbl| lbl.to_string()).collect(),
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

    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn contains(&self, label: &str) -> bool {
        self.labels.iter()
            .position(|lbl| lbl == label)
            .is_some()
    }

}

impl PartialEq for FlexDataVector {
    fn eq(&self, other: &FlexDataVector) -> bool {
        self.index == other.index && self.data.iter().zip( other.data.iter() ).all(|(a,b)| a == b)
    }
}