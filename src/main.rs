#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(custom_test_frameworks)]
#![feature(panic_info_message)]
#![test_runner(qmos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::Translate;

use qmos::{memory, print, println};
use qmos::memory::BootInfoFrameAllocator;

mod panic;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Welcome to QMOS!");

    qmos::init();

    #[cfg(test)]
    test_main();

    println!("Booted successfully!");
    qmos::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}