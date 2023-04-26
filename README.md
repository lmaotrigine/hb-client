# Heartbeat Client

A client for [Heartbeat](https://github.com/5HT2B/Heartbeat) written in Rust.

## Why?

There already is a [client](https://github.com/5HT2B/heartbeat-unix) maintained for Unix
systems, and if that works for you, I'd suggest sticking with that. This has some
QoL improvements for me personally and hopefully covers some users on systems the
official client doesn't support. For example:

- Support Windows (experimental, tested on the latest insiders build of Windows 11 Pro)
- Zero<sup>\[[1]\]</sup> dependencies
- DE-agnostic<sup>\[[2]\]</sup> lock screen detection
- Better&trade; configuration options
- Support for [this fork](https://github.com/lmaotrigine/heartbeat-rs)

[1]: #1
[2]: #2

<span id="1">\[1\]: Of course it has dependencies, but all of those are pre-installed in
any sane system.</span>

<span id="2">\[2\]: see [this file](lib/idle-time/src/linux.rs) for supported
screensavers. Feel free to add more that I've missed :3</span>

## Configuration

Configuration is handled by [figment](https://lib.rs/figment), and can be passed via a
TOML file or environment variables (which override the former).

The location for the configuration file is:

- `%APPDATA%\heartbeat\config.toml` on Windows
- `$HOME/Library/Application Support/heartbeat/config.toml` on macOS
- `$XDG_CONFIG_DIR/heartbeat/config.toml` on Linux, falling back to
  `$HOME/.config/heartbeat/config.toml`

<!-- TODO: document the fields, don't have mental capacity to English right now -->
