use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
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