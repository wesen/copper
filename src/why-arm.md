# Why the ARM Cortex-M?

> Why not AVR or PIC or ...?

The main reason is that there are not many microcontroller families that are currently supported by
Rust. Popular microcontrollers like AVR and PIC are, currently, not (oficially) supported by LLVM
and as the Rust compiler relies on LLVM for code generation it doesn't support them either. However,
an [unofficial AVR backend][0] for LLVM is currently undergoing the process of being merged back
into upstream LLVM, so this situation will likely improve in the near future.

[0]: https://github.com/avr-llvm/llvm

Do note that some of the concepts covered here like the tooling and how to use common peripherals
are transferable to other microcontroller families.
