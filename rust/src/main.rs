
pub mod env_check;
pub mod file_sys;
pub mod info;
fn main() {
    
}

#[cfg(test)]
mod test {
    use playgit::create_bare;
    use super::file_sys;
    use playsqlit::repo_info_insert;
    #[test]
    fn create_repo(){
        file_sys::remove();
        let result = create_bare(String::from("playgit"), String::from("测试仓库描述选项"));
        println!("-----------result------------");
        println!("{}",result);
    }
    #[test]
    fn create_table() {
        println!("repo");
        repo_info_insert();
    }

    #[test]
    fn get_dir() {
        let result = file_sys::get_dir("C:\\Users\\chinchou\\Documents\\AA_MyDocs");
        println!("{:#?}",result);
    }

}
