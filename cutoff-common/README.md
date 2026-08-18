# cutoff-common

A collection of common utilities and helpers used across Cutoff projects.

## Features

- Common traits and utility functions
- Collections utilities
- I/O utilities
- URN handling
- Optional logging utilities (with the `tracing-subscriber` feature)
- Optional serialization support (with the `serde` feature)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cutoff-common = { path = "../path/to/cutoff-common" }
```

To enable optional features:

```toml
[dependencies]
cutoff-common = { path = "../path/to/cutoff-common", features = ["serde", "tracing-subscriber"] }
```

## ⚠️ Warning

This crate is meant to be used internally by the Cutoff projects and crates as a dependency. It is not meant to be used by external projects. No effort will be made in terms of backward-compatibility.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
