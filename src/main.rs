#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::process::exit;
use std::str;
use file::Configuration;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::MutexGuard;
use crate::index::{IndexItem};
use eframe::egui;

mod index;
mod gpg;
mod file;

struct AppState {
    vector_length: usize,
    hashset_length: usize,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            vector_length: 0,
            hashset_length: 0,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.hashset_length = read_master_hashset_size();
        self.vector_length = read_master_vector_size();
        let master_vector_copy = read_master_vector();
        egui::CentralPanel::default().show(ctx, |ui| {
            for item in master_vector_copy {
                ui.heading(format!("{}", item.get_index()));
                ui.label(format!("path: {}", item.get_system_path()));
                ui.label(format!("linkage: {}", item.get_system_linkage()));
                ui.label(format!("filesize: {}", item.get_filesize()));
                ui.label(format!("filetype: {}", item.get_filetype()));
                ui.separator();
            }
        });
    }
}

lazy_static! {
    static ref APPDATA: Mutex<(Vec<index::IndexItem>,HashSet<String>)> = Mutex::new(
        (   
            Vec::new(),
            HashSet::new()
        )
    );
}

fn load_master_data(data_filepath: &String) {
    let result: Result<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>, std::sync::PoisonError<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>>> = APPDATA.lock();
    match result {
        Err(error) => panic!("panic! data access error: {:?}", error),
        Ok(mut mutex_guard) => {
            mutex_guard.0.clear(); //clear previous
            let loaded_file: Vec<String> = file::read_paths_list(&data_filepath);
            for (i,val) in loaded_file.iter().enumerate() {
                if val.len() == 0 {
                    continue;
                } else {
                    let index_item = index::IndexItem::new(
                        i,
                        &val,
                        &file::validate_path_desc(&val),
                        data_filepath,
                    );
                    mutex_guard.0.push(index_item);
                    mutex_guard.1.insert(val.to_owned());
                }
            }
        },
    }
}

fn read_master_hashset_size() -> usize {
    let result: Result<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>, std::sync::PoisonError<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>>> = APPDATA.lock();
    match result {
        Err(error) => panic!("panic! data access error: {:?}", error),
        Ok(mutex_guard) => {
            return mutex_guard.0.len();
        }
    }
}

fn read_master_hashset() -> HashSet<String> {
    let result: Result<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>, std::sync::PoisonError<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>>> = APPDATA.lock();
    match result {
        Err(error) => panic!("panic! data access error: {:?}", error),
        Ok(mutex_guard) => {
            return mutex_guard.1.clone();
        }
    }
}

fn read_master_vector_size() -> usize {
    let result: Result<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>, std::sync::PoisonError<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>>> = APPDATA.lock();
    match result {
        Err(error) => panic!("panic! data access error: {:?}", error),
        Ok(mutex_guard) => {
            return mutex_guard.1.len();
        }
    }
}

fn read_master_vector() -> Vec<IndexItem> {
    let result: Result<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>, std::sync::PoisonError<MutexGuard<'_, (Vec<IndexItem>, HashSet<String>)>>> = APPDATA.lock();
    match result {
        Err(error) => panic!("panic! data access error: {:?}", error),
        Ok(mutex_guard) => {
            return mutex_guard.0.clone();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    load_master_data(&String::from("./data/data.ms"));
    eframe::run_native(
        "memoryspace",
        options,
        Box::new(|_cc| Box::<AppState>::default()),
    )
}
