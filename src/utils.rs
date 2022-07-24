use std::ffi::CStr;

pub fn get_errno() -> Option<i32> {
    let errno_addr = unsafe { libc::__errno_location() };

    if errno_addr.is_null() {
        return None;
    }

    let errno = unsafe { *errno_addr };

    Some(errno)
}

pub fn errno_string(errno: i32) -> String {
    let string_ptr = unsafe { libc::strerror(errno) };

    if string_ptr.is_null() {
        return String::new();
    }

    let string = unsafe { CStr::from_ptr(string_ptr) }
        .to_str()
        .unwrap()
        .to_owned();

    string
}
