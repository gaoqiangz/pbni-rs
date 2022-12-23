use crate::{pbx::session::Session, primitive::*};
pub use std::{marker::PhantomData, ptr::NonNull};
#[cfg(feature = "visualobject")]
pub use winapi::shared::{minwindef::HINSTANCE, windef::HWND};

pub mod ffi;

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
    pub invoke: unsafe extern "C" fn(NonNull<T>, Session, pbobject, MethodId, pbcallinfo) -> PBXRESULT,
    pub get_inherit_ptr: unsafe extern "C" fn(NonNull<T>, type_id: u64) -> *const ()
}

#[cfg(feature = "visualobject")]
#[repr(C)]
pub struct VOM<T: Sized> {
    pub ctx: NonNull<T>,
    pub type_id: u64,
    pub cls_name: LPCTSTR,
    pub destory: unsafe extern "C" fn(NonNull<T>),
    pub invoke: unsafe extern "C" fn(NonNull<T>, Session, pbobject, MethodId, pbcallinfo) -> PBXRESULT,
    pub get_inherit_ptr: unsafe extern "C" fn(NonNull<T>, type_id: u64) -> *const (),
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
    E_VALUE_IS_NULL = -10000,
    E_INVALID_FIELD_ID = -10001
}

impl PBXRESULT {
    pub fn is_ok(self) -> bool { self == PBXRESULT::OK }
    pub fn is_err(self) -> bool { self != PBXRESULT::OK }
}

impl From<i32> for PBXRESULT {
    fn from(v: i32) -> Self { unsafe { std::mem::transmute(v) } }
}

impl<T: Default> From<PBXRESULT> for crate::pbx::Result<T> {
    fn from(pbxr: PBXRESULT) -> Self {
        if pbxr == PBXRESULT::OK {
            Ok(Default::default())
        } else {
            Err(pbxr)
        }
    }
}

impl<T> From<crate::pbx::Result<T>> for PBXRESULT {
    fn from(pbxr: crate::pbx::Result<T>) -> Self {
        if let Err(e) = pbxr {
            e
        } else {
            PBXRESULT::OK
        }
    }
}
