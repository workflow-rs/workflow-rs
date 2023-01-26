/*
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

fn get_data_from_file()->Arc<Mutex<BTreeMap<String, Arc<Vec<String>>>>>{

    Arc::new(Mutex::new(BTreeMap::new()))
}
fn get_storage()->Arc<Mutex<BTreeMap<String, Arc<Vec<String>>>>>{
    lazy_static! {
        static ref MAP: Arc<Mutex<BTreeMap<String, Arc<Vec<String>>>>> = get_data_from_file();
    }

    MAP.clone()
}
pub fn get_attributes(name:String)->Option<Arc<Vec<String>>>{
    let m = get_storage();
    let map = m.lock().unwrap();
    match map.get(&name){
        Some(list)=>Some(list.clone()),
        None=>None
    }
}

pub fn set_attributes(name:String, attr:Vec<String>){
    let m = get_storage();
    let mut map = m.lock().unwrap();
    map.insert(name, Arc::new(attr));
}
*/
