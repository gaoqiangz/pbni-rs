use std::{borrow::Cow, mem::transmute, ops::Deref};
use widestring::{WideCStr, WideCString, WideChar};

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    NoType = 0,
    Int,
    Long,
    Real,
    Double,
    Decimal,
    String,
    Boolean,
    Any,
    Uint,
    Ulong,
    Blob,
    Date,
    Time,
    DateTime,
    Cursor,
    Proc,
    Basic,
    Char,
    Handle,
    LongLong,
    Byte
}

impl From<pbuint> for ValueType {
    fn from(v: pbuint) -> Self { unsafe { transmute(v) } }
}
impl From<i32> for ValueType {
    fn from(v: i32) -> Self { unsafe { transmute(v as u16) } }
}

pub type pbint = i16;
pub type pbuint = u16;
pub type pblong = i32;
pub type pbulong = u32;
pub type pblonglong = i64;
pub type pbbyte = u8;
pub type pbreal = f32;
pub type pbdouble = f64;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct pbboolean(i16);

impl pbboolean {
    #[inline]
    pub fn to_bool(self) -> bool {
        if self.0 == 1 {
            true
        } else {
            false
        }
    }
}

impl PartialEq<bool> for pbboolean {
    fn eq(&self, other: &bool) -> bool { self.to_bool() == *other }
}

impl From<bool> for pbboolean {
    fn from(b: bool) -> Self {
        pbboolean(if b {
            1
        } else {
            0
        })
    }
}

impl From<pbboolean> for bool {
    fn from(b: pbboolean) -> Self { b.to_bool() }
}

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
impl AsPBStr for Cow<'_, str> {
    fn as_pbstr(&self) -> Cow<'_, PBStr> {
        PBString::from_str(self).expect("incompatible utf-8 string").into()
    }
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

/// 函数ID
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodId(u16);

impl MethodId {
    /// 创建一个函数ID
    ///
    /// # Safety
    ///
    /// 指定无效的函数ID可能导致未定义行为
    pub unsafe fn new(id: u16) -> MethodId { MethodId(id) }

    /// 函数ID的值
    pub fn value(self) -> u16 { self.0 }

    pub(crate) fn is_undefined(self) -> bool {
        const kUndefinedMethodID: u16 = 0xffff;
        self.0 == kUndefinedMethodID
    }
}

impl PartialEq<u16> for MethodId {
    fn eq(&self, other: &u16) -> bool { self.0.eq(other) }
}
impl PartialOrd<u16> for MethodId {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> { self.0.partial_cmp(other) }
}

/// 字段ID
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldId(u16);

impl FieldId {
    /// 创建一个字段ID
    ///
    /// # Safety
    ///
    /// 指定无效的字段ID可能导致未定义行为
    pub unsafe fn new(id: u16) -> FieldId { FieldId(id) }

    /// 字段ID的值
    pub fn value(self) -> u16 { self.0 }

    pub(crate) fn is_undefined(self) -> bool {
        const kUndefinedFieldID: u16 = 0xffff;
        self.0 == kUndefinedFieldID
    }
}

impl PartialEq<u16> for FieldId {
    fn eq(&self, other: &u16) -> bool { self.0.eq(other) }
}
impl PartialOrd<u16> for FieldId {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> { self.0.partial_cmp(other) }
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
            ::std::mem::transmute::<_, &$crate::primitive::PBStr>(
                $crate::primitive::__private::const_utf16::encode_null_terminated!($str) as &[u16]
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
        $crate::primitive::pbstr!($str).to_ucstring()
    };
}
pub use pbstring;

#[doc(hidden)]
pub mod __private {
    pub use const_utf16;
}
