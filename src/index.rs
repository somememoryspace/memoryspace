use std::io::Write;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use tabled::{Table, Tabled, settings::Style};

use crate::ARRAY;
use crate::input;
use crate::index;
use crate::input::confirmation_bool;

const DATAPATH: &str = "./data/data.ms";

#[derive(Tabled)]
#[derive(Eq, PartialEq)]
pub struct IndexItem {
    index: usize,
    system_path: String,
    system_linkage: String,
    datapath: String,
}
impl IndexItem {
    pub fn new(index: usize, system_path: String, system_linkage: String) -> Self {
        return IndexItem { 
            index: index,
            system_path: system_path,
            system_linkage: system_linkage,
            datapath: "./data/data.ms".to_string(),
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
    pub fn get_datapath(&self) -> &String {
        return &self.datapath;
    }
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

pub fn index_file_add_entry() {
    index_table_display();
    let new_entry: String = input::input_handle("new file path", false);
    let result = ARRAY.lock();
    let _result = match result {
        Ok(mut mg) => {
            let index_item = index::IndexItem::new(
                mg.len(),
                new_entry.clone(),
                index::index_validate_path(new_entry)
            );
            mg.push(index_item);
        },
        Err(error) => panic!("panic! remove index element error: {:?}", error)    
    };
    write_to_file();
}

pub fn index_file_remove_entry() {
    index_table_display();
    let selection = input::input_handle("selection:", false);
    match &selection.parse::<usize>() {
        Err(error) => {
            println!("err: invalid entry. {}", error);
            return;
        },
        Ok(value) => {
            let result = ARRAY.lock();
            let _result = match result {
                Ok(mut mg) => {
                    mg.remove(*value);
                },
                Err(error) => panic!("panic! remove index element error: {:?}", error)    
            };
        }
    }
    write_to_file();
}

pub fn write_to_file() {
    index_file_init();
    let result = ARRAY.lock();
    let _result = match result {
        Ok(mg) => {
            let index_file_load_result = OpenOptions::new()
                .append(true)
                .open(&DATAPATH.to_string());
            let mut loaded_file = match index_file_load_result {
                Ok(file) => file,
                Err(error) => panic!("panic! opening file error: {:?}", error)    
            };
            for item in mg.iter() {
                let write_result = writeln!(loaded_file, "{}",item.get_system_path());
                let _result = match  write_result {
                    Ok(()) => (),
                    Err(error) => panic!("panic! writing file error: {:?}", error)    
                };
            }
        },
        Err(error) => panic!("panic! mutex error: {:?}", error)    
    };
}

pub fn index_table_display() {
    let result = ARRAY.lock();
    let _result = match result {
        Ok(mg) => {
            let table = Table::new(mg.iter()).with(Style::psql()).to_string();
            println!();
            println!("{}", table);
            println!();
        },
        Err(error) => panic!("panic! table display error: {:?}", error)    
    };
}