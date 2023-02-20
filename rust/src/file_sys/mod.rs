use std::fs::remove_dir_all;
pub fn remove(){
    let remove_status = remove_dir_all("C:\\Users\\chinchou\\code\\git\\playgit.git");
    if remove_status.is_ok() {
        println!("Delete success!")
    }
}