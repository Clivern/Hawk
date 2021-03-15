// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

// file module
pub mod file {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    // Check if path exists
    pub fn path_exists(path: &str) -> bool {
        return Path::new(path).exists();
    }

    // Check if path is file
    pub fn is_file(path: &str) -> bool {
        return Path::new(path).is_file();
    }

    // Write to file
    pub fn write_file(path: &str, content: &str) -> bool {
        let mut file = File::create(path).expect("create file failed");
        file.write_all(content.as_bytes())
            .expect("write file failed");
        return true;
    }

    // Delete file
    pub fn delete_file(path: &str) -> bool {
        fs::remove_file(path).expect("could not remove file");
        return true;
    }

    // Write file
    pub fn read_file(path: &str) -> String {
        let contents =
            fs::read_to_string(path).expect("Something went wrong while reading file");
        return format!("{}", contents);
    }
}

// dir module
pub mod dir {
    use std::path::Path;

    // Check if path is dir
    pub fn is_dir(path: &str) -> bool {
        return Path::new(path).is_dir();
    }
}

#[test]
fn test_path_exists() {
    assert_eq!(file::path_exists("not_found.md"), false);
    assert_eq!(file::path_exists("README.md"), true);
    assert_eq!(file::path_exists("src"), true);
}

#[test]
fn test_is_file_exists() {
    assert_eq!(file::is_file("README.md"), true);
    assert_eq!(file::is_file("main.rs"), false);
}

#[test]
fn test_is_dir_exists() {
    assert_eq!(dir::is_dir("src"), true);
    assert_eq!(dir::is_dir("dir"), false);
}

#[test]
fn test_write_to_file() {
    assert_eq!(file::write_file("cache/data1.txt", "Line 1\nLine 2."), true);
    assert_eq!(file::is_file("cache/data1.txt"), true);
    assert_eq!(file::delete_file("cache/data1.txt"), true);
}

#[test]
fn test_delete_file() {
    assert_eq!(file::write_file("cache/data2.txt", "Line 1\nLine 2."), true);
    assert_eq!(file::delete_file("cache/data2.txt"), true);
    assert_eq!(file::is_file("cache/data2.txt"), false);
}

#[test]
fn test_read_file() {
    assert_eq!(file::write_file("cache/data3.txt", "Line 1\nLine 2."), true);
    assert_eq!(
        file::read_file("cache/data3.txt"),
        "Line 1\nLine 2.".to_string()
    );
    assert_eq!(file::delete_file("cache/data3.txt"), true);
}
