pub mod api;
pub mod cli;

pub fn print_service<T: api::structs::ServiceRatingString>(service: api::structs::Service<T>) {
    println!("  - {} ({})", service.name, service.id);
    println!("    - {}", service.rating.to_rating_string());

    // Service URLs
    println!("    - URLs:");
    for url in service.urls {
        println!("      - {}", url);
    }

    // Link to service on Wikipedia
    println!("    - Wikipedia: {}", service.wikipedia);

    // Link to service on ToS;DR
    if let Some(links) = service.links {
        println!("    - ToS;DR: {}", links.crisp.service);
    } else {
        println!("    - ToS;DR: https://tosdr.org/en/service/{}", service.id);
    }

    // Points
    if let Some(points) = service.points {
        println!("    - Points:");
        for point in points {
            println!("      - {} ({})", point.title, point.id);
        }
    }
}
