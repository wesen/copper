# Zero-cost type-safe register manipulation

Goals:

- Go from hexadecimal gibberish to structs.
- Prevent writing to read-only registers and viceversa.
- Prevent reading to,writing to or modifying reserved bits.
- Nicer debugging experience: `print GPIOA` -> you get all the registers in that register block and
  their values.
