use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    domain: String,
}

mod structs;

fn main() {
    let args = Args::parse();

    let resp = ureq::get(&format!(
        "https://api.tosdr.org/search/v4/?query={}",
        args.domain
    ))
    .call()
    .unwrap()
    .into_json::<structs::Resp>()
    .unwrap();

    println!("Results for \"{}\":", args.domain);
    for service in resp.parameters.services {
        println!("  - {} ({})", service.name, service.id);
        println!("    - {}", service.rating.human);
        println!("    - URLs:");
        for url in service.urls {
            println!("      - {}", url);
        }
        println!("    - Wikipedia: {}", service.wikipedia);
    }
}
