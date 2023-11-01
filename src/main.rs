use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand, Debug)]
enum CliCommands {
    Search {
        #[clap(flatten)]
        args: CliSearchArgs,
    },
}

#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
struct CliSearchArgs {
    #[clap(short, long, help = "Search for a service by domain")]
    domain: Option<String>,

    #[clap(short, long, help = "Search for services")]
    query: Option<String>,
}

mod structs;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        CliCommands::Search { args } => {
            let was_domain = args.domain.is_some();
            let query = args
                .query
                .clone()
                .unwrap_or(unsafe { args.domain.clone().unwrap_unchecked() });
            // TODO: there should be a better way to do this than cloning

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
    }
}
