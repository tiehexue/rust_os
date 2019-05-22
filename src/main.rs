#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use rust_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  println!("{}", _info);
  rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  rust_os::test_panic_handler(info);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

  use x86_64::VirtAddr;
  use x86_64::structures::paging::{Page, MapperAllSizes};
  use rust_os::memory::BootInfoFrameAllocator;

  println!("Hello World{}, {}", "!", boot_info.physical_memory_offset);
  rust_os::init();

  let addresses = [
    // the identity-mapped vga buffer page
    0xb8000,
    // some code page
    0x20010a,
    // some stack page
    0x57ac_001f_fe48,
    // virtual address mapped to physical address 0
    boot_info.physical_memory_offset,
  ];

  let mut mapper = unsafe { rust_os::memory::init(boot_info.physical_memory_offset) };

  for &address in &addresses {
    let virt = VirtAddr::new(address);
    let phys = mapper.translate_addr(virt);

    println!("{:?} -> {:?}", virt, phys);
  }

  let mut frame_allocator = unsafe {
    BootInfoFrameAllocator::init(&boot_info.memory_map)
  };

  // map a previously unmapped page
  let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
  rust_os::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

  // write the string `New!` to the screen through the new mapping
  let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
  unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

  #[cfg(test)]
  test_main();

  rust_os::hlt_loop();
}
