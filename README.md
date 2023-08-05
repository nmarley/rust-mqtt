# Rust MQTT

Rust-mqtt is native MQTT client for `no_std` environments.

Currently only supports MQTT v5.

## Constraints

Client supports following:
- QoS 0 & QoS 1 (All QoS 2 packets are mapped for future client extension)
- Only clean session
- Retain not supported
- Auth packet not supported
- Packet size is not limited, it is totally up to user (packet size and buffer sizes have to align)

## Acknowledgment

This project could not be in state in which currently is without Ulf Lilleengen
and rest of the community from [Drogue IoT](https://github.com/drogue-iot).
