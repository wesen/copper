# macOS

You can install most of the required tools using `brew`:

```
# NOTE if you get "Error: Unknown command: cask", then run this command: `brew tap Caskroom/cask`
# and try again
$ brew cask install gcc-arm-embedded
$ brew install openocd qemu
```

To install Rust and Cargo, I recommend using [rustup]:

[rustup]: https://www.rustup.rs/

```
$ curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly
```

Or if you already have rustup installed, switch to the nightly channel with:

```
$ rustup default nightly
```

Finally, to install Xargo simply use:

```
$ cargo install xargo
```

## Testing OpenOCD

Follow [these instructions] to test connecting to your programmer/debugger using
OpenOCD.

[these instructions]: linux.html#First%20OpenOCD%20connection
