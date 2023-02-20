
pub mod env_check;
pub mod file_sys;
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
}
