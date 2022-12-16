pub use crate::primitive::*;
pub use std::{
    marker::PhantomData, mem::{self, MaybeUninit}, ptr::{self, NonNull}
};

mod library;
mod ffi;

pub use ffi::*;

lazy_static::lazy_static! {
pub static ref API: ffi::Api = unsafe { ffi::Api::load().unwrap() };
}

macro_rules! bitfield {
    (@get $bitfields:expr,$shift:expr,$mask:expr) => {
        ((($bitfields) & $mask) >> $shift)
    };
    (@modify $bitfields:expr,$value:expr,$shift:expr,$mask:expr) => {
        ((($value) << ($shift)) | (($bitfields) & !($mask)))
    };
}
pub(crate) use bitfield;

/// 返回值错误码
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PBRESULT {
    OK,
    E_MISMATCHED_DATA_TYPE,
    E_INVOKE_WRONG_NUM_ARGS,
    E_ARRAY_INDEX_OUTOF_BOUNDS,
    E_VALUE_IS_NULL,
    ObjectError(ffi::OB_ERROR)
}

impl PBRESULT {
    pub fn is_ok(self) -> bool { self == PBRESULT::OK }
    pub fn is_err(self) -> bool { self != PBRESULT::OK }
}

impl From<ffi::OB_ERROR> for PBRESULT {
    fn from(v: ffi::OB_ERROR) -> Self {
        if v == ffi::OB_ERROR::OB_SUCCESS {
            PBRESULT::OK
        } else {
            PBRESULT::ObjectError(v)
        }
    }
}

impl<T> From<crate::syslib::Result<T>> for PBRESULT {
    fn from(pbr: crate::syslib::Result<T>) -> Self {
        if let Err(e) = pbr {
            e
        } else {
            PBRESULT::OK
        }
    }
}
