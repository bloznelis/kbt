<h1 align=center> kbt </h1>

<p align=center> (<b>k</b>bt <b>b</b>oard <b>t</b>ester) </p>
<p align=center> <img alt="GitHub release (latest SemVer)" src="https://img.shields.io/github/v/release/bloznelis/kbt"> <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/bloznelis/kbt/ci.yaml"> </p>

![kbt-gif](https://github.com/bloznelis/kbt/assets/33397865/f986b89e-4482-4457-bf56-6bc53edce965)

## Motivation
I got tired with semi-broken online keyboard testers, so here we go – one on a solid platform – terminal.

## Features
  * Caputres key presses directly from X server
  * Multiple keyboard layouts
  * Interactive menu

## Limitations
* Works on Linux, but not on Wayland
* Might or might not work on MacOS (not tested)
* Does not work on Windows

## Installation
### AUR
`paru -S kbt`

### Cargo
`cargo install kbt`

**note**: Default location of the installed binary is `$HOME/.cargo/bin`

### Prebuilt binaries
You can grab built binary from the latest [release](https://github.com/bloznelis/kbt/releases) (only linux-x86_64)

### Building from source
  1. `make build`
  2. `cp target/release/kbt /usr/local/bin/`

#### Prerequisites
  * `rust`

### Acknowledgments
Built with [ratatui](https://github.com/ratatui-org/ratatui)
