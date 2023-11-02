use tosdr_cli::api::structs;
use tosdr_cli::cli::CliSearchArgs;

pub fn main(args: &CliSearchArgs, json: &bool) {
    let was_domain = args.domain.is_some();
    let query = args
        .query
        .clone()
        .unwrap_or(args.domain.clone().unwrap_or("".to_string()));
    // TODO: there should be a better way to do this than cloning

    let request = ureq::get(&format!("https://api.tosdr.org/search/v4/?query={}", query)).call();

    match request {
        Ok(response) => {
            if *json {
                println!("{}", response.into_string().unwrap());
                return;
            }

            let response_json = response.into_json::<structs::SearchApiResponse>();

            match response_json {
                Ok(response_json) => {
                    println!("Results for \"{}\":", query);
                    for service in response_json.parameters.services {
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

                Err(error) => {
                    println!("Error parsing ToS;DR API response: {}", error);
                }
            }
        }

        Err(error) => {
            println!("Error requesting ToS;DR API: {}", error);
        }
    }
}
