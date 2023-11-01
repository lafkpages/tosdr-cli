use std::env;

mod structs;

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
    .into_json::<structs::Resp>()
    .unwrap();
    println!("{:#?}", resp);
}
