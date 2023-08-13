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

use super::{Device, OperatingSystem, UserAgent};

#[test]
fn init() {
    super::init().unwrap();
}

#[test]
fn firefox() {
    let ua: UserAgent = "Mozilla/5.0 (X11; Linux i686; rv:70.0) Gecko/20100101 Firefox/70.0"
        .parse()
        .unwrap();

    assert_eq!(ua.family, "Firefox");
    assert_eq!(ua.version.major.unwrap(), "70");
    assert_eq!(ua.version.minor.unwrap(), "0");
    assert!(ua.version.patch.is_none());
    assert!(ua.version.patch_minor.is_none());

    let os: OperatingSystem =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14.0; rv:70.0) Gecko/20100101 Firefox/70.0"
            .parse()
            .unwrap();

    assert_eq!(os.family, "Mac OS X");
    assert_eq!(os.version.major.unwrap(), "10");
    assert_eq!(os.version.minor.unwrap(), "14");
    assert_eq!(os.version.patch.unwrap(), "0");
    assert!(os.version.patch_minor.is_none());

    let device: Device = "Mozilla/5.0 (iPhone; CPU iPhone OS 12_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/18.1 Mobile/16B92 Safari/605.1.15"
        .parse()
        .unwrap();

    assert_eq!(device.family, "iPhone");
    assert_eq!(device.brand.unwrap(), "Apple");
    assert_eq!(device.model.unwrap(), "iPhone");
}

#[test]
fn safari() {
    let ua: UserAgent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.1.2 Safari/605.1.15"
        .parse()
        .unwrap();

    assert_eq!(ua.family, "Safari");
    assert_eq!(ua.version.major.unwrap(), "12");
    assert_eq!(ua.version.minor.unwrap(), "1");
    assert_eq!(ua.version.patch.unwrap(), "2");
    assert!(ua.version.patch_minor.is_none());

    let os: OperatingSystem = "Mozilla/5.0 (iPhone; CPU iPhone OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.1 Mobile/15E148 Safari/604.1"
        .parse()
        .unwrap();

    assert_eq!(os.family, "iOS");
    assert_eq!(os.version.major.unwrap(), "12");
    assert_eq!(os.version.minor.unwrap(), "2");
    assert!(os.version.patch.is_none());
    assert!(os.version.patch_minor.is_none());

    let device: Device = "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.1 Mobile/15E148 Safari/604.1"
        .parse()
        .unwrap();

    assert_eq!(device.family, "iPad");
    assert_eq!(device.brand.unwrap(), "Apple");
    assert_eq!(device.model.unwrap(), "iPad");
}

#[test]
fn chrome() {
    let ua: UserAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36"
        .parse()
        .unwrap();

    assert_eq!(ua.family, "Chrome");
    assert_eq!(ua.version.major.unwrap(), "76");
    assert_eq!(ua.version.minor.unwrap(), "0");
    assert_eq!(ua.version.patch.unwrap(), "3809");
    assert!(ua.version.patch_minor.is_none());

    let os: OperatingSystem = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36"
        .parse()
        .unwrap();

    assert_eq!(os.family, "Linux");
    assert!(os.version.major.is_none());
    assert!(os.version.minor.is_none());
    assert!(os.version.patch.is_none());
    assert!(os.version.patch_minor.is_none());

    let device: Device = "Mozilla/5.0 (Linux; Android 8.0.0;) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.89 Mobile Safari/537.36"
        .parse()
        .unwrap();

    assert_eq!(device.family, "Generic Smartphone");
    assert_eq!(device.brand.unwrap(), "Generic");
    assert_eq!(device.model.unwrap(), "Smartphone");
}

