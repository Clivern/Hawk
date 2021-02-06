// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

// dir module
pub mod dir {
    use std::path::Path;

    // Check if path is dir
    pub fn is_dir(path: &str) -> bool {
        Path::new(path).is_dir()
    }
}

#[test]
fn test_is_dir_exists() {
    assert_eq!(dir::is_dir("src"), true);
    assert_eq!(dir::is_dir("dir"), false);
}
