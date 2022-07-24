use file_watch::*;

fn main() {
    let mut watcher = Watcher::init().unwrap();
    let wd = watcher.add_watch(std::path::PathBuf::from("testfile"), EventTypes::Modify);
    let event = watcher.wait_for_event().unwrap();

    println!("{:?}", event);

    watcher.remove_watch(wd.unwrap());
}
