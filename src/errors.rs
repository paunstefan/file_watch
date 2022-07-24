#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    InotifyInit,
    AddWatch(String),
    WaitEvent,
    RemoveWatch(String),
    IOError(std::io::Error),
    Other,
}
