#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

pub(crate) mod common;
pub(crate) mod fmt;

pub mod ble_advertise;
pub mod ble_advertise_multiple;
pub mod ble_bas_central;
pub mod ble_bas_central_sec;
pub mod ble_bas_peripheral;
pub mod ble_bas_peripheral_sec;
pub mod ble_beacon;
pub mod ble_l2cap_central;
pub mod ble_l2cap_peripheral;
pub mod ble_scanner;
pub mod high_throughput_ble_l2cap_central;
pub mod high_throughput_ble_l2cap_peripheral;

#[cfg(feature = "std")]
mod alloc;
#[cfg(feature = "std")]
pub use alloc::BigAlloc;
