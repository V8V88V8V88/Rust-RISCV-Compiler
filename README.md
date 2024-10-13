# Mini Virtual Machine

This project implements a simple virtual machine (VM) in Rust, featuring a basic instruction set, memory management, and a colorful Terminal User Interface (TUI).

## Features

- 8 general-purpose registers
- 1024 bytes of simulated memory
- Basic instruction set (load, store, arithmetic, branching)
- Stack support for function calls
- Colorful TUI displaying VM state in real-time

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

## Building and Running

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/mini-vm.git
   cd mini-vm
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the VM:
   ```
   cargo run --release
   ```

## What to Expect

When you run the VM, you'll see a TUI updating every half second, showing:

- Register values (blue)
- Memory contents (green)
- Stack state (yellow)
- Current instruction (red)

The VM will execute a pre-defined example program demonstrating various operations.

## Modifying the Program

To run a different program, edit the `program` array in `src/main.rs` and recompile the project.

## License

This project is open source and available under the MIT License.
