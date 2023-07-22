<h1 align=center> kbt </h1>
<p align=center> Terminal based keyboard tester </p>

> **k**bt **b**oard **t**ester

![image](https://github.com/bloznelis/kbt/assets/33397865/cfcc78ab-37f0-4db0-a107-9a21fa283f99)

## Features
  * Caputres key presses directly from X server
  * Multiple keyboard layouts
  * Interactive menu

## Limitations
* Only supports Linux running Xorg

## Installation
### Package managers
**TBA**
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
