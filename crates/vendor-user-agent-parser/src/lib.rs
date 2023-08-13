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

//! Parse strings from the [User-Agent request header][uaheader].
//!
//! The parsers are derived from regular expressions published in the
//! [ua-parser/ua-core][uacore] repository. The regular expressions are
//! fetched in a custom build step, then used to generate static Rust
//! code that is compiled into the lib.
//!
//! There is a one-off initialisation cost to load the parser objects at
//! runtime, which is paid when you call the [`init` function][init]. If
//! `init` is not called explicitly, initialisation occurs lazily instead
//! and parsing will block until it finishes.
//!
//! ## Usage
//!
//! ```
//! use fast_uaparser::{Device, OperatingSystem, UserAgent};
//!
//! // Pay initialisation costs
//! fast_uaparser::init().unwrap();
//!
//! // Parse user-agent information
//! let ua: UserAgent =
//!     "Mozilla/5.0 (X11; Linux i686; rv:70.0) Gecko/20100101 Firefox/70.0"
//!         .parse()
//!         .unwrap();
//!
//! assert_eq!(ua.family, "Firefox");
//! assert_eq!(ua.version.major.unwrap(), "70");
//! assert_eq!(ua.version.minor.unwrap(), "0");
//! assert!(ua.version.patch.is_none());
//! assert!(ua.version.patch_minor.is_none());
//!
//! // Parse OS information
//! let os: OperatingSystem =
//!     "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.1 Mobile/15E148 Safari/604.1"
//!         .parse()
//!         .unwrap();
//!
//! assert_eq!(os.family, "iOS");
//! assert_eq!(os.version.major.unwrap(), "12");
//! assert_eq!(os.version.minor.unwrap(), "2");
//! assert!(os.version.patch.is_none());
//! assert!(os.version.patch_minor.is_none());
//!
//! // Parse device information
//! let device: Device =
//!     "Mozilla/5.0 (Windows Mobile 10; Android 8.0.0; Microsoft; Lumia 950XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.89 Mobile Safari/537.36 Edge/40.15254.369"
//!         .parse()
//!         .unwrap();
//!
//! assert_eq!(device.family, "Lumia 950XL");
//! assert_eq!(device.brand.unwrap(), "Nokia");
//! assert_eq!(device.model.unwrap(), "Lumia 950XL");
//! ```
//!
//! [uaheader]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
//! [uacore]: https://github.com/ua-parser/uap-core
//! [init]: fn.init.html

#[cfg(test)]
mod test;

use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

use fancy_regex_fork_pb::{Captures, Error as RegexError, Regex, RegexBuilder};

include!("parsers.rs");
define_parsers!(Regex: Debug);
include!(concat!(env!("OUT_DIR"), "/parsers.rs"));

/// A parsed user-agent.
///
/// ## Usage
///
/// ```
/// use fast_uaparser::UserAgent;
///
/// let ua: UserAgent =
///     "Mozilla/5.0 (X11; Linux i686; rv:70.0) Gecko/20100101 Firefox/70.0"
///         .parse()
///         .unwrap();
///
/// assert_eq!(ua.family, "Firefox");
/// assert_eq!(ua.version.major.unwrap(), "70");
/// assert_eq!(ua.version.minor.unwrap(), "0");
/// assert!(ua.version.patch.is_none());
/// assert!(ua.version.patch_minor.is_none());
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct UserAgent {
    /// Family name, e.g. `Firefox`.
    pub family: String,

    /// Version details.
    pub version: Version,
}

/// Version information.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Version {
    /// Major revision.
    pub major: Option<String>,

    /// Minor revision.
    pub minor: Option<String>,

    /// Patch revision.
    pub patch: Option<String>,

    /// Minor patch revision.
    pub patch_minor: Option<String>,
}

/// A parsed operating system.
///
/// ## Usage
///
/// ```
/// use fast_uaparser::OperatingSystem;
///
/// let os: OperatingSystem =
///     "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0"
///         .parse()
///         .unwrap();
///
/// assert_eq!(os.family, "Mac OS X");
/// assert_eq!(os.version.major.unwrap(), "10");
/// assert_eq!(os.version.minor.unwrap(), "14");
/// assert!(os.version.patch.is_none());
/// assert!(os.version.patch_minor.is_none());
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct OperatingSystem {
    /// Family name, e.g. `Windows`.
    pub family: String,

    /// Version details.
    pub version: Version,
}

/// A parsed device.
///
/// ## Usage
///
/// ```
/// use fast_uaparser::Device;
///
/// let device: Device =
///     "Mozilla/5.0 (iPhone; CPU iPhone OS 12_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/18.1 Mobile/16B92 Safari/605.1.15"
///         .parse()
///         .unwrap();
///
/// assert_eq!(device.family, "iPhone");
/// assert_eq!(device.brand.unwrap(), "Apple");
/// assert_eq!(device.model.unwrap(), "iPhone");
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Device {
    /// Family name, e.g. `iPhone`.
    pub family: String,

    /// Brand name, e.g. `Apple`.
    pub brand: Option<String>,

    /// Model name.
    pub model: Option<String>,
}

