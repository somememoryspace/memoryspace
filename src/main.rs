use std::io::{stdout, Write};
use std::process::exit;
use std::str;

use input::clear_screen;

mod input;
mod index;
mod gpg;

const VERSION: f32 = 0.1;

fn command_prompt() {
    let index = vec![
        "index-list",
        "   - print the index list",
        "index-init",
        "   - init a new index list (destructive)",
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
}

fn boot_sequence() {
    clear_screen();
    println!("welcome to memoryspace");
}

fn main() {
    boot_sequence();
    loop {
        let command: String = input::input_handle("memoryspace", true);
        command_proc(&command, VERSION)
    }
}
