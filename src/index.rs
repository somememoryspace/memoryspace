use std::io::Write;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::MutexGuard;
use tabled::{Table, Tabled, settings::Style};

use crate::index;

const DATAPATH: &str = "./data/data.ms";

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
    pub fn new(index: usize, system_path: String, system_linkage: String) -> Self {
        return IndexItem { 
            index: index,
            system_path: system_path.clone(),
            system_linkage: system_linkage,
            file_type: filetype(system_path.clone()),
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

fn filetype(filepath: String) -> String {
    if filepath.contains(".txt") {
        return "txt file".to_string();
    }
    if filepath.contains(".zip") {
        return "zip archive".to_string();
    }
    if filepath.contains(".tar") {
        return "tarbell archive".to_string();
    }
    if filepath.contains(".gz") {
        return "gunzip archive".to_string();
    }
    return "other".to_string();
} 

pub fn index_validate_path(filepath: String) -> String {
    if Path::new(&filepath).exists() {
        return "exists".to_string();
    }
    return "dead".to_string();
}

pub fn index_validate_path_bool(filepath: String) -> bool {
    match Path::new(&filepath).exists() {
        true => return true,
        false => return false,
    }
}

pub fn index_file_init() {
    let init_file_dir = fs::create_dir("./data");
    let _result = match init_file_dir {
        Ok(()) =>(),
        Err(_error) => (),
    };
    let init_file = File::create(&DATAPATH.to_string());
    let _result = match init_file {
        Ok(file) => file,
        Err(error) => panic!("panic! create file error: {:?}", error)
    };
}

pub fn index_file_add_entry(mut mutex_guard: MutexGuard<'_,Vec<IndexItem>>, filepath: String) {
    let index_item = index::IndexItem::new(
        mutex_guard.len(),
        filepath.clone(),
        index::index_validate_path(filepath.clone())
    );
    mutex_guard.push(index_item);
    write_to_file(&mutex_guard);
}

pub fn index_file_remove_entry(selection: usize, mut mutex_guard: MutexGuard<'_,Vec<IndexItem>>) {
    mutex_guard.remove(selection);
    write_to_file(&mutex_guard);
}

pub fn write_to_file(mutex_guard: &MutexGuard<'_,Vec<IndexItem>>) {
    index_file_init();
    let index_file_load_result = OpenOptions::new()
        .append(true)
        .open(&DATAPATH.to_string());
    let mut loaded_file = match index_file_load_result {
        Ok(file) => file,
        Err(error) => panic!("panic! opening file error: {:?}", error)    
    };
    for item in mutex_guard.iter() {
        let write_result = writeln!(loaded_file, "{}",item.get_system_path());
        let _result = match  write_result {
            Ok(()) => (),
            Err(error) => panic!("panic! writing file error: {:?}", error)    
        };
    }
}

pub fn index_table_display(mutex_guard: &MutexGuard<'_,Vec<IndexItem>>) {
    let table = Table::new(mutex_guard.iter()).with(Style::psql()).to_string();
    println!();
    println!("{}", table);
    println!();
}