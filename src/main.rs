use std::io::{stdout, Write};
use std::process::exit;
use std::str;
use std::fs;
use lazy_static::lazy_static;
use std::sync::Mutex;

use input::clear_screen;

mod input;
mod index;
mod gpg;
mod file;

const VERSION: f32 = 0.1;
const DATAPATH: &str = "./data/data.ms";

lazy_static! {
    static ref ARRAY: Mutex<Vec<index::IndexItem>> = Mutex::new(vec![]);
}

fn load_array() {
    let result = ARRAY.lock();
    let _result = match result {
        Ok(mut mg) => {
            mg.clear(); //clear previous
            let loaded_file: Vec<String> = fs::read_to_string(&DATAPATH.to_string())
            .expect("panic! load error")
            .split("\n")
            .map(|line| line.to_string())
            .collect();
            for (i,val) in loaded_file.iter().enumerate() {
                if val.len() == 0 {
                    continue;
                } else {
                    let index_item = index::IndexItem::new(
                        i,
                        val.to_string(),
                        file::validate_path_desc(val.to_string())
                    );
                    mg.push(index_item);
                }
            }
        },
        Err(error) => panic!("panic! writing array error: {:?}", error)    
    };
}


fn command_prompt() {
    let index = vec![
        "index-list",
        "   - print the index list",
        "index-add",
        "   - add entry to the index list",
        "index-remove",
        "   - remove entry from the index list",
        "index-decrypt",
        "   - decrypt entry from the index list",
        "index-encrypt",
        "   - encrypt new entry to the index list",
    ];
    let sys = vec![
        "sys-version",
        "   - print the current binary version",
        "sys-exit",
        "   - quit the program",
    ];
    let other = vec![
        "clear",
        "   - clear the screen",
        "help",
        "   - list the runnable commands",
    ];
    println!("--index--");
    for x in &index {
        println!("{x}");
    }
    println!();
    println!("--sys--");
    for x in &sys {
        println!("{x}");
    }
    println!();
    println!("--other--");
    for x in &other {
        println!("{x}");
    }
    println!();
    stdout().flush().ok();
}

fn command_proc(command: &str, version: f32) {
    println!();
    let result = ARRAY.lock();
    let _result = match result {
        Ok(mutex_guard) => {
            match command {
                "index-list" => {
                    println!("index: printing current index file");
                    index::index_table_display(&mutex_guard) 
                },
                "index-add" => {
                    println!("index: adding new path entry to index"); 
                    index::index_table_display(&mutex_guard);
                    index::index_file_add_entry(
                        mutex_guard, 
                        input::input_handle("new file path",false)
                    );
                    load_array();
                },
                "index-remove" => {
                    println!("index: removing entry");
                    index::index_table_display(&mutex_guard);
                    index::index_file_remove_entry(
                        input::input_handle_integer(), 
                        mutex_guard
                    );
                    load_array();
                },
                "index-encrypt" => {
                    println!("index: encrypt an entry"); 
                    index::index_table_display(&mutex_guard);
                    let filepath = input::input_handle("new file path",false);
                    file::create_file(filepath.to_owned());
                    let success = gpg::gpg_encrypt_handle(input::password_input_handle(), filepath.to_owned());
                    match success {
                        true => {
                            index::index_file_add_entry(mutex_guard, filepath.to_owned() + ".gpg");
                            load_array();
                            return;
                        },
                        false => {
                            println!("err: encrypt process failed");
                            return;
                        }
                    }

                },
                "index-decrypt" => {
                    println!("index: decrypt an entry");
                    index::index_table_display(&mutex_guard);
                    let temp_file_bool = input::confirmation_bool("produce output file?".to_string());
                    let selection = input::input_handle_integer();
                    let filepath = mutex_guard.get(selection);
                    match filepath {
                        None => panic!("panic! array indexing error"),
                        Some(index_item) => {
                            let filepath = index_item.get_system_path();
                            let linkage = index_item.get_system_linkage();
                            if linkage.contains("dead") {
                                println!("err: attempting to decrypt a dead file");
                                return;
                            }
                            let success = gpg::gpg_decrypt_handle(input::password_input_handle(), filepath.to_string());
                            match success {
                                true => {
                                    match temp_file_bool {
                                        true => {
                                            return;
                                        },
                                        false => {
                                            println!("{}", filepath);
                                            let filepath = filepath.replace(".gpg", "");
                                            file::output_temp_file(&filepath);
                                            file::delete_temp_file(&filepath);
                                            return;
                                        }
                                    }
                                },
                                false => {
                                    println!("err: decrypt process failed");
                                    return;
                                }
                            }

                        },
                    }
                },
                "sys-version" => {
                    println!("sys: version {}", version); 
                },
                "sys-exit" => {
                    println!("sys: exit"); 
                    exit(0);
                },
                "help" => {
                    println!("help: commands you can run");
                    command_prompt();
                },
                "clear" => {
                    input::clear_screen();
                },
                &_=> {
                    println!("err: invalid command.");
                },
            };
            println!();
        },
        Err(error) => panic!("panic! table display error: {:?}", error)    
    };

}

fn boot_sequence() {
    clear_screen();
    println!("welcome to memoryspace");
    if !(file::validate_file_bool(DATAPATH.to_string())) {
        index::index_file_init();
    }
}

fn main() {
    boot_sequence();
    load_array();
    loop {
        let command: String = input::input_handle("memoryspace", true);
        command_proc(&command, VERSION)
    }
}
