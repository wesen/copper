# Blinking an LED

(with too many hexadecimals)

> **HEADS UP** Most of the links in this page are PDFs!

So far we have only used the processor inside our microcontroller. The processor can only do math
and logic, and, on its own, it can't interact with the external world: it can't drive a LED or a
motor, read a sensor or communicate with other devices.

To make our programs more useful (and fun!) we must learn to use *peripherals*. A peripheral is an
extra piece of electronics that's built, alongside the processor, in the same microcontroller
package. Peripherals give the processor the extra functionality it needs to interact with the
external world.

> Awesome, what can I do with these peripherals?

All sort of things! There are several different types of peripherals, each one provides a different
functionality. Microcontrollers manufacturers call them by different names even if the provide the
same functionality though. Here are some of the most common ones (using STM32 naming convention):

- `GPIO`. General Purpose Input/Output. Can be used to turn on/off external devices (e.g. a motor, a
  lamp, etc.) or to read the state of a "switch" (e.g. a two-state (ON/OFF) switch, a keyboard,
  etc.).
- `ADC`. Analog-to-Digital Converter. Can be used to "read" analog sensors (e.g. a thermometer, a
  light (intensity) sensor, etc.) or signals (e.g. voltage level of a battery, electric current,
  etc.).
- `TIM`. Timers. Can be used to perform periodic tasks (e.g. every 100 ms), measure lengths of time
  (e.g. for how long was this button pressed?) or generate periodic signals with variable
  [duty cycle][dc] (AKA [Pulse Width Modulation][pwm] (PWM)). PWM is mainly used to control how much
  power is supplied to an electric machine like a motor which, in turn, lets you indirectly control
  other parameters like speed and torque.

[dc]: https://en.wikipedia.org/wiki/Duty_cycle
[pwm]: https://en.wikipedia.org/wiki/Pulse-width_modulation

We'll explore these and several other peripherals in more detail in a [later chapter].

[later chapter]: /peripherals.html

> So, how do I use these peripherals?

Thanks to some magic called [memory mapped I/O][MMIO], to the processor, peripherals appear as
memory regions (!). This means that, for example, by writing to some special memory address you can
use the digital I/O peripheral to turn on/off a LED. Another example: By reading from some special
memory address you can use the Analog to Digital Converter peripheral to "read" an analog sensor
like a thermometer and get the current environment temperature as a digital/discrete value.

[MMIO]: https://en.wikipedia.org/wiki/Memory-mapped_I/O

A chunk of memory associated to a single peripheral is known as a "register block".  As other types
of memory, these regions are usually accessed in word (32-bit on ARM) sized chunks. Each of these
word sized chunks is referred to as a (hardware) register; though registers can also be half-word or
byte sized. Each of these registers has a human-friendly name and an address associated to it.

A concrete example: The STM32F100 microcontroller has a peripheral known as Reset and Clock Control
(RCC). The register block associated with this peripheral starts at address `0x4002_10000`. This
register block is comprised of several registers as seen on its [register map]. One of registers
associated with this peripheral is the ["APB2 peripheral clock enable register"][APB2ENR] (AKA
`APB2ENR`) which lives at address `0x4002_1004`. This particular register can be used to power
on/off other peripherals.

[register map]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/a2/2d/02/4b/78/57/41/a3/CD00246267.pdf/files/CD00246267.pdf/jcr:content/translations/en.CD00246267.pdf#page=99&zoom=auto,67,754

To get familiar with the use of peripherals, we'll write the microcontroller version of the "hello
world" program: Blinking an LED.

## The device-agnostic plan

This is an overview of what our program will do:

1. Power on the digital output peripheral.

To save energy, most of the peripherals in a microcontroller boot in a powered off state. We have to
explicitly "turn on" the peripherals we want to use.

2. Put the *pin* that's connected to the LED in *output mode*.

A microcontroller pin is a exposed metal contact that can be electrically connected to another
device. A pin can either be in input mode or in output mode, but it must be in output mode to be
able to drive (supply current to) an external device. Most pins boot in input mode to avoid
spuriously driving external devices.
  
3. Set the pin *high* or *low* to turn on the LED.

