use tabled::{Table, Tabled, settings::Style};

use crate::file::{self, validate_path_desc};

#[derive(Tabled)]
#[derive(Eq, PartialEq, Clone)]
pub struct IndexItem {
    index: usize,
    system_path: String,
    filesize: String,
    system_linkage: String,
    file_type: String,
    datapath: String,
}

impl IndexItem {
    pub fn new(index: usize, system_path: &String, system_linkage: &String, data_filepath: &String) -> Self {
        return IndexItem { 
            index: index,
            system_path: system_path.to_owned(),
            filesize: file::get_filesize(system_path),
            system_linkage: system_linkage.to_owned(),
            file_type: file::filetype(system_path),
            datapath: data_filepath.to_owned(),
        }
    }
    pub fn get_index(&self) -> &usize {
        return &self.index;
    }
    pub fn get_filetype(&self) -> &String {
        return &self.file_type;
    }
    pub fn get_filesize(&self) -> &String {
        return &self.filesize;
    }
    pub fn get_system_path(&self) -> &String {
        return &self.system_path;
    }
    pub fn get_system_linkage(&self) -> &String {
        return &self.system_linkage;
    }
}

pub fn produce_volatile_list(discovery: &Vec<String>) -> Vec<IndexItem> {
    let mut volatile_list: Vec<IndexItem> = vec![];
    for (val, item) in discovery.iter().enumerate() {
        let index_item_volatile = IndexItem::new(
            val.to_owned(),
            &item.to_owned(),
            &validate_path_desc(&item),
            &String::from("not saved in datafile"),
        );
        volatile_list.push(index_item_volatile);
    }
    return volatile_list;
}

pub fn index_table_display(master_vector: &Vec<IndexItem>) {
    let table = Table::new(master_vector.iter()).with(Style::psql()).to_string();
    println!();
    println!("{}", &table);
    println!();
}