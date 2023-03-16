#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(custom_test_frameworks)]
#![feature(panic_info_message)]
#![test_runner(qmos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use qmos::{print, println};
mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to QMOS!");

    qmos::init();

    #[cfg(test)]
    test_main();

    println!("Booted");
    qmos::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}