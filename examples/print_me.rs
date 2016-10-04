extern crate nix;

use nix::libc::{getuid, getgid};

fn main(){
    unsafe {
        println!("uid: {}, gid: {}", getuid(), getgid());
    }
}