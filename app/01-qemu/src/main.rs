#![feature(lang_items)]
#![no_main]
#![no_std]

#[no_mangle]
pub fn start() -> ! {
    let x = 42;
    let y = x;

    loop {}
}

mod vector_table {
    #[link_section = ".reset"]
    static RESET: fn() -> ! = ::start;
}

mod lang_items {
    #[lang = "panic_fmt"]
    extern fn panic_fmt() {}

    #[lang = "eh_personality"]
    fn eh_personality() {}
}
