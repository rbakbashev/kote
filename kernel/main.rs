// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
#![allow(clippy::identity_op)]
#![allow(clippy::inline_always)]
#![allow(clippy::module_name_repetitions)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(fn_traits)]
#![feature(int_roundings)]
#![feature(once_cell)]

#[macro_use]
mod printk;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x64/mod.rs"]
mod arch;

mod bootloader;
mod console;
mod elf;
mod mm;
mod panic;
mod process;
mod sched;
mod serial;
mod small_vec;
mod spinlock;
mod types;

#[no_mangle]
pub extern "C" fn kmain() {
    serial::init();

    println_serial!("Booting kote...");

    let mut info = bootloader::get_info();
    mm::init(&mut info);
    console::init(&info);

    arch::init();

    arch::interrupts::init();

    println!("Available memory:");
    print!("{}", &info.free_areas);

    println!("Kernel sections:");
    print!("{}", info.section_headers.as_ref().unwrap());

    sched::init();

    arch::interrupts::enable();

    sched::next();

    println!("Kernel exit");
}
