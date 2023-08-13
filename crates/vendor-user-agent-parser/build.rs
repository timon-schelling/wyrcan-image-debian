// Copyright © 2019 Phil Booth
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License. You may obtain
// a copy of the License at:
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied. See the License for the specific language governing
// permissions and limitations under the License.

use std::{env, fs::File, io::Write, path::Path};

use reqwest::Client;
use serde::Deserialize;

include!("src/parsers.rs");
define_parsers!(String: Deserialize);

#[tokio::main]
async fn main() {
    let dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&dir).join("parsers.rs");
    let mut file = File::create(&path).unwrap();

    let client = Client::new();
    let response = client
        .get("https://raw.githubusercontent.com/ua-parser/uap-core/master/regexes.yaml")
        .send()
        .await;
    let body = response.unwrap().text().await;

    let parsers: Parsers = serde_yaml::from_str(&body.unwrap()).unwrap();
    parsers.serialise(&mut file);
}

impl Parsers {
    fn serialise(&self, file: &mut File) {
        file.write_all(
            b"
use lazy_static::lazy_static;
use parking_lot::RwLock;

lazy_static! {
    static ref PARSERS: RwLock<Option<Parsers>> = RwLock::new(None);
}

/// Force initialisation of the underlying regular expressions.
///
/// Calling this function early on during your program's initialisation
/// will ensure that subsequent parser calls do not have to pay
/// the initialisation cost at runtime.
///
/// ## Usage
///
/// ```
/// fast_uaparser::init().unwrap();
/// ```
pub fn init() -> Result<bool, ParserError>
{
    let mut parsers = PARSERS.write();

    if parsers.is_some() {
        return Ok(false);
    }

    let mut user_agent_parsers = Vec::new();
    let mut os_parsers = Vec::new();
    let mut device_parsers = Vec::new();",
        )
        .unwrap();
        self.user_agent_parsers
            .iter()
            .for_each(|p| p.serialise(file));
        self.os_parsers.iter().for_each(|p| p.serialise(file));
        self.device_parsers.iter().for_each(|p| p.serialise(file));
        file.write_all(
            b"
    *parsers = Some(Parsers {
        user_agent_parsers,
        os_parsers,
        device_parsers,
    });

    Ok(true)
}",
        )
        .unwrap();
    }
}

impl UaParser {
    fn serialise(&self, file: &mut File) {
        let (builder, regex_flag) = serialise_regex_flag(&self.regex_flag);
        write!(
            file,
            "
    user_agent_parsers.push(UaParser {{
        regex: RegexBuilder::new({})
            .size_limit(20*(1<<20))
            {}
            .build()
            .map_err(|error| ParserError::InitError(error))?,
        regex_flag: {},
        family_replacement: {},
        v1_replacement: {},
        v2_replacement: {},
        v3_replacement: {},
    }});",
            serialise_str(&self.regex),
            builder,
            regex_flag,
            serialise_option(&self.family_replacement),
            serialise_option(&self.v1_replacement),
            serialise_option(&self.v2_replacement),
            serialise_option(&self.v3_replacement),
        )
        .unwrap();
    }
}

fn serialise_regex_flag(regex_flag: &Option<String>) -> (String, String) {
    let mut builder = "".to_owned();
    if let Some(ref regex_flag) = regex_flag {
        if regex_flag.contains("i") {
            builder.push_str(".case_insensitive(true)");
        }
        if regex_flag.contains("m") {
            builder.push_str(".multi_line(true)");
        }
        if regex_flag.contains("s") {
            builder.push_str(".dot_matches_new_line(true)");
        }
        if regex_flag.contains("u") {
            builder.push_str(".unicode(true)");
        }
    }
    (builder, serialise_option(regex_flag))
}

fn serialise_option(option: &Option<String>) -> String {
    if let Some(ref value) = option {
        format!("Some({})", serialise_string(value))
    } else {
        "None".to_owned()
    }
}

fn serialise_string(value: &str) -> String {
    format!("{}.to_owned()", serialise_str(value))
}

fn serialise_str(value: &str) -> String {
    format!("\"{}\"", &value.replace("\\", "\\\\").replace("\"", "\\\""))
}

impl OsParser {
    fn serialise(&self, file: &mut File) {
        let (builder, regex_flag) = serialise_regex_flag(&self.regex_flag);
        write!(
            file,
            "
    os_parsers.push(OsParser {{
        regex: RegexBuilder::new({})
            .size_limit(20*(1<<20))
            {}
            .build()
            .map_err(|error| ParserError::InitError(error))?,
        regex_flag: {},
        os_replacement: {},
        os_v1_replacement: {},
        os_v2_replacement: {},
        os_v3_replacement: {},
        os_v4_replacement: {},
    }});",
            serialise_str(&self.regex),
            builder,
            regex_flag,
            serialise_option(&self.os_replacement),
            serialise_option(&self.os_v1_replacement),
            serialise_option(&self.os_v2_replacement),
            serialise_option(&self.os_v3_replacement),
            serialise_option(&self.os_v4_replacement),
        )
        .unwrap();
    }
}

impl DeviceParser {
    fn serialise(&self, file: &mut File) {
        let (builder, regex_flag) = serialise_regex_flag(&self.regex_flag);
        write!(
            file,
            "
    device_parsers.push(DeviceParser {{
        regex: RegexBuilder::new({})
            .size_limit(20*(1<<20))
            {}
            .build()
            .map_err(|error| ParserError::InitError(error))?,
        regex_flag: {},
        device_replacement: {},
        brand_replacement: {},
        model_replacement: {},
    }});",
            serialise_str(&self.regex),
            builder,
            regex_flag,
            serialise_option(&self.device_replacement),
            serialise_option(&self.brand_replacement),
            serialise_option(&self.model_replacement),
        )
        .unwrap();
    }
}
