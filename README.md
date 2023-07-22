<h1 align=center> kbt </h1>
<p align=center> Terminal based keyboard tester </p>

> **k**bt **b**oard **t**ester

![image](https://github.com/bloznelis/kbt/assets/33397865/cfcc78ab-37f0-4db0-a107-9a21fa283f99)

## Motivation
I got tired with semi-broken online keyboard testers, so here we go – a more stable platform to build on – terminal.

## Features
  * Caputres key presses directly from X server
  * Multiple keyboard layouts
  * Interactive menu

## Limitations
* Only supports Linux running Xorg

## Installation
### Package managers
**TBA**
### Building from source
  1. `cargo build --release`
  2. `cp target/release/<your-application> /usr/local/bin/`

#### Prerequisites
  * `rust`

### Acknowledgments
Built with [tui-rs](https://github.com/fdehau/tui-rs)
