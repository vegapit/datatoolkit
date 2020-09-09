use chrono::{DateTime, Utc};
use crate::HistoricalData;
use crate::DataPoint;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct TimeSeries<T> {
    id: String,
    index: Vec<DateTime<Utc>>,
    data: Vec<T>,
    optmaxsize: Option<usize>,
    counter: usize
}

impl TimeSeries<DataPoint> {

    /// Insert item or add data if it exists
    pub fn insert_add(&mut self, item: DataPoint) {
        if self.id.as_str() == item.get_id() {
            if let Some(k) = self.index.iter().position(|&x| &x == item.get_time_signature() ) {
                let val = self.data[k].get();
                self.data[k].set( val + item.get() );
            } else {
                self.data.push(item);
                self.data.sort();
                if let Some( maxsize ) = self.optmaxsize {
                    if self.data.len() > maxsize {
                        self.data.remove(0);
                    }
                }
                self.index = self.data.iter().map(|e| e.get_time_signature()).cloned().collect();
            }
        }
    }

    /// Create the Timeseries of its cumulated sum
    pub fn cumsum(&self) -> TimeSeries<DataPoint> {
        let mut running_total = 0f64;
        let mut ts = TimeSeries::new( self.id.as_str(), self.optmaxsize );
        for mut dp in self.clone().into_iter() {
            running_total += dp.get();
            dp.set( running_total );
            ts.insert_update( dp );
        }
        ts
    }

}

impl<T: Clone + Ord + HistoricalData> Index<i32> for TimeSeries<T> {
    type Output = T;

    fn index<'a>(&'a self, index: i32) -> &'a T {
        if index >= 0 {
            &self.data[index as usize]
        } else {
            &self.data[self.data.len() - (-index as usize)]
        }
    }
}

impl<T: Clone + Ord + HistoricalData> Iterator for TimeSeries<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.data.len() {
            self.counter += 1;
            Some( self.data[self.counter - 1].clone() )
        } else {
            None
        }
    }
}

impl<T: Clone + Ord + HistoricalData> TimeSeries<T> {

    pub fn new(id: &str, optmaxsize: Option<usize>) -> TimeSeries<T> {
        TimeSeries{
            id: id.to_string(),
            index: Vec::<DateTime<Utc>>::new(),
            data: Vec::<T>::new(), optmaxsize: optmaxsize,
            counter: 0
        }
    }

    pub fn from_vec(id: &str, items: Vec<T>) -> TimeSeries<T> {
        let mut ts = TimeSeries::new( id, None );
        for item in items {
            ts.insert_update( item );
        }
        ts
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    /// Insert item or update if it already exists
    pub fn insert_update(&mut self, item: T) {
        if self.id.as_str() == item.get_id() {
            if let Some(k) = self.index.iter().position(|x| x == item.get_time_signature() ) {
                self.data[k] = item;
            } else {
                self.data.push(item);
                self.data.sort();
                if let Some( maxsize ) = self.optmaxsize {
                    if self.data.len() > maxsize {
                        self.data.remove(0);
                    }
                }
                self.index = self.data.iter().map(|e| e.get_time_signature()).cloned().collect();
            }
        }
    }

    /// Get item corresponding to timesignature
    pub fn at(&self, time_signature: &DateTime<Utc>, offset: i32) -> Option<T> {
        let optpos = self.index.iter().rposition(|e| e == time_signature);
        match optpos {
            Some( pos ) => {
                if offset > 0{
                    let newpos = pos + (offset as usize);
                    if newpos < self.index.len() {
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
    pub fn range_at(&self, timesignature: &DateTime<Utc>, size: usize, offset: i32) -> Vec<T> {
        let mut res : Vec<T> = Vec::new();
        for i in 0..size {
            if let Some( data ) = self.at(timesignature, offset + (i as i32)) {
                res.push( data );
            }
        }
        res
    }

    /// Get range  
    pub fn range(&self, start: i32, end: i32) -> Vec<T> {
        let is : usize = if start >= 0 { start as usize } else { self.data.len() + start as usize };
        let ie : usize = if end >= 0 { end as usize } else { self.data.len() + end as usize };
        let mut res : Vec<T> = Vec::new();
        for i in is..ie {
            res.push( self[i as i32].clone() )
        }
        res
    }

}