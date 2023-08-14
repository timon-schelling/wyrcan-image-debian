use std::{str::FromStr, fmt::Display};

use axum::{
    body::BoxBody,
    extract::Path,
    http::{HeaderMap, HeaderValue, Response, StatusCode},
    response::{IntoResponse, Redirect, Html},
    routing::get,
    Router,
};
use fast_uaparser::{OperatingSystem, ParserError};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new().route("/:zone/:route", get(handler))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Zone {
    name: Option<String>,
    routes: Option<Vec<Route>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Route {
    name: Option<String>,
    target: Option<Target>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum Target {
    Url { url: String },
    Random { targets: Vec<Target> },
    YouTube { video: String },
    Spotify { track: String },
    Image { url: String },
}

#[derive(Clone, Debug)]
enum Os {
    Linux,
    Android,
    Windows,
    Darwin,
    AppleMobile,
    Unknown,
}

impl FromStr for Os {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let os: OperatingSystem = s.parse()?;
        match os.family.as_str() {
            "Linux" => Ok(Os::Linux),
            "Android" => Ok(Os::Android),
            "Windows" => Ok(Os::Windows),
            "Darwin" | "Mac OS" | "Mac OS X" => Ok(Os::Darwin),
            "iOS" | "tvOS" | "WatchOS" => Ok(Os::AppleMobile),
            _ => Ok(Os::Unknown),
        }
    }
}

#[derive(Clone, Debug)]
enum Resolved {
    Redirect(String),
    Html(String),
}

impl Display for Resolved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resolved::Redirect(url) => write!(f, "Redirect({})", url),
            Resolved::Html(html) => write!(f, "HTML(\n{}\n)", html),
        }
    }
}

impl Target {
    fn resolve(&self, headers: HeaderMap) -> Option<Resolved> {
        let user_agent_str = headers.get("user-agent")?.to_str().ok()?;
        let os = user_agent_str.parse::<Os>().ok()?;
        match self {
            Target::Url { url } => Some(Resolved::Redirect(url.clone())),
            Target::Random { targets } => {
                let target = targets.choose(&mut rand::thread_rng())?;
                target.resolve(headers)
            },
            Target::YouTube { video } => Some(Resolved::Redirect(match os {
                Os::Android => format!("intent://youtu.be/{}#Intent;package=com.google.android.youtube;scheme=https;end", video),
                Os::AppleMobile => format!("vnd.youtube://www.youtube.com/watch?v={}", video),
                _ => format!("https://www.youtube.com/watch?v={}", video),
            })),
            Target::Spotify { track } => Some(Resolved::Redirect(match os {
                Os::Android => format!("intent://open.spotify.com/track/{}#Intent;package=com.spotify.music;scheme=https;end", track),
                Os::AppleMobile => format!("spotify://track/{}", track),
                _ => format!("https://open.spotify.com/track/{}", track),
            })),
            Target::Image { url } => {
                let html = format!(r#"<!DOCTYPE html>
                    <html>
                        <head>
                            <meta name="viewport" content="width=device-width, initial-scale=1">
                            <style>
                                body, html {{
                                    height: 100%;
                                    margin: 0;
                                    background-color: #141414;
                                    color: #414143;
                                }}
                                .bg {{
                                    background-image: url("{}");
                                    height: 100%;
                                    background-position: center;
                                    background-repeat: no-repeat;
                                    background-size: contain;
                                }}
                            </style>
                        </head>
                        <body>
                            <div class="bg"></div>
                        </body>
                    </html>
                "#, url);
                Some(Resolved::Html(html))
            },
        }
    }
}

async fn handler(
    headers: HeaderMap,
    Path((zone_name, route_name)): Path<(String, String)>,
) -> Response<BoxBody> {
    let resp = get_raw_file_content().await;

    let zones_str = match resp {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let zones_str = zones_str.as_str();

    let zones = serde_yaml::Deserializer::from_str(zones_str).map(Zone::deserialize);

    let zone = zones
        .filter_map(|r| match r {
            Ok(r) => Some(r),
            Err(e) => {
                println!("Error {}", e);
                None
            }
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

    let target = match &route.target {
        Some(target) => target,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let resolved = match target.resolve(headers) {
        Some(target) => target,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    match resolved {
        Resolved::Html(_) => {
            tracing::info!("{}/{} -> HTML", zone_name, route_name);
            tracing::debug!("{}/{} -> {}", zone_name, route_name, resolved)
        },
        _ => tracing::info!("{}/{} -> {}", zone_name, route_name, resolved),
    }


    match resolved {
        Resolved::Redirect(url) => {
            Redirect::temporary(&url).into_response()
        }
        Resolved::Html(html) => {
            Html(html).into_response()
        }
    }

}

async fn get_raw_file_content() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let owner = "timon-schelling";
    let repo = "rtrs";
    let path = "zones.yaml";

    let token = "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";

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
