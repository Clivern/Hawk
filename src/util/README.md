#### IO Functions

```rust
io::file::path_exists("file.md");
io::file::is_file("file.md");
io::file::is_dir("dir");
io::file::delete_file("dir/file.txt");
io::file::write_file("dir/file.txt", "Line 1\nLine 2.");
io::file::read_file("dir/file.txt");
```

#### Config Functions

```rust
environ::get_config("EDITOR")
```
