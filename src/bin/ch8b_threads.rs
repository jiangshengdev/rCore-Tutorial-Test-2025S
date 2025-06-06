#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate user_lib;

use alloc::vec;
use user_lib::{exit, thread_create, waittid};

pub fn thread_a() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("a");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(1)
}

pub fn thread_b() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("b");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(2)
}

pub fn thread_c() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("c");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(3)
}

#[no_mangle]
pub fn main() -> i32 {
    let v = vec![
        thread_create(thread_a as usize, 0),
        thread_create(thread_b as usize, 0),
        thread_create(thread_c as usize, 0),
    ];
    for tid in v.iter() {
        let exit_code = waittid(*tid as usize);
        println!("thread#{} exited with code {}", tid, exit_code);
        assert_eq!(*tid, exit_code);
    }
    println!("main thread exited.");
    println!("threads test passed!");
    0
}
