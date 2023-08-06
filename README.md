# Rust MQTT

Rust-mqtt is native MQTT client for `no_std` environments.

This is a `no_std` (only) fork based on the original [upstream Rust-MQTT client by Ondrej Babec](https://github.com/obabec/rust-mqtt/). This fork does not require nightly Rust, and doesn't at this time support async runtimes.

Currently only supports MQTT v5.

## Constraints

Client supports following:
- QoS 0 & QoS 1 (All QoS 2 packets are mapped for future client extension)
- Only clean session
- Retain not supported
- Auth packet not supported
- Packet size is not limited, it is totally up to user (packet size and buffer sizes have to align)

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
