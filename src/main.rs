#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(inajuma::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use inajuma::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to the inajuma os 1.0");
    inajuma::init();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }

    #[cfg(test)]
    test_main();

    println!("[did not crash]");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    inajuma::test_panic_handler(info)
}
