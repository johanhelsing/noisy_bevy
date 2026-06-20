#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "gpu")]
mod gpu;

#[cfg(feature = "gpu")]
pub use gpu::*;

#[cfg(feature = "cpu")]
mod cpu;

#[cfg(feature = "cpu")]
pub use cpu::*;
