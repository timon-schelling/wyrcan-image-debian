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

macro_rules! define_parsers {
    ($regex_type:ty: $($derive_type:ident),+) => {
        #[derive($($derive_type),+)]
        struct Parsers {
            pub user_agent_parsers: Vec<UaParser>,
            pub os_parsers: Vec<OsParser>,
            pub device_parsers: Vec<DeviceParser>,
        }

        #[derive($($derive_type),+)]
        struct UaParser {
            pub regex: $regex_type,
            pub regex_flag: Option<String>,
            pub family_replacement: Option<String>,
            pub v1_replacement: Option<String>,
            pub v2_replacement: Option<String>,
            pub v3_replacement: Option<String>,
        }

        #[derive($($derive_type),+)]
        struct OsParser {
            pub regex: $regex_type,
            pub regex_flag: Option<String>,
            pub os_replacement: Option<String>,
            pub os_v1_replacement: Option<String>,
            pub os_v2_replacement: Option<String>,
            pub os_v3_replacement: Option<String>,
            pub os_v4_replacement: Option<String>,
        }

        #[derive($($derive_type),+)]
        struct DeviceParser {
            pub regex: $regex_type,
            pub regex_flag: Option<String>,
            pub device_replacement: Option<String>,
            pub brand_replacement: Option<String>,
            pub model_replacement: Option<String>,
        }
    }
}
