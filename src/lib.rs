use std::{ffi::CString, os::unix::prelude::OsStrExt};

use bitflags::bitflags;
use libc::c_int;

mod errors;

use errors::Error;

bitflags! {
    pub struct Events: u32 {
        const Open = libc::IN_OPEN;
        const Close = libc::IN_CLOSE;
        const Modify = libc::IN_MODIFY;
        const Deleted = libc::IN_DELETE_SELF;
        const Access = libc::IN_ACCESS;
        const Move = libc::IN_MOVE;
        const ChangeAttributes = libc::IN_ATTRIB;
        const CreatedInDir = libc::IN_CREATE;
        const DeletedInDir = libc::IN_DELETE;
    }
}

#[derive(Debug)]
pub struct Watcher {
    fd: c_int,
}

impl Watcher {
    /// Create a new Watcher instance
    ///
    /// Internally is represented as an `inotify` instance
    /// inside the kernel
    pub fn init() -> Result<Self, Error> {
        let fd = unsafe { libc::inotify_init() };

        dbg!(fd);

        // Negative value on error
        // TODO: for more details, also read perror
        if fd < 0 {
            return Err(Error::InotifyInit);
        }

        Ok(Watcher { fd })
    }

    /// Add watch to a file or directory
    ///
    /// A watch consists of a file and a series of events
    /// to watch for. When any of the events happen, the
    /// watcher will be notified.
    pub fn add_watch(&self, file_path: std::path::PathBuf, events: Events) -> Result<i32, Error> {
        let path_pointer = CString::new(file_path.as_os_str().as_bytes())
            .map_err(|_x| Error::Other)?
            .as_ptr();
        let wd = unsafe { libc::inotify_add_watch(self.fd, path_pointer, events.bits()) };

        if wd < 0 {
            let errno = get_errno().expect("Can't get errno value.");

            todo!()
        }

        todo!()
    }
}

pub fn get_errno() -> Option<i32> {
    let errno_addr = unsafe { libc::__errno_location() };
    dbg!(errno_addr);

    if errno_addr.is_null() {
        return None;
    }

    let errno = unsafe { *errno_addr };
    dbg!(errno);

    Some(errno)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
