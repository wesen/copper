# Exceptions: Crashing your micro

In the previous chapter we wrote a very simple program, built a binary from it, verified that said
binary was well-formed (i.e. that it had a vector table where expected), and even executed it under
an emulator and also on real hardware. Everything went smoothly! However, when developing things are
bound to go awry at some point. In the worst case scenario, one might, by mistake, ask the
microcontroller to perform an operation that it can't handle, like:

- Reading/writing to a invalid memory address e.g. beyond the limits of the RAM region.
- Writing to flash memory using simple assignments (`*ptr = value`). Flash memory has an elaborate
  write mechanism.
- Executing an instruction that the microcontroller doesn't support. Like trying to execute a
  floating point instruction (e.g. `vadd.f32`) on a device without a FPU.
  
Under these situations, the processor will raise a *hardware exception*. When an exception is
raised, the normal flow of our program is *interrupted* (stopped) and the processor jumps to an
exception *handler* (recall: handler is just another name for function).

There are several types of exceptions but most of them signify a catastrophic failure and, in real
applications, are usually handled by resetting the system (if bare metal) or by resetting/killing
the faulty process (if running on top of an OS). These catastrophic exceptions should happen rarely
though (if ever) during the execution of a program; hence the name "exception".

As we are just starting out and we are going to be running our programs under the debugger, we'll
handle all the exceptions the same way: we'll halt the processor and yield control back to the
debugger using the `bkpt` instruction.

> **NOTE** Executing the `bkpt` instruction when there is no debugger attached to the processor will
> make the processor raise an exception!

## Installing the exception handlers

As the official ARM [documentation] states, the exception vectors (recall: a vector is a pointer to a
handler, i.e. a function pointer) must be stored in the vector table just right after the reset
vector. We'll extend our linker script, `layout.ld`, to account for this:

[documentation]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0552a/BABIFJFG.html

``` diff
   .vector_table ORIGIN(FLASH) :
   {
     LONG(ORIGIN(RAM) + LENGTH(RAM))
     KEEP(*(.reset))
+    KEEP(*(.exceptions))
   } > FLASH
```

The change is quite literal: "Place the exceptions (section) right after the reset (section)".

Now we need to populate this section in our Rust program:

``` rust
// Add this to `src/main.rs`

#![feature(asm)]

mod exception {
    pub fn handler() -> ! {
        unsafe {
            asm!("bkpt");
        }

        loop {}
    }
}

mod vector_table {
    #[link_section = ".exceptions"]
    static EXCEPTIONS: [Option<fn() -> !>; 14] = [Some(::exception::handler),  // NMI
                                                  Some(::exception::handler),  // Hard fault
                                                  Some(::exception::handler),  // Memory management fault
                                                  Some(::exception::handler),  // Bus fault
                                                  Some(::exception::handler),  // Usage fault
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  Some(::exception::handler),  // SVCall
                                                  None, // Reserved for Debug
                                                  None, // Reserved
                                                  Some(::exception::handler),  // PendSV
                                                  Some(::exception::handler)]; // Systick
}
```

Let's go over each section added to our program:

``` rust
mod exception {
    pub fn handler() -> ! {
        unsafe {
            asm!("bkpt");
        }

        loop {}
    }
}
```

`exception::handler` is the function that we'll use to handle all the exceptions. This handler uses
the `bkpt` instruction to trigger a manual breakpoint and then loops endlessly. We mark this
function as divergent to avoid returning from it. "Why?" Because whatever may have caused the
exception could have corrupted the stack or left some value uninitialized so continuing the
execution of our program after an exception would led to unexpected behavior.

``` rust
mod vector_table {
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
```

On the second part, we populate the `exceptions` section with the addresses of the exception
handlers. The [documentation] states that there are 14 exception "slots" right after the reset
handler so we'll place a 14-element array of function pointers in the `exceptions` section. Some
of the exceptions slots, the ones marked as "Reserved" on the docs, will never be used at runtime;
we'll leave those uninitialized (i.e. zeroed). In Rust, function pointers `fn()` are non-nullable.
To get nullable function pointers we must wrap the `fn()`s in an `Option`, then the `None` variant
becomes the equivalent of the null pointer.

## Build and inspect

It's always a good idea to inspect the output binary to check that everything looks as expected:

