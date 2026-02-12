# Game Boy Emulator

A Game Boy emulator written in Rust, implementing the Sharp LR35902 CPU and related hardware components.

## Overview

This project aims to create a fully functional Game Boy emulator from scratch using Rust. The emulator will be capable of running Game Boy ROMs and accurately reproducing the behavior of the original hardware.

## Current Status

🚧 **Work in Progress** 🚧

Currently implemented:
- ✅ CPU Registers (8-bit: A, B, C, D, E, F, H, L)
- ✅ 16-bit register pairs (AF, BC, DE, HL)
- ✅ Stack Pointer (SP) and Program Counter (PC)
- ✅ CPU flags (Zero, Subtract, Half-carry, Carry)
- ✅ Memory Management Unit (MMU) with 64KB flat memory
- ✅ ROM loading (up to 32KB)
- ✅ Comprehensive test suites for registers and MMU

## Features

### CPU Registers
The emulator implements the Game Boy's CPU register system:
- **8-bit registers**: A, B, C, D, E, F, H, L
- **16-bit registers**: PC (Program Counter), SP (Stack Pointer)
- **Flag register (F)**: Z (Zero), N (Subtract), H (Half-carry), C (Carry)
- **Register pairing**: Registers can be accessed as 16-bit pairs (AF, BC, DE, HL)

### Hardware Specifications
The Game Boy uses a Sharp LR35902 CPU, which is similar to the Z80 but with some differences:
- 4.19 MHz clock speed
- 8-bit data bus
- 16-bit address bus (64KB addressable memory)

## Building and Running

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

### Run Tests
```bash
cargo test
```

## Project Structure

```
src/
├── main.rs      # Entry point
├── cpu.rs       # CPU implementation (registers, flags)
└── mmu.rs       # Memory management unit
```

## Roadmap

- [x] CPU register implementation
- [x] Memory management unit (MMU)
- [ ] CPU instruction set implementation
- [ ] Interrupt handling
- [ ] Timer implementation
- [ ] Graphics (PPU - Pixel Processing Unit)
- [ ] Sound (APU - Audio Processing Unit)
- [ ] Input handling (joypad)
- [x] ROM loading
- [ ] Save state support
- [ ] Debugger

## Resources

- [Pan Docs](https://gbdev.io/pandocs/) - The most comprehensive Game Boy technical reference
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
- [Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI)

## License

This project is open source and available under the MIT License.

## Acknowledgments

Built with Rust 🦀
