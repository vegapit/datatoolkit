use std::cmp::Ordering;
use std::ops::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPoint<T,U> {
    index: T,
    data: U
}

impl<T,U> DataPoint<T,U> {
    pub fn new(index: T, data: U) -> DataPoint<T,U> {
        DataPoint{ data, index }
    }
    
    pub fn get_index(&self) -> &T {
        &self.index
    }
    
    pub fn get(&self) -> &U {
        &self.data
    }

    pub fn set(&mut self, value: U) {
        self.data = value;
    }

    pub fn apply(&mut self, f: impl Fn(&U) -> U) {
        self.data = f(&self.data);
    }
}

impl<T: Ord,U: PartialOrd> Ord for DataPoint<T,U> {
    fn cmp(&self, other: &DataPoint<T,U>) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl<T: Ord,U: PartialOrd> PartialOrd for DataPoint<T,U> {
    fn partial_cmp(&self, other: &DataPoint<T,U>) -> Option<Ordering> {
        Some( self.index.cmp(&other.index) )
    }
}

impl<T: Ord,U: PartialOrd> PartialEq for DataPoint<T,U> {
    fn eq(&self, other: &DataPoint<T,U>) -> bool {
        self.index == other.index && self.data == other.data
    }
}

impl<T: Ord,U: PartialOrd> Eq for DataPoint<T,U> {}

impl<T: Ord + Clone,U: PartialOrd + AddAssign + Copy> Add<&DataPoint<T,U>> for &DataPoint<T,U> {
    type Output = Option<DataPoint<T,U>>;

    fn add(self, other: &DataPoint<T,U>) -> Self::Output {
        if self.index == other.index {
            let mut dp = self.clone();
            dp.data += other.data;
            Some( dp )
        } else {
            None
        }
    }
}

impl<T: Ord + Clone,U: PartialOrd + SubAssign + Copy> Sub<&DataPoint<T,U>> for &DataPoint<T,U> {
    type Output = Option<DataPoint<T,U>>;

    fn sub(self, other: &DataPoint<T,U>) -> Self::Output {
        if self.index == other.index {
            let mut dp = self.clone();
            dp.data -= other.data;
            Some( dp )
        } else {
            None
        }
    }
}

impl<T: Ord + Clone,U: PartialOrd + MulAssign + Copy> Mul<&DataPoint<T,U>> for &DataPoint<T,U> {
    type Output = Option<DataPoint<T,U>>;

    fn mul(self, other: &DataPoint<T,U>) -> Self::Output {
        if self.index == other.index {
            let mut dp = self.clone();
            dp.data *= other.data;
            Some( dp )
        } else {
            None
        }
    }
}

impl<T: Ord + Clone,U: PartialOrd + DivAssign + Copy> Div<&DataPoint<T,U>> for &DataPoint<T,U> {
    type Output = Option<DataPoint<T,U>>;

    fn div(self, other: &DataPoint<T,U>) -> Self::Output {
        if self.index == other.index {
            let mut dp = self.clone();
            dp.data /= other.data;
            Some( dp )
        } else {
            None
        }
    }
}