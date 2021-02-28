# Minecraft Query
[![Crates.io](https://img.shields.io/crates/v/mcquery)](https://crates.io/crates/mcquery) ![Build](https://github.com/Ewpratten/mcq/workflows/Build/badge.svg)

`mcq` is a small CLI tool for checking who is playing on a Minecraft server.

## Installation

### From Source

```sh
git clone https://github.com/ewpratten/mcq
cd mcq
cargo install --path .
```

### From Crates.io

```sh
cargo install mcquery
```

## Usage

```
# mcq --help

Minecraft Query 1.0.0
Evan Pratten <ewpratten@gmail.com>

USAGE:
    mcq [OPTIONS] <server>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>    Server port [default: 25565]

ARGS:
    <server>    Minecraft server address
```

<!-- ## Screenshots

![Screenshot](./screenshot.png) -->