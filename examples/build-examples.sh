#!/bin/bash
SCRIPT_DIR=$( dirname -- "${BASH_SOURCE[0]}" )
clang++ -S -emit-llvm "$SCRIPT_DIR/hello.cpp" -o "$SCRIPT_DIR/hello.ll"
