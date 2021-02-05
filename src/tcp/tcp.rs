// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::net::TcpListener;

// SingleThreadedTcp Type
pub struct SingleThreadedTcp {
    address: String,
}

// SingleThreadedTcp Methods
impl SingleThreadedTcp {
    pub fn new(address: &str) -> SingleThreadedTcp {
        SingleThreadedTcp {
            address: address.to_string(),
        }
    }

    // Set address
    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }

    // Get address
    pub fn get_address(self) -> String {
        self.address
    }

    // Listen to connections
    pub fn listen(self) {
        let listener = TcpListener::bind(self.address.to_string()).unwrap();

        println!("Listen to tcp connection on {}..", self.address.to_string());

        for stream in listener.incoming() {
            let _stream = stream.unwrap();
            println!("Connection established!");
        }
    }
}
