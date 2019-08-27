## ontio-wasm-build

[![Build Status](https://travis-ci.com/ontio/ontio-wasm-build.svg?branch=master)](https://travis-ci.com/ontio/ontio-wasm-build)

English | [中文](README_CN.md)

A wasm contract validation and optimization tool for Ontology.Before deploying the contract to the chain, the tool can parse and verify the binary code of wasm contract, at the same time clean up and delete the invalid information in the contract, reduce the size of the contract and reduce the deployment cost.

Major Check Optimizations:
* The contract has an entry function `invoke', and both parameters and return values are null.
* Clean up functions not used in contracts, import and export items;
* Check Floating Point Instructions in Contracts;
* Check that all imports are Ontology runtime API and that the input and output parameters match perfectly
* Check the memory of the contract and whether the upper limit of Table usage exceeds the specified value to prevent malicious contract attacks;
* Clean up zeros in data section
* Clean up custom section
* Check that the size of the optimized contract does not exceed the specified value

## Install

1. download binary file from [releases](https://github.com/ontio/ontio-wasm-build/releases)

2. cargo install
```bash
cargo install --git=https://github.com/ontio/ontio-wasm-build
```
3. install with source code
```bash
git clone https://github.com/ontio/ontio-wasm-build
cd ontio-wasm-build
cargo build --release
```

## Usage
```
$ ontio-wasm-build --help
ontio-wasm-build 0.1.0

USAGE:
    ontio-wasm-build [FLAGS] <input> <output>

FLAGS:
    -h, --help           Prints help information
        --keep-custom    Keep custom section in output wasm file
    -V, --version        Prints version information

ARGS:
    <input>     Wasm file generated by rustc compiler
    <output>    Output wasm file name
```

`input`parameter is used to specify the wasm contract file to be optimized, generated with [ontology-wasm-cdt-cpp](https://github.com/ontio/ontology-wasm-cdt-cpp) and [ontology-wasm-cdt-rust](https://github.com/ontio/ontology-wasm-cdt-rust)。

`output`parameter is used to specify the optimized wasm contract file name

`keep-custom` is the wasm file used to set the output retained `custom_section', only for debugging purposes



## License

This project is licensed under the [MIT license](LICENSE).

