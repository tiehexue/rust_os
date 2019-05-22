#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rust_os::{println, serial_print, serial_println};

entry_point!(test_boot);

fn test_boot(_boot_info: &'static BootInfo) -> ! {
  test_main();

  loop {}
}

fn test_runner(_tests: &[&dyn Fn()]) {
  unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  rust_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
  serial_print!("test_println... ");
  println!("test_println output");
  serial_println!("[ok]");
}