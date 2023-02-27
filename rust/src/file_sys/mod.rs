use std::{
    fs::{
        remove_dir_all,
        read_dir}, ffi::OsStr};
pub mod explore;
pub mod file;

pub fn remove(){
    let remove_status = remove_dir_all("C:\\Users\\chinchou\\code\\git\\playgit.git");
    if remove_status.is_ok() {
        println!("Delete success!")
    }
}

pub fn get_dir(dir: &str){
    let res = read_dir(dir).unwrap();
    for entry in res {
        let entry = entry.unwrap();
        let path = entry.path();
        let none = OsStr::new("none");
        if path.is_dir() {
            let dir = path.to_str().unwrap_or("none");
            println!("{:?}",dir);
        } else if path.extension().unwrap_or(none).eq("md") {
            println!("{:?}", path);
        }
    }
}
