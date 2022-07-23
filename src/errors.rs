#[derive(Debug)]
pub enum Error {
    InotifyInit,
    AddWatch(String),
    WaitEvent,
    RemoveWatch(String),
    IOError(std::io::Error),
    Other,
}
