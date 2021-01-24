<h1 align="center">rjvm</h1>
<h3 align="center">Toy JVM is written in Rust</h3>

<p align="center">
<a href="LICENSE">
<img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT">
</a>
</p>

## Overview

A Status of the rjvm is WIP.
It is a JVM written in Rust that I'm making for fun.
I implement it according to [this specification](https://docs.oracle.com/javase/specs/jvms/se15/html/index.html).

## Build and Run

Currently, the code in "sample/\*.java" works.
It works as follows:

```
$ cargo run samples/HelloWorld.class
```

### Intetgration test

```
./test.sh
```
