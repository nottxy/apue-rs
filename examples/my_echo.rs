extern crate nix;

use std::process::exit;
use std::io::Error;
use nix::libc::{c_char, c_void, size_t, ssize_t, STDIN_FILENO, STDOUT_FILENO, read, write};

const BUFFSIZE: usize = 4096;

fn main(){
    let mut buf: [c_char; BUFFSIZE] = [0; BUFFSIZE];
    let mut n: ssize_t;

    unsafe {
        loop {
            n = read(STDIN_FILENO, &mut buf[0] as *mut c_char as *mut c_void, BUFFSIZE as size_t);
            if n <= 0 {
                break;
            }

            if n != write(STDOUT_FILENO, &mut buf[0] as *mut c_char as *mut c_void, n as size_t){
                println!("write error: {}", Error::last_os_error());
                exit(1);
            }
        }

        if n < 0 {
            println!("read error: {}", Error::last_os_error());
            exit(1);
        }
    }
}