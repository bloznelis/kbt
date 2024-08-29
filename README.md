<h1 align=center> kbt </h1>

<p align=center> (<b>k</b>bt <b>b</b>oard <b>t</b>ester) </p>
<p align=center> <img alt="GitHub release (latest SemVer)" src="https://img.shields.io/github/v/release/bloznelis/kbt"> <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/bloznelis/kbt/ci.yaml"> </p>

![kbt-1240](https://github.com/bloznelis/kbt/assets/33397865/d9af5ee9-c981-4be7-bcc7-144f3485805a)

## Motivation
I got tired with semi-broken online keyboard testers, so here we are – one on a solid platform – terminal.

## Features
  * Multiple keyboard layouts
  * Interactive menu
  * Linux, MacOS, Windows support

## Limitations
* Wayland is not supported

## Installation
### Arch Linux
`pacman -S kbt`

### nix
`nix-shell -p kbt`

### Cargo
`cargo install kbt`

**note**: Default location of the installed binary is `$HOME/.cargo/bin`

### Homebrew

```
brew tap bloznelis/tap
brew install kbt
```

**note**: During the first run you might need to grant Accessibility access.

### Prebuilt binaries
Grab a binary from the latest [release](https://github.com/bloznelis/kbt/releases)

### Building from source
  1. `make build`
  2. `cp target/release/kbt /usr/local/bin/`

#### Prerequisites
  * `rust`

### Acknowledgments
Built with [ratatui](https://github.com/ratatui-org/ratatui)
