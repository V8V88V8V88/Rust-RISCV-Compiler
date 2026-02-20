# Rust-RISCV-Compiler

Compiler in Rust targeting RISC-V. Lexer, parser, AST, codegen.

```
src/
├── main.rs
├── lexer.rs
├── parser.rs
├── ast.rs
└── code_gen.rs
```

## Build

```bash
cargo build
cargo run -- <input_file>
```

## Language

- Arithmetic: `+`, `-`, `*`, `/`
- Variables: `let x = expr;`
- Conditionals: `if (cond) { ... } else { ... }`

## Example

```
let x = 5;
let y = 10;
```

Output is RISC-V assembly. Use a simulator or emulator to run it.
