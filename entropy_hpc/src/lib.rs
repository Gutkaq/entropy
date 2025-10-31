// src/lib.rs

pub type I32 = i32;
pub type I64 = i64;
pub type U64 = u64;

pub mod cint;
pub mod hint;
pub mod oint;
pub mod display;
pub mod simd;

pub use cint::{CInt, CIFraction, CIntError};
pub use hint::{HInt, HIFraction, HIntError};
pub use oint::{OInt, OIFraction, OIntError};
pub use simd::simd_engine;

