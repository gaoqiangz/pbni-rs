use std::{borrow::Cow, ops::Deref};
use widestring::{WideCStr, WideCString, WideChar};

pub type TCHAR = WideChar;
pub type LPTSTR = *mut WideChar;
pub type LPCTSTR = *const WideChar;
pub type PBChar = WideChar;
pub type PBStr = WideCStr;
pub type PBString = WideCString;

/// `PBStr`抽象
pub trait AsPBStr {
    fn as_pbstr(&self) -> Cow<'_, PBStr>;
}

impl AsPBStr for &PBStr {
    fn as_pbstr(&self) -> Cow<'_, PBStr> { (*self).into() }
}
impl AsPBStr for PBString {
    fn as_pbstr(&self) -> Cow<'_, PBStr> { self.deref().into() }
}
impl AsPBStr for String {
    fn as_pbstr(&self) -> Cow<'_, PBStr> {
        PBString::from_str(self).expect("incompatible utf-8 string").into()
    }
}
impl AsPBStr for &str {
    fn as_pbstr(&self) -> Cow<'_, PBStr> {
        PBString::from_str(self).expect("incompatible utf-8 string").into()
    }
}
impl AsPBStr for Cow<'_, PBStr> {
    fn as_pbstr(&self) -> Cow<'_, PBStr> { self.as_ref().into() }
}

pub trait FromPBStrPtr {
    unsafe fn from_pbstr_unchecked(ptr: LPCTSTR) -> Self;
}

impl FromPBStrPtr for String {
    unsafe fn from_pbstr_unchecked(ptr: LPCTSTR) -> Self { PBStr::from_ptr_str(ptr).to_string_lossy() }
}
impl FromPBStrPtr for PBString {
    unsafe fn from_pbstr_unchecked(ptr: LPCTSTR) -> Self { PBStr::from_ptr_str(ptr).to_ucstring() }
}

/// 构造PB字符串`&'static PBStr`,编译时生成对应编码格式
///
/// # Exmaples
///
/// ```
/// static PBNI_RS: &'static PBStr = pbstr!("pbni-rs");
///
/// fn foo() {
///     let pbni_rs = pbstr!("pbni-rs");
/// }
/// ```
#[macro_export]
macro_rules! pbstr {
    ($str:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            ::core::mem::transmute::<_, &$crate::pbstr::PBStr>(
                $crate::pbstr::__private::const_utf16::encode_null_terminated!($str) as &[u16]
            )
        }
    }};
}
pub use pbstr;

/// 构造PB字符串`PBString`,编译时生成对应编码格式
///
/// # Exmaples
///
/// ```
/// fn foo() {
///     let pbni_rs = pbstring!("pbni-rs");
/// }
/// ```
#[macro_export]
macro_rules! pbstring {
    ($str:expr) => {
        $crate::pbstr!($str).to_ucstring()
    };
}
pub use pbstring;

#[doc(hidden)]
pub mod __private {
    pub use const_utf16;
}
