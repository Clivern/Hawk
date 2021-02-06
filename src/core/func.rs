// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

// Pass by refernce
pub fn double1(x: &mut [i32; 10]) {
    for i in 0..10 {
        x[i] += 2;
    }
}

// Pass by value
pub fn double2(x: [i32; 10]) -> [i32; 10] {
    let mut y: [i32; 10] = x;
    for i in 0..10 {
        y[i] += 2;
    }
    y
}

// Pass by value
pub fn double3(mut x: [i32; 10]) -> [i32; 10] {
    for i in 0..10 {
        x[i] += 2;
    }
    x
}

// Generics
pub fn multi(x: i32, y: i32) -> i32 {
    x * y
}
