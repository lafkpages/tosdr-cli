use std::env;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Resp {
    error: u16,
    message: String,
    parameters: RespParameters,
}

#[derive(Deserialize, Debug)]
struct RespParameters {
    services: Vec<RespService>,
}

#[derive(Deserialize, Debug)]
struct RespService {
    id: u32,
    name: String,
    slug: String,
    is_comprehensively_reviewed: bool,
    urls: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: tosdr <domain>");
        return;
    }

    let resp = ureq::get(&format!(
        "https://api.tosdr.org/search/v4/?query={}",
        args[1]
    ))
    .call()
    .unwrap()
    .into_json::<Resp>()
    .unwrap();
    println!("{:?}", resp);
}