```
$ xargo build --target $TARGET
```
```
$ arm-none-eabi-objdump -CD target/$TARGET/debug/app
```
```
target/cortex-m3/debug/app:     file format elf32-littlearm


Disassembly of section .vector_table:

08000000 <app::vector_table::RESET::ha7f0c0bde3147d64-0x4>:
 8000000:	20002000 	andcs	r2, r0, r0

08000004 <app::vector_table::RESET::ha7f0c0bde3147d64>:
 8000004:	08000041 	stmdaeq	r0, {r0, r6}

08000008 <app::vector_table::EXCEPTIONS::h6c345024c0ffa34c>:
 8000008:	08000045 	stmdaeq	r0, {r0, r2, r6}
 800000c:	08000045 	stmdaeq	r0, {r0, r2, r6}
 8000010:	08000045 	stmdaeq	r0, {r0, r2, r6}
 8000014:	08000045 	stmdaeq	r0, {r0, r2, r6}
 8000018:	08000045 	stmdaeq	r0, {r0, r2, r6}
	...
 800002c:	08000045 	stmdaeq	r0, {r0, r2, r6}
	...
 8000038:	08000045 	stmdaeq	r0, {r0, r2, r6}
 800003c:	08000045 	stmdaeq	r0, {r0, r2, r6}

Disassembly of section .text:

08000040 <start>:
 8000040:	e7ff      	b.n	8000042 <start+0x2>
 8000042:	e7fe      	b.n	8000042 <start+0x2>

08000044 <app::exception::handler::ha8f05ee13115908a>:
 8000044:	be00      	bkpt	0x0000
 8000046:	e7ff      	b.n	8000048 <app::exception::handler::ha8f05ee13115908a+0x4>
 8000048:	e7fe      	b.n	8000048 <app::exception::handler::ha8f05ee13115908a+0x4>
```

The two most relevant things to note: The `exception::handler` is located at address `0x0800_0044`.
And, `vector_table::EXCEPTIONS`, the part of the vector table where the exception handlers reside,
is mainly filled with the THUMB address of `exception::handler` i.e. `0x0800_0045`; which is what we
wanted. You will also see a few `...` under `vector_table::EXCEPTIONS`, these are the reserved
exceptions slots and they are basically zeroed/uninitialized chunks of memory.

## Let's crash!

> **NOTE** This program won't work on QEMU. AFAICT, raising an exception on a emulated device is
> rather hard. For some reason (probably to simplify the implementation), actions that normally
> raise an exception on real hardware are allowed in QEMU. *shrugs*

Let's write a program that raises an exception to test the exception handler. Our program will
attempt to read memory beyond the RAM region. Here is it:

``` rust
fn start() -> ! {
    unsafe {
        let ram_boundary = *(0x0000_0000 as *const u32);
        let _crash = *(ram_boundary as *const u32);
    }

    loop {}
}
```

Let' run the program under the debugger on real hardware.

```
# On another terminal
$ [sudo] openocd (..)
```

```
$ arm-none-eabi-gdb target/cortex-m3/debug/app
```
```
(gdb) target remote :3333
(gdb) load
(gdb) step
9               let ram_boundary = *(0x0000_0000 as *const u32)
(gdb) step
10              let _crash = *(ram_boundary as *const u32)
(gdb) print/x ram_boundary
$1 = 0x20002000
(gdb) step
app::exception::handler () at src/main.rs:19
19                  asm!("bkpt")
```

Remember that the address `0x0000_0000` is the first element of the vector table and contains the
highest RAM address. Dereferencing that address (`ram_boundary`) will load memory beyond the RAM
region.

Upon trying to read invalid memory, the processor raises *an* exception (we don't know *which* one)
and then proceeds to execute the `exception::handler`. While executing the handler, the processor
encounters the `bkpt` instruction, halts and yields control back to the debugger.

We'll use this very simple exception handler to uncover programming mistakes while debugging. We'll
revisit the exception handler in the future to make it tell us which exception was raised and which
instruction (and line of code) generated the exception.

## Homework

What do you think would have happened if we executed the last `start` function that reads memory
beyond the RAM boundary **if** we didn't *install* the exception handlers, or IOW if we didn't add
the `exceptions` section or the `EXCEPTIONS` variable to our program?

> **HINT** Look at the disassembly (`objdump`) of such binary. Take note of the addresses. What does
> the vector table look like?

> **WARNING** Don't actually run a program that raises a hardware exception but doesn't handle it!
