use std::sync::Mutex;

use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Data{
    pub data1: String,
    pub data2: usize
}   


lazy_static!{
    pub static ref DATA: Mutex<Data> = Mutex::new(Data::default());
}