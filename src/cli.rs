use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    Search {
        #[clap(flatten)]
        args: CliSearchArgs,

        #[clap(short, long, help = "Show output as JSON")]
        json: bool,
    },
}

#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
pub struct CliSearchArgs {
    #[clap(short, long, help = "Search for a service by domain")]
    pub domain: Option<String>,

    #[clap(short, long, help = "Search for services")]
    pub query: Option<String>,
}
