extern crate nix;
extern crate errno;

use nix::libc::{STDERR_FILENO, EACCES, ENOENT, c_char, fdopen, strerror, perror, fprintf};
use std::ffi::CString;
use std::env;
use errno::{Errno, set_errno};

fn main() {
    unsafe {
        let stderr = fdopen(STDERR_FILENO, &('w' as c_char));
        fprintf(stderr, CString::new("EACCES: %s\n").unwrap().as_ptr(), strerror(EACCES));

        set_errno(Errno(ENOENT));
        perror(CString::new(env::args().next().unwrap()).unwrap().as_ptr());
    }
}