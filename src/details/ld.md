# Linker script

As we saw in the previous section, our program must comply with a specific memory layout for the
microcontroller to work properly (otherwise it won't boot!).

The linker is what ultimately determines the memory layout of our program. To some extend, we can
control the linker, and therefore the memory layout of our program, using a file called linker
script.

For our first program, I gave you the linker script `layout.ld`. In this section, I'll explain its
contents.

## Terminology

First, let's define some terms you'll often hear when dealing with linker scripts.

### Linking

The process of producing a binary from a Rust source code involves two steps (or several more if you
peek inside `rustc`'s pipeline): compiling and linking. Compiling is the action of producing an
intermediate file known as "object file" from the source code. In Rust, the minimal compilation unit
is the crate which can be a single `.rs` or a collection of them. Each crate gets compiled to
an object file (actually to an `.rlib` which is an object file plus "metadata"). To produce the
final binary/executable, the compiled crates get "merged" together; this process is known as
linking.

### Symbol

A symbol is either a function or a static variable. Each symbol has a name, a start address and
occupies some space memory. For example, our program entry point is a symbol with name `start` and
address `0x8` and occupies 12 bytes (6 16-bit instructions) in memory. 

### Section

A section is a collection of symbols stored in contiguous memory. Other way to think about this is
that symbols are *organized* in sections. In a bit we'll see why it's advantageous to group symbols
into sections.

An example of a section: In our program, the vector table can be thought of as a section composed of
two symbols: the first is the initial SP value with address `0x0`, and the second symbol is the
reset vector with address `0x4`.

### Region

A (memory) region is a span of memory that's described using a start address and a length (in
bytes). For example, the LM3S6965 has two memory regions: Its flash memory region which starts at
address `0x0` and has a size of 256 KiB, and its RAM region which starts at address `0x2000_0000` and
has a size of 64 KiB.

### Object files

The linker takes as input one or more object files and outputs a single object file. In this book,
we'll be exclusively working with object files in the ELF format. ELF also happens to be the
format Linux executables use, but there exist other formats like Mach-O (macOS) and COFF (Windows).

## `layout.ld`

Instead of explaining the linker script syntax (which is documented [elsewhere]), I'm going to
focus on explaining the contents of the `layout.ld` file and how this file relates to the boot
process we covered in the previous section.

[elsewhere]: https://sourceware.org/binutils/docs/ld/Scripts.html

For convenience, here's the full linker script.

```
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM  : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    KEEP(*(.reset))
  } > FLASH

  .text :
  {
    *(.text*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
```

Let's go over it block by block

### MEMORY

```
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM  : ORIGIN = 0x20000000, LENGTH = 64K
}
```

> **NOTE** In linker scripts, `K` means `1024` (not `1000`!) , `M` means `1048576` (i.e. `1024 *
> 1024`) and so on.

The `MEMORY` block declares two memory regions: one named FLASH and the other named RAM. These
regions represent the flash memory and RAM regions of the LM3S6965.

### SECTIONS

```
SECTIONS
{
  /* .. */
}
```

The `SECTIONS` block declares sections and assigns each section to a memory region. It's important
to assign each and every section to a memory region, otherwise you'll get weird "overlap" linker
errors.

#### .vector_table

```
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    KEEP(*(.reset))
  } > FLASH
```

This section represents the vector table which, as I've mentioned before, must follow a specific
layout:

1. It must be located at address `0x0`.
2. Its first element is: the initial SP value.
3. Its second element is: the reset vector.

This is how we meet these constraints in the linker script:

1. `.vector_table ORIGIN(FLASH) :`. "Place the `vector_table` section at address `ORIGIN(FLASH)`
   (i.e. `0x0`)". And, `} > FLASH`. "Place the `vector_table` section in the FLASH region".
2. `LONG(ORIGIN(RAM) + LENGTH(RAM))`. "The first symbol in the `vector_table` section is an
   *anonymous* symbol with value `0x2001_0000`". We are using the highest valid RAM address as the
   initial SP value and the expression `LONG(ORIGIN(RAM) + LENGTH(RAM))` evaluates to that (refer
   to the `MEMORY` block for the values of `ORIGIN(RAM)` and `LENGTH(RAM)`).
3. `KEEP(*(.reset))`. "Place all the `reset` sections found in the input files right after the
   anonymous symbol we defined before". On its own, this is not enough. What we really want to place
   here is    a function pointer not a section. The following Rust code from `src/main.rs` completes
   the operation:
   
``` rust
mod vector_table {
    #[link_section = ".reset"]
    static RESET: fn() -> ! = ::start;
}
```

This Rust code places a pointer to the `start` function (the reset vector!) in a section named
`reset`. `src/main.rs` then gets compiled to an object file that has a `reset` section with the
function pointer in it. The linker will take that `reset` section and place it in the output
executable right after the anonymous "initial SP value" symbol.

> **NOTE** "Place **all** the `reset` sections ...". This may become a problem if we start linking
> more crates and one those crates also happens to contain its own `reset` section. In the future,
> we'll add protective measures against this scenario.

#### .text

```
  .text :
  {
    *(.text*)
  } > FLASH
```

As I mentioned before, the `text` section contains the program code. What this block does is merge
the `text` sections from all the input files in a big `text` section and places that merged section
in the FLASH region. The last bit is the most important: the program code must reside in
non-volatile memory, i.e. flash memory.

#### /DISCARD/

```
  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
```

`/DISCARD/` is not a real section. Everything in this "special section" gets discarded by the linker
and doesn't make it to the output object. So, why are we discarding these specific symbols?

- `.ARM.exidx*` These symbols are related to unwinding. Which we aren't using! If we leave these
  symbols in, they'll in turn demand other symbols related to unwinding and cause "undefined
  reference" linker error.
- `note.gnu.build-id*`. These symbols are introduced by `gcc`, which is the linker we are using. If
  not removed, these symbols will try to place themselves at address `0x0` causing "overlap" linker
  errors.

## Inspect again

Let's look again at the `objdump` output from section [3.1]. We should now be able to understand it
better:

[3.1]: ./first/build.html

```
$ arm-none-eabi-objdump -CD target/cortex-m3/debug/app
target/cortex-m3/debug/app:     file format elf32-littlearm


Disassembly of section .vector_table:

00000000 <app::vector_table::RESET::ha7f0c0bde3147d64-0x4>:
   0:	20010000 	andcs	r0, r1, r0

00000004 <app::vector_table::RESET::ha7f0c0bde3147d64>:
   4:	00000009 	andeq	r0, r0, r9

Disassembly of section .text:

00000008 <start>:
   8:	b082      	sub	sp, #8
   a:	202a      	movs	r0, #42	; 0x2a
   c:	9001      	str	r0, [sp, #4]
   e:	9000      	str	r0, [sp, #0]
  10:	e7ff      	b.n	12 <start+0xa>
  12:	e7fe      	b.n	12 <start+0xa>
  
(..)
```

Let's go over it part by part but in reverse!

```
Disassembly of section .text:

00000008 <start>:
   8:	b082      	sub	sp, #8
   a:	202a      	movs	r0, #42	; 0x2a
   c:	9001      	str	r0, [sp, #4]
   e:	9000      	str	r0, [sp, #0]
  10:	e7ff      	b.n	12 <start+0xa>
  12:	e7fe      	b.n	12 <start+0xa>
```

The `text` section is our program code. These are the instructions that the microcontroller will
execute. There's only one symbol in this section: the `start` function. Note the address of the
`start` function: `0x8`; this function is located in the FLASH region / in flash memory.

```
Disassembly of section .vector_table:

00000000 (..)

00000004 (..)
```

The vector table resides at address `0x0`.

```
00000000 <app::vector_table::RESET::ha7f0c0bde3147d64-0x4>:
   0:	20010000 	andcs	r0, r1, r0
```

The first element of the vector table is an "anonymous" symbol (though the linker automatically
generates a name for it, based on the names of nearby symbols) with value `0x2001_0000`.
`0x2001_0000` is the highest valid RAM address of the LM3S6965.

```
00000004 <app::vector_table::RESET::ha7f0c0bde3147d64>:
   4:	00000009 	andeq	r0, r0, r9
```

The second element of the vector table is the symbol `app::vector_table::RESET`. This symbol is a
pointer to the `start` function. Its value is `0x9`. Wait ... the address of the `start` function is
`0x8` not `0x9`! Did we do something wrong? No. Don't worry, this is correct. What happens here is
that the microcontroller (the processor actually) is operating in THUMB mode and in THUMB mode
function pointers have their Least Significant Bit (LSB) set to 1. So the `0x8` becomes `0x9` (`0x8
| 0b1`).

## Closing comment

You don't need to commit all this to memory right now -- I know it's a lot to take in. But, yes,
linker scripts are important and we'll be dealing with them again in the future. At that time, feel
free to refer back to this section as a reference. For now, the take home message is:

- When programming against hardware, your program must follow a device-specific memory layout.
- You can use a linker script to adjust your program memory layout to fit those constraints.
