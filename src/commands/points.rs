use tosdr_cli::api::{request, request_json, structs};
use tosdr_cli::print_service;

pub fn main(service_id: &u32, json: &bool) {
    let url = format!("https://api.tosdr.org/rest-service/v3/{}.json", service_id);

    if *json {
        let response = request(url);
        println!("{}", response.into_string().unwrap());
        return;
    }

    let response_json = request_json::<structs::ServiceApiResponse>(url);

    print_service(response_json.parameters);
}
