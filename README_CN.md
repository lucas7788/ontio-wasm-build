## ontio-wasm-build

[![Build Status](https://travis-ci.com/laizy/ontio-wasm-build.svg?branch=master)](https://travis-ci.com/laizy/ontio-wasm-build)

[English](README.md) | 中文

Ontology wasm合约验证和优化工具, wasm合约要想在Ontology链上成功运行,需要满足Ontology链的一些约定,比如，合约入口函数必须是`invoke`、合约中不支持浮点数、和链的交互只能调用Ontology暴露出的接口,为了防止恶意的攻击，Ontology wasm合约还限制了每个合约内存占用上限以及`table`限制。在将高级语言编写的程序编译成wasm字节码的过程中, 会生成许多无用的`exports`，这些无用的`exports`反过来又会生成很多无用的`import`,该工具会清理无用的`export_section`模块代码。

## License

This project is licensed under the [MIT license](LICENSE).

## 生成可执行文件
执行如下的命令生成可执行文件
```
cargo build
```

生成的可执行文件`ontio-wasm-build`在`target`文件夹。

通过 `--help`查看`ontio-wasm-build`的使用方法

```
$ ./target/debug/ontio-wasm-build --help
ontio-wasm-build 0.1.0
does awesome things

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

`input`参数用来指定要优化的wasm文件
`output`参数用来指定优化后的wasm字节码文件名

## WASM字节码优化

在将高级语言如C、C++和Rust等编写的代码使用`emscripten`编译器编译许多无用的`exports`,这些`exports`反过来编译出许多无用的`imports`并保留了未使用的函数,这些未使用的代码，会使得wasm合约文件较大，部署到链上需要更多的手续费，该工具会删除这些未使用的代码，使得生成的wasm文件更加精简.

## WASM字节码校验

WASM合约要想运行在Ontology链上，要使用Ontology约定的统一入口函数`invoke`,并将该函数导出`export`以供外部调用,该函数签名不应该有返回值并且该函数签名不接受参数。
