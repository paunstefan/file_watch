# file_watch

Rust library to simplify the `inotify` file monitoring system.

## Usage

The usage is similar to the `inotify` API, but simplified. Some less important
events are missing, same for more complex functionality (like cookies). For a more
complete inotify implementation, use the `inotify-rs` crate.

Basic usage example:

```rust
// Initialize the watcher
let mut watcher = Watcher::init().unwrap();
// Create a watch to look for modifications to the `testfile`
let wd = watcher.add_watch(std::path::PathBuf::from("testfile"), EventTypes::Modify);
// Block until the file is modified
let event = watcher.wait_for_event().unwrap();

println!("{:?}", event);

watcher.remove_watch(wd.unwrap());
```
