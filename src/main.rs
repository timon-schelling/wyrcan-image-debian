use std::net::{SocketAddr, IpAddr};

use clap::{arg, crate_authors, crate_description, crate_name, crate_version, command};

use axum::{Router, routing::get, extract::Path, response::{IntoResponse, Redirect}, http::{Response, StatusCode}, body::BoxBody};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let matches = command!(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            arg!(-p --port <PORT> "Sets server port").default_value("8080"),
        )
        .arg(
            arg!(--host <HOST> "Sets server host").default_value("::1"),
        )
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

    let router = Router::new().route("/:user/:route", get(handler));

    println!("Server listening on {}", address);
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Route {
    name: Option<String>,
    target: Option<String>,
    targets: Option<Vec<String>>,
}

impl Route {
    fn targets(self) -> Vec<String> {
        let mut targets = match  self.targets {
            Some(targets) => targets,
            None => vec![],
        };

        if let Some(target) = self.target {
            targets.push(target);
        }

        targets
    }
}

async fn handler(
    Path((user, route)): Path<(String, String)>,
) -> Response<BoxBody> {
    let user_str = match std::fs::read_to_string(format!("{}.yaml", user)) {
        Ok(s) => s,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let route = route.clone();

    let route = serde_yaml::Deserializer::from_str(user_str.as_str()).map(Route::deserialize).filter_map(|r| {
        match r {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }).filter(|r| match r.name {
        Some(ref name) => name == &route,
        None => false,
    }).next();

    let route = match route {
        Some(route) => route,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let targets = route.targets();
    let target = match targets.choose(&mut rand::thread_rng()) {
        Some(target) => target,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    Redirect::temporary(&target).into_response()
}
