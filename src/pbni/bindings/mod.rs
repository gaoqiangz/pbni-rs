use crate::{pbni::session::Session, pbstr::*};
pub use std::{marker::PhantomData, ptr::NonNull};

#[cfg(feature = "visualobject")]
pub use winapi::shared::{minwindef::HINSTANCE, windef::HWND};

pub mod ffi;

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

pub fn type_id<T: ?Sized + 'static>() -> u64 {
    use std::any::TypeId;
    let tid = TypeId::of::<T>();
    unsafe { *(&tid as *const TypeId as *const u64) }
}

macro_rules! declare_handle {
    ($name:ident, $inner:ident) => {
        #[repr(C)]
        pub struct $inner([u8; 0]);
        pub type $name = NonNull<$inner>;
    };
}

declare_handle!(pbvm, _IPB_VM);
declare_handle!(pbsession, _IPB_Session);
declare_handle!(pbvalue, _IPB_Value);
declare_handle!(pbarguments, _IPB_Arguments);
declare_handle!(pbclass, _pbclass);
declare_handle!(pbgroup, _pbgroup);
declare_handle!(pbstring, _pbstring);
declare_handle!(pbobject, _pbobject);
declare_handle!(pbarray, _pbarray);
declare_handle!(pbdec, _pbdec);
declare_handle!(pbdate, _pbdate);
declare_handle!(pbtime, _pbtime);
declare_handle!(pbdatetime, _pbdatetime);
declare_handle!(pbblob, _pbblob);
declare_handle!(pbuserobject, _IUserObject);

#[cfg(feature = "nonvisualobject")]
#[repr(C)]
pub struct NVOM<T: Sized> {
    pub ctx: NonNull<T>,
    pub type_id: u64,
    pub destory: unsafe extern "C" fn(NonNull<T>),
    pub invoke: unsafe extern "C" fn(NonNull<T>, Session, pbobject, MethodId, pbcallinfo) -> PBXRESULT
}

#[cfg(feature = "visualobject")]
#[repr(C)]
pub struct VOM<T: Sized> {
    pub ctx: NonNull<T>,
    pub type_id: u64,
    pub cls_name: LPCTSTR,
    pub destory: unsafe extern "C" fn(NonNull<T>),
    pub invoke: unsafe extern "C" fn(NonNull<T>, Session, pbobject, MethodId, pbcallinfo) -> PBXRESULT,
    pub create_control:
        unsafe extern "C" fn(NonNull<T>, u32, LPCTSTR, u32, i32, i32, i32, i32, HWND, HINSTANCE) -> HWND,
    pub get_event_id: unsafe extern "C" fn(NonNull<T>, HWND, u16, u32, u32) -> i32
}

#[repr(C)]
pub struct _PBCallInfo {
    pub pArgs: pbarguments,
    pub returnValue: pbvalue,
    pub returnClass: pbclass
}
pub type pbcallinfo = NonNull<_PBCallInfo>;

#[repr(C)]
pub struct _PBArrayInfo {
    // OUT variable, automatically set by GetArrayInfo(), don't set manually
    pub arrayType: ArrayType,
    pub itemGroup: pbgroup,
    pub valueType: ValueType,
    pub numDimensions: pbuint,
    pub bounds: [ArrayBound; 0]
}
pub type pbarrayinfo = NonNull<_PBArrayInfo>;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ArrayType {
    BoundedArray,
    UnboundedArray
}

#[repr(C)]
pub struct ArrayBound {
    pub upperBound: pblong,
    pub lowerBound: pblong
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

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GroupType {
    Application = 0,
    DataWindow,
    Function,
    Menu,
    Proxy,
    Structure,
    UserObject,
    Window,
    Unknown
}

impl From<i32> for GroupType {
    fn from(v: i32) -> Self { unsafe { std::mem::transmute(v) } }
}

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
    Dummy1,
    Dummy2,
    Dummy3,
    Char,
    Dummy4,
    LongLong,
    Byte
}

impl From<pbuint> for ValueType {
    fn from(v: pbuint) -> Self { unsafe { std::mem::transmute(v) } }
}
impl From<i32> for ValueType {
    fn from(v: i32) -> Self { unsafe { std::mem::transmute(v as u16) } }
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RoutineType {
    Function = 0,
    Event,
    Any
}

impl From<i32> for RoutineType {
    fn from(v: i32) -> Self { unsafe { std::mem::transmute(v) } }
}

/// 返回值错误码
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PBXRESULT {
    OK = 0,
    //SUCCESS = 0,
    //FAIL = -1,
    E_NO_REGISTER_FUNCTION = -1,
    E_REGISTRATION_FAILED = -2,
    E_BUILD_GROUP_FAILED = -3,
    E_INVALID_ARGUMENT = -4,
    E_INVOKE_METHOD_INACCESSABLE = -5,
    E_INVOKE_WRONG_NUM_ARGS = -6,
    E_INVOKE_REFARG_ERROR = -7,
    E_INVOKE_METHOD_AMBIGUOUS = -8,
    E_INVOKE_FAILURE = -9,
    E_MISMATCHED_DATA_TYPE = -10,
    E_OUTOF_MEMORY = -11,
    E_GET_PBVM_FAILED = -12,
    E_NO_SUCH_CLASS = -13,
    E_CAN_NOT_LOCATE_APPLICATION = -14,
    E_INVALID_METHOD_ID = -15,
    E_READONLY_ARGS = -16,
    E_ARRAY_INDEX_OUTOF_BOUNDS = -100,

    //pbni-rs Custom
    E_NULL_ERROR = -10000
}

impl PBXRESULT {
    pub fn is_ok(self) -> bool { self == PBXRESULT::OK }
    pub fn is_err(self) -> bool { self != PBXRESULT::OK }
}

impl From<i32> for PBXRESULT {
    fn from(v: i32) -> Self { unsafe { std::mem::transmute(v) } }
}

impl<T: Default> From<PBXRESULT> for crate::pbni::Result<T> {
    fn from(pbxr: PBXRESULT) -> Self {
        if pbxr == PBXRESULT::OK {
            Ok(Default::default())
        } else {
            Err(pbxr)
        }
    }
}

impl<T> From<crate::pbni::Result<T>> for PBXRESULT {
    fn from(pbxr: crate::pbni::Result<T>) -> Self {
        if let Err(e) = pbxr {
            e
        } else {
            PBXRESULT::OK
        }
    }
}
