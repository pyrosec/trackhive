use clap::{Command, Arg};
use reqwest::{blocking, blocking::Client, Error};
use std::{collections::HashMap, env, fs, io::Write, path::{Path}};
use serde::{Serialize, Deserialize};
use serde_json;



fn construct_create_tracking_request(tracking: String, slug: String) -> Result<blocking::Response, Error>{
    let token = env::var("TRACKHIVE_API_KEY").unwrap();
    let mut body: HashMap<String, String> = HashMap::new();
    
    body.insert("tracking_number".to_string(), tracking);
    body.insert("slug".to_string(), slug);


    let proxy = reqwest::Proxy::http("socks5://ugpbduqu:5B3D4D37F40BEF8F001B6C53BAC7F29F@161.77.80.202:19088").unwrap();


    let client = Client::new();

    let response = client.post("https://api.trackinghive.com/trackings")
        .bearer_auth(token)
        .json(&body)
        .send();

    return response;
}

#[derive(Serialize, Deserialize, Debug)]
struct Trackhive {
    tracking_number: String,
    api: String,

    
}

impl Trackhive {
    fn save(self: Self) {
        let path = Path::new(&(env::var("HOME").unwrap())).join(".trackhive");
        fs::create_dir(&path).unwrap();
        let mut handle = fs::File::create(path.clone().join("session.json")).unwrap();
        handle.write_all(serde_json::to_string(&self).unwrap().as_bytes()).unwrap();

    }

    fn load(self: Self){
        let path = Path::new(&(env::var("HOME").unwrap())).join(".trackhive");
        if(path.is_dir()){
            let mut session = fs::File::open(path.clone().join("session.json")).unwrap();
            let contents = serde_json::from_reader(session);
            

        }
    }
}


fn main() {


    let cmd = Command::new("trackhive")
        // Create tracking
        .subcommand(Command::new("couriers-list"))
        .subcommand(Command::new("create_tracking"))
            .arg(
                Arg::new("tracking_number")
                    .short('t')
                    .required(true)
                    .num_args(1))
            .arg(
                Arg::new("slug")
                    .short('s')
                    .required(true)
                    .num_args(1))
            .get_matches();

    if let Some(cmd) = cmd.subcommand_matches("couriers-list"){

    } else {
        let tracking = cmd.get_one::<String>("tracking_number").unwrap().to_string();
        let slug = cmd.get_one::<String>("slug").expect("usps").to_string();
        let get_tracking = construct_create_tracking_request(tracking, slug).unwrap();

        println!("status: {:?}", get_tracking.status());
        println!("body: {:?}", get_tracking);

    }
    
}


