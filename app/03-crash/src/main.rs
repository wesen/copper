#![feature(asm)]
#![feature(lang_items)]
#![no_main]
#![no_std]

#[no_mangle]
pub fn start() -> ! {
    unsafe {
        let sram_boundary = *(0x0000_0000 as *const u32);
        let _crash = *(sram_boundary as *const u32);
    }

    loop {}
}

mod exception {
    pub fn handler() -> ! {
        unsafe {
            asm!("bkpt");
        }

        loop {}
    }
}

mod vector_table {
    #[link_section = ".reset"]
    static RESET: fn() -> ! = ::start;

    #[link_section = ".exceptions"]
    static EXCEPTIONS: [Option<fn() -> !>; 14] = [Some(::exception::handler), // NMI
                                                  Some(::exception::handler), // Hard fault
                                                  Some(::exception::handler), // Memory management fault
                                                  Some(::exception::handler), // Bus fault
                                                  Some(::exception::handler), // Usage fault
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  Some(::exception::handler), // SVCall
                                                  None, // Reserved for Debug
                                                  None, // Reserved
                                                  Some(::exception::handler), // PendSV
                                                  Some(::exception::handler)]; // Systick
}

mod lang_items {
    #[lang = "panic_fmt"]
    fn panic_fmt() {}

    #[lang = "eh_personality"]
    fn eh_personality() {}
}
