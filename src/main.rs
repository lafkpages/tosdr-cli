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

        #[clap(short, long, help = "Show output as JSON")]
        json: bool,
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
        CliCommands::Search { args, json } => {
            let was_domain = args.domain.is_some();
            let query = args
                .query
                .clone()
                .unwrap_or(unsafe { args.domain.clone().unwrap_unchecked() });
            // TODO: there should be a better way to do this than cloning

            let resp = ureq::get(&format!("https://api.tosdr.org/search/v4/?query={}", query))
                .call()
                .unwrap();

            if *json {
                println!("{}", resp.into_string().unwrap());
                return;
            }

            let resp_json = resp.into_json::<structs::Resp>().unwrap();

            println!("Results for \"{}\":", query);
            for service in resp_json.parameters.services {
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
