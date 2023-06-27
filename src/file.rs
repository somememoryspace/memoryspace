pub fn create_file(filepath: String) {
    let mut filepath_vec: Vec<String> = filepath.split("/").map(|s| s.to_string()).collect();
    let filename = filepath_vec.remove(&filepath_vec.len()-1);
    println!("{}", filename);
    println!("{:?}", filepath_vec);
}