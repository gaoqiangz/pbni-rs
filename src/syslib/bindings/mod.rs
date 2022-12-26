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

pub fn array_var_info(item_type: ValueType) -> OB_CLASS_ID {
    let style = match item_type {
        ValueType::Long | ValueType::Ulong => OB_DATASTYLE::LONG_STYLE,
        ValueType::Int | ValueType::Uint | ValueType::Byte | ValueType::Char | ValueType::Boolean => {
            OB_DATASTYLE::INT_STYLE
        },
        ValueType::Real => OB_DATASTYLE::FLOAT_STYLE,
        _ => OB_DATASTYLE::PTR_STYLE
    };
    let group = OB_GROUPTYPE::OB_SIMPLE;
    (((OB_MEMBER_ACCESS::OB_PUBLIC_MEMBER as OB_INFO_FLAGS) << DATA_ACCESS_SHIFT) |
        ((group as OB_INFO_FLAGS) << DATA_GROUP_SHIFT) |
        (0 << DATA_FIELDTYPE_SHIFT) |
        ((style as OB_INFO_FLAGS) << DATA_STYLE_SHIFT) |
        ((OB_STATUS::USED as OB_INFO_FLAGS) << DATA_STATUS_SHIFT) |
        ((OB_REFTYPE::OB_DIRECT_REF as OB_INFO_FLAGS) << DATA_REFTYPE_SHIFT) |
        (0 << DATA_TYPEARGS_SHIFT)) as OB_INFO_FLAGS
}

/// 返回值错误码
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PBRESULT {
    OK,
    E_INVALID_ARGUMENT,
    E_MISMATCHED_DATA_TYPE,
    E_WRONG_NUM_ARGS,
    E_OUT_OF_BOUNDS,
    E_OUT_OF_MEMORY,
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
