use std::net::{IpAddr, SocketAddr};

use clap::{command, crate_name, crate_version, crate_authors, crate_description, arg};

#[tokio::main]
async fn main() {
    let matches = command!(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(arg!(-p --port <PORT> "Sets server port").default_value("8080"))
        .arg(arg!(--host <HOST> "Sets server host").default_value("::1"))
        .get_matches();

    let host = match matches.get_one::<String>("host") {
        Some(host) => host,
        None => todo!(),
    };

    let ip = match host.parse::<IpAddr>() {
        Ok(ip) => ip,
        Err(_) => todo!(),
    };

    let port_str = match matches.get_one::<String>("port") {
        Some(host) => host,
        None => todo!(),
    };

    let port = match port_str.parse::<u16>() {
        Ok(port) => port,
        Err(_) => todo!(),
    };

    let address = SocketAddr::new(ip, port);

    let router = rtrs::router();

    println!("Initializing user agent parser");
    fast_uaparser::init().expect("Failed to initialize user agent parser");

    println!("Server listening on {}", address);
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
