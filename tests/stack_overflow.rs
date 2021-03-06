#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use rust_os::{exit_qemu, QemuExitCode, serial_println, serial_print};

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: &mut InterruptStackFrame, _error_code: u64) {
  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);
  loop {}
}

lazy_static! {
  static ref TEST_IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
      idt.double_fault
         .set_handler_fn(test_double_fault_handler)
         .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
  };
}

pub fn init_test_idt() {
  TEST_IDT.load();
}

entry_point!(test_stackoverflow_main);

fn test_stackoverflow_main(_boot_info: &'static BootInfo) -> ! {
  serial_print!("stack_overflow... ");

  rust_os::gdt::init();
  init_test_idt();

  // trigger a stack overflow
  stack_overflow();

  panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
  stack_overflow(); // for each recursion, the return address is pushed
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  rust_os::test_panic_handler(info)
}
