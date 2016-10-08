extern crate nix;

use std::{env, process};
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;
use std::io::Error;
use nix::libc::{c_int, fcntl, F_GETFL, O_ACCMODE, O_RDONLY, O_WRONLY, O_RDWR, O_APPEND,
                O_NONBLOCK, O_SYNC, O_FSYNC};

fn main() {
    let args: Vec<CString> = env::args_os().map(|s| CString::new(s.into_vec()).unwrap()).collect();

    if args.len() != 2 {
        println!("usage: {} <file_descriptor>", args[0].to_str().unwrap());
        process::exit(1);
    }

    let file_no = args[1].to_str().unwrap();

    let file_no = match file_no.parse::<c_int>() {
        Ok(v) => v,
        Err(e) => {
            println!("invalid file descriptor: {}, {}", file_no, e);
            process::exit(1)
        }
    };

    unsafe {
        let val = fcntl(file_no, F_GETFL, 0);
        if val < 0 {
            println!("fcntl error for fd {}: {}", file_no, Error::last_os_error());
            process::exit(1);
        }

        match val & O_ACCMODE {
            O_RDONLY => {
                print!("read only");
            }
            O_WRONLY => {
                print!("write only");
            }
            O_RDWR => {
                print!("read write");
            }
            _ => {
                println!("unknown access mode");
                process::exit(1);
            }
        }

        if val & O_APPEND == O_APPEND {
            print!(", append");
        }

        if val & O_NONBLOCK == O_NONBLOCK {
            print!(", nonblocking");
        }

        if val & O_SYNC == O_SYNC {
            print!(", synchronous wirtes");
        }

        if val & O_FSYNC == O_FSYNC {
            print!(", fsyncronous writes");
        }

        print!("\n");
    }

}
