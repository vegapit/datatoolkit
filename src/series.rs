use crate::DataPoint;
use std::ops::*;
use std::convert::From;

#[derive(Debug, Clone)]
pub struct Series<T,U> {
    id: String,
    data: Vec<DataPoint<T,U>>,
    opt_max_size: Option<usize>,
    counter: usize
}

impl<T: Ord + Clone,U: PartialOrd + Clone> Series<T,U> {

    pub fn new(id: &str, opt_max_size: Option<usize>) -> Series<T,U> {
        Series{
            id: id.to_string(),
            data: Vec::<DataPoint<T,U>>::new(),
            opt_max_size: opt_max_size,
            counter: 0
        }
    }

    pub fn from_vec(id: &str, items: Vec<DataPoint<T,U>>) -> Series<T,U> {
        let mut ts = Series::new( id, None );
        for item in items {
            ts.insert_update( item );
        }
        ts
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    /// Insert item or update if it already exists
    pub fn insert_update(&mut self, item: DataPoint<T,U>) {
        if let Some(k) = self.data.iter().position(|x| x.get_index() == item.get_index() ) {
            self.data[k] = item;
        } else {
            self.data.push(item);
            self.data.sort();
            if let Some( maxsize ) = self.opt_max_size {
                if self.data.len() > maxsize {
                    self.data.remove(0);
                }
            }
        }
    }

    /// Get item corresponding to timesignature
    pub fn at(&self, index: &T, offset: i32) -> Option<DataPoint<T,U>> {
        let optpos = self.data.iter().rposition(|x| x.get_index() == index);
        match optpos {
            Some( pos ) => {
                if offset > 0{
                    let newpos = pos + (offset as usize);
                    if newpos < self.data.len() {
                        Some( self.data[newpos].clone() )
                    } else {
                        None
                    }
                } else {
                    if pos >= (-offset as usize) {
                        let newpos = pos - (-offset as usize);
                        Some( self.data[newpos].clone() )
                    } else {
                        None
                    }
                }
            },
            None => None
        }
    }

    /// Get range with reference timesignature and startoffset
    pub fn range_at(&self, index: &T, size: usize, offset: i32) -> Vec<DataPoint<T,U>> {
        let mut res : Vec<DataPoint<T,U>> = Vec::new();
        for i in 0..size {
            if let Some( data ) = self.at(index, offset + (i as i32)) {
                res.push( data );
            }
        }
        res
    }

    /// Get range from start to end inclusive
    pub fn range(&self, start: i32, end: i32) -> Vec<DataPoint<T,U>> {
        let is : usize = if start >= 0 { start as usize } else { self.data.len() - start.abs() as usize };
        let ie : usize = if end >= 0 { end as usize } else { self.data.len() - end.abs() as usize };
        let mut res : Vec<DataPoint<T,U>> = Vec::new();
        for i in is..=ie {
            res.push( self[i].clone() )
        }
        res
    }

}

impl<T: Ord + Clone,U: PartialOrd + Add<Output=U> + Copy> Series<T,U> {

    /// Insert item or add data if it exists
    pub fn insert_add(&mut self, item: DataPoint<T,U>) {
        if let Some(k) = self.data.iter().position(|x| x.get_index() == item.get_index() ) {
            let val = self.data[k].get().to_owned();
            self.data[k].set( val + *item.get() );
        } else {
            self.data.push(item);
            self.data.sort();
            if let Some( maxsize ) = self.opt_max_size {
                if self.data.len() > maxsize {
                    self.data.remove(0);
                }
            }
        }
    }

}

impl<T: Ord + Clone,U: PartialOrd + AddAssign + From<u8> + Copy> Series<T,U> {

    /// Create the Series of its cumulated sum
    pub fn cumsum(&self) -> Series<T,U> {
        let mut running_total : U = 0.into();
        let mut ts = Series::<T,U>::new( self.id.as_str(), self.opt_max_size );
        for mut dp in self.clone().into_iter() {
            running_total += *dp.get();
            dp.set( running_total );
            ts.insert_update( dp );
        }
        ts
    }

}

// Implement [] operator
impl<T,U> Index<i32> for Series<T,U> {
    type Output = DataPoint<T,U>;
    fn index<'a>(&'a self, index: i32) -> &'a DataPoint<T,U> {
        if index >= 0 {
            &self.data[index as usize]
        } else {
            &self.data[self.data.len() - (-index as usize)]
        }
    }
}

impl<T,U> Index<usize> for Series<T,U> {
    type Output = DataPoint<T,U>;
    fn index<'a>(&'a self, index: usize) -> &'a DataPoint<T,U> {
        &self.data[index]
    }
}


impl<T: Clone,U: Clone> Iterator for Series<T,U> {
    type Item = DataPoint<T,U>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.data.len() {
            self.counter += 1;
            Some( self.data[self.counter - 1].clone() )
        } else {
            None
        }
    }
}