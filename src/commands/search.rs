use tosdr_cli::api::{request, request_json, structs};
use tosdr_cli::cli::CliSearchArgs;
use tosdr_cli::print_service;

pub fn main(args: &CliSearchArgs, json: &bool) {
    let was_domain = args.domain.is_some();
    let query = args
        .query
        .clone()
        .unwrap_or(args.domain.clone().unwrap_or("".to_string()));
    // TODO: there should be a better way to do this than cloning

    let url = format!("https://api.tosdr.org/search/v4/?query={}", query);

    if *json {
        let response = request(url);
        println!("{}", response.into_string().unwrap());
        return;
    }

    let response_json = request_json::<structs::SearchApiResponse>(url);

    println!("Results for \"{}\":", query);
    for service in response_json.parameters.services {
        if was_domain && !service.urls.contains(&query) {
            continue;
        }

        print_service(service);
    }
}
