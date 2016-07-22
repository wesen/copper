#![feature(asm)]
#![feature(lang_items)]
#![no_main]
#![no_std]

#[no_mangle]
pub fn start() -> ! {
    turn_on_gpioc();
    put_pc8_in_output_mode();
    set_pc8_high();
    set_pc8_low();

    loop {}
}

fn turn_on_gpioc() {
    /// Start address of the RCC register block
    const RCC: u32 = 0x4002_1000;

    /// Offset address of the APB2ENR register
    const RCC_APB2ENR: u32 = 0x18;

    /// IOPCEN bit mask
    const RCC_APB2ENR_IOPCEN: u32 = 1 << 4;

    unsafe {
        // Pointer to the APB2ENR register
        let apb2enr = (RCC + RCC_APB2ENR) as *mut u32;

        // IOPECN = 1
        *apb2enr |= RCC_APB2ENR_IOPCEN;
    }
}

/// Start address of the GPIOC register block
const GPIOC: u32 = 0x4001_1000;

/// Offset address of the BSRR register
const GPIOC_BSRR: u32 = 0x10;

fn put_pc8_in_output_mode() {
    /// Offset address of the CRH register
    const GPIOC_CRH: u32 = 0x4;

    unsafe {
        // Pointer to the CRH register
        let crh = (GPIOC + GPIOC_CRH) as *mut u32;

        // CNF8 = 0b00, MODE8 = 0b10
        *crh = *crh & !0b1111 | 0b0010;
    }
}

fn set_pc8_high() {
    unsafe {
        // Pointer to the BSRR register
        let bsrr = (GPIOC + GPIOC_BSRR) as *mut u32;

        // BS8 = 1
        *bsrr = 1 << 8;
    }
}

fn set_pc8_low() {
    unsafe {
        // Pointer to the BSRR register
        let bsrr = (GPIOC + GPIOC_BSRR) as *mut u32;

        // BR8 = 1
        *bsrr = 1 << (16 + 8);
    }
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
    extern fn panic_fmt() {}

    #[lang = "eh_personality"]
    fn eh_personality() {}
}
