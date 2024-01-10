use config::Config;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.ini"))
        .build()
        .unwrap();

    let s: HashMap<String, String> = settings.try_deserialize().unwrap();
    println!("{:?}", s);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No arguments supplied.");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "new" => new_task(&args[2], &args[3]),
        "remove" => println!("remove"),
        _ => println!("none"),
    }
}

fn new_task(name: &str, description: &str) {
    println!("---New Task---");
    println!("Name: {:?}", name);
    println!("Desc: {:?}", description);
}
