use crate::primitive::*;
use std::{borrow::Cow, slice::from_raw_parts};

/// 数组索引抽象
pub trait AsArrayIndex {
    fn as_array_index(&self) -> &[pblong];
}

impl AsArrayIndex for pblong {
    #[inline]
    fn as_array_index(&self) -> &[pblong] { unsafe { from_raw_parts(self as *const pblong, 1) } }
}

impl AsArrayIndex for &[pblong] {
    #[inline]
    fn as_array_index(&self) -> &[pblong] { self }
}

impl<const N: usize> AsArrayIndex for &[pblong; N] {
    #[inline]
    fn as_array_index(&self) -> &[pblong] { &self[..] }
}

/// 函数名抽象
pub trait AsMethodName {
    fn as_method_name(&self) -> (Cow<'_, PBStr>, Cow<'_, PBStr>);
}

impl<T: AsPBStr> AsMethodName for T {
    #[inline]
    fn as_method_name(&self) -> (Cow<'_, PBStr>, Cow<'_, PBStr>) { (self.as_pbstr(), "".as_pbstr()) }
}

impl<T: AsPBStr, S: AsPBStr> AsMethodName for (T, S) {
    #[inline]
    fn as_method_name(&self) -> (Cow<'_, PBStr>, Cow<'_, PBStr>) {
        let (name, sign) = self;
        (name.as_pbstr(), sign.as_pbstr())
    }
}
