# Target specification

In this section, we explain what the mysterious `cortex-m3.json` file is!

Building a Rust program for a Cortex-M microcontroller involves cross compiling. Cross compilation
is the process of building a binary that will run on a **target** device/system that's different
from the **host** device/system where the binary was produced. In our case, the target is the
Cortex-M microcontroller and the host is your laptop or your PC.

Out of the box, `rustc`, and therefore Cargo, can cross compile for a bunch of different targets
using the `--target` flag:

```
$ rustc -V
```
```
rustc 1.11.0-nightly (ad7fe6521 2016-06-23)
```
```
$ rustc --print target-list | column -c 100
```
```
aarch64-apple-ios               i686-linux-android              x86_64-apple-darwin
aarch64-linux-android           i686-pc-windows-gnu             x86_64-apple-ios
aarch64-unknown-linux-gnu       i686-pc-windows-msvc            x86_64-pc-windows-gnu
arm-linux-androideabi           i686-unknown-dragonfly          x86_64-pc-windows-msvc
arm-unknown-linux-gnueabi       i686-unknown-freebsd            x86_64-rumprun-netbsd
arm-unknown-linux-gnueabihf     i686-unknown-linux-gnu          x86_64-sun-solaris
armv7-apple-ios                 i686-unknown-linux-musl         x86_64-unknown-bitrig
armv7-linux-androideabi         le32-unknown-nacl               x86_64-unknown-dragonfly
armv7-unknown-linux-gnueabihf   mips-unknown-linux-gnu          x86_64-unknown-freebsd
armv7s-apple-ios                mips-unknown-linux-musl         x86_64-unknown-linux-gnu
asmjs-unknown-emscripten        mipsel-unknown-linux-gnu        x86_64-unknown-linux-musl
i386-apple-ios                  mipsel-unknown-linux-musl       x86_64-unknown-netbsd
i586-pc-windows-msvc            powerpc-unknown-linux-gnu       x86_64-unknown-openbsd
i586-unknown-linux-gnu          powerpc64-unknown-linux-gnu
i686-apple-darwin               powerpc64le-unknown-linux-gnu
```

Unfortunately, Cortex-M microcontrollers are not in this list. This is not a problem though, because
we can teach `rustc` about new targets using target specification files. A target specification
file is a JSON file that describes the characteristics of the target: its architecture, its OS, etc.
Currently, documentation about these files is scrambled in the compiler source code. These are the
relevant documentation bits:

- [rustc_back::target](https://github.com/rust-lang/rust/blob/1ab87b65a220a933dc9d171ef0fd865ddd88fe89/src/librustc_back/target/mod.rs#L11-L45)
- [rustc_back::target::Target](https://github.com/rust-lang/rust/blob/1ab87b65a220a933dc9d171ef0fd865ddd88fe89/src/librustc_back/target/mod.rs#L151-L171)
- [rustc_back::target::TargetOptions](https://github.com/rust-lang/rust/blob/1ab87b65a220a933dc9d171ef0fd865ddd88fe89/src/librustc_back/target/mod.rs#L178-L299)

> **ATTENTION** These links don't point to the latest revision of the compiler source!

In a [previous section], I gave you a target specification file: the `cortex-m3.json`, which
you used to cross compile for the LM3S6965. That file can be used to cross compile to any micro that
contains a Cortex-M3 processor. But, if you want to cross compile for a microcontroller that has a
different processor, you are going to need a different target specification file.

[previous section]: ./first/build.html

The list below is a collection of target specification files for all the existing Cortex-M
processors and their most common variants:

- [cortex-m0](/target/cortex-m0.json). Cortex-M0.
- [cortex-m0plus](/target/cortex-m0plus.json). Cortex-M0+.
- [cortex-m1](/target/cortex-m1.json). Cortex-M1.
- [cortex-m3](/target/cortex-m3.json). Cortex-M3.
- [cortex-m4](/target/cortex-m4.json). Cortex-M4 without a [Floating Point Unit][FPU] (FPU).
- [cortex-m4f](/target/cortex-m4f.json). Cortex-M4 with a FPU. Supports single precision FP
  instructions (e.g. `vadd.f32`).
- [cortex-m7](/target/cortex-m7.json). Cortex-M7 without a FPU.
- [cortex-m7f](/target/cortex-m7f.json). Cortex-M7 with a FPU. Supports both single and double
  precision FP instructions.
- [cortex-m7f-sp](/target/cortex-m7f-sp.json). Cortex-M7 with a FPU. Only supports single precision
  FP instructions.
  
[FPU]: https://en.wikipedia.org/wiki/Floating-point_unit

> **NOTE** All these targets use the soft float ABI

These files should cover most of your cross compilation needs. To figure out which file you need to
use for your microcontroller, first check which processor your micro has and whether it has a FPU or
not, then choose a file from the above list based on its description.

If you want to know why these files have the contents they have, check [this RFC]. The information
therein should be useful if you want to tweak these target specification files for some reason.

[this RFC]: https://github.com/japaric/rfcs/blob/cortex-m-targets/text/0000-cortex-m-targets.md#target-specifications

