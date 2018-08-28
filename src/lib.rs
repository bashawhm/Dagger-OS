#![cfg_attr(test, allow(dead_code, unused_macros))]
#![feature(lang_items)]
#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#[macro_use]
extern crate lazy_static;
extern crate multiboot2;
extern crate uart_16550;
extern crate x86_64;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod serial;
extern crate volatile;
extern crate rlibc;
extern crate spin;

mod memory;
#[macro_use]
extern crate bitflags;

#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;


pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;
    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}


#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    println!("Booting...");
//    serial_println!("Booting...");
	use memory::FrameAllocator;
	let boot_info = unsafe {multiboot2::load(multiboot_information_address)};
	let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

	println!("Memory areas:");
	for area in memory_map_tag.memory_areas() {
		println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
	}

	let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");

	println!("Kernel Sections:");
	for section in elf_sections_tag.sections() {
		println!("    addr: 0x{:X}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
	}

	let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
	let kernel_end = (kernel_start as usize) + (boot_info.total_size as usize);
	println!("kernel_start: {:x}", kernel_start);
	println!("kernel_end: {:x}\n", kernel_end);

	let multiboot_start = multiboot_information_address;
	let multiboot_end = (multiboot_start as usize) + (boot_info.total_size as usize);
	println!("multiboot start: {:x}", multiboot_start);
	println!("multiboot end: {:x}", multiboot_end);

	println!("");
	//let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end, memory_map_tag.memory_areas());
	//println!("{:?}", frame_allocator.allocate_frame());

	let mut frame_allocator = memory::AreaFrameAllocator::new(
		kernel_start as usize, kernel_end as usize, multiboot_start,
		multiboot_end, memory_map_tag.memory_areas());

	for i in 0.. {
		if let None = frame_allocator.allocate_frame() {
		    println!("allocated {} frames", i);
		    break;
		}
	}

	unsafe {exit_qemu();}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality(){

}

#[panic_implementation]
#[no_mangle]
pub extern fn panic_fmt(_info: &PanicInfo) -> ! {
	println!("\n\nPANIC");
	unsafe {exit_qemu();}
	loop {}
}
