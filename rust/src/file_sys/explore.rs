use std::{ fs::read_dir, path::PathBuf};

use super::file::FileNodeItem;
// TODO：
// 存储整个文件目录
// 将目录输出为前端可解析格式
// 暴露增删改查文件接口
//
pub fn init_explore(root: PathBuf) {
    let mut explore = FileNodeItem::new(root.clone());
    match read_dir(root) {
        Ok(res) => {
            for item in res {
                let file = item.unwrap().path();
                explore.children.push(FileNodeItem::new(file));
            }
            let result_data = serde_json::to_string(&explore).unwrap();
            println!("{}",result_data);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::init_explore;

    #[test]
    fn run() {
        let mut root = PathBuf::new();
        root.push("C:\\Users\\chinchou\\Documents\\AA_MyDocs");
        assert_eq!(init_explore(root), ());
    }
}
