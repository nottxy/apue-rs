extern crate nix;

use nix::libc::{STDIN_FILENO, SEEK_CUR, lseek};

fn main() {
    unsafe {
        if lseek(STDIN_FILENO, 0, SEEK_CUR) == -1 {
            println!("cannot seek");
        } else {
            println!("seek OK");
        }
    }
}
