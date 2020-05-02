use chrono::{DateTime, Utc};
use crate::HistoricalData;
use crate::DataPoint;

#[derive(Debug, Clone)]
pub struct TimeSeries<T> {
    pub id: String,
    pub index: Vec<DateTime<Utc>>,
    pub data: Vec<T>,
    optmaxsize: Option<usize>,
    counter: usize
}

impl TimeSeries<DataPoint> {

    /// Insert item or add data if it exists
    pub fn insert_add(&mut self, item: DataPoint) {
        if self.id == item.id {
            if let Some(k) = self.index.iter().position(|&x| x == item.timesignature ) {
                self.data[k].data += item.data;
            } else {
                self.data.push(item);
                self.data.sort();
                if let Some( maxsize ) = self.optmaxsize {
                    if self.data.len() > maxsize {
                        self.data.remove(0);
                    }
                }
                self.index = self.data.iter().map(|ref e| e.timesignature()).collect();
            }
        }
    }

    /// Create the Timeseries of its cumulated sum
    pub fn cumsum(&self) -> TimeSeries<DataPoint> {
        let mut running_total = 0f64;
        let mut ts = TimeSeries::new( self.id.as_str(), self.optmaxsize );
        for dp in self.clone().into_iter() {
            let mut adj_dp = dp;
            adj_dp.data += running_total;
            running_total = adj_dp.data;
            ts.insert_update( adj_dp );
        }
        ts
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

    /// Insert item or update if it already exists
    pub fn insert_update(&mut self, item: T) {
        if self.id == item.id() {
            if let Some(k) = self.index.iter().position(|&x| x == item.timesignature() ) {
                self.data[k] = item;
            } else {
                self.data.push(item);
                self.data.sort();
                if let Some( maxsize ) = self.optmaxsize {
                    if self.data.len() > maxsize {
                        self.data.remove(0);
                    }
                }
                self.index = self.data.iter().map(|ref e| e.timesignature()).collect();
            }
        }
    }

    /// Get item corresponding to timesignature
    pub fn get(&self, timesignature: &DateTime<Utc>, offset: i32) -> Option<T> {
        let optpos = self.index.iter().rposition(|e| e == timesignature);
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
    pub fn range(&self, timesignature: &DateTime<Utc>, size: usize, offset: i32) -> Option<Vec<T>> {
        let mut res : Vec<T> = Vec::new();
        for i in 0..size {
            if let Some( data ) = self.get(timesignature, offset + (i as i32)) {
                res.push( data );
            } else {
                return None;
            }
        }
        Some( res )
    }

    /// Get range starting from the end of the time series with offset 
    pub fn previous_range(&self, size: usize, negoffset: usize) -> Option<Vec<T>> {
        if let Some( lastdata ) = self.previous( negoffset ) {
            self.range( &lastdata.timesignature(), size, -(size as i32) + 1 )
        } else {
            None
        }
    }

    /// Retrieve last element of the time Series
    pub fn previous(&self, negoffset: usize) -> Option<T> {
        let totalnum = self.data.len();
        if totalnum >= negoffset + 1 {
            Some( self.data[totalnum - negoffset - 1].clone() )
        } else {
            None
        }
    }

}