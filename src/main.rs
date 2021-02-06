// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

mod networking {
    pub mod tcp;
}

// import util
mod util {
    pub mod environ;
    pub mod io;
}

fn main() {
    // Single Threaded Tcp Server
    // let server = networking::tcp::SingleThreadedTcp::new("127.0.0.1:8000");
    // server.listen();
    println!("Seal!");
}