#[test]
fn edge() {
    let ua: UserAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36 Edg/44.18362.267.0"
        .parse()
        .unwrap();

    assert_eq!(ua.family, "Edge");
    assert_eq!(ua.version.major.unwrap(), "44");
    assert_eq!(ua.version.minor.unwrap(), "18362");
    assert_eq!(ua.version.patch.unwrap(), "267");
    assert!(ua.version.patch_minor.is_none());

    let os: OperatingSystem = "Mozilla/5.0 (Windows Mobile 10; Android 8.0.0; Microsoft; Lumia 950XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.89 Mobile Safari/537.36 Edge/40.15254.369"
        .parse()
        .unwrap();

    assert_eq!(os.family, "Android");
    assert_eq!(os.version.major.unwrap(), "8");
    assert_eq!(os.version.minor.unwrap(), "0");
    assert_eq!(os.version.patch.unwrap(), "0");
    assert!(os.version.patch_minor.is_none());

    let device: Device = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; Xbox; Xbox One) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36 Edge/40.15063.0"
        .parse()
        .unwrap();

    assert_eq!(device.family, "");
    assert!(device.brand.is_none());
    assert!(device.model.is_none());
}

#[test]
fn bots() {
    let ua: UserAgent = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
        .parse()
        .unwrap();

    assert_eq!(ua.family, "Googlebot");
    assert_eq!(ua.version.major.unwrap(), "2");
    assert_eq!(ua.version.minor.unwrap(), "1");
    assert!(ua.version.patch.is_none());
    assert!(ua.version.patch_minor.is_none());

    let os: OperatingSystem = "Googlebot/2.1 (+http://www.google.com/bot.html)"
        .parse()
        .unwrap();

    assert_eq!(os.family, "");
    assert!(os.version.major.is_none());
    assert!(os.version.minor.is_none());
    assert!(os.version.patch.is_none());
    assert!(os.version.patch_minor.is_none());

    let device: Device = "Mozilla/5.0 (Linux; Android 6.0.1; Nexus 5X Build/MMB29P) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.96 Mobile Safari/537.36 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
        .parse()
        .unwrap();

    assert_eq!(device.family, "Spider");
    assert_eq!(device.brand.unwrap(), "Spider");
    assert_eq!(device.model.unwrap(), "Smartphone");
}

#[test]
fn fennec() {
    let ua: UserAgent =
        "Mozilla/5.0 (Android; Linux armv7l; rv:9.0b12) Gecko/20111216 Fennec/9.0b12"
            .parse()
            .unwrap();

    assert_eq!(ua.family, "Firefox Mobile");
    assert_eq!(ua.version.major.unwrap(), "9");
    assert_eq!(ua.version.minor.unwrap(), "0");
    assert_eq!(ua.version.patch.unwrap(), "b12");
    assert!(ua.version.patch_minor.is_none());
}

#[test]
fn t_mobile() {
    let device: Device =
        "Mozilla/5.0 (T-Mobile myTouch 3G wibble Build 42; rv:70.0) Gecko/20100101 Firefox/70.0"
            .parse()
            .unwrap();

    assert_eq!(device.family, "T-Mobile myTouch 3G wibble");
    assert_eq!(device.brand.unwrap(), "HTC");
    assert_eq!(device.model.unwrap(), "myTouch 3G wibble");
}

#[test]
fn last_ua() {
    let ua: UserAgent = "Viafree-tvOS-EN/42.42.42".parse().unwrap();

    assert_eq!(ua.family, "ViaFree");
    assert_eq!(ua.version.major.unwrap(), "42");
    assert_eq!(ua.version.minor.unwrap(), "42");
    assert_eq!(ua.version.patch.unwrap(), "42");
    assert!(ua.version.patch_minor.is_none());
}

#[test]
fn last_os() {
    let os: OperatingSystem = "Roku/DVP-42.42".parse().unwrap();

    assert_eq!(os.family, "Roku");
    assert_eq!(os.version.major.unwrap(), "42");
    assert_eq!(os.version.minor.unwrap(), "42");
    assert!(os.version.patch.is_none());
    assert!(os.version.patch_minor.is_none());
}

#[test]
fn last_device() {
    let device: Device = "WAP Up.Browser".parse().unwrap();

    assert_eq!(device.family, "Generic Feature Phone");
    assert_eq!(device.brand.unwrap(), "Generic");
    assert_eq!(device.model.unwrap(), "Feature Phone");
}
