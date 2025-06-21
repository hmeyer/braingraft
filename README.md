# braingraft
Compiler from LLVM-IR to https://github.com/hmeyer/brainstem

## Prerequisites

On a Debian-based system, you can install the required dependencies with the following command:
```bash
sudo apt-get update && sudo apt-get install -y clang llvm-18 libpolly-18-dev libzstd-dev
```

## Building the C++ examples

To build the C++ examples into LLVM-IR, run the following command:

```bash
./examples/build-examples.sh
```

This will generate the `examples/*.ll` files containing the LLVM-IR representation of the C++ code.

## Building the Project

You can build the project using `cargo`:

```bash
cargo build
```

## Running the compiler

To run the compiler on one of the examples:
```bash
cargo run -- examples/hello.ll
```
