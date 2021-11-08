use libc::{c_char, c_int, c_void, size_t};
use std::ptr::{null, null_mut};

#[no_mangle]
pub extern "C" fn magic_open(_flags: c_int) -> *mut c_void {
    null_mut()
}

#[no_mangle]
pub extern "C" fn magic_close(_cookie: *mut c_void) {
    // NOP
}

#[no_mangle]
pub extern "C" fn magic_getpath(_magicfile: *const c_char, _action: c_int) -> *const c_char {
    // actual fallback is something like "/usr/magic/magic.mgc"
    null()
}

#[no_mangle]
pub extern "C" fn magic_file(_cookie: *mut c_void, _filename: *const c_char) -> *const c_char {
    null()
}

#[no_mangle]
pub extern "C" fn magic_descriptor(_cookie: *mut c_void, _fd: c_int) -> *const c_char {
    null()
}

#[no_mangle]
pub extern "C" fn magic_buffer(
    _cookie: *mut c_void,
    _buffer: *const c_void,
    _length: size_t,
) -> *const c_char {
    null()
}

#[no_mangle]
pub extern "C" fn magic_error(_cookie: *mut c_void) -> *const c_char {
    null()
}

#[no_mangle]
pub extern "C" fn magic_getflags(_cookie: *mut c_void) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn magic_setflags(_cookie: *mut c_void, _flags: c_int) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn magic_version() -> c_int {
    541
}

#[no_mangle]
pub extern "C" fn magic_load(_cookie: *mut c_void, _filename: *const c_char) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn magic_load_buffers(
    _cookie: *mut c_void,
    _buffers: *mut *mut c_void,
    _sizes: *mut size_t,
    _nbuffers: size_t,
) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn magic_compile(_cookie: *mut c_void, _filename: *const c_char) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn magic_check(_cookie: *mut c_void, _filename: *const c_char) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn magic_list(_cookie: *mut c_void, _filename: *const c_char) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn magic_errno(_cookie: *mut c_void) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn magic_setparam(
    _cookie: *mut c_void,
    _param: c_int,
    _value: *const c_void,
) -> c_int {
    -1
}

#[no_mangle]
pub extern "C" fn magic_getparam(
    _cookie: *mut c_void,
    _param: c_int,
    _value: *mut c_void,
) -> c_int {
    -1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
