pub mod structs;

pub fn request(url: String) -> ureq::Response {
    let request = ureq::get(&url).call();

    match request {
        Ok(response) => {
            return response;
        }

        Err(error) => {
            eprintln!("Error requesting ToS;DR API: {}", error);
            std::process::exit(2);
        }
    }
}

pub fn request_json<T: serde::de::DeserializeOwned>(url: String) -> T {
    match request(url).into_json::<T>() {
        Ok(response_json) => {
            return response_json;
        }

        Err(error) => {
            eprintln!("Error parsing ToS;DR API response: {}", error);
            std::process::exit(3);
        }
    }
}