/// Error type.
#[derive(Debug, PartialEq)]
pub enum ParserError {
    /// Failed to compile a regular expression during initialisation.
    InitError(RegexError),

    /// Failed to parse a user-agent string.
    ParseError(RegexError),
}

macro_rules! capture {
    ($parser:ident, $replacement:ident, $result:expr, $captures:ident, $capture_index:literal) => {
        if let Some(ref $replacement) = $parser.$replacement {
            $result = replace_captures($replacement, &$captures);
        } else {
            $result = $captures
                .get($capture_index)
                .map_or_else(|| "", |m| m.as_str())
                .to_owned();
        }
    };
}

macro_rules! optional_capture {
    ($parser:ident, $replacement:ident, $result:expr, $captures:ident, $capture_index:literal) => {
        if let Some(ref $replacement) = $parser.$replacement {
            $result = Some(replace_captures($replacement, &$captures));
        } else {
            $result = $captures.get($capture_index).and_then(|m| {
                let s = m.as_str();
                if s == "" {
                    None
                } else {
                    Some(s.to_owned())
                }
            });
        }
    };
}

fn replace_captures(string: &str, captures: &Captures) -> String {
    let mut result = string.to_owned();
    for i in 1..captures.len() {
        result = result.replace(
            &format!("${}", i),
            captures.get(i).map_or_else(|| "", |m| m.as_str()),
        );
    }
    result
}

/// User-agent parsing is implemented using the `FromStr` trait.
impl FromStr for UserAgent {
    type Err = ParserError;

    /// Parse a user-agent from a user-agent string.
    fn from_str(user_agent: &str) -> Result<Self, Self::Err> {
        optional_init(|parsers| {
            let mut result = Self::default();

            for parser in parsers.user_agent_parsers.iter() {
                if let Some(captures) = parser
                    .regex
                    .captures(user_agent)
                    .map_err(|error| ParserError::ParseError(error))?
                {
                    capture!(parser, family_replacement, result.family, captures, 1);
                    optional_capture!(parser, v1_replacement, result.version.major, captures, 2);
                    optional_capture!(parser, v2_replacement, result.version.minor, captures, 3);
                    optional_capture!(parser, v3_replacement, result.version.patch, captures, 4);
                    break;
                }
            }

            Ok(result)
        })
    }
}

fn optional_init<T, F>(action: F) -> Result<T, ParserError>
where
    F: Fn(&Parsers) -> Result<T, ParserError>,
{
    {
        if let Some(ref parsers) = *PARSERS.read() {
            return action(parsers);
        }
    }

    init()?;
    action(PARSERS.read().as_ref().unwrap())
}

/// OS parsing is implemented using the `FromStr` trait.
impl FromStr for OperatingSystem {
    type Err = ParserError;

    /// Parse an operating system from a user-agent string.
    fn from_str(user_agent: &str) -> Result<Self, Self::Err> {
        optional_init(|parsers| {
            let mut result = Self::default();

            for parser in parsers.os_parsers.iter() {
                if let Some(captures) = parser
                    .regex
                    .captures(user_agent)
                    .map_err(|error| ParserError::ParseError(error))?
                {
                    capture!(parser, os_replacement, result.family, captures, 1);
                    optional_capture!(parser, os_v1_replacement, result.version.major, captures, 2);
                    optional_capture!(parser, os_v2_replacement, result.version.minor, captures, 3);
                    optional_capture!(parser, os_v3_replacement, result.version.patch, captures, 4);
                    optional_capture!(
                        parser,
                        os_v4_replacement,
                        result.version.patch_minor,
                        captures,
                        5
                    );
                    break;
                }
            }

            Ok(result)
        })
    }
}

/// Device parsing is implemented using the `FromStr` trait.
impl FromStr for Device {
    type Err = ParserError;

    /// Parse a device from a user-agent string.
    fn from_str(user_agent: &str) -> Result<Self, Self::Err> {
        optional_init(|parsers| {
            let mut result = Self::default();

            for parser in parsers.device_parsers.iter() {
                if let Some(captures) = parser
                    .regex
                    .captures(user_agent)
                    .map_err(|error| ParserError::ParseError(error))?
                {
                    capture!(parser, device_replacement, result.family, captures, 1);
                    optional_capture!(parser, brand_replacement, result.brand, captures, 2);
                    optional_capture!(parser, model_replacement, result.model, captures, 3);
                    break;
                }
            }

            Ok(result)
        })
    }
}

impl Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParserError::InitError(ref inner) => {
                write!(formatter, "initialisation failed: {}", inner)
            }
            ParserError::ParseError(ref inner) => write!(formatter, "parse failed: {}", inner),
        }
    }
}

impl Error for ParserError {}
