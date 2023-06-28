use std::sync::MutexGuard;
use tabled::{Table, Tabled, settings::Style};

use crate::file;

#[derive(Tabled)]
#[derive(Eq, PartialEq)]
pub struct IndexItem {
    index: usize,
    system_path: String,
    system_linkage: String,
    file_type: String,
    datapath: String,
}
impl IndexItem {
    pub fn new(index: usize, system_path: &String, system_linkage: &String) -> Self {
        return IndexItem { 
            index: index,
            system_path: system_path.to_owned(),
            system_linkage: system_linkage.to_owned(),
            file_type: file::filetype(system_path),
            datapath: "./data/data.ms".to_string(),
        }
    }
    pub fn _get_index(&self) -> &usize {
        return &self.index;
    }
    pub fn get_system_path(&self) -> &String {
        return &self.system_path;
    }
    pub fn get_system_linkage(&self) -> &String {
        return &self.system_linkage;
    }
    pub fn _get_datapath(&self) -> &String {
        return &self.datapath;
    }
}

pub fn index_table_display(mutex_guard: &MutexGuard<'_,Vec<IndexItem>>) {
    let table = Table::new(mutex_guard.iter()).with(Style::psql()).to_string();
    println!();
    println!("{}", &table);
    println!();
}