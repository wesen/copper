# Why Rust?

> Why program in Rust and not in C?

C is (probably) the most used programming language for development of embedded systems (citation
needed). However I've decided to use Rust in this material for the following reasons:

- Rust gives you C-like low level control over aspects like memory management and struct layout /
  memory representation while not imposing a runtime and still providing high level features like
  closures, traits/generics, tagged unions, pattern matching and someday, I hope, (state-machine)
  [generators][0] (this [blog post][1] fleshes out the idea a bit more).
  
[0]: https://github.com/rust-lang/rfcs/issues/1081#issuecomment-221396554
[1]: https://dwrensha.github.io/capnproto-rust/2016/05/28/async-generators.html
  
- The Rust distribution ships with a package manager, Cargo, that encourages splitting your
  programs in reusable crates (libraries) by greatly simplifying dependency management. Cargo also
  encourages not reinventing the wheel by providing access to a central repository where the Rust
  community publishes and shares their crates.
  
- One can use pretty much all of the existing C tooling on Rust programs. For example:
  - You can use binary inspection tools like `objdump`, `nm`, `readelf`, `size`, etc.
  - You can use `gdb`/`lldb` to debug your program. Things like step-by-step execution, breakpoints,
    watchpoints, printing variables, etc. Just Work.
  - You can run your program under an emulator like QEMU.

- Some people find Rust more approachable and/or less scary than C. This means that they are more
  likely to explore this area of programming using Rust than using C.
  
- And, most importantly, I like Rust and its awesome community.

> **TODO** Rust & C++ comparison

## Disadvantages of not using C

Or advantages of using C instead of Rust:

- Device support. Some less known architectures, like the Xtensa, but with huge communities behind
  them, like the ESP8266 (google it!), are not supported by LLVM and therefore you can't build Rust
  programs for microcontrollers of that architecture.

- Commercial/IDE support. There are lots (compared to Rust) of commercial IDEs, SDKs, RTOSes for
  developing firmware in C.

> **TODO** More advantages of using C. I haven't though this thoroughly.
