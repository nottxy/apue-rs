extern crate nix;

use std::{process, mem};
use std::ffi::CString;
use std::io::Error;
use nix::libc;

fn main() {
    unsafe {
        let mut stat_buf: libc::stat = mem::zeroed();
        let foo = new_cstr("foo");
        if libc::stat(foo.as_ptr(), &mut stat_buf) < 0 {
            println!("stat error for foo: {}", Error::last_os_error());
            process::exit(1);
        }

        if libc::chmod(foo.as_ptr(),
                       (stat_buf.st_mode & !libc::S_IXGRP) | libc::S_ISGID as libc::mode_t) <
           0 {
            println!("chmod error for foo: {}", Error::last_os_error());
            process::exit(1);
        }

        if libc::chmod(new_cstr("bar").as_ptr(),
                       libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH) <
           0 {
            println!("chmod error for bar: {}", Error::last_os_error());
            process::exit(1);
        }
    }
}

fn new_cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}
