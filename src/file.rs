use std::{fs};
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::fs::OpenOptions;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use serde_yaml::{self};
use glob::{MatchOptions, glob_with};
use file_shred;
use crate::index::IndexItem;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Configuration {
    data_filepath: String,
    configuration_filepath: String,
    temp_file_output_default: bool,
}

impl Configuration {
    pub fn new(data_filepath: &String, configuration_filepath: &String, temp_file_output_default: &bool) -> Self {
        return Configuration { 
            data_filepath: data_filepath.to_owned(),
            configuration_filepath: configuration_filepath.to_owned(),
            temp_file_output_default: temp_file_output_default.to_owned(),
        }
    }
    pub fn get_data_filepath(&self) -> &String {
        return &self.data_filepath;
    }
    pub fn get_configuration_path(&self) -> &String {
        return &self.configuration_filepath;
    }
    pub fn parse_from_file(configuration_filepath: &String) -> Self {
        let open_configuration = std::fs::File::open(&configuration_filepath);
        match open_configuration {
            Err(_error) => panic!("panic! configuration file load error"),
            Ok(file) => {
                let scrape_configuration: Configuration = serde_yaml::from_reader(&file).expect("panic! file parse error");
                return scrape_configuration;
            }
        };
        
    }
}

pub fn create_file(filepath: &String, create_new: bool) {
    let mut filepath_vec: Vec<String> = filepath.split("/").map(|s| s.to_string()).collect();
    filepath_vec.pop();
    let directory_tree_for_file = filepath_vec.iter().map(|x| x.to_string() + "/").collect::<String>();
    let creation = fs::create_dir_all(&directory_tree_for_file);
    match creation {
        Err(error) => panic!("panic! creating directory error: {:?}", error),
        Ok(()) => {
            let init_file = File::create(filepath);
            let _result = match init_file {
                Err(error) => panic!("panic! create file error: {:?}", error),
                Ok(_file) => {
                    match Path::new(filepath).exists() {
                        false => println!("err: file create validation error, check manually"),
                        true =>  {
                            match create_new {
                                true => println!("msg: {} created", filepath),
                                false => println!("msg: {} edited", filepath),
                            }
                        }
                    }
                },
            }; 
        },   
    };
}

pub fn filetype(filepath: &String) -> String {
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
    if filepath.contains(".key") {
        return "key file".to_string();
    }
    return "other".to_string();
}

pub fn get_filesize(filepath: &String) -> String {
    let metadata = fs::metadata(filepath);
    match metadata {
        Ok(metadata) => {
            let size = metadata.len();
            return size.to_string() + " Bytes";
        },
        Err(_error) => {
            return String::from("0") + " Bytes";
        }
    };
} 

pub fn validate_file_bool(filepath: &String) -> bool {
    match Path::new(filepath).exists() {
        true => return true,
        false => return false,
    }
}

pub fn validate_path_desc(filepath: &String) -> String {
    if Path::new(filepath).exists() {
        return "exists".to_string();
    }
    return "dead".to_string();
}

pub fn read_paths_list(filepath: &String) -> Vec<String> {
    let vector: Vec<String> = fs::read_to_string(filepath)
        .expect("err: array load error")
        .split("\n")
        .map(|line| line.to_string())
        .collect();
    return vector;
}

pub fn delete_temp_file(filepath: &String) {
    let verbosity = file_shred::Verbosity::Quiet;
    let shared_config = file_shred::ShredConfig::non_interactive(
        vec!(&filepath), 
        verbosity, 
        false, 
        10, 
        10,
    );
    let _shredder = file_shred::shred(&shared_config);

}

pub fn output_temp_file(filepath: &String) {
    let init_file = fs::read_to_string(filepath);
    let result = match init_file {
        Err(error) => panic!("panic! read file error: {:?}", error),
        Ok(file) => file,
    };
    println!("{}",result);
}

pub fn overwrite_file(filepath: &String, master_vector: &Vec<IndexItem>) {
    create_file(filepath, false);
    let index_file_load_result = OpenOptions::new()
        .append(true)
        .open(filepath);
    let mut loaded_file = match index_file_load_result {
        Err(error) => panic!("panic! opening file error: {:?}", error),
        Ok(file) => file,   
    };
    for item in master_vector.iter() {
        let write_result = writeln!(loaded_file, "{}",item.get_system_path());
        match  write_result {
            Err(error) => panic!("panic! writing file error: {:?}", error),
            Ok(()) => (),   
        };
    }
}

pub fn discover_files(directory: &String, pattern: &String, master_hashset: &HashSet<String>) -> Vec<String> {
    let mut discovered_files: Vec<String> = vec![];
    let pattern_complete = directory.to_owned() + pattern.to_owned().as_str();
    let traverse_options = MatchOptions {
        case_sensitive: false, 
        require_literal_leading_dot: false, 
        require_literal_separator: false,
    };
    for detection in glob_with(&pattern_complete.as_str(),traverse_options).expect("err: error on reading glob pattern") {
        match detection {
            Ok(found_path) => {
                discovered_files.push(found_path.display().to_string());
            },
            Err(_error) => {
                println!("err: during discover of files process");
                continue;
            }
        };
    }
    let mut uniques: Vec<String> = vec![];
    for item in &discovered_files {
        if master_hashset.contains(item) {
            continue;
        } else {
            uniques.push(item.to_string());
        }
    }
    return uniques;
}