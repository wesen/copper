#![feature(asm)]
#![feature(lang_items)]
#![no_main]
#![no_std]

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    power_on_gpioe();
    put_pe9_in_output_mode();

    let ticks = 100_000;
    loop {
        set_pe9_high();
        delay(ticks);
        set_pe9_low();
        delay(ticks);
    }
}

fn delay(n: u32) {
    for _ in 0..n {}
}

fn power_on_gpioe() {
    /// Start address of the RCC register block
    const RCC: u32 = 0x4002_1000;

    /// Offset address of the AHBENR register
    const RCC_AHBENR: u32 = 0x14;

    /// IOPCEN bit mask
    const RCC_AHBENR_IOPEEN: u32 = 1 << 21;

    unsafe {
        // Pointer to the AHBENR register
        let ahbenr = (RCC + RCC_AHBENR) as *mut u32;

        // IOPECN = 1
        *ahbenr |= RCC_AHBENR_IOPEEN;
    }
}

/// Start address of the GPIOC register block
const GPIOE: u32 = 0x4800_1000;

/// Offset address of the BSRR register
const GPIOE_BSRR: u32 = 0x18;

fn put_pe9_in_output_mode() {
    /// Offset address of the CRH register
    const GPIOE_MODER: u32 = 0x0;

    unsafe {
        // Pointer to the MODER register
        let moder = (GPIOE + GPIOE_MODER) as *mut u32;

        // MODER9 = 0b01
        *moder = (*moder & !(0b11 << 18)) | (0b01 << 18)
    }
}

fn set_pe9_high() {
    unsafe {
        // Pointer to the BSRR register
        let bsrr = (GPIOE + GPIOE_BSRR) as *mut u32;

        // BS9 = 1
        *bsrr = 1 << 9;
    }
}

fn set_pe9_low() {
    unsafe {
        // Pointer to the BSRR register
        let bsrr = (GPIOE + GPIOE_BSRR) as *mut u32;

        // BR9 = 1
        *bsrr = 1 << (16 + 9);
    }
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
