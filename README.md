<h1 align=center> kbt </h1>

<p align=center> Terminal based keyboard tester. </p>

> **kbt** **k**bt **b**oard **t**ester

## Features
  * Caputres key presses directly from X server
  * Multiple keyboard layouts
  * Interactive menu

## Installation
### Manual
  1. `cargo build --release`
  2. `cp target/release/<your-application> /usr/local/bin/`

### Building from source
  1. Checkout the code
  2. `make build`
  3. `./execs/typioca`

#### Prerequisites
  * `rust`

### Acknowledgments
Built with [tui-rs](https://github.com/fdehau/tui-rs)
