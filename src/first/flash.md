# Run it on real hardware

> **TODO** Document gotchas. Some dev boards may need to plug/unplug jumpers to enable the built-in
> programmer/debugger.

It's time to test our program on real hardware! We'll use OpenOCD  to *flash* our program into the
microcontroller and then hook `gdb` and OpenOCD to debug our program just like we did before.

> Wait, what's flashing?

*Flashing* means we are going to transfer our program from the host machine (PC, laptop, etc.) to
the target device (the microcontroller). Once flashed, the micro will execute the flashed program
every time it boots or resets.

> **ATTENTION!** The flashing instructions here will overwrite the program that's currently stored
> in your microcontroller. Make sure that it's either something you are not gonna miss or that's
> something you can easily get a copy of.

## Compile for a different target device

Unless you happen to have a real LM3S6965EVB board right next to you, the binary we produced in the
previous section won't work for your device. To produce a valid binary for your device, you'll have
to change the cross compilation target from the LM3S6965 to *your* device and then rebuild the
program.

For the rest of this section, I'll be using the [STM32VLDISCOVERY] as the target device.

[STM32VLDISCOVERY]: http://www.st.com/content/st_com/en/products/evaluation-tools/product-evaluation-tools/mcu-eval-tools/stm32-mcu-eval-tools/stm32-mcu-discovery-kits/stm32vldiscovery.html

### Update the linker script

One of the things that you'll always have to do when changing the cross compilation target is to
update the device-specific parts of the linker script.

The STM32VLDISCOVERY contains a [STM32F100RBT6B] micro with 128KiB of flash and 8KiB of RAM. A
peculiarity of STM32 devices is that their flash memory starts at address `0x0800_0000`.

[STM32F100RBT6B]: http://www.st.com/resource/en/datasheet/stm32f100rb.pdf

*My* linker script adjustments look like this (yours will probably look different):

``` diff
 MEMORY
 {
-  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
-  RAM   : ORIGIN = 0x20000000, LENGTH = 64K
+  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
+  RAM   : ORIGIN = 0x20000000, LENGTH = 8K
 }
```

### Change the target specification

Depending on the micro in your dev board, you may also have to change the `rustc` target from
`cortex-m3` to something else. In my case, the STM32F100RBT6B also contains a Cortex-M3 processor so
I don't need to change the argument of the `--target` flag.

## Build & inspect

Now that the changes have been committed, we can rebuild the program:

```
$ xargo build --target $TARGET
```

> **NOTE** From now on, I'm going to use `$TARGET`, instead of e.g. `cortex-m3`, as a placeholder
>  for the cross compilation target to make sure you don't use the wrong target triple.

> **HEADS UP** Make sure that Cargo actually rebuilds the binary! Cargo doesn't trigger a rebuild
> when the linker script changes. So, if the only thing you changed was the linker script and not
> the `rustc` target, then you'll have to `cargo clean` first and then call `build`.

As usual, it's a good idea to inspect the binary with `objdump` to verify that the vector table is
where expected.

```
$ arm-none-eabi-objdump -CD target/$TARGET/debug/app
```
```
target/$TARGET/debug/app:     file format elf32-littlearm


Disassembly of section .vector_table:

08000000 <app::vector_table::RESET::ha7f0c0bde3147d64-0x4>:
 8000000:	20002000 	andcs	r2, r0, r0

08000004 <app::vector_table::RESET::ha7f0c0bde3147d64>:
 8000004:	08000009 	stmdaeq	r0, {r0, r3}

Disassembly of section .text:

08000008 <start>:
 8000008:	b082      	sub	    sp, #8
 800000a:	202a      	movs	r0, #42	; 0x2a
 800000c:	9001      	str	    r0, [sp, #4]
 800000e:	9000      	str	    r0, [sp, #0]
 8000010:	e7ff      	b.n	    8000012 <start+0xa>
 8000012:	e7fe      	b.n	    8000012 <start+0xa>
```

Looks good! The vector table is at `0x0800_0000` as expected for my device.

## Establishing an OpenOCD connection

Before we flash the program we have to "open" an OpenOCD connection between the device and the host
machine. You should be already familiar with these steps from the [development environment][de]
chapter:

