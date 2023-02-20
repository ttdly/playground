use std::{
    fs::{
        remove_dir_all,
        read_dir,
        read_to_string}, ffi::OsStr};
pub fn remove(){
    let remove_status = remove_dir_all("C:\\Users\\chinchou\\code\\git\\playgit.git");
    if remove_status.is_ok() {
        println!("Delete success!")
    }
}

pub fn get_dir() {
    let res = read_dir("C:\\Users\\chinchou\\Documents\\AA_MyDocs").unwrap();
    for entry in res {
        let entry = entry.unwrap();
        let path = entry.path();
        let none = OsStr::new("none");
        if path.extension().unwrap_or(none).eq("md") {
            println!("{:?}", path.file_name().unwrap_or(none));
            println!("{:?}", read_to_string(path));
        }
    }
}
