pub mod api;
pub mod cli;

pub fn print_service(service: api::structs::Service) {
    println!("  - {} ({})", service.name, service.id);
    println!("    - {}", service.rating.human);
    println!("    - URLs:");
    for url in service.urls {
        println!("      - {}", url);
    }
    println!("    - Wikipedia: {}", service.wikipedia);
    println!("    - ToS;DR: {}", service.links.crisp.service)
}
