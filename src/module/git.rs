// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::process::Command;
use std::str;

// Git type
pub struct Git {}

// Git type methods
impl Git {
    // print git version
    pub fn print_version(&self) {
        Command::new("sh")
            .arg("-c")
            .arg("git version")
            .spawn()
            .expect("failed to execute process");
    }

    // get git version
    pub fn get_version(&self) -> String {
        let out = Command::new("sh")
            .arg("-c")
            .arg("git version")
            .output()
            .expect("failed to execute process");

        return format!("{}", str::from_utf8(&*out.stdout).unwrap());
    }
}
