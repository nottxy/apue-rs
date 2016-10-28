extern crate nix;

use std::{env, process, mem};
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;
use std::io::Error;
use nix::libc;

fn main() {
    for arg in env::args_os().skip(1) {
        let path = CString::new(arg.into_vec()).unwrap();
        print!("{}: ", path.to_str().unwrap());

        unsafe {
            let mut buf: libc::stat = mem::zeroed();
            if libc::lstat(path.as_ptr(), &mut buf) < 0 {
                println!("lstat error: {}", Error::last_os_error());
                continue;
            }

            let file_type = match buf.st_mode & libc::S_IFMT {
                libc::S_IFREG => "regular",
                libc::S_IFDIR => "directory",
                libc::S_IFCHR => "character special",
                libc::S_IFBLK => "block special",
                libc::S_IFIFO => "fifo",
                libc::S_IFLNK => "symbolic link",
                libc::S_IFSOCK => "socket",
                _ => "** unknown mode **",
            };

            print!("{}\n", file_type);
        }
    }
}
