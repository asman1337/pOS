#![no_std]
#![no_main]

use core::panic::PanicInfo;

use pos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(" _______  _______  _______
(  ____ )(  ___  )(  ____ \\
| (    )|| (   ) || (    \\/
| (____)|| |   | || (_____
|  _____)| |   | |(_____  )
| (      | |   | |      ) |
| )      | (___) |/\\____) |
|/       (_______)\\_______)\\

                    v0.1.0");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}