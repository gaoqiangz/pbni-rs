#![doc = include_str!("../README.md")]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod primitive;
mod comm;

#[cfg(feature = "pbx")]
pub mod pbx;

#[cfg(feature = "syslib")]
pub mod syslib;

pub mod prelude {
    pub use crate::primitive::*;
    #[doc(no_inline)]
    pub use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
    #[doc(no_inline)]
    pub use rust_decimal::prelude::*;
}
