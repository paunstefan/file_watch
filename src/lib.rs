use std::{
    collections::HashMap,
    ffi::CString,
    fs::File,
    io::{Cursor, Read},
    os::unix::prelude::{AsRawFd, FromRawFd, OsStrExt},
    path::PathBuf,
};

use bitflags::bitflags;
use byteorder::{NativeEndian, ReadBytesExt};
use libc::c_int;

mod errors;

use errors::Error;

bitflags! {
    pub struct EventTypes: u32 {
        /// File or directory was opened
        const Open = libc::IN_OPEN;
        /// File or directory was closed
        const Close = libc::IN_CLOSE;
        /// File was modified
        const Modify = libc::IN_MODIFY;
        /// File or directory was itself deleted
        const Deleted = libc::IN_DELETE_SELF;
        /// File was accessed
        const Access = libc::IN_ACCESS;
        /// File or directory was moved/renamed
        const Move = libc::IN_MOVE;
        /// Metadata changed
        const ChangeAttributes = libc::IN_ATTRIB;
        /// File or directory was created inside watched directory
        const CreatedInDir = libc::IN_CREATE;
        /// File or directory was deleted inside watched directory
        const DeletedInDir = libc::IN_DELETE;
    }
}
#[derive(Debug)]

pub struct Event {
    file: PathBuf,
    ev_type: EventTypes,
    name: Option<String>,
    is_dir: bool,
    unmounted: bool,
    removed: bool,
}

impl Event {
    pub fn new(file: PathBuf, ev_type: EventTypes, name: Option<String>) -> Self {
        let is_dir = ev_type.bits() & libc::IN_ISDIR > 0;
        let unmounted = ev_type.bits() & libc::IN_UNMOUNT > 0;
        let removed = ev_type.bits() & libc::IN_IGNORED > 0;

        Event {
            file,
            ev_type,
            name,
            is_dir,
            unmounted,
            removed,
        }
    }
}

#[derive(Debug)]
pub struct Watcher {
    fd: File,
    watches: HashMap<c_int, PathBuf>,
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
        if fd < 0 {
            return Err(Error::InotifyInit);
        }

        Ok(Watcher {
            fd: unsafe { File::from_raw_fd(fd) },
            watches: HashMap::new(),
        })
    }

    /// Add watch to a file or directory
    ///
    /// A watch consists of a file and a series of events
    /// to watch for. When any of the events happen, the
    /// watcher will be notified.
    pub fn add_watch(&mut self, file_path: PathBuf, events: EventTypes) -> Result<i32, Error> {
        let cstring = CString::new(file_path.as_os_str().as_bytes()).map_err(|_x| Error::Other)?;
        let path_pointer = cstring.as_ptr();
        unsafe {
            libc::printf(path_pointer);
        }

        let wd =
            unsafe { libc::inotify_add_watch(self.fd.as_raw_fd(), path_pointer, events.bits()) };

        if wd < 0 {
            let errno = get_errno().expect("Can't get errno value.");
            dbg!(errno);
            todo!()
        }

        self.watches.insert(wd, file_path);

        Ok(wd)
    }

    /// Blocks until an event is received for a configured watch
    pub fn wait_for_event(&mut self) -> Result<Event, Error> {
        let mut buffer = [0; std::mem::size_of::<libc::inotify_event>() + 255 + 1];

        let read_count = self.fd.read(&mut buffer).map_err(Error::IOError)?;
        //dbg!(read_count);

        if read_count < std::mem::size_of::<libc::inotify_event>() {
            return Err(Error::WaitEvent);
        }

        let mut rdr = Cursor::new(buffer);

        let event = {
            let wd = rdr.read_i32::<NativeEndian>().unwrap();
            let mask = rdr.read_u32::<NativeEndian>().unwrap();
            let cookie = rdr.read_u32::<NativeEndian>().unwrap();
            let len = rdr.read_u32::<NativeEndian>().unwrap();
            libc::inotify_event {
                wd,
                mask,
                cookie,
                len,
            }
        };

        // dbg!(event.wd);
        // dbg!(event.mask);
        // dbg!(event.len);

        let name = if event.len > 0 {
            let mut name = vec![0; event.len as usize];
            rdr.read_exact(&mut name).map_err(Error::IOError)?;
            let name: String = name
                .iter()
                .take_while(|b| **b != 0)
                .map(|b| *b as char)
                .collect();
            Some(name)
        } else {
            None
        };
        //dbg!(&name);

        // Safety: all bits fit into the 32bit number. Unchecked is needed because
        // some flags that can't be set by the user can be returned by the API
        let flags = unsafe { EventTypes::from_bits_unchecked(event.mask) };

        Ok(Event::new(
            self.watches.get(&event.wd).unwrap().clone(),
            flags,
            name,
        ))
    }

    pub fn remove_watch(&mut self, wd: i32) -> Result<(), Error> {
        let rc = unsafe { libc::inotify_rm_watch(self.fd.as_raw_fd(), wd) };

        if rc < 0 {
            let errno = get_errno().expect("Can't get errno value.");

            todo!()
        }

        self.watches.remove(&wd);

        Ok(())
    }
}

fn get_errno() -> Option<i32> {
    let errno_addr = unsafe { libc::__errno_location() };

    if errno_addr.is_null() {
        return None;
    }

    let errno = unsafe { *errno_addr };

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
