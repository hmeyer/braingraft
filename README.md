# braingraft
Compiler from LLVM-IR to https://github.com/hmeyer/brainstem

## Documentation

### Building the C++ examples

To build the C++ examples into LLVM-IR, you need to have `clang++` installed. You can then run the following command:

```bash
.examples/build.sh
```

This will generate the `examples/*.ll` files containing the LLVM-IR representation of the C++ code.
