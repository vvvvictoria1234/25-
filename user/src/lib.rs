#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

mod syscall;  // 先声明模块
pub mod console;  // console 模块需要使用 lib.rs 导出的函数
mod lang_items;

// 从 syscall 模块导入函数并重新导出
pub use syscall::sys_write as write;
pub use syscall::sys_exit as exit;
use crate::syscall::sys_yield;

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| {
        unsafe { (addr as *mut u8).write_volatile(0); }
    });
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
pub fn yield_() -> isize { sys_yield() }

