use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Account {
  name: String,
  password: String,
}

fn main() {
  let args: Vec<String> = env::args().collect();
  
  if args.len() != 3 {
    println!("Error: You did not follow the format: cargo run -- [account] [password]");
    return;
  }
let mut account_info = HashMap::new();
let acc = Account {
  name: args[1].clone(),
  password: args[2].clone(),
};

account_info.insert(acc.name, acc.password);
println!("Account saved");

let json = serde_json::to_string_pretty(&account_info).expect("Failed to serialize data");

let mut file = File::create("passwords.json").expect("Failed to create file");
file.write_all(json.as_bytes()).expect("Failed to write to file");
}
