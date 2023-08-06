#![no_std]
// #![allow(dead_code)]
pub(crate) mod fmt;

pub mod client;
pub mod encoding;
pub mod network;
pub mod packet;
pub mod tests;
pub mod utils;

pub use self::client::{Client, ClientConfig, RawClient};
