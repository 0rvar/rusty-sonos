#![warn(missing_docs)]

//! # rusty-sonos
//! A Rust library that allows you to discover and interact with Sonos speakers
//!
//! Connecting to a Sonos speaker:
//! ```rust
//! let ip_addr = "192.168.1.0";
//!
//! let speaker = Speaker::new(ip_addr).await.unwrap();
//! ```
//!
//! Discovering speakers on the current network:
//! ```rust
//! let devices = discover_devices(2, 5).await.unwrap();
//!
//! for device in devices {
//!    println!("{}, {}", device.friendly_name, device.room_name)
//! }
//! ```

pub mod discovery;
pub mod errors;
pub mod responses;
mod services;
pub mod speaker;
mod xml;

// LOWERCASE ALL ERRORS
// SEE IF 'kind's CAN BE MERGED INTO STRUCTS
// IMPLEMENT DISPLAY AND ERROR FOR ALL ERRORS
// DOCUMENT ERRORS
