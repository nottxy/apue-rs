extern crate nix;

use std::os::unix::ffi::OsStringExt;
use std::ffi::{CStr, CString};
use std::{env, mem, ptr, process};
use std::io::Error;

use nix::libc;

fn main() {
    let args: Vec<CString> = env::args_os().map(|s| CString::new(s.into_vec()).unwrap()).collect();

    if args.len() != 2 {
        println!("usage: {} directory_name", args[0].to_str().unwrap());
        process::exit(1);
    }

    unsafe {
        let dp = libc::opendir(args[1].as_ptr());

        if dp.is_null() {
            println!("can't open '{}': {}",
                     args[1].to_str().unwrap(),
                     Error::last_os_error());
            process::exit(1);
        }

        let mut entry = mem::zeroed();
        let mut entry_ptr = ptr::null_mut();
        loop {
            if libc::readdir_r(dp, &mut entry, &mut entry_ptr) != 0 {
                println!("read dir error: {}", Error::last_os_error());
                process::exit(1);
            }

            if entry_ptr.is_null() {
                break;
            }

            println!("{}",
                     CStr::from_ptr(entry.d_name.as_ptr()).to_str().unwrap());
        }

        libc::closedir(dp);
    }
}
