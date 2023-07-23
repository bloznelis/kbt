<h1 align=center> kbt </h1>

<p align=center> (<b>k</b>bt <b>b</b>oard <b>t</b>ester)* </p>
<p align=center> <img alt="GitHub release (latest SemVer)" src="https://img.shields.io/github/v/release/bloznelis/kbt"> <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/bloznelis/kbt/ci.yaml"> </p>

![image](https://github.com/bloznelis/kbt/assets/33397865/cfcc78ab-37f0-4db0-a107-9a21fa283f99)

## Motivation
I got tired with semi-broken online keyboard testers, so here we go – one on a solid platform – terminal.

## Features
  * Caputres key presses directly from X server
  * Multiple keyboard layouts
  * Interactive menu

## Limitations
* Only supports Linux running Xorg

## Installation
### AUR
`paru -S kbt`

### Building from source
  1. `cargo build --release`
  2. `cp target/release/<your-application> /usr/local/bin/`

#### Prerequisites
  * `rust`

### Acknowledgments
Built with [ratatui](https://github.com/ratatui-org/ratatui)
