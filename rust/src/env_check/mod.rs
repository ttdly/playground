use std::process::Command;
pub fn check_nodejs() -> bool {
    let result = Command::new("node")
    .arg("--version")
    .output()
    .expect("Error");
    result.status.code().eq(&Some(0))
}

pub fn check_pnpm() -> bool {
    let result = Command::new("cmd")
    .args(["/C", "pnpm --version"])
    .output()
    .expect("Error");
    println!("{:?}",result);
    result.status.code().eq(&Some(0))
}

#[cfg(test)]
mod test {
    use crate::env_check::check_pnpm;
    use super::check_nodejs;
    #[test]
    fn run(){
        if check_nodejs() {
            println!("NodeJs include.");
        } else {
            println!("Don't have nodeJs.")
        }
    }
    #[test]
    fn pnpm () {
        if check_pnpm() {
            println!("pnpm include.")
        }
    }
}