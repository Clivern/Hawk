// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

pub struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    pub fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    pub fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
}

#[test]
fn test_person() {
    let mut p = Person::new("John", "Smith");

    assert_eq!(p.first_name, "John".to_string());
    assert_eq!(p.last_name, "Smith".to_string());
    p.set_first_name("Joe");
    p.set_last_name("Doe");
    assert_eq!(p.first_name, "Joe".to_string());
    assert_eq!(p.last_name, "Doe".to_string());
}
