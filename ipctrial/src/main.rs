extern crate libc;

use libc::ftok;
use libc::shmget;
use libc::c_char;
use libc::IPC_CREAT;

fn main() {
    let port: *const c_char = "/shaerd_files".as_ptr() as *const i8;
    unsafe{
        let key = ftok(port, 0);
        println!("print {} arguments", key);
    shmget(key, 4*1024, 0644 | IPC_CREAT);
    
    }
    println!("Hello, world!");
}
