// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

mod networking {
    pub mod tcp;
}

mod core {
    pub mod func;
}

mod util {
    pub mod environ;
    pub mod io;
}

fn main() {
    //
    // Networking
    //
    // Single Threaded Tcp Server
    // let server = networking::tcp::SingleThreadedTcp::new("127.0.0.1:8000");
    // server.listen();
    println!("Seal!");

    //
    // Core
    //
    // let mut ar: [i32; 10] = [3; 10];
    // core::func::double1(&mut ar);
    // println!("{:?}", ar);

    // println!("{:?}", core::func::double2(ar));
    // println!("{:?}", core::func::double3(ar));

    // let mut a: i32 = 10;
    // let p: &mut i32 = &mut a;

    //*p = 40;

    // println!("{}", p);

    // Casting
    // println!("{}", core::func::multi(2i32, 4i64 as i32) as i64);
}
