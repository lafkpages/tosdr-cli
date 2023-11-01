use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[clap(flatten)]
    args: CliArgsGroup,
}

#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
struct CliArgsGroup {
    #[clap(short, long)]
    query: Option<String>,

    #[clap(short, long)]
    domain: Option<String>,
}

mod structs;

fn main() {
    let cli = CliArgs::parse();

    println!("CLI: {:#?}", cli);

    let was_domain = cli.args.domain.is_some();
    let query = cli
        .args
        .query
        .unwrap_or(unsafe { cli.args.domain.unwrap_unchecked() });

    let resp = ureq::get(&format!("https://api.tosdr.org/search/v4/?query={}", query))
        .call()
        .unwrap()
        .into_json::<structs::Resp>()
        .unwrap();

    println!("Results for \"{}\":", query);
    for service in resp.parameters.services {
        if was_domain && !service.urls.contains(&query) {
            continue;
        }

        println!("  - {} ({})", service.name, service.id);
        println!("    - {}", service.rating.human);
        println!("    - URLs:");
        for url in service.urls {
            println!("      - {}", url);
        }
        println!("    - Wikipedia: {}", service.wikipedia);
    }
}
