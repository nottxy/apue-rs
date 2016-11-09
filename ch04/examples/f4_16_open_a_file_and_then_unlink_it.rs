extern crate nix;

use std::ffi::CString;
use std::io::Error;
use std::process;
use nix::libc;

fn main() {
    unsafe {
        let temp_file = new_cstr("temp_file");

        if libc::open(temp_file.as_ptr(), libc::O_RDWR) < 0 {
            println!("open error: {}", Error::last_os_error());
            process::exit(1);
        }

        if libc::unlink(temp_file.as_ptr()) < 0 {
            println!("unlink error: {}", Error::last_os_error());
            process::exit(1);
        }

        println!("file unlinked");

        libc::sleep(15);
        println!("done");
    }
}

fn new_cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}