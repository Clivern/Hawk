// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use log;

// Get license
pub fn license() -> Result<String, String> {
    log::info!("Start license command handler");

    Ok(format!("{}", "MIT License\n\nCopyright (c) 2021 Clivern"))
}

#[test]
fn test_license() {
    assert_eq!(
        license(),
        Ok(String::from("MIT License\n\nCopyright (c) 2021 Clivern"))
    );
}
