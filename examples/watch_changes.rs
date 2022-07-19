use file_watch::*;

fn main() {
    println!("Hello");

    let watcher = Watcher::init();
    let errno = get_errno();
}
