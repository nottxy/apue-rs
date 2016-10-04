extern crate nix;

use std::ffi::{CStr, CString};
use nix::libc::{STDIN_FILENO, SIGINT, SIG_ERR, c_char, c_int, pid_t, sighandler_t, signal, printf,
                fdopen, fgets, strlen, fork, waitpid};
use std::io::Error;
use std::{process, ptr, str};

const MAX_LINE: usize = 4096;

extern "C" {
    pub fn execlp(file: *const c_char, arg: *const c_char, _: *const c_char) -> c_int;
}

extern "C" fn sig_int(_signo: c_int) {
    println!("SIGINT interrupt\n%");
}

fn main() {
    let mut buf: [c_char; MAX_LINE] = [0; MAX_LINE];
    let mut pid: pid_t;
    let mut status: c_int = 0;

    unsafe {
        if signal(SIGINT, sig_int as sighandler_t) == SIG_ERR {
            println!("signal error: {}", Error::last_os_error());
            process::exit(1);
        }

        let stdin = fdopen(STDIN_FILENO, &('r' as c_char));

        loop {
            printf(CString::new("%% ").unwrap().as_ptr());

            if fgets(&mut buf[0], MAX_LINE as c_int, stdin).is_null() {
                break;
            }

            let end_char_pos = strlen(&buf[0]) as usize - 1;
            if buf[end_char_pos] == '\n' as c_char {
                buf[end_char_pos] = 0;
            }

            pid = fork();

            if pid < 0 {
                println!("fork error: {}", Error::last_os_error());
                process::exit(1);
            } else if pid == 0 {
                // child
                execlp(&buf[0], &buf[0], ptr::null());

                if nix::errno::errno() != 0 {
                    println!("couldn't execute {}: {}",
                             str::from_utf8(CStr::from_ptr(&buf[0]).to_bytes()).unwrap(),
                             Error::last_os_error());
                }
                process::exit(127);
            }

            pid = waitpid(pid, &mut status, 0);
            if pid < 0 {
                println!("waitpid error: {}", Error::last_os_error());
            }
        }
    }
}
