# fast_uaparser

[![Build status](https://gitlab.com/philbooth/uaparser-rs/badges/master/pipeline.svg)](https://gitlab.com/philbooth/fast_uaparser/pipelines)
[![Crate status](https://img.shields.io/crates/v/fast_uaparser.svg)](https://crates.io/crates/fast_uaparser)
[![Downloads](https://img.shields.io/crates/d/fast_uaparser.svg)](https://crates.io/crates/fast_uaparser)
[![License](https://img.shields.io/crates/l/fast_uaparser.svg)](https://www.apache.org/licenses/LICENSE-2.0)

Parse [User-Agent request header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) strings.

* [How does it work?](#how-does-it-work)
* [How do I install it?](#how-do-i-install-it)
* [How do I use it?](#how-do-i-use-it)
* [How do I set up the dev environment?](#how-do-i-set-up-the-dev-environment)
* [Where are the API docs?](#where-are-the-api-docs)
* [Where is the change log?](#where-is-the-change-log)
* [What license is it released under?](#what-license-is-it-released-under)

## How does it work?

Parsers are derived from regular expressions
published in the [ua-parser/ua-core](https://github.com/ua-parser/uap-core) repository.
The regular expressions are fetched in a custom build step,
then used to generate static Rust code
that is compiled into the lib.

There is a one-off initialisation cost
to load the parser objects at runtime,
which is paid when you call
the [`init` function](https://philbooth.gitlab.io/uaparser-rs/fast_uaparser/fn.init.html).
If `init` is not called explicitly,
initialisation occurs lazily instead
and parsing will block
until it finishes.

## How do I install it?

Add it to your dependencies
in `Cargo.toml`:

```toml
[dependencies]
fast_uaparser = "1"
```

## How do I use it?

For more detailed information
see the [API docs](https://philbooth.gitlab.io/uaparser-rs/fast_uaparser/),
but the general gist
is as follows:

```rust
use fast_uaparser::{Device, OperatingSystem, UserAgent};

// Pay initialisation costs up-front
fast_uaparser::init().unwrap();

// Parse user-agent information
let ua: UserAgent =
    "Mozilla/5.0 (X11; Linux i686; rv:70.0) Gecko/20100101 Firefox/70.0"
        .parse()
        .unwrap();

assert_eq!(ua.family, "Firefox");
assert_eq!(ua.version.major.unwrap(), "70");
assert_eq!(ua.version.minor.unwrap(), "0");
assert!(ua.version.patch.is_none());
assert!(ua.version.patch_minor.is_none());

// Parse OS information
let os: OperatingSystem =
    "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.1 Mobile/15E148 Safari/604.1"
        .parse()
        .unwrap();

assert_eq!(os.family, "iOS");
assert_eq!(os.version.major.unwrap(), "12");
assert_eq!(os.version.minor.unwrap(), "2");
assert!(os.version.patch.is_none());
assert!(os.version.patch_minor.is_none());

// Parse device information
let device: Device =
    "Mozilla/5.0 (Windows Mobile 10; Android 8.0.0; Microsoft; Lumia 950XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.89 Mobile Safari/537.36 Edge/40.15254.369"
        .parse()
        .unwrap();

assert_eq!(device.family, "Lumia 950XL");
assert_eq!(device.brand.unwrap(), "Nokia");
assert_eq!(device.model.unwrap(), "Lumia 950XL");
```

## How do I set up the dev environment?

f you don't already have Rust installed,
get that first using [`rustup`](https://rustup.rs/):

```
curl https://sh.rustup.rs -sSf | sh
```

Then you can build the project:

```
cargo b
```

And run the tests:

```
cargo t
```

## Where are the API docs?

[Here](https://philbooth.gitlab.io/uaparser-rs/fast_uaparser/).

## Where is the change log?

[Here](CHANGELOG.md)

## What license is it released under?

[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0).
