use std::io::{stdout, Write};
use std::process::exit;
use std::str;

mod input;
mod index;
mod gpg;

const VERSION: f32 = 0.1;

fn command_prompt() {
    let index = vec![
        "index-list",
        "index-init",
        "index-add",
        "index-remove",
        "index-decrypt",
        "index-encrypt",

    ];
    let sys = vec![
        "sys-version",
        "sys-exit",
    ];
    let other = vec![
        "clear",
        "help",
    ];
    println!("--index--");
    for x in &index {
        println!(" - {x}");
    }
    println!("--sys--");
    for x in &sys {
        println!(" - {x}");
    }
    println!("--other--");
    for x in &other {
        println!(" - {x}");
    }
    stdout().flush().ok();
}

fn command_proc(command: &str, version: f32) {
    println!();
    match command {
        "index-init" => {
            println!("index: initializing a new index file"); 
            index::index_file_init();
        },
        "index-list" => {
            println!("index: printing current index file"); 
            let _read = index::index_file_load();
        },
        "index-add" => {
            println!("index: adding new path entry to index"); 
            index::index_file_add_entry();
        },
        "index-remove" => {
            println!("index: removing entry");
            index::index_file_remove_entry(); 
        },
        "index-encrypt" => {
            println!("index: encrypt an entry"); 
        },
        "index-decrypt" => {
            println!("index: decrypt an entry"); 
            gpg::unlock_and_read();
        },
        "sys-version" => {
            println!("sys: version {}", version); 
        },
        "sys-exit" => {
            println!("sys: exit"); 
            exit(0);
        },
        "help" => {
            command_prompt();
        },
        "clear" => {
            input::clear_screen();
        },
        &_=> {
            println!("err: invalid command");
        },
    };
    println!();
}

fn main() {
    loop {
        let command = input::input_handle("root", true);
        command_proc(&command, VERSION)
    }
}
