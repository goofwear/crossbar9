#![feature(lang_items, core_intrinsics, i128_type, conservative_impl_trait)]
#![no_std]
#![no_main]

#[macro_use]
extern crate bitutils;
extern crate rlibc;

mod ffistr;
mod gfx;
mod io;
pub mod interrupts;
pub mod intrinsics;
mod power;
mod realtime;
pub mod tests;
mod unique;

#[no_mangle]
pub extern fn main() {
    tests::main();

    ::gfx::log(b"Press SELECT to power off.\n");
    while !::io::hid::buttons_pressed().0[::io::hid::Button::SELECT.trailing_zeros() as usize] {}
    ::power::power_off()
}