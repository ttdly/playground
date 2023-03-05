use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct RequstBody{
    cmd: String,
    pram: String
}
#[cfg(test)]
mod test {
    use super::RequstBody;
    #[test]
    fn run(){
        let s = "{
            \"cmd\":\"read_file\",
            \"pram\":\"{oooo:sasa,aaaaa:llll}\"
        }";
        let requst:RequstBody =  serde_json::from_str(s).unwrap();
        println!("We will excute {} with parm:{}", requst.cmd, requst.pram);
    }
}