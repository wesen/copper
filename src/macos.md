# macOS

> **WARNING** Totally untested. If this does/doesn't work, let me know!

> **TODO** Macports as an alternative to brew?

> **TODO** From which fork should we get the `arm-none-eabi-*` packages? AFAICT, they are not in the
> original brew repo.

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

## First OpenOCD connection

> **TODO** Empty section `:-)`, but most of this stuff looks like the [Linux] version.

[Linux]: /linux.html#First%20OpenOCD%20connection

> **TODO** Any permission problems on Mac? i.e. Do we actually need to use `sudo`?
