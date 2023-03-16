use core::panic::PanicInfo;
use crate::println;

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

    loop {}
}