# What dev board should I use/get?

> **FIXME** I haven't defined what a "dev board" is yet!

I won't force you to use a specific dev board. This document has been written in a device-agnostic
way so you can follow it with pretty much any dev board you want. Nonetheless, some dev boards are
more easy to work with than others. This section will provide you some advice on how to pick a dev
board that will make your life easier.

Get/use a dev board:

## That's supported by the OpenOCD project.

We'll use OpenOCD to "flash" and debug the programs we'll develop. It's best if you can get a dev
board with a built-in debugger that's supported by OpenOCD. You can see a list of such boards
[here].

[here]: https://github.com/ntfreak/openocd/tree/master/tcl/board

> **NOTE** Not all the dev boards listed in that link have an ARM Cortex-M microcontroller, some of
> them have Cortex-A processors, some have microcontrollers with a different architecture (e.g.
> AVR).

One example of such dev board is the [STM32VLDISCOVERY][1] which I'm going to use throughout the
examples of this document. It seems that most of the [other evaluation boards][2] by STM32 also have
a built-in debugger and are supported by OpenOCD. I'm not familiar with other vendors like Atmel and
NXP but they probably have similar offerings.

[1]: http://www.st.com/content/st_com/en/products/evaluation-tools/product-evaluation-tools/mcu-eval-tools/stm32-mcu-eval-tools/stm32-mcu-discovery-kits/stm32vldiscovery.html?sc=stm32-discovery
[2]: http://www.st.com/content/st_com/en/products/evaluation-tools/product-evaluation-tools/mcu-eval-tools/stm32-mcu-eval-tools.html?querycriteria=productId=SS1532

> **TODO** Add picture of the STM32VLDISCOVERY

Alternatively, you can use a board that doesn't have a built-in debugger if the microcontroller in
it is supported by OpenOCD (check [this list][target]) and the board exposes a JTAG/SWD connector,
but you'll also need an external debugger supported by OpenOCD (check [this list][interface]). This
is more complicated as you'll have to read about the JTAG/SWD interface to make an informed decision
and you'll also have to figure out how to wire up everything together.

[target]: https://github.com/ntfreak/openocd/tree/master/tcl/target
[interface]: https://github.com/ntfreak/openocd/tree/master/tcl/interface

> **NOTE** The "target list" linked above not only contains Cortex-M microcontrollers but also
> devices with different architectures. Likewise, the "interface list" linked above not only lists
> JTAG/SWD interfaces.

> **TODO** Add picture of what the connection looks like

## That has good documentation

The microcontroller in it should have documentation about:

- The peripherals it provides: How they work and the registers associated with them. This document
  is known as the *Reference Manual* ([example][rm]), although sometimes this information is
  contained in the *Data Sheet* ([example][ds]).
- Hardware bugs (yes, hardware can have bugs too!) or device limitations that may be present in the
  revision of the hardware that you own and how to work around them. This document is known as
  *Silicon Errata* or just *Errata Sheet* ([example][se]).
  
[rm]: http://www.st.com/resource/en/reference_manual/cd00246267.pdf
[ds]: http://www.ti.com/lit/ds/symlink/lm3s6965.pdf
[se]: http://www.st.com/resource/en/errata_sheet/cd00260217.pdf
