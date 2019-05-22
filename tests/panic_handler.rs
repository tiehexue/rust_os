#![no_std]
#![no_main]
#![feature(panic_info_message)]

use bootloader::{BootInfo, entry_point};
use core::fmt;
use core::fmt::Write;
use core::panic::PanicInfo;
use rust_os::{QemuExitCode, exit_qemu, serial_print, serial_println};

const MESSAGE: &str = "Example panic message from panic_handler test";
const PANIC_LINE: u32 = 18; // adjust this when moving the `panic!` call

entry_point!(test_panic_main);

fn test_panic_main(_boot_info: &'static BootInfo) -> ! {
  serial_print!("panic_handler... ");
  panic!(MESSAGE); // must be in line `PANIC_LINE`
}

fn check_location(info: &PanicInfo) {
  let location = info.location().unwrap_or_else(|| fail("no location"));

  if location.file() != file!() {
    fail("file name wrong");
  }

  if location.line() != PANIC_LINE {
    fail("file line wrong");
  }
}

fn check_message(info: &PanicInfo) {
  let message = info.message().unwrap_or_else(|| fail("no message"));
  let mut compare_message = CompareMessage { equals: false };
  write!(&mut compare_message, "{}", message)
    .unwrap_or_else(|_| fail("write failed"));
  if !compare_message.equals {
    fail("message not equal to expected message");
  }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  check_message(info);
  check_location(info);

  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);
  loop {}
}

fn fail(error: &str) -> ! {
  serial_println!("[failed]");
  serial_println!("{}", error);
  exit_qemu(QemuExitCode::Failed);
  loop {}
}

struct CompareMessage {
  equals: bool,
}

impl fmt::Write for CompareMessage {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    if s == MESSAGE {
      self.equals = true;
    }
    Ok(())
  }
}
