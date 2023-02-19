use std::fs::{remove_dir_all};
fn remove() -> std::io::Result<()>{
    let remove_status = remove_dir_all("C:\\Users\\chinchou\\code\\git\\playgit.git");
    Ok(())
}
#[cfg(test)]
mod test {
    use super::remove;
    #[test]
    fn run(){
        remove();
    }
}