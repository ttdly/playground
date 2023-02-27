use std::{
    cmp::{Ord, Ordering, PartialOrd},
    path::PathBuf,
    vec::Vec,
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileNodeItem {
    pub path_buf: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FileNodeItem>,
}
// 继承排序方法
impl PartialOrd for FileNodeItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.is_dir, other.is_dir) {
            (true, false) => return Some(Ordering::Greater),
            (false, true) => return Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}
// 继承排序方法
impl Ord for FileNodeItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FileNodeItem {
    pub fn new(path: PathBuf) -> FileNodeItem {
        let is_dir = path.is_dir();
        FileNodeItem {
            path_buf: path,
            is_dir,
            children: Vec::new(),
        }
    }
    // 对文件夹内文件排序
    pub fn sorted_children(&self) -> Vec<FileNodeItem> {
        let mut children = self.children.clone();
        children.sort();
        children
    }
    // TODO获取文件实体
    pub fn get_file_node() {}
}
