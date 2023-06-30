use std::sync::MutexGuard;
use tabled::{Table, Tabled, settings::Style};

use crate::file::{self, validate_path_desc};

#[derive(Tabled)]
#[derive(Eq, PartialEq)]
pub struct IndexItem {
    index: usize,
    system_path: String,
    system_linkage: String,
    file_type: String,
    datapath: String,
}

#[derive(Tabled)]
#[derive(Eq, PartialEq)]
pub struct IndexItemVolatile {
    index: usize,
    system_path: String,
    system_linkage: String,
    file_type: String,
    in_index: bool,
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
impl IndexItemVolatile {
    pub fn new(index: usize, system_path: &String, system_linkage: &String) -> Self {
        return IndexItemVolatile { 
            index: index,
            system_path: system_path.to_owned(),
            system_linkage: system_linkage.to_owned(),
            file_type: file::filetype(system_path),
            in_index: false,

        }
    }
    pub fn get_index(&self) -> &usize {
        return &self.index;
    }
    pub fn get_system_path(&self) -> &String {
        return &self.system_path;
    }
    pub fn get_system_linkage(&self) -> &String {
        return &self.system_linkage;
    }
}

pub fn produce_volatile_list(discovery: &Vec<String>) -> Vec<IndexItemVolatile> {
    let mut volatile_list: Vec<IndexItemVolatile> = vec![];
    for (val, item) in discovery.iter().enumerate() {
        let index_item_volatile = IndexItemVolatile::new(
            val.to_owned(),
            &item.to_owned(),
            &validate_path_desc(&item)
        );
        volatile_list.push(index_item_volatile);
    }
    return volatile_list;
}

pub fn index_table_display_volatile(volatile_list: &Vec<IndexItemVolatile>) {
    let table = Table::new(volatile_list.iter()).with(Style::psql()).to_string();
    println!();
    println!("{}", &table);
    println!();
}

pub fn index_table_display(mutex_guard: &MutexGuard<'_,Vec<IndexItem>>) {
    let table = Table::new(mutex_guard.iter()).with(Style::psql()).to_string();
    println!();
    println!("{}", &table);
    println!();
}