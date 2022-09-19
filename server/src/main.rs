#![warn(clippy::nursery, clippy::pedantic)]

use clap::Parser;
use warp::Filter;

/// Command line arguments for server
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Sets the port of the embedded webserver
    #[clap(parse(try_from_str))]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Command line parsing
    let args = Args::parse();

    // Health check route for service managers
    let health = warp::path!("health").map(|| "OK");

    // Static file warp routes
    let static_route = warp::fs::dir("www");

    // Warp routes for the server
    let routes = health.or(static_route);

    // Running the server with routes
    warp::serve(routes).run(([0, 0, 0, 0], args.port)).await;

    Ok(())
}
