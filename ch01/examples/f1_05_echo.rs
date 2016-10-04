extern crate nix;

use nix::libc::{fdopen, fgetc, fputc, ferror, exit, c_int, c_char, STDIN_FILENO, STDOUT_FILENO,
                EOF};
use std::io::Error;

fn main() {
    let mut c: c_int;

    unsafe {
        let stdin = fdopen(STDIN_FILENO, &('r' as c_char));
        let stdout = fdopen(STDOUT_FILENO, &('w' as c_char));
        loop {
            c = fgetc(stdin);
            if c == EOF {
                break;
            }

            if fputc(c, stdout) == EOF {
                println!("write error: {}", Error::last_os_error());
                exit(1);
            }
        }

        if ferror(stdin) != 0 {
            println!("read error: {}", Error::last_os_error());
            exit(1);
        }
    }
}
