#![feature(asm)]
#![feature(lang_items)]
#![no_main]
#![no_std]

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    unsafe {
        let sram_boundary = *(0x0000_0000 as *const u32);
        let _crash = *(sram_boundary as *const u32);
    }

    loop {}
}

mod exception {
    pub extern "C" fn handler() {
        unsafe {
            asm!("bkpt");
        }

        loop {}
    }

    #[export_name = "_EXCEPTIONS"]
    pub static EXCEPTIONS: [Option<extern "C" fn()>; 14] = [Some(handler), // NMI
                                                            Some(handler), // Hard fault
                                                            Some(handler), // Memmanage fault
                                                            Some(handler), // Bus fault
                                                            Some(handler), // Usage fault
                                                            None, // Reserved
                                                            None, // Reserved
                                                            None, // Reserved
                                                            None, // Reserved
                                                            Some(handler), // SVCall
                                                            None, // Reserved for Debug
                                                            None, // Reserved
                                                            Some(handler), // PendSV
                                                            Some(handler)]; // Systick
}

mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}
