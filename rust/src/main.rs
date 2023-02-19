
pub mod env_check;
pub mod file_sys;
use playgit::create_bare;
fn main() {
    let result = create_bare(String::from("playgit"), String::from("测试仓库描述选项"));
    println!("-----------result------------");
    println!("{}",result)
}
