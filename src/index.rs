use std::io::Write;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

use crate::input;
use crate::index;

struct IndexItem {
    index: usize,
    path: String,
    linkage: String,
}
impl IndexItem {
    pub fn get_index(&self) -> &usize {
        return &self.index;
    }
    pub fn get_path(&self) -> &String {
        return &self.path;
    }
    pub fn get_linkage(&self) -> &String {
        return &self.linkage;
    }
}

pub fn index_file_init() {
    let init_file_dir = fs::create_dir("./data");
    let _result = match init_file_dir {
        Ok(()) =>(),
        Err(_error) => (),
    };
    let init_file = File::create("./data/data.ms");
    let _result = match init_file {
        Ok(file) => file,
        Err(error) => panic!("panic! create file error: {:?}", error)
    };
}

pub fn index_file_add_entry() {
    let index_file_load_result = OpenOptions::new()
        .append(true)
        .open("./data/data.ms");
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
                .open("./data/data.ms"
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
    let loaded_file: Vec<String> = fs::read_to_string("./data/data.ms")
    .expect("panic! load error")
    .split("\n")
    .map(|line| line.to_string())
    .collect();
    for (i,val) in loaded_file.iter().enumerate() {
        if val.len() == 0 {
            continue;
        } else {
            let index_item = IndexItem {
                index: i,
                path: val.to_string(),
                linkage: index_validate_path(&val)
            };
            println!("[index:{}][path:{}][linkage:{}]", 
                index_item.get_index(), 
                index_item.get_path(), 
                index_item.get_linkage()
            );
        }
    }
    return loaded_file;
}

pub fn index_validate_path(filepath: &String) -> String {
    if Path::new(filepath).exists() {
        return "exists".to_string();
    }
    return "dead".to_string();
}