# ngrok &emsp; ![Build] [![Crate]](https://crates.io/crates/ngrok) [![Docs]](https://docs.rs/ngrok/)

[build]: https://github.com/nkconnor/ngrok/workflows/tests/badge.svg
[crate]: https://img.shields.io/crates/v/ngrok
[docs]: https://docs.rs/ngrok/badge.svg

A minimal and concise [`ngrok`](https://ngrok.com/) wrapper for Rust. The library has many use cases, for example:

- open public HTTP tunnels to your development server(s) for integrations tests
- simplified networking and development on devices like a Raspberry Pi

TCP support, while not available, should be trivial to support.

Tested on windows ✅

## Getting Started

```toml
[dependencies]
ngrok-wrapper = "0.5.0"
```

## Usage

```rust
use ngrok_wrapper;

use url::Url;

fn main() -> std::io::Result<()> {
    let tunnel = ngrok_wrapper::builder()
        .http()
        .port(8556)
        .executable("./ngrokclinet.exe")
        .run()?;

    let public_url: &Url = tunnel.http()?;

    println!("Tunnel is open at {:?}", public_url.as_str());

    Ok(())
}

```

This assumes that `ngrok` is on your path. To change this, use the `.executable()` method in the builder when
creating your tunnel.

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `ngrok` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
