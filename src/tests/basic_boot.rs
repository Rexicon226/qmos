#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;
use blog_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}