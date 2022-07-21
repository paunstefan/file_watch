use file_watch::*;

fn main() {
    println!("Hello");

    let mut watcher = Watcher::init().unwrap();
    let wd = watcher.add_watch(std::path::PathBuf::from("testfile"), Events::Open);
    watcher.wait_for_event();
}
