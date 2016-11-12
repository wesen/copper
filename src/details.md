# Nitty-gritty details

> My program works! I have no idea why ...

Great! We wrote a program that worked in the emulator. But I omitted some
important details to simplify things, so we could focus our attention on the
tooling. In particular, I didn't tell you that our Cargo project generates
binaries that only work on the LM3S6965! It's time to take a step back and
understand all the pieces involved. In particular:

- What's the role of that extra file, `layout.ld`, that we added to the Cargo
  project?
- Why we used `thumbv7m-none-eabi` as the cross compilation target?
- Why we had to add a `.cargo/config` to our Cargo project?

Once you understand that, you'll be able to write programs for **any** Cortex-M
microcontroller.
