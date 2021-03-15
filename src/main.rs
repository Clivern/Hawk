// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

// import cmd
mod cmd {
    pub mod build;
    pub mod check;
    pub mod init;
    pub mod license;
    pub mod release;
}

// import module
mod module {
    pub mod git;
}

// import util
mod util {
    pub mod environ;
    pub mod io;
}

use clap::{App, SubCommand};
use log;

fn main() {
    env_logger::init();

    log::info!("Run hawk_rs with info logs");

    let version = "0.0.1";

    let mut config = ".hawk_rs.yml";

    let _matches = App::new("hawk_rs")
        .version(version)
        .author("Clivern <hello@clivern.com>")
        .about("Release Rust projects easily!")
        .args_from_usage("--config=[FILE] 'Sets a custom config file'")
        .subcommand(SubCommand::with_name("init").about("Generate .hawk_rs.yml file"))
        .subcommand(SubCommand::with_name("build").about("Build the current project"))
        .subcommand(
            SubCommand::with_name("release").about("Release the current project"),
        )
        .subcommand(
            SubCommand::with_name("check").about("Validate config file .hawk_rs.yml"),
        )
        .subcommand(SubCommand::with_name("license").about("Show hawk_rs license"))
        .get_matches();

    // Override default config
    if let Some(c) = _matches.value_of("config") {
        if util::io::file::is_file(c) {
            config = c;
            log::info!("Override config file: {}", config);
        } else {
            log::info!("Use default config file: {}", config);
        }
    }

    // Init command
    if let Some(_matches) = _matches.subcommand_matches("init") {
        log::info!("Trigger init command handler");

        match cmd::init::init(&*config) {
            Ok(content) => {
                println!("{}", content);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    if let Some(_matches) = _matches.subcommand_matches("build") {
        log::info!("Trigger build command handler");

        match cmd::build::build(&*config) {
            Ok(content) => {
                println!("{}", content);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    if let Some(_matches) = _matches.subcommand_matches("release") {
        log::info!("Trigger release command handler");

        match cmd::release::release(&*config) {
            Ok(content) => {
                println!("{}", content);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    if let Some(_matches) = _matches.subcommand_matches("check") {
        log::info!("Trigger check command handler");

        match cmd::check::check(&*config) {
            Ok(content) => {
                println!("{}", content);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    if let Some(_matches) = _matches.subcommand_matches("license") {
        log::info!("Trigger license command handler");

        match cmd::license::license() {
            Ok(content) => {
                println!("{}", content);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