[de]: linux.html#First%20OpenOCD%20connection

```
# Physically connect the dev board to the host machine (probably your laptop), then
$ [sudo] openocd -f board/$BOARD
```
```
(...)
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v1 JTAG v11 API v2 SWIM v0 VID 0x0483 PID 0x3744
Info : using stlink api v2
Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

## Flash and debug

We'll use the `gdb` shell to both flash and debug the program. So, fire up `gdb`:

> **NOTE** You can't use `lldb` this time `:-(`. AFAICT, there's no `lldb` equivalent to the
> `monitor` and `load` commands provided by `gdb`. And those commands are required in this section.

```
$ arm-none-eabi-gdb target/$TARGET/debug/app
```

Next, we need to connect `gdb` and OpenOCD. The command is the same as the one we used with QEMU:

```
(gdb) target remote :3333
Remote debugging using :3333
0x00000000 in ?? ()
```

You should also see extra output on the OpenOCD terminal but maybe not the exact same output shown
here:

``` diff
 Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
+Info : accepting 'gdb' connection on tcp/3333
+Info : device id = 0x10016420
+Info : flash size = 128kbytes
```

Now that we are connected to the device via `gdb`. Let's flash the program using the `load` command:

```
(gdb) load
Loading section .vector_table, size 0x8 lma 0x8000000
Loading section .text, size 0xc lma 0x8000008
Start address 0x8000008, load size 20
Transfer rate: 246 bytes/sec, 10 bytes/write.
```

You should also see new output on the OpenOCD terminal:

``` diff
 Info : accepting 'gdb' connection on tcp/3333
 Info : device id = 0x10016420
 Info : flash size = 128kbytes
+stm32f1x.cpu: target state: halted
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000008 msp: 0x20002000
+stm32f1x.cpu: target state: halted
+target halted due to breakpoint, current mode: Thread
+xPSR: 0x61000000 pc: 0x2000003a msp: 0x20002000
+stm32f1x.cpu: target state: halted
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000008 msp: 0x20002000
```

The program is now flashed and the device is halted at the program's entry point, i.e. the `start`
function. Let's repeat the debug session we used for the previous QEMU run:

```
(gdb) step
7           let x = 42
(gdb) step
8           let y = x
(gdb) print x
$1 = 42
(gdb) print &x
$2 = (i32 *) 0x20001ffc
(gdb) print y
$3 = -2052926870
(gdb) print/x y
$4 = 0x85a2d26a
(gdb) print &y
$5 = (i32 *) 0x20001ff8
(gdb) step
10          loop {}
(gdb) print y
$5 = 42
```

Yay! This time the uninitialized value of `y` looks more random; I got `-2052926870` on this run.

While you executed the above commands, you should have seen more output on the OpenOCD terminal.
Each time you stepped over the program, OpenOCD printed the *program counter* which is the address
of the instruction the processor will execute next.

``` diff
 xPSR: 0x01000000 pc: 0x08000008 msp: 0x20002000
+Info : halted: PC: 0x0800000a
+Info : halted: PC: 0x0800000c
+Info : halted: PC: 0x0800000e
+Info : halted: PC: 0x08000010
```

Here's one more trick for you to try:

```
(gdb) monitor reset halt
```

This will generate the following OpenOCD output:

```
 Info : halted: PC: 0x08000010
+stm32f1x.cpu: target state: halted
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000008 msp: 0x20002000
```

This will reset your microcontroller (!) and halt your program at the reset handler, i.e. the
`start` function.

For fun, let's inspect the `x` and `y` variables **before** they are initialized:

```
(gdb) step
7           let x = 42
(gdb) print x
$6 = 42
(gdb) print &x
$7 = (i32 *) 0x20001ffc
(gdb) print y
$8 = 42
(gdb) print &y
$9 = (i32 *) 0x20001ff8
```

Surprise! Both variables appear to have been already initialized! Except, that's not the case. What
actually happened is that resetting the microcontroller doesn't affect the RAM (i.e. it doesn't
power it off). Therefore, the RAM still holds the information from the previous run and that's why
`x` and `y` hold the value `42` they were assigned on the *previous* run of the program.

That's all for this section! You can now close both OpenOCD and `gdb`. Let's move onto more complex
programs!
