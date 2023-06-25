use std::io::Write;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use tabled::{Table, Tabled, settings::Style};

use crate::input;
use crate::index;
use crate::input::confirmation_bool;

const DATAPATH: &str = "./data/data.ms";

#[derive(Tabled)]
pub struct IndexItem {
    index: usize,
    system_path: String,
    system_linkage: String,
}
impl IndexItem {
    pub fn new(index: usize, system_path: String, system_linkage: String) -> Self {
        return IndexItem { 
            index: index,
            system_path: system_path,
            system_linkage: system_linkage 
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

pub fn index_validate_path(filepath: &String) -> String {
    if Path::new(filepath).exists() {
        return "exists".to_string();
    }
    return "dead".to_string();
}

pub fn index_file_init() {
    if index_validate_path(&DATAPATH.to_string()).contains("exists") {
        print!("found previous data. continue? ");
        let confirmation = confirmation_bool();
        match confirmation {
            true => {
                let init_file = File::create(&DATAPATH.to_string());
                let _result = match init_file {
                    Ok(file) => file,
                    Err(error) => panic!("panic! create file error: {:?}", error)
                };
            },
            false => {
                return;
            }
        }
    }
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
    let index_file_load_result = OpenOptions::new()
        .append(true)
        .open(&DATAPATH.to_string());
    let mut loaded_file = match index_file_load_result {
        Ok(file) => file,
        Err(error) => panic!("panic! opening file error: {:?}", error)    
    };
    let new_entry: String = input::input_handle("new file path", false);
    //let new_entry = "\n".to_owned() + new_entry.as_str();
    let write_result = writeln!(loaded_file, "{}",new_entry);
    let _result = match  write_result {
        Ok(()) => (),
        Err(error) => panic!("panic! writing file error: {:?}", error)    
    };
}

pub fn index_file_remove_entry() {
    let mut index = index::index_file_load();
    let selection = input::input_handle("selection:", false);
    match &selection.parse::<usize>() {
        Err(error) => {
            println!("err: invalid entry. {}", error);
            return;
        },
        Ok(value) => {
            index.remove(*value);
            index::index_file_init();
            let index_file_load_result = OpenOptions::new()
                .append(true)
                .open(&DATAPATH.to_string()
            );
            let mut loaded_file = match index_file_load_result {
                Ok(file) => file,
                Err(error) => panic!("panic! opening file error: {:?}", error)    
            };
            for val in index.iter() {
                if val.len() == 0 {
                    continue;
                } else {
                    let write_result = writeln!(loaded_file, "{}", &val);
                    let _result = match  write_result {
                        Ok(()) => (),
                        Err(error) => panic!("panic! writing file error: {:?}", error)    
                    };
                }
            }
        }
    }
}

//index file loaded into an array and printed
pub fn index_file_load() -> Vec<String>{
    let loaded_file: Vec<String> = fs::read_to_string(&DATAPATH.to_string())
    .expect("panic! load error")
    .split("\n")
    .map(|line| line.to_string())
    .collect();
    let mut index_elements: Vec<IndexItem> = vec![];
    for (i,val) in loaded_file.iter().enumerate() {
        if val.len() == 0 {
            continue;
        } else {
            let index_item = IndexItem {
                index: i,
                system_path: val.to_string(),
                system_linkage: index_validate_path(&val)
            };
            index_elements.push(index_item);
        }
    }
    let table = Table::new(index_elements).with(Style::psql()).to_string();
    println!("{}", table);
    return loaded_file;
}