use file_watch::*;

fn main() {
    println!("Hello");

    let mut watcher = Watcher::init().unwrap();
    let wd = watcher.add_watch(
        std::path::PathBuf::from("testdir"),
        EventTypes::CreatedInDir,
    );
    dbg!(&wd);
    let event = watcher.wait_for_event().unwrap();
    println!("{:?}", event);
    watcher.remove_watch(wd.unwrap());
}
