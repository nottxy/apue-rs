extern crate nix;

use nix::libc::getpid;

fn main(){
    unsafe {
        let pid = getpid();
        println!("pid: {}", pid);
    }
}