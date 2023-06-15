use std::net::{IpAddr, SocketAddr};

use clap::{arg, command, crate_authors, crate_description, crate_name, crate_version};

use axum::{
    body::BoxBody,
    extract::Path,
    http::{HeaderValue, Response, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

    let router = Router::new().route("/:zone/:route", get(handler));

    println!("Server listening on {}", address);
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Zone {
    name: Option<String>,
    routes: Option<Vec<Route>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Route {
    name: Option<String>,
    targets: Option<Vec<String>>,
}

async fn handler(Path((zone_name, route_name)): Path<(String, String)>) -> Response<BoxBody> {
    let resp = get_raw_file_content("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".to_string()).await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let zones_str = resp;

    // let zones_str = match std::fs::read_to_string("zones.yaml") {
    //     Ok(s) => s,
    //     Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    // };
    let zones_str = zones_str.as_str();

    let zones = serde_yaml::Deserializer::from_str(zones_str).map(Zone::deserialize);

    let zone = zones
        .filter_map(|r| match r {
            Ok(r) => Some(r),
            Err(_) => None,
        })
        .filter(|r| match r.name {
            Some(ref name) => name == &zone_name,
            None => false,
        })
        .next();

    let routes = match zone {
        Some(zone) => zone.routes,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let routes = match routes {
        Some(routes) => routes,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let route = routes
        .iter()
        .filter(|r| match r.name {
            Some(ref name) => name == &route_name,
            None => false,
        })
        .next();

    let route = match route {
        Some(route) => route,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let targets = match &route.targets {
        Some(targets) => targets,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let target = match targets.choose(&mut rand::thread_rng()) {
        Some(target) => target,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    println!("{}/{} -> {}", zone_name, route_name, target);

    Redirect::temporary(&target).into_response()
}

async fn get_raw_file_content(token: String) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let owner = "timon-schelling";
    let repo = "rtrs";
    let path = "zones.yaml";

    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        owner, repo, path
    );

    let response = client
        .get(&url)
        .header("User-Agent", HeaderValue::from_static("test/0.1.0 (test)"))
        .header(
            "Accept",
            HeaderValue::from_static("application/vnd.github.raw"),
        )
        .header(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .send()
        .await?;

    let content = response.text().await?;

    Ok(content)
}