*Low* means outputting zero volts (0V) on the pin whereas *high* means outputting a non-zero
voltage, usually the power supply voltage (3.3V on most Cortex-M micros), on the pin. Depending on
how the LED is wired to the pin, setting the pin low/high should turn it off/on or vice versa.

After we've confirmed that we can turn the LED on/off, we'll modify the program to toggle the state
of the LED pin every few seconds.
  
## The device-specific details

Now we must fill in the device-specific details to realize our plan. All the needed information will
come from the microcontroller reference manual ([here's mine][rm]) and the dev board user manual
([here's mine][um]).

[rm]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/a2/2d/02/4b/78/57/41/a3/CD00246267.pdf/files/CD00246267.pdf/jcr:content/translations/en.CD00246267.pdf
[um]: http://www.st.com/content/ccc/resource/technical/document/user_manual/f3/16/fb/63/d6/3d/45/aa/CD00267113.pdf/files/CD00267113.pdf/jcr:content/translations/en.CD00267113.pdf

### Which LED, which pin?

First, we must pick a LED on the dev board to work with. Your dev board very likely has at least one
"user LED" that's connected to one of the microcontroller's pin (check its user manual). Don't
confuse an "user LED" with the "power LED". The latter is an indicator of whether the board is
powered on or off and can't be controlled by the microcontroller.

> **TODO** What do I do if my dev board doesn't have an "user LED"?

The STM32VLDISCOVERY has [two user LEDs][leds]: one green and one blue. For this example, I'll be
using the blue one which is connected to the pin *PC8*. Because micros have many I/O pins, these
pins are usually grouped in *ports*. A port is a collection of 8, 16, or some other number of pins.
Ports are usually identified with letters: A, B, etc. and the pins in it are usually identified with
numbers: 0, 1, etc. Therefore, you can think of the the pin PC8 as the 9th (because numbering starts
at 0) pin in the port C.

[leds]: http://www.st.com/content/ccc/resource/technical/document/user_manual/f3/16/fb/63/d6/3d/45/aa/CD00267113.pdf/files/CD00267113.pdf/jcr:content/translations/en.CD00267113.pdf#page=11&zoom=auto,67,278

### How to power up a peripheral?

Micros have a dedicated peripheral that's in charge of "clocking" other peripherals. Clocking in
this context means powering on/off a peripheral.
A peripheral that doesn't receive a clock signal is basically powered off -- it can't be used and it
doesn't (actively) consume energy.

For STM32 micros this peripheral is called [RCC]. The family of `*ENR` registers in this peripheral
control the clocking of other peripherals. In my case, I'm interested in the [APB2ENR] register
which contains a `IOPCEN` bit that controls the clocking of the `GPIOC` port.

[RCC]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/a2/2d/02/4b/78/57/41/a3/CD00246267.pdf/files/CD00246267.pdf/jcr:content/translations/en.CD00246267.pdf#page=69&zoom=auto,67,755
[APB2ENR]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/a2/2d/02/4b/78/57/41/a3/CD00246267.pdf/files/CD00246267.pdf/jcr:content/translations/en.CD00246267.pdf#page=90&zoom=auto,67,754

### How to put the pin in output mode?

In my case, I need to put the pin `PC8` in output mode. Some register in the [GPIO] peripheral
should let me do that. After looking through the documentation, I found that the `CR*` registers do
that. In particular, the [CRH] register contains two bitfields: `CNF8` and `MODE8` which control the
pin `PC8`. I'll use the following combination:

[GPIO]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/a2/2d/02/4b/78/57/41/a3/CD00246267.pdf/files/CD00246267.pdf/jcr:content/translations/en.CD00246267.pdf#page=100&zoom=auto,67,755
[CRH]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/a2/2d/02/4b/78/57/41/a3/CD00246267.pdf/files/CD00246267.pdf/jcr:content/translations/en.CD00246267.pdf#page=112&zoom=auto,67,754

- `MODE8 = 0b10` Puts the pin in output mode and caps the toggling speed to 2MHz.
- `CNF8 = 0b00` Puts the pin in general purpose push-pull output mode.

### Driving the pin high and low

Again the register that I want must be in the `GPIOC` peripheral. In this case, it's the `BSRR`
register. It can individually *set* or *reset* a pin. Here, *reset* means putting the pin low and
*set* means driving the pin high.

## Putting it all together

Here's a detailed specification of the program:

1. Turn on the GPIOC peripheral: Set the `IOPCEN` bit in the `RCC->APB2ENR` register to `1`.
2. Put the PC8 pin in output mode: Set the `MODE8` and `CNF8` bitfields in the `GPIOC->CRH` register
   to `0b10` and `0b00` respectively.
3. Set the PC8 pin high: Set the `BS8` bit in the `GPIOC->BSRR` register to `1`.
3. Set the PC8 pin low: Set the `BR8` bit in the `GPIOC->BSRR` register to `1`.

## The code

And here's the code. I'm omitting the `exceptions`, `vector_table` and `lang_items` modules haven't
changed since [our previous program].

[our previous program]: ./exceptions.html#Installing%20the%20exception%20handlers

``` rust
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
```

Quite unsightly, right? So many magic values. In a [later chapter], we'll refactor this code to get
rid of the magic values, the pointer arithmetic and the raw pointers. The final code will look like
[this]. But this code will make do for now!

[later chapter]: ./registers.html
[this]: https://github.com/japaric/cu/blob/master/src/bin/01-led.rs

## Test it

Time to test our code! Don't feel discouraged if your program crashes or doesn't work on the first
try! I certainly get most of my embedded programs wrong when I'm just starting to write drivers and
have to deal with all these magic values and/or have to jump back and forth between the
microcontroller reference manual and my library/program.

OK, here's how I'd debug this program:

1. Starting from the program entry point, `start`, repeatedly `step` over the program until you hit
   the the "set the pin high" statement, in my case this is the `*bsrr = 1 << 8`. If you didn't hit
   an exception, congratulations! Head to step 3, otherwise go to step 2.
2. If you hit an exception, you should now know which statement triggered it. Reset your
   microcontroller with `monitor reset halt`, then step all the way until your reach the faulty
   statement but don't execute it!. At this point, inspect the address of the register that will be
   modified by the faulty statement. Is the address right/valid? If not, fix it then go to step 1.
3. You should now be about to execute the instruction that sets the LED pin high. Step from here all
   the way to the endless `loop`. This should toggle the state of the LED at least once. If it
   doesn't, then quite a few things could have gone wrong ... See below:
   
- Wrong register address as seen in step 2.
- `GPIO` has not been powered on or configured properly. You'll have to "examine" (`(gdb) x
  $ADDRESS`) all the related registers.
- You are driving the wrong pin, i.e. one that's not connected to an LED. Confirm this against your
  dev board user manual. 

## Adding a loop

Now that we know that we can toggle the state of the LED. Making the LED blink is relatively easy.
We need to add a `delay` function and then move the LED toggling code inside a loop:

``` rust
#[no_mangle]
pub fn start() -> ! {
    turn_on_gpioc();
    put_pc8_in_output_mode();

    let mut ticks = 100_000;
    loop {
        set_pc8_high();
        delay(ticks);
        set_pc8_low();
        delay(ticks);
    }
}

fn delay(n: u32) {
    for _ in 0..n {}
}
```

I have no way of telling you what value of `n` will give you e.g. a delay of 1 second because that
depends on the built-in internal clock of your microcontroller (mine is 8 MHz) and the actual
instructions that `delay` compiles to in debug mode. However, using a value between `10_000` and
`100_000` for `ticks` should make the LED blink at a visible rate.

## Test it again

To test, simply flash the program and let it run from the debugger:

```
(gdb) continue
```

You should now see the LED blink at some rate. To make the LED blink faster make the value of
`ticks` smaller. To do this, first manually break the program by pressing `Crtl-C` at `gdb`'s
prompt, then use the following commands:

```
(gdb) # break somewhere inside the loop
(gdb) break src/main.rs:13
(gdb) continue
Breakpoint 1, app::start () at (..)/src/main.rs:13
13              set_pc8_high()
(gdb) # make ticks smaller
(gdb) set ticks = 20000
(gdb) # clear breakpoint
(gdb) clear src/main.rs:13
(gdb) continue
```

The LED should now blink at a faster rate. You can repeat the experiment but setting `ticks` to a
larger value.
