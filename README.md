# wikibot

Bot for automating tasks in the [Rust wiki](https://runrust.miraheze.org)

## Usage

First, make sure you have [Rust](https://www.rust-lang.org) and [Cargo](https://doc.rust-lang.org/cargo/index.html) installed.

The bot reads two environment variables, `USERNAME` and `PASSWORD`. The default username is `Dev-WikiBot`. The password _must_ be specified:

```fish
> PASSWORD="secret" cargo run
```

Use the `--release` flag to run the bot with optimizations.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
