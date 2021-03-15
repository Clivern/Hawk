// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

// environ module
pub mod environ {
    use std::env;

    // Get config var
    pub fn get_config(key: &str) -> String {
        return env::var(key).unwrap();
    }
}

#[test]
fn test_get_config() {
    assert_eq!(environ::get_config("CARGO_PKG_NAME"), "hawk_rs");
}
