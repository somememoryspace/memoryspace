use std::fs;
use std::fs::File;
use std::path::Path;

pub fn create_file(filepath: String) {
    let mut filepath_vec: Vec<String> = filepath.split("/").map(|s| s.to_string()).collect();
    filepath_vec.pop();
    let directory_tree_for_file = filepath_vec.iter().map(|x| x.to_string() + "/").collect::<String>();
    println!("{}", &directory_tree_for_file);
    let creation = fs::create_dir_all(&directory_tree_for_file);
    let _creation = match creation {
        Err(error) => panic!("panic! creating directory error: {:?}", error),
        Ok(()) => {
            let init_file = File::create(&filepath);
            let _result = match init_file {
                Err(error) => panic!("panic! create file error: {:?}", error),
                Ok(_file) => {
                    match Path::new(&filepath).exists() {
                        true => println!("msg: {} created", filepath),
                        false => panic!("err: file create validation error"),
                    }
                },
            }; 
        },   
    };
}

pub fn filetype(filepath: String) -> String {
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

pub fn validate_file_bool(filepath: String) -> bool {
    match Path::new(&filepath).exists() {
        true => return true,
        false => return false,
    }
}

pub fn validate_path_desc(filepath: String) -> String {
    if Path::new(&filepath).exists() {
        return "exists".to_string();
    }
    return "dead".to_string();
}

pub fn delete_temp_file(filepath: &String) {
    let init_file = fs::remove_file(filepath);
    let _result = match init_file {
        Ok(()) => (),
        Err(error) => panic!("panic! delete file error: {:?}", error)
    };
}

pub fn output_temp_file(filepath: &String) {
    let init_file = fs::read_to_string(filepath);
    let result = match init_file {
        Ok(file) => file,
        Err(error) => panic!("panic! read file error: {:?}", error)
    };
    println!("{}",result);
}