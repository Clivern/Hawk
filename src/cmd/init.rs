// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use log;

// init command
pub fn init(config: &str) -> Result<String, String> {
    log::info!("Start init command handler with config file: {}", config);

    Ok(String::from(""))
}
