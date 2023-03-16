use core::panic::PanicInfo;
use crate::println;
use qmos::{exit_qemu, QemuExitCode, serial_println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "\nPANIC at {}:{}\n{}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("PANIC: no location information available");
    }
    qmos::hlt_loop();
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    qmos::test_panic_handler(info);
}