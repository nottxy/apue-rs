extern crate nix;

use std::{env, process};
use std::io::Error;
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;
use nix::libc;

fn main() {
    let args: Vec<CString> = env::args_os().map(|s| CString::new(s.into_vec()).unwrap()).collect();

    if args.len() != 2 {
        println!("usage: {} <pathname>", args[0].to_str().unwrap());
        process::exit(1);
    }

    let path_ptr = args[1].as_ptr();
    unsafe {
        if libc::access(path_ptr, libc::R_OK) < 0 {
            println!("access error: {}", Error::last_os_error());
        } else {
            println!("read access OK");
        }

        if libc::open(path_ptr, libc::O_RDONLY) < 0 {
            println!("open error: {}", Error::last_os_error());
        } else {
            println!("open for reading OK");
        }
    }

}
