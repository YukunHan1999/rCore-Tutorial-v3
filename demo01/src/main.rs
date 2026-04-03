//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]

use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod sbi;

#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}


#[repr(C)]
#[derive(Copy, Clone)]
struct Bytes {
    pre: u8,
    next: u8,
}

#[repr(C)]
union MyUnion {
    word: u16,      // 假设你的 uint_t 是 16 位，如果是 32 位则用 u32
    byte: Bytes,
}

/// the rust entry-point of os
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    // println!("[kernel] Hello, demo01!");

    let u = MyUnion { word: 0xffff}; // 0x78
    unsafe {
        println!("u.word: {:#x}", u.word);
        println!("u.byte.pre: {:#x}", u.byte.pre);
        println!("u.byte.next: {:#x}", u.byte.next);
    }

    use crate::board::QEMUExit;
    crate::board::QEMU_EXIT_HANDLE.exit_success(); // CI autotest success
                                                   //crate::board::QEMU_EXIT_HANDLE.exit_failure(); // CI autoest failed
}