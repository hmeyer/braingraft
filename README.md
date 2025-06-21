# braingraft
Compiler from LLVM-IR to https://github.com/hmeyer/brainstem

## Documentation

### Building the C++ examples

To build the C++ examples into LLVM-IR, you need to have `clang++` installed. 
On a Debian-based system, you can install the LLVM and Polly dependencies with the following command:
```bash
sudo apt-get install -y clang
```

You can then run the following command:

```bash
.examples/build.sh
```

This will generate the `examples/*.ll` files containing the LLVM-IR representation of the C++ code.

### Building the Project

To build the `braingraft` library, you will need to have the following dependencies installed:
* Rust and Cargo
* Clang
* LLVM 18
* Polly for LLVM 18

On a Debian-based system, you can install the LLVM and Polly dependencies with the following command:
```bash
sudo apt-get install -y llvm-18 libpolly-18-dev
```

The project is configured to automatically find the LLVM installation, so no environment variables are needed.

Finally, you can build the project using `cargo`:

```bash
cargo build
```
