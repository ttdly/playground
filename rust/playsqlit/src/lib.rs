

use rusqlite::{Connection};

fn repo_info_create(conn:Connection) -> bool{
    match conn.execute("create table if not exists repo (
        id integer primary key,
        name text not null unique,
        description text not null
    )",[]) {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}

pub fn repo_info_insert(){
    let conn = Connection::open("./repo.db").unwrap();
    match conn.execute("select id from repo", []) {
        Ok(ok) => {
            println!("{}",ok.ilog10());
            println!("exist it");
        }
        Err(err) => {
            println!("{:#?}",err);
            if repo_info_create(conn) {
                println!("done");
            } else {
                println!("Unknown Error!")
            }
        }
    }
}