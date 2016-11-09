extern crate nix;

use std::io::Error;
use std::process;
use std::ffi::CString;
use nix::libc::{SEEK_SET, creat, write, lseek, close, c_void};

fn main() {
    unsafe {
        let buf1 = CString::new("abcdefghij").unwrap();
        let buf2 = CString::new("ABCDEFGHIJ").unwrap();

        let fd = creat(CString::new("file.hole").unwrap().as_ptr(), 0777);
        if fd < 0 {
            println!("create file error: {}", Error::last_os_error());
            process::exit(1);
        }

        if write(fd, buf1.as_ptr() as *const c_void, 10) != 10 {
            println!("write buf1 error: {}", Error::last_os_error());
            process::exit(1);
        }

        if lseek(fd, 1638400000, SEEK_SET) == -1 {
            println!("lseek error: {}", Error::last_os_error());
            process::exit(1);
        }

        if write(fd, buf2.as_ptr() as *const c_void, 10) != 10 {
            println!("write buf2 error: {}", Error::last_os_error());
            process::exit(1);
        }

        close(fd);
    }
}
