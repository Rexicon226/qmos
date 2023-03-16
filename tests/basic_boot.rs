#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(qmos::test_runner)]

use core::panic::PanicInfo;

use qmos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    qmos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}