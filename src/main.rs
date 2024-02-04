use clap::{Command, Arg};
use reqwest::{blocking, blocking::Client, Error};
use std::{collections::HashMap, env, fs, io::{Read, Write}, path::{Path}};
use serde::{Serialize, Deserialize};
use serde_json::{self, from_str};


fn construct_create_tracking_request(tracking: String, slug: String) -> Result<blocking::Response, Error>{
    let token = env::var("TRACKHIVE_API_KEY").unwrap();
    let mut body: HashMap<String, String> = HashMap::new();
    
    body.insert("tracking_number".to_string(), tracking);
    body.insert("slug".to_string(), slug);

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

    fn load() -> Result <Self, Error> {
        let path = Path::new(&(env::var("HOME").unwrap())).join(".trackhive");
        if path.is_dir(){
            let session = fs::read_to_string(path.clone().join("session.json")).expect("There is no session file");
            // parse file 
            let tracking: Trackhive = from_str(&session).expect("Unable to parse session");
            Ok(tracking)
        } else {
            panic!("No session file found.")
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


