# `Copper`

> Rust on ARM Cortex-M microcontrollers

Inspired by [IntermezzOs], I decided to create a book-like website about using Rust to program
microcontrollers. The target audience will be programmers familiar with Rust that have no experience
with "bare metal" programming or embedded systems.

[IntermezzOs]: http://intermezzos.github.io/

I haven't started writing and I'm not quite sure how far I want to go with this book, but here's a
tentative list of topics that I'll cover, not necessarily in this order:

- Setting up the development environment: cargo + gdb + OpenOCD
- Linker scripts and memory layout
- Target specification files
- Basic debugging
- Obligatory blinking LED
- Volatile memory access
- Type safe register manipulation
- Clocks and timers
- Interrupts and exceptions
- The .data section
- Digital input: Buttons and debouncing
- Analog input: ADC
- Debug prints with ITM
- USART: Serial communication
- DMA
- I2C and state machines
- Scheduling methods

The book will be hosted in a different repository, but I'll link to the github pages from here when
the site is up. In the meantime you can look at the code hosted here, which will be used in the
book.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
