extern crate nix;

use std::process;
use std::ffi::CString;
use std::io::Error;
use nix::libc;

fn main() {
    unsafe {
        let rwrwrw = libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IWGRP |
                     libc::S_IROTH | libc::S_IWOTH;

        libc::umask(0);
        if libc::creat(new_cstr("foo").as_ptr(), rwrwrw) < 0 {
            println!("create error for foo: {}", Error::last_os_error());
            process::exit(1);
        }

        libc::umask(libc::S_IRGRP | libc::S_IWGRP | libc::S_IROTH | libc::S_IWOTH);
        if libc::creat(new_cstr("bar").as_ptr(), rwrwrw) < 0 {
            println!("create error for bar: {}", Error::last_os_error());
            process::exit(1);
        }
    }

}

fn new_cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}
