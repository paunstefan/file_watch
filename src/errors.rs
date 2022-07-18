#[derive(Debug)]
pub enum Error {
    InotifyInit,
    AddWatch(String),
    Other,
}
