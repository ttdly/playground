use git2::{Repository, RepositoryInitOptions};
pub fn create_bare(name:String,desc:String)-> String{
    // 获取到仓库根路径
    let base_dir = "C:\\Users\\chinchou\\code\\git\\";
    // 拼接仓库路径
    let path = format!("{}{}.git",base_dir,name);
    // 初始化仓库选项
    let mut binding = RepositoryInitOptions::new();
    // 对初始化选项输入裸仓库以及描述条件 TODO:描述不生效
    let repo_opts = binding.bare(true).description(&desc);
    // 创建仓库
    let repo = Repository::init_opts(path, repo_opts);
    match repo {
        Ok(repo) => {
            let result = repo.path().to_str().unwrap_or("no path");
            result.to_string()
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}
#[cfg(test)]
mod test {
    use crate::create;

    #[test]
    fn run(){
        create("playgit".to_owned());
    }
}