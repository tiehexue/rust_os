use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::ExceptionStackFrame;

use crate::gdt;
use crate::println;

use lazy_static::lazy_static;

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe {
      idt.double_fault.set_handler_fn(double_fault_handler)
         .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
      }
      idt
  };
}

pub fn init_idt() {
  IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
  println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
  panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_breakpoint_exception() {
  serial_print!("test_breakpoint_exception...");
  // invoke a breakpoint exception
  x86_64::instructions::int3();
  serial_println!("[ok]");
}
