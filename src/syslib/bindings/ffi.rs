#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::{library::*, *};
pub use winapi::{
    shared::{
        minwindef::{
            BOOL, BYTE, DWORD, GLOBALHANDLE, HFILE, HINSTANCE, INT, LPARAM, LPBYTE, LPVOID, UINT, ULONG, WORD, WPARAM
        }, ntdef::{HANDLE, HRESULT, LPCSTR, LPCWSTR, LPSTR, LPWSTR, SHORT, USHORT}, windef::{HDC, HMENU, HWND, LPRECT, POINT, RECT}, wtypesbase::LPOLESTR
    }, ucrt::corecrt::time_t, um::wingdi::LOGFONTW
};

pub const NO_TYPE: OB_CLASS_ID = 0;
pub const INT_TYPE: OB_CLASS_ID = 1;
pub const LONG_TYPE: OB_CLASS_ID = 2;
pub const FLOAT_TYPE: OB_CLASS_ID = 3;
pub const DOUBLE_TYPE: OB_CLASS_ID = 4;
pub const DEC_TYPE: OB_CLASS_ID = 5;
pub const STRING_TYPE: OB_CLASS_ID = 6;
pub const BOOL_TYPE: OB_CLASS_ID = 7;
pub const ANY_TYPE: OB_CLASS_ID = 8;
pub const UINT_TYPE: OB_CLASS_ID = 9;
pub const ULONG_TYPE: OB_CLASS_ID = 10;
pub const BINARY_TYPE: OB_CLASS_ID = 11;
pub const DATE_TYPE: OB_CLASS_ID = 12;
pub const TIME_TYPE: OB_CLASS_ID = 13;
pub const DATETIME_TYPE: OB_CLASS_ID = 14;
pub const CURSOR_TYPE: OB_CLASS_ID = 15;
pub const PROC_TYPE: OB_CLASS_ID = 16;
pub const BASIC_TYPE: OB_CLASS_ID = 17;
pub const CHAR_TYPE: OB_CLASS_ID = 18;
pub const HANDLE_TYPE: OB_CLASS_ID = 19;
pub const LONGLONG_TYPE: OB_CLASS_ID = 20;
pub const BYTE_TYPE: OB_CLASS_ID = 21;

pub const DATA_OBJTYPE_MASK: OB_INFO_FLAGS = 1;
pub const DATA_INSTTYPE_MASK: OB_INFO_FLAGS = 2;
pub const DATA_OBJTYPE_SHIFT: u32 = 0;
pub const DATA_INSTTYPE_SHIFT: u32 = 1;
pub const DATA_NULLVAL_MASK: OB_INFO_FLAGS = 1;
pub const DATA_TYPEARGS_MASK: OB_INFO_FLAGS = 62;
pub const DATA_REFTYPE_MASK: OB_INFO_FLAGS = 192;
pub const DATA_STATUS_MASK: OB_INFO_FLAGS = 256;
pub const DATA_FIELDTYPE_MASK: OB_INFO_FLAGS = 512;
pub const DATA_STYLE_MASK: OB_INFO_FLAGS = 7168;
pub const DATA_GROUP_MASK: OB_INFO_FLAGS = 8192;
pub const DATA_ACCESS_MASK: OB_INFO_FLAGS = 49152;
pub const DATA_NULLVAL_SHIFT: u32 = 0;
pub const DATA_TYPEARGS_SHIFT: u32 = 1;
pub const DATA_REFTYPE_SHIFT: u32 = 6;
pub const DATA_STATUS_SHIFT: u32 = 8;
pub const DATA_FIELDTYPE_SHIFT: u32 = 9;
pub const DATA_STYLE_SHIFT: u32 = 10;
pub const DATA_GROUP_SHIFT: u32 = 13;
pub const DATA_ACCESS_SHIFT: u32 = 14;
pub const DATA_INFO_CONSTANT_MASK: OB_INFO_FLAGS = 1;
pub const DATA_INFO_INDIRECT_MASK: OB_INFO_FLAGS = 2;
pub const DATA_INFO_ISWRITE_MASK: OB_INFO_FLAGS = 4;
pub const DATA_INFO_ISREAD_MASK: OB_INFO_FLAGS = 8;
pub const DATA_INFO_ISINHERITED_MASK: OB_INFO_FLAGS = 16;
pub const DATA_INFO_ISSYSTEM_MASK: OB_INFO_FLAGS = 32;
pub const DATA_INFO_ISINSTANCE_MASK: OB_INFO_FLAGS = 64;
pub const DATA_INFO_ISPROPERTY_MASK: OB_INFO_FLAGS = 128;
pub const DATA_INFO_CONSTANT_SHIFT: u32 = 0;
pub const DATA_INFO_INDIRECT_SHIFT: u32 = 1;
pub const DATA_INFO_ISWRITE_SHIFT: u32 = 2;
pub const DATA_INFO_ISREAD_SHIFT: u32 = 3;
pub const DATA_INFO_ISINHERITED_SHIFT: u32 = 4;
pub const DATA_INFO_ISSYSTEM_SHIFT: u32 = 5;
pub const DATA_INFO_ISINSTANCE_SHIFT: u32 = 6;
pub const DATA_INFO_ISPROPERTY_SHIFT: u32 = 7;

pub const ID_MASK: u32 = 32767;
pub const LEVEL_MASK: u32 = 32768;
pub const ID_SHIFT: u32 = 0;
pub const LEVEL_SHIFT: u32 = 15;

pub const TYPE_ID_MASK: u32 = 16383;
pub const TYPE_KIND_MASK: u32 = 49152;
pub const TYPE_ID_SHIFT: u32 = 0;
pub const TYPE_KIND_SHIFT: u32 = 14;

pub const DEC_ARRAY_LEN: u32 = 7;
pub const DEC_SIGN_SHIFT: u32 = 0;
pub const DEC_OVERFLOW_SHIFT: u32 = 1;
pub const DEC_UNDERFLOW_SHIFT: u32 = 2;
pub const DEC_DIVIDE_BY_ZERO_SHIFT: u32 = 3;
pub const DEC_UNDETERMINED_SHIFT: u32 = 4;
pub const DEC_PRECISION_SHIFT: u32 = 8;

pub const SHTIME_TIME_NULL: BYTE = -1i32 as BYTE;
pub const SHTIME_DATE_NULL: SHORT = -32767i32 as SHORT;

pub type LOGFONT = LOGFONTW;

#[repr(C)]
pub struct IUnknown__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IUnknown {
    pub vtable_: *const IUnknown__bindgen_vtable
}
pub type PFV = ::std::option::Option<unsafe extern "C" fn()>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pbstg_statistics {
    pub lStgCount: ::std::os::raw::c_long,
    pub lStgHigh: ::std::os::raw::c_long,
    pub lStgCurrent: ::std::os::raw::c_long,
    pub lStgAlloc: ::std::os::raw::c_long,
    pub lStgMemAlloc: ::std::os::raw::c_long,
    pub lStgMemFree: ::std::os::raw::c_long
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct stg_subpool_entryS {
    pub pNext: *mut stg_subpool_entryS,
    pub pPrev: *mut stg_subpool_entryS,
    pub lpstrPoolName: LPTSTR
}
pub type pbstg_subpool = *mut stg_subpool_entryS;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct stg_anchorS {
    pub uiBlockSize: UINT,
    pub iAllocFlags: UINT,
    pub lpszOwner: LPTSTR,
    pub pSubpoolList: pbstg_subpool,
    pub pNext: *mut stg_anchorS
}
pub type ppbstg_anchor = *mut stg_anchorS;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sh_dbg_node {
    pub unused: INT,
    pub code: INT
}
pub type SH_DBG_NODE = sh_dbg_node;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sh_dbg_this {
    pub stgthis: ppbstg_anchor,
    pub sh_dbg_state: INT,
    pub dbg_state: INT,
    pub dbg_outfile: LPTSTR,
    pub dbg_fd_out: HFILE,
    pub dbg_open_count: INT,
    pub first_open: INT,
    pub dbg_nodes: *mut SH_DBG_NODE,
    pub num_dbg_nodes: INT,
    pub dbg_indent_count: INT,
    pub header_flag: INT,
    pub indent_flag: INT,
    pub console_flag: INT
}
pub type SH_DBG_THIS = sh_dbg_this;
impl ob_status {
    pub const OB_NOTINIT: ob_status = ob_status::USED;
}
impl ob_status {
    pub const OB_INIT: ob_status = ob_status::FREE;
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_status {
    FREE = 0,
    USED = 1
}
pub use self::ob_status as OB_STATUS;
pub type POB_GROUP = *mut ob_group;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_group_types {
    OB_SIMPLE = 0,
    OB_ARRAY = 1
}
pub use self::ob_group_types as OB_GROUPTYPE;
pub type POB_GROUPTYPE = *mut ob_group_types;
impl ob_ref_types {
    pub const OB_ANCESTOR_REF: ob_ref_types = ob_ref_types::OB_ARGUMENT_REF;
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_ref_types {
    OB_DIRECT_REF = 0,
    OB_GLOBAL_REF = 1,
    OB_ARGUMENT_REF = 2,
    OB_ARGUMENT_READONLY = 3
}
pub use self::ob_ref_types as OB_REFTYPE;
impl OB_FIELD_TYPE {
    pub const OB_GLOBAL_VAR: OB_FIELD_TYPE = OB_FIELD_TYPE::OB_TYPEDEF_FIELD;
}
impl OB_FIELD_TYPE {
    pub const OB_SHARED_VAR: OB_FIELD_TYPE = OB_FIELD_TYPE::OB_INSTVAR_FIELD;
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_FIELD_TYPE {
    OB_TYPEDEF_FIELD = 0,
    OB_INSTVAR_FIELD = 1
}
pub type POB_FIELD_TYPE = *mut OB_FIELD_TYPE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_MEMBER_ACCESS {
    OB_PUBLIC_MEMBER = 0,
    OB_PRIVATE_MEMBER = 1,
    OB_PROTECTED_MEMBER = 2,
    OB_SYSTEM_MEMBER = 3
}
pub type POB_MEMBER_ACCESS = *mut OB_MEMBER_ACCESS;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_data_styles {
    UNDECLARED_STYLE = 0,
    INT_STYLE = 1,
    FLOAT_STYLE = 2,
    PTR_STYLE = 3,
    CONST_STYLE = 4,
    ID_STYLE = 5,
    OBINST_STYLE = 6,
    LONG_STYLE = 7
}
pub use self::ob_data_styles as OB_DATASTYLE;
pub type OB_BASE_ID = USHORT;
pub type OB_GROUP_ID = OB_BASE_ID;
pub type POB_GROUP_ID = *mut OB_BASE_ID;
pub type OB_GROUP_HNDL = OB_GROUP_ID;
pub type POB_GROUP_HNDL = *mut OB_GROUP_ID;
pub type OB_CLASS_ID = OB_BASE_ID;
pub type POB_CLASS_ID = *mut OB_BASE_ID;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_class_hndl {
    pub group_hndl: OB_GROUP_HNDL,
    pub class_id: OB_CLASS_ID
}
pub type OB_CLASS_HNDL = ob_class_hndl;
pub type POB_CLASS_HNDL = *mut ob_class_hndl;
pub type OB_INST_ID = *mut ::std::os::raw::c_void;
pub type POB_INST_ID = *mut OB_INST_ID;
pub type OB_ARRAY_ID = *mut ::std::os::raw::c_void;
pub type OB_EVT_ID = OB_BASE_ID;
pub type OB_EVT_TOKEN_ID = OB_BASE_ID;
pub type POB_EVT_TOKEN_ID = *mut OB_BASE_ID;
pub type OB_VTABLE_ID = OB_BASE_ID;
pub type POB_VTABLE_ID = *mut OB_BASE_ID;
pub type OB_PROTO_ID = OB_BASE_ID;
pub type POB_PROTO_ID = *mut OB_BASE_ID;
pub type OB_ROUT_ID = OB_BASE_ID;
pub type POB_ROUT_ID = *mut OB_BASE_ID;
pub type OB_MODULE_ID = OB_BASE_ID;
pub type POB_MODULE_ID = *mut OB_BASE_ID;
pub type OB_SYM_ID = OB_BASE_ID;
pub type POB_SYM_ID = *mut OB_BASE_ID;
pub type OB_CONST_REF = ULONG;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_TYPE_KIND {
    OB_SIMPLE_TYPE = 0,
    OB_SYSTEM_TYPE = 1,
    OB_USER_TYPE = 2,
    OB_UNDEFINED_TYPE = 3
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_ALLOC_TYPE {
    OB_DYN = 0,
    OB_STATIC = 1,
    OB_ASSOC = 2
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_func_type {
    OB_LOCAL_FUNC_DEF = 0,
    OB_GLOBAL_FUNC_REF = 1,
    OB_DLL_FUNC_DEF = 2,
    OB_SYSTEM_FUNC_DEF = 3,
    OB_RPC_FUNC_DEF = 4,
    OB_SYSDLL_FUNC_DEF = 5,
    OB_PSPP_FUNC_DEF = 6
}
pub use self::ob_func_type as OB_FUNC_TYPE;
pub type POB_FUNC_TYPE = *mut ob_func_type;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_protoarg_type {
    OB_ARG_VAL = 0,
    OB_ARG_REF = 1,
    OB_ARG_VARLIST = 2,
    OB_ARG_READONLY = 3
}
pub use self::ob_protoarg_type as OB_PROTOARG_TYPE;
pub type POB_PROTOARG_TYPE = *mut ob_protoarg_type;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_script_type {
    OB_EXTERNAL_FUNC_SCRIPT = 0,
    OB_OBJECT_FUNC_SCRIPT = 1,
    OB_EVENT_SCRIPT = 2,
    OB_EVENTCALL_SCRIPT = 3
}
pub use self::ob_script_type as OB_SCRIPT_TYPE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_ROUT_TYPE {
    OB_FUNCTION = 0,
    OB_EVENT = 1,
    OB_ANY_ROUT_TYPE = 2
}
pub type OB_SUBPOOL = pbstg_subpool;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_SOURCE_BLK_TYPE {
    OB_FORWARD_BLOCK = 0,
    OB_VAR_BLOCK = 1,
    OB_VAR_DECL_BLOCK = 2,
    OB_TYPEDEF_BLOCK = 3,
    OB_ON_EVT_BLOCK = 4,
    OB_FUNC_BLOCK = 5,
    OB_SUBROUTINE_BLOCK = 6,
    OB_PROTOTYPE_BLOCK = 7,
    OB_INSTVAR_BLOCK = 8,
    OB_FWDPROTO_BLOCK = 9,
    OB_NAMESPACE_BLOCK = 10
}
pub type POB_SOURCE_BLK_TYPE = *mut OB_SOURCE_BLK_TYPE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_glob_refstyle {
    OB_GLOB_PARENT_REF = 0,
    OB_GLOB_ATTR_REF = 1,
    OB_GLOB_OTHER_REF = 2,
    OB_GLOB_NOT_REF = 3
}
pub use self::ob_glob_refstyle as OB_GLOB_REFSTYLE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_BINARY_DYNTYPE {
    OB_UNBOUNDED_BINARY = 0,
    OB_FIXED_BINARY = 1
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_OBJECT_TYPE_FLAG {
    OB_OBJECT_TYPE = 0,
    OB_NON_OBJECT_TYPE = 1
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_INSTANTIATE_TYPE {
    OB_MANUAL_INSTANTIATE = 0,
    OB_AUTO_INSTANTIATE = 1
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union shlnode_prev {
    pub prev: *mut shlistnode,
    pub vprev: *mut ::std::os::raw::c_void
}
pub type SHLNODE_PREV = shlnode_prev;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union shlnode_next {
    pub next: *mut shlistnode,
    pub vnext: *mut ::std::os::raw::c_void
}
pub type SHLNODE_NEXT = shlnode_next;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union shlnode_data {
    pub data: *mut ::std::os::raw::c_void,
    pub vdata: *mut ::std::os::raw::c_void
}
pub type SHLNODE_DATA = shlnode_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shlistnode {
    pub p: SHLNODE_PREV,
    pub n: SHLNODE_NEXT,
    pub d: SHLNODE_DATA
}
pub type shlnode = shlistnode;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union shlisthead {
    pub head: *mut shlnode,
    pub vhead: *mut ::std::os::raw::c_void
}
pub type SHLISTHEAD = shlisthead;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union shlisttail {
    pub tail: *mut shlnode,
    pub vtail: *mut ::std::os::raw::c_void
}
pub type SHLISTTAIL = shlisttail;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct shlistx {
    pub h: SHLISTHEAD,
    pub t: SHLISTTAIL,
    pub current: *mut shlnode,
    pub sa: ppbstg_anchor,
    pub subpool: pbstg_subpool,
    pub count: UINT
}
pub type shlist = shlistx;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sh_growblock {
    pub block: *mut ::std::os::raw::c_void,
    pub incr: UINT,
    pub size: UINT,
    pub pos: UINT,
    pub struct_size: UINT
}
pub type SH_GROWBLOCK = sh_growblock;
pub type PSH_GROWBLOCK = *mut sh_growblock;
pub type RT_BREAK_PROC = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void) -> INT
>;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rt_mode {
    RT_DEVELOPMENT_MODE = 0,
    RT_RUNTIME_MODE = 1
}
pub use self::rt_mode as RT_MODE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rt_opt_mode {
    RT_OPTIMIZED = 0,
    RT_NOT_OPTIMIZED = 1
}
pub use self::rt_opt_mode as RT_OPT_MODE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_mode {
    OB_LINK_AS_YOU_GO_MODE = 0,
    OB_BUILD_EXE_MODE = 1,
    OB_RUN_EXE_MODE = 2,
    OB_COMPILE_MODE = 3,
    OB_DEBUG_MODE = 4,
    OB_DEFAULT_MODE = 5,
    OB_BUILD_APPL_REPORT = 6,
    OB_BUILD_COMPILE_LIST_MODE = 7,
    OB_BUILD_OBJECT_REPORT = 8,
    OB_OBJECT_LOAD = 9
}
pub use self::ob_mode as OB_MODE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_exe_code_type {
    OB_PCODE_EXE = 0,
    OB_CCODE_EXE = 1
}
pub use self::ob_exe_code_type as OB_EXE_CODE_TYPE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_runtime_error_info {
    pub group: POB_GROUP,
    pub class_id: OB_CLASS_ID,
    pub routine_name: LPTSTR,
    pub line_no: UINT,
    pub script_type: OB_SCRIPT_TYPE
}
pub type OB_RUNTIME_ERROR_INFO = ob_runtime_error_info;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_response_window_stack_node {
    pub routine_level: UINT,
    pub expr_stack_ptr: INT
}
pub type ResponseWindowStackNode = ob_response_window_stack_node;
pub type POB_THIS = *mut ob_this;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_this {
    pub __vtbl: *mut ::std::os::raw::c_void,
    pub obthis: POB_THIS,
    pub rtthis: POB_THIS,
    pub dbgthis: *mut SH_DBG_THIS,
    pub stgthis: ppbstg_anchor,
    pub curr_pcode: UINT,
    pub pcode_done: BOOL,
    pub pBreakPointList: *mut shlist,
    pub bStep: INT,
    pub rtBreakCallback: [RT_BREAK_PROC; 2usize],
    pub pUserData: *mut ::std::os::raw::c_void,
    pub pTransactionList: *mut shlist,
    pub bIgnoreErrors: BOOL,
    pub bTerminateRuntime: BOOL,
    pub clshndl: OB_CLASS_HNDL,
    pub appclshndl: OB_CLASS_HNDL,
    pub event_id: OB_EVT_ID,
    pub appinst: OB_INST_ID,
    pub subpool: pbstg_subpool,
    pub mode: RT_MODE,
    pub smthis: *mut ::std::os::raw::c_void,
    pub obMode: OB_MODE,
    pub iPCodeCounter: UINT,
    pub opt_mode: RT_OPT_MODE,
    pub pDllList: *mut shlist,
    pub curr_pcode_blk: *mut ::std::os::raw::c_void,
    pub iContextCount: UINT,
    pub bHaltClose: BOOL,
    pub bDontTerminate: BOOL,
    pub curr_line_block_loc: UINT,
    pub last_break_line_no: UINT,
    pub last_pcode_line_no: UINT,
    pub pThreadStart: *mut *mut ::std::os::raw::c_void,
    pub pThreadNode: *mut *mut ::std::os::raw::c_void,
    pub halt_close_nest: UINT,
    pub cmthis: *mut cm_this,
    pub pLibraryList: *mut shlist,
    pub pNameList: *mut shlist,
    pub pGroupList: *mut ::std::os::raw::c_void,
    pub pLookSymKeyFunction: *mut ::std::os::raw::c_void,
    pub scnthis: *mut ::std::os::raw::c_void,
    pub iGroupListIncr: UINT,
    pub iGroupListPos: UINT,
    pub iGroupListSize: UINT,
    pub client_subpool: pbstg_subpool,
    pub perm_subpool: pbstg_subpool,
    pub result_subpool: pbstg_subpool,
    pub temp_subpool: pbstg_subpool,
    pub group_stack: *mut shlist,
    pub curr_obj_group: *mut ::std::os::raw::c_void,
    pub sys_typedef_group: *mut ::std::os::raw::c_void,
    pub curr_routnode: *mut ::std::os::raw::c_void,
    pub curr_obinst: OB_INST_ID,
    pub lib_handle: *mut ::std::os::raw::c_void,
    pub run_stack: *mut shlist,
    pub rtinst_stack: *mut ::std::os::raw::c_void,
    pub def_appl_groupname: LPTSTR,
    pub def_appl_libname: LPTSTR,
    pub appl_group: *mut ::std::os::raw::c_void,
    pub bInRuntime: BOOL,
    pub appl_filter: UINT,
    pub hExecutable: *mut ::std::os::raw::c_void,
    pub iExeGroupCounter: UINT,
    pub pExeGroupList: *mut shlist,
    pub pExeResourceHash: *mut ::std::os::raw::c_void,
    pub expr_stack: *mut ot_eval_node,
    pub expr_stack_size: UINT,
    pub expr_stack_ptr: *mut ot_eval_node,
    pub arglist_stack: *mut shlist,
    pub error_list: *mut shlist,
    pub bGotLinkError: BOOL,
    pub lvalue_subpool: pbstg_subpool,
    pub bNoDuplicateSymbols: BOOL,
    pub unused: *mut shlist,
    pub curr_class_hndl: OB_CLASS_HNDL,
    pub curr_array_inst: OB_ARRAY_ID,
    pub ErrorCode: UINT,
    pub set_return_called: BOOL,
    pub sys_group_hndl: OB_GROUP_HNDL,
    pub pSavedGroupList: *mut ::std::os::raw::c_void,
    pub lmithis: *mut ::std::os::raw::c_void,
    pub curr_runstk_node: *mut ::std::os::raw::c_void,
    pub ps_options: ULONG,
    pub exe_code_type: OB_EXE_CODE_TYPE,
    pub pExecutableName: LPTSTR,
    pub evaled_arglist: *mut ::std::os::raw::c_void,
    pub lvalue_info: *mut ::std::os::raw::c_void,
    pub curr_arg_pos: UINT,
    pub return_value: *mut ot_eval_node,
    pub called_return_value: *mut ot_eval_node,
    pub p_error_info: *mut ::std::os::raw::c_void,
    pub routine_level: ::std::os::raw::c_long,
    pub ulDServFlags: ULONG,
    pub working_group: *mut ::std::os::raw::c_void,
    pub callback_data: *mut ::std::os::raw::c_void,
    pub callback_data2: *mut ::std::os::raw::c_void,
    pub parentObThis: POB_THIS,
    pub check_for_locked_menu: BOOL,
    pub pEntryBuffer: LPTSTR,
    pub pHugeEntryBuffer: *mut TCHAR,
    pub lEntryPos: ::std::os::raw::c_long,
    pub lEntrySize: ::std::os::raw::c_long,
    pub pTransPool: *mut ::std::os::raw::c_void,
    pub pGroupIdArray: PSH_GROWBLOCK,
    pub criticalSection: *mut ::std::os::raw::c_void,
    pub pCmCompilerError: *mut ::std::os::raw::c_void,
    pub GeneralFlags: ULONG,
    pub unused_2: *mut ::std::os::raw::c_void,
    pub bIgnoreLinkErrors: BOOL,
    pub new_next_pcode: UINT,
    pub pSharedObjMgr: *mut ::std::os::raw::c_void,
    pub traceStruct: LPVOID,
    pub bTracing: BOOL,
    pub pWatchPointList: *mut shlist,
    pub dbg_callstk: *mut shlist,
    pub bStepLine: UINT,
    pub bStepRoutineLevel: INT,
    pub SecureRuntimeSession: ULONG,
    pub lGCTimeLimit: DWORD,
    pub lLastGCTime: DWORD,
    pub bGarbageCollecting: BOOL,
    pub pRequestQueue: *mut ::std::os::raw::c_void,
    pub pErrorCallback: *mut ::std::os::raw::c_void,
    pub pWndDispatchHndlr: *mut ::std::os::raw::c_void,
    pub idOwnerThread: DWORD,
    pub bActiveSession: BOOL,
    pub bIsAsyncCall: BOOL,
    pub pLocalSession: *mut ::std::os::raw::c_void,
    pub local_variables: *mut ::std::os::raw::c_void,
    pub num_variables: UINT,
    pub curr_stack_bias: UINT,
    pub bDeferredDelete: BOOL,
    pub pSessOB_ICONTEXT: *mut ::std::os::raw::c_void,
    pub pDefltOB_ICONTEXT: *mut ::std::os::raw::c_void,
    pub pSessionList: *mut shlist,
    pub lpstrTypdefPblName: LPTSTR,
    pub bNoMessageBoxForError: BOOL,
    pub bInSystemError: BOOL,
    pub last_break_routine: *mut ::std::os::raw::c_void,
    pub bInitDebugMode: BOOL,
    pub pContextObject: *mut ::std::os::raw::c_void,
    pub pLocalDebugSession: *mut ::std::os::raw::c_void,
    pub breakpointId: ULONG,
    pub pGroupLoader: *mut ::std::os::raw::c_void,
    pub pResultsetInfo: *mut ::std::os::raw::c_void,
    pub pErrorCallbackState: *mut ::std::os::raw::c_void,
    pub exception_stack: *mut shlist,
    pub thrown_exception: *mut ::std::os::raw::c_void,
    pub curr_exception: *mut ::std::os::raw::c_void,
    pub gosub_stack: *mut shlist,
    pub response_window_stack: ob_this_ResponseWindowStack,
    pub objects_creating: *mut shlist,
    pub pb_session: *mut ::std::os::raw::c_void,
    pub pMetaObject: *mut ::std::os::raw::c_void
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_this_ResponseWindowStack {
    pub stack: *mut ResponseWindowStackNode,
    pub capacity: UINT,
    pub count: UINT
}
pub type OB_THIS = ob_this;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union ob_value {
    pub int_val: SHORT,
    pub fl_val: f32,
    pub ptr: *mut ::std::os::raw::c_void,
    pub const_ref: OB_CONST_REF,
    pub ob_inst: *mut ::std::os::raw::c_void,
    pub id: USHORT,
    pub uint_val: USHORT,
    pub long_val: ::std::os::raw::c_long,
    pub ulong_val: ULONG,
    pub byte_val: ::std::os::raw::c_uchar
}
pub type OB_VALUE = ob_value;
pub type OB_INFO_FLAGS = USHORT;
pub type POB_INFO_FLAGS = *mut USHORT;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_info {
    pub info: OB_INFO_FLAGS,
    pub type_: OB_CLASS_ID
}
pub type OB_INFO = ob_info;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_data {
    pub val: OB_VALUE,
    pub info: OB_INFO_FLAGS,
    pub type_: OB_CLASS_ID
}
pub type OB_DATA = ob_data;
pub type POB_DATA = *mut ob_data;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_var {
    pub val: OB_VALUE,
    pub null_val: UINT
}
pub type OB_VAR = ob_var;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_ARRAY_SYMBOL_STYLE {
    OB_UNBOUNDED_ARRAY = 0,
    OB_SIMPLE_ARRAY = 1,
    OB_COMPLEX_ARRAY = 2
}
pub type KEY_FUNC = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct shhashx {
    pub sa: ppbstg_anchor,
    pub subpool: pbstg_subpool,
    pub table: *mut *mut shlist,
    pub key_func: KEY_FUNC,
    pub key_arg: *mut ::std::os::raw::c_void,
    pub no_slots: INT,
    pub current_slot: INT,
    pub entries: UINT,
    pub unique: BOOL,
    pub listHash: BOOL
}
pub type shhash = shhashx;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_ERROR {
    OB_SUCCESS = 0,
    OB_OPEN_ERROR = 1,
    OB_READ_ERROR = 2,
    OB_WRITE_ERROR = 3,
    OB_SCAN_ERROR = 4,
    OB_VERSION_ERROR = 5,
    OB_NOTFOUND = 6,
    OB_SEMI_COMPILED_OBJ_ERROR = 7,
    OB_MISSING_ANCESTOR_ERROR = 8,
    OB_DUPLICATE_ANCESTOR_ERROR = 9,
    OB_INTERNAL_OVERFLOW = 10,
    OB_GOT_RUNTIME_ERROR = 11,
    OB_EXECUTION_ERROR = 12,
    OB_GENERAL_ERROR = 13,
    OB_GROUP_WRONG_FORMAT_ERROR = 14
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct shBinary {
    pub len: ULONG,
    pub data: [::std::os::raw::c_uchar; 1usize]
}
pub type PSH_BINARY = *mut shBinary;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_CONPOOL_ITEM_TYPE {
    OB_CONPOOL_STRING = 0,
    OB_CONPOOL_SHORT = 1,
    OB_CONPOOL_LONG = 2,
    OB_CONPOOL_FLOAT = 3,
    OB_CONPOOL_DOUBLE = 4,
    OB_CONPOOL_DEC = 5,
    OB_CONPOOL_TIME = 6,
    OB_CONPOOL_FUNCARG = 7,
    OB_CONPOOL_ARRAYDEF = 8,
    OB_CONPOOL_DBSTMT = 9,
    OB_CONPOOL_DBOUTVAR = 10,
    OB_CONPOOL_PCODE = 11,
    OB_CONPOOL_FLDNAMEID = 12,
    OB_CONPOOL_ROUTNAMEID = 13,
    OB_CONPOOL_OBINFO = 14,
    OB_CONPOOL_OBDATA = 15,
    OB_CONPOOL_FUNCTMPLTARG = 16,
    OB_CONPOOL_FUNCTMPLT = 17,
    OB_CONPOOL_CLSNAMEID = 18,
    OB_CONPOOL_ARRAYDATA = 19,
    OB_CONPOOL_DBVARS = 20,
    OB_CONPOOL_DBSTMT_INDIRECT = 21,
    OB_CONPOOL_CLASSID = 22,
    OB_CONPOOL_LONGLONG = 23
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct conpool_map {
    pub offset: OB_CONST_REF,
    pub item_type: SHORT,
    pub no_items: USHORT
}
pub type OB_CONPOOL_MAP = conpool_map;
pub type POB_CONPOOL_MAP = *mut conpool_map;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct old_conpool_map {
    pub item_type: SHORT,
    pub offset: OB_CONST_REF,
    pub no_items: USHORT
}
pub type OB_OLD_CONPOOL_MAP = old_conpool_map;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct perm_conpool {
    pub pool_size: ULONG,
    pub map_size: ULONG
}
pub type OB_PERM_CONPOOL = perm_conpool;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct temp_conpool {
    pub pool: LPBYTE,
    pub map: POB_CONPOOL_MAP,
    pub alloc_incr: UINT,
    pub next_free: ULONG,
    pub next_maploc: ULONG,
    pub subpool: OB_SUBPOOL
}
pub type OB_TEMP_CONPOOL = temp_conpool;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_conpool {
    pub ps: OB_PERM_CONPOOL,
    pub ts: OB_TEMP_CONPOOL,
    pub strings_in_pool: *mut shhash
}
pub type OB_CONPOOL = ob_conpool;
pub type POB_CONPOOL = *mut ob_conpool;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_con_string_node {
    pub name: OB_CONST_REF
}
pub type OB_CON_STRING_NODE = ob_con_string_node;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_sym_node {
    pub sym_id: OB_SYM_ID,
    pub name: OB_CONST_REF
}
pub type OB_SYM_NODE = ob_sym_node;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_symtab {
    pub table: *mut shhash,
    pub no_slots: INT,
    pub curr_id: INT,
    pub conpool: POB_CONPOOL,
    pub subpool: OB_SUBPOOL
}
pub type OB_SYMTAB = ob_symtab;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_LOOKUP_READONLY_VAL {
    OB_NOT_READONLY = 0,
    OB_IS_READONLY = 1
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_LOOKUP_INDIRECT_VAL {
    OB_NOT_INDIRECT = 0,
    OB_IS_INDIRECT = 1
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_LOOKUP_CONSTANT_VAL {
    OB_NOT_CONSTANT = 0,
    OB_IS_CONSTANT = 1
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_LOOKUP_DUPFIELD_VAL {
    OB_NOT_DUPFIELD = 0,
    OB_IS_DUPFIELD = 1
}
pub type OB_LOOKUP_INFO = USHORT;
pub type POB_LOOKUP_INFO = *mut USHORT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_lookup_entry {
    pub flags: OB_LOOKUP_INFO,
    pub padding: SHORT,
    pub sym_info: OB_CONST_REF,
    pub name: OB_CONST_REF,
    pub data: OB_DATA
}
pub type OB_LOOKUP_ENTRY = ob_lookup_entry;
pub type POB_LOOKUP_ENTRY = *mut ob_lookup_entry;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct perm_lookup {
    pub alloc_size: USHORT
}
pub type OB_PERM_LOOKUP = perm_lookup;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct temp_lookup {
    pub table: POB_LOOKUP_ENTRY,
    pub alloc_incr: UINT,
    pub no_slots: UINT,
    pub slot_incr: UINT,
    pub no_entries: UINT,
    pub symtab: *mut OB_SYMTAB,
    pub conpool: POB_CONPOOL,
    pub subpool: OB_SUBPOOL
}
pub type OB_TEMP_LOOKUP = temp_lookup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ob_lookup_table {
    pub ps: OB_PERM_LOOKUP,
    pub ts: OB_TEMP_LOOKUP
}
pub type OB_LOOKUP = ob_lookup_table;
pub type POB_LOOKUP = *mut ob_lookup_table;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct perm_looksym {
    pub lookup_slots: USHORT,
    pub conpool_size: USHORT,
    pub symtab_slots: USHORT
}
pub type OB_PERM_LOOKSYM = perm_looksym;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct temp_looksym {
    pub lookup: POB_LOOKUP,
    pub conpool: POB_CONPOOL,
    pub symtab: *mut OB_SYMTAB
}
pub type OB_TEMP_LOOKSYM = temp_looksym;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_look_symtab {
    pub ps: OB_PERM_LOOKSYM,
    pub padding: SHORT,
    pub ts: OB_TEMP_LOOKSYM
}
pub type OB_LOOK_SYMTAB = ob_look_symtab;
pub type POB_LOOK_SYMTAB = *mut ob_look_symtab;
pub type PDYNARR_INIT_FN = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: POB_THIS,
        arg2: *mut tag_OB_DYNARRAY,
        arg3: *mut ::std::os::raw::c_void
    ) -> BOOL
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tag_OB_DYNARRAY {
    pub alloc_obthis: POB_THIS,
    pub alloc_subpool: OB_SUBPOOL,
    pub elementSize: USHORT,
    pub blockSize: USHORT,
    pub usedSize: ULONG,
    pub numBlocks: ULONG,
    pub initFn: PDYNARR_INIT_FN,
    pub userData: *mut ::std::os::raw::c_void,
    pub blockArray: *mut LPBYTE
}
pub type OB_DYNARRAY = tag_OB_DYNARRAY;
pub type PTNULL = *mut LPBYTE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_narray_type {
    OB_ARRAY_STATIC = 0,
    OB_ARRAY_DYNAMIC = 1
}
pub use self::ob_narray_type as OB_NARRAY_TYPE;
pub type PNARRAY_INIT_FN = ::std::option::Option<
    unsafe extern "C" fn(arg1: POB_THIS, arg2: *mut tag_OB_NARRAY, arg3: *mut ::std::os::raw::c_void) -> BOOL
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tag_OB_NARRAY {
    pub elementType: OB_CLASS_HNDL,
    pub type_: USHORT,
    pub userData: USHORT,
    pub nitems: ULONG,
    pub elementSize: USHORT,
    pub nDims: USHORT,
    pub initFn: PNARRAY_INIT_FN,
    pub data: *mut ::std::os::raw::c_void,
    pub nullData: PTNULL,
    pub bFreeData: SHORT,
    pub padding: USHORT,
    pub bounds: [::std::os::raw::c_long; 2usize]
}
pub type OB_NARRAY = tag_OB_NARRAY;
pub type OB_ARRAY_INST = OB_NARRAY;
pub type POB_ARRAY_INST = *mut OB_NARRAY;
pub use self::OB_NARRAY_TYPE as OB_ARRAY_TYPE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_arraydef {
    pub flags: USHORT,
    pub varinfo: OB_INFO_FLAGS,
    pub bounds: [::std::os::raw::c_long; 1usize]
}
pub type OB_ARRAYDEF = ob_arraydef;
pub type POB_ARRAYDEF = *mut ob_arraydef;
pub type OB_PCODE_OPCODE = USHORT;
pub type OB_PCODE_OPERAND = USHORT;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_pcode_node {
    pub opcode: OB_PCODE_OPCODE,
    pub op1: OB_PCODE_OPERAND,
    pub op2: OB_PCODE_OPERAND,
    pub op3: OB_PCODE_OPERAND,
    pub op4: OB_PCODE_OPERAND,
    pub op5: OB_PCODE_OPERAND
}
pub type OB_PCODE_NODE = ob_pcode_node;
pub type OB_THREAD_OPCODE = PFV;
pub type OB_THREAD_OPERAND = ULONG;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_thread_node {
    pub semantic: OB_THREAD_OPCODE,
    pub op1: OB_THREAD_OPERAND,
    pub op2: OB_THREAD_OPERAND,
    pub op3: OB_THREAD_OPERAND,
    pub op4: OB_THREAD_OPERAND,
    pub op5: OB_THREAD_OPERAND
}
pub type OB_THREAD_NODE = ob_thread_node;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_pcode_line_node {
    pub line_no: USHORT,
    pub pcode_loc: USHORT
}
pub type OB_PCODE_LINE_NODE = ob_pcode_line_node;
pub type POB_PCODE_LINE_NODE = *mut ob_pcode_line_node;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_perm_pcode_blk {
    pub len: USHORT,
    pub no_line_block: USHORT,
    pub max_stack_depth: USHORT
}
pub type OB_PERM_PCODE_BLK = ob_perm_pcode_blk;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_temp_pcode_blk {
    pub block: LPBYTE,
    pub line_block: POB_PCODE_LINE_NODE,
    pub len_incr: UINT,
    pub line_block_incr: UINT,
    pub curr_pos: UINT,
    pub curr_line_pos: UINT,
    pub block_is_thread: BOOL,
    pub thread: LPBYTE,
    pub thd_len: SHORT,
    pub thread_line_block: POB_PCODE_LINE_NODE
}
pub type OB_TEMP_PCODE_BLK = ob_temp_pcode_blk;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ob_pcode_blk {
    pub ps: OB_PERM_PCODE_BLK,
    pub ts: OB_TEMP_PCODE_BLK
}
pub type OB_PCODE_BLK = ob_pcode_blk;
pub type POB_PCODE_BLK = *mut ob_pcode_blk;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_perm_routnode {
    pub status: USHORT,
    pub proto_id: OB_PROTO_ID
}
pub type OB_PERM_ROUTNODE = ob_perm_routnode;
pub type POB_PERM_ROUTNODE = *mut ob_perm_routnode;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_routnode {
    pub perm_entry: POB_PERM_ROUTNODE,
    pub source: LPTSTR,
    pub len: UINT,
    pub pcode: POB_PCODE_BLK,
    pub rout_symtab: OB_LOOK_SYMTAB,
    pub local_conpool: POB_CONPOOL
}
pub type OB_ROUTNODE = ob_routnode;
pub type POB_ROUTNODE = *mut ob_routnode;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_perm_routlist {
    pub no_slots: USHORT
}
pub type OB_PERM_ROUTLIST = ob_perm_routlist;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_temp_routlist {
    pub list: POB_ROUTNODE,
    pub alloc_incr: UINT,
    pub alloc_size: UINT,
    pub perm_list: POB_PERM_ROUTNODE,
    pub perm_alloc_incr: UINT,
    pub perm_alloc_size: UINT,
    pub slot_incr: UINT,
    pub next_free: UINT
}
pub type OB_TEMP_ROUTLIST = ob_temp_routlist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ob_routlist {
    pub ps: OB_PERM_ROUTLIST,
    pub ts: OB_TEMP_ROUTLIST
}
pub type OB_ROUTLIST = ob_routlist;
pub type POB_ROUTLIST = *mut ob_routlist;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _tagSH_DEC {
    pub v: [USHORT; 7usize],
    pub flags: USHORT
}
pub type SH_DEC = _tagSH_DEC;
pub type PSH_DEC = *mut _tagSH_DEC;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SH_TIME {
    pub tm_msec: ::std::os::raw::c_long,
    pub tm_year: SHORT,
    pub tm_mon: ::std::os::raw::c_uchar,
    pub tm_mday: ::std::os::raw::c_uchar,
    pub tm_hour: ::std::os::raw::c_uchar,
    pub tm_min: ::std::os::raw::c_uchar,
    pub tm_sec: ::std::os::raw::c_uchar,
    pub tm_filler: ::std::os::raw::c_uchar
}
pub type PSH_TIME = *mut SH_TIME;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_data_info {
    pub sym_id: OB_SYM_ID,
    pub name: LPTSTR,
    pub class_hndl: OB_CLASS_HNDL,
    pub data: OB_DATA,
    pub enumname: LPTSTR,
    pub scope: OB_MEMBER_ACCESS,
    pub read_access: OB_MEMBER_ACCESS,
    pub write_access: OB_MEMBER_ACCESS,
    pub flags: UINT,
    pub set_func: LPTSTR,
    pub get_func: LPTSTR,
    pub lookupObject: *mut ::std::os::raw::c_void,
    pub pWatchpoint: *mut ::std::os::raw::c_void,
    pub isArrayElement: BOOL,
    pub arrayIndex: ::std::os::raw::c_long,
    pub array_set_func: LPTSTR,
    pub array_get_func: LPTSTR,
    pub array_upper_func: LPTSTR
}
pub type OB_DATA_INFO = ob_data_info;
pub type POB_DATA_INFO = *mut ob_data_info;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_arg_info {
    pub argname: LPTSTR,
    pub datatype: LPTSTR,
    pub argtype: OB_PROTOARG_TYPE,
    pub grouping: OB_GROUPTYPE,
    pub array_bounds: LPTSTR
}
pub type OB_ARG_INFO = ob_arg_info;
pub type POB_ARG_INFO = *mut ob_arg_info;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_class_info {
    pub classname: LPTSTR,
    pub class_hndl: OB_CLASS_HNDL
}
pub type OB_CLASS_INFO = ob_class_info;
pub type POB_CLASS_INFO = *mut ob_class_info;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_event_info {
    pub event_name: LPTSTR,
    pub token_name: LPTSTR,
    pub class_hndl: OB_CLASS_HNDL,
    pub vtable_id: OB_VTABLE_ID
}
pub type OB_EVENT_INFO = ob_event_info;
pub type POB_EVENT_INFO = *mut ob_event_info;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_CALL_TYPE {
    OB_SYSFUNC_CALL = 0,
    OB_DLLFUNC_CALL = 1,
    OB_GLOBFUNC_CALL = 2,
    OB_OBJFUNC_CALL = 3,
    OB_LOCALFUNC_CALL = 4,
    OB_PARENTFUNC_CALL = 5,
    OB_PRIMARYFUNC_CALL = 6
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_PROTOREF_ERROR {
    OB_PROTOREF_OK = 0,
    OB_PROTOREF_NOTFOUND = 1,
    OB_PROTOREF_BADNOARGS = 2,
    OB_PROTOREF_BADARGS = 3,
    OB_PROTOREF_INACCESSABLE = 4,
    OB_PROTOREF_BADREFARG = 5,
    OB_PROTOREF_BAD = 6,
    OB_PROTOREF_BADREFTYPE = 7,
    OB_PROTOREF_BADOVERLOAD = 8,
    OB_PROTOREF_ANCREFTYPE = 9,
    OB_PROTOREF_AMBIGUOUS = 10
}
pub type POB_PROTOREF_ERROR = *mut OB_PROTOREF_ERROR;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_funccall_info {
    pub funcname: LPTSTR,
    pub argtypes: POB_CLASS_ID,
    pub no_args: UINT,
    pub functype: OB_CLASS_ID,
    pub id: UINT,
    pub calltype: OB_CALL_TYPE,
    pub dllname: LPTSTR,
    pub group_id: OB_GROUP_HNDL,
    pub class_id: OB_CLASS_ID
}
pub type OB_FUNCCALL_INFO = ob_funccall_info;
pub type POB_FUNCCALL_INFO = *mut ob_funccall_info;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_array_dim {
    pub upbound: INT,
    pub lowbound: INT
}
pub type OB_ARRAY_DIM = ob_array_dim;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_array_info_tag {
    pub array_data: *mut ::std::os::raw::c_void,
    pub no_dims: UINT,
    pub dimensions: [OB_ARRAY_DIM; 1usize]
}
pub type OB_ARRAY_INFO = ob_array_info_tag;
pub type POB_ARRAY_INFO = *mut ob_array_info_tag;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_enum_info {
    pub name: LPTSTR,
    pub value: INT
}
pub type OB_ENUM_INFO = ob_enum_info;
pub type POB_ENUM_INFO = *mut ob_enum_info;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_mac_target {
    OB_MAC_POWERPC_TARGET = 0,
    OB_MAC_68K_TARGET = 1,
    OB_MAC_FAT_TARGET = 2
}
pub use self::ob_mac_target as OB_MAC_TARGET;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_exec_optimize {
    OB_OPTIMIZE_SPEED = 0,
    OB_OPTIMIZE_SPACE = 1,
    OB_OPTIMIZE_NONE = 2,
    OB_OPTIMIZE_DEBUG = 3
}
pub use self::ob_exec_optimize as OB_EXEC_OPTIMIZE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_exec_category {
    EXEC_CHECKING_REFERENCES = 0,
    EXEC_WRITING_OBJECT = 1,
    EXEC_GENERATING_CODE_FOR_OBJECT = 2,
    EXEC_COMPILING_FILE = 3,
    EXEC_LINKING = 4
}
pub use self::ob_exec_category as OB_EXEC_STAGE;
pub type POB_EXEC = *mut ob_exec;
pub type OB_EXEC_CALLBACK = ::std::option::Option<unsafe extern "C" fn(arg1: POB_EXEC) -> BOOL>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_exec {
    pub bBuildExe: BOOL,
    pub bBuildInterfaceLib: BOOL,
    pub bGenerateCode: BOOL,
    pub bGenLineInfo: BOOL,
    pub bGenTrace: BOOL,
    pub bGen16bit: BOOL,
    pub bOpenServer: BOOL,
    pub eOptimize: OB_EXEC_OPTIMIZE,
    pub eMacTarget: OB_MAC_TARGET,
    pub lpszExeName: LPTSTR,
    pub lpszIconName: LPTSTR,
    pub pLibList: *mut ::std::os::raw::c_void,
    pub pGlobals: *mut ::std::os::raw::c_void,
    pub hParent: *mut ::std::os::raw::c_void,
    pub iErrorCode: INT,
    pub lpszErrorArg: LPTSTR,
    pub bNewVisualStyleControls: BOOL,
    pub execStage: OB_EXEC_STAGE,
    pub lpszStageArgument: LPTSTR,
    pub lpszLibraryName: LPTSTR,
    pub fnCallback: OB_EXEC_CALLBACK,
    pub pCodeGen: *mut ::std::os::raw::c_void,
    pub bGenerateCodeOnly: BOOL,
    pub lpszCompany: LPTSTR,
    pub lpszDescription: LPTSTR,
    pub lpszCopyright: LPTSTR,
    pub lpszProduct: LPTSTR,
    pub lpszVersion: LPTSTR,
    pub lpszVersionNum: LPTSTR,
    pub lpszFileVersion: LPTSTR,
    pub lpszFileVersionNum: LPTSTR
}
pub type OB_EXEC = ob_exec;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_exec_lib {
    pub bDynamic: BOOL,
    pub lpszLibName: LPTSTR,
    pub lpszDynamicLibFile: LPTSTR,
    pub lpszResFile: LPTSTR
}
pub type OB_EXEC_LIB = ob_exec_lib;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_conflict_list {
    pub error_type: OB_ERROR,
    pub original_group_name: LPTSTR,
    pub conflict_group_name: LPTSTR,
    pub class_name: LPTSTR
}
pub type OB_CONFLICT_LIST = ob_conflict_list;
pub type POB_CONFLICT_LIST = *mut ob_conflict_list;
pub type POB_SOURCE_BLOCK = *mut TCHAR;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_compile_list {
    pub lib_name: LPTSTR,
    pub entry_name: LPTSTR
}
pub type OB_COMPILE_LIST = ob_compile_list;
pub type POB_COMPILE_LIST = *mut ob_compile_list;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_hierarchy_list {
    pub class_name: LPTSTR,
    pub class_hndl: OB_CLASS_HNDL,
    pub parent_loc: UINT
}
pub type OB_HIERARCHY_LIST = ob_hierarchy_list;
pub type POB_HIERARCHY_LIST = *mut ob_hierarchy_list;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_field_filter {
    OB_ANY_FIELDS = 0,
    OB_INSTANCE_FIELDS_ONLY = 1,
    OB_TYPEDEF_FIELDS_ONLY = 2
}
pub use self::ob_field_filter as OB_FIELD_FILTER;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_appl_report {
    pub lpszLibraryName: LPTSTR,
    pub lpszName: LPTSTR,
    pub pList: *mut ::std::os::raw::c_void,
    pub iType: UINT,
    pub bIsInstanced: BOOL
}
pub type OB_APPL_REPORT = ob_appl_report;
pub type POB_APPL_REPORT = *mut ob_appl_report;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_proto_overload_error {
    OB_NO_OVERLOAD_ERROR = 0,
    OB_ARG_TYPE_ERROR = 1,
    OB_RETURN_TYPE_ERROR = 2
}
pub use self::ob_proto_overload_error as OB_PROTO_OVERLOAD_ERROR;
pub type POB_PROTO_OVERLOAD_ERROR = *mut ob_proto_overload_error;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_lib_include_type {
    EXCLUDE_ALL = 0,
    INCLUDE_REFERENCED = 1,
    INCLUDE_REFERENCED_AND_DWS = 2,
    INCLUDE_INDEPENDENT_OBJECTS = 3,
    INCLUDE_ALL = 4
}
pub use self::ob_lib_include_type as OB_LIB_INCLUDE_TYPE;
pub type POB_LIB_INCLUDE_TYPE = *mut ob_lib_include_type;
pub type PBD_ARRAY = *mut LPTSTR;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_compile_list_type {
    OB_INCREMENTAL_LIST = 0,
    OB_FULL_LIST = 1,
    OB_MIGRATION_LIST = 2
}
pub use self::ob_compile_list_type as OB_COMPILE_LIST_TYPE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_inconsistency_type {
    OB_NO_INCONSISTENCY = 0,
    OB_INCONSISTENT_VERSION = 1,
    OB_INCONSISTENT_COMPILE = 2
}
pub use self::ob_inconsistency_type as OB_INCONSISTENCY_TYPE;
pub type POB_INCONSISTENCY_TYPE = *mut ob_inconsistency_type;
pub type OS_CALLC_FUNC = ::std::option::Option<unsafe extern "C" fn()>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cmdb_main {
    _unused: [u8; 0]
}
pub type PCMDB_MAIN = *mut cmdb_main;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CM_COMPILE_TYPE {
    CM_COMPILE_TYPEDEFS_ONLY = 0,
    CM_COMPILE_SCRIPTS_ONLY = 1,
    CM_COMPILE_ALL = 2
}
pub type CM_DBSIGNON_PROC = ::std::option::Option<
    unsafe extern "C" fn(pbThis: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct cm_this {
    pub dbgthis: *mut SH_DBG_THIS,
    pub obthis: POB_THIS,
    pub grthis: *mut ::std::os::raw::c_void,
    pub stgthis: ppbstg_anchor,
    pub code_gen: BOOL,
    pub must_declare: BOOL,
    pub pErrorList: *mut shlist,
    pub bGotCompileError: BOOL,
    pub pDBSignonProc: CM_DBSIGNON_PROC,
    pub pDBCompile: PCMDB_MAIN,
    pub curr_group: *mut ::std::os::raw::c_void,
    pub curr_class_id: OB_CLASS_ID,
    pub init_compile: BOOL,
    pub bInGroupCompile: BOOL,
    pub in_forward_ref: BOOL,
    pub curr_mod_id: OB_MODULE_ID,
    pub temp_subpool: pbstg_subpool,
    pub perm_subpool: pbstg_subpool,
    pub in_append_block: BOOL,
    pub curr_rout_type: OB_CLASS_ID,
    pub return_in_script: BOOL,
    pub curr_rout_name: LPTSTR,
    pub in_fwdproto_decl: BOOL,
    pub pDBSignoffProc: CM_DBSIGNON_PROC,
    pub in_proto: BOOL,
    pub compile_type: CM_COMPILE_TYPE,
    pub expr_context_stack: PSH_GROWBLOCK,
    pub expr_context_pos: UINT,
    pub subscript_state: INT,
    pub no_subscripts: INT,
    pub array_item_pos: UINT,
    pub expr_type_stack: PSH_GROWBLOCK,
    pub expr_type_pos: UINT,
    pub in_rvalue_expr: BOOL,
    pub rout_name_stack: *mut ::std::os::raw::c_void,
    pub rout_name_stack_size: UINT,
    pub rout_name_pos: UINT,
    pub func_arg_list: PSH_GROWBLOCK,
    pub functmplt_arg_list: PSH_GROWBLOCK,
    pub functmplt_list: PSH_GROWBLOCK,
    pub func_throws_list: PSH_GROWBLOCK,
    pub curr_class_qualifier: LPTSTR,
    pub curr_on_class: OB_CLASS_ID,
    pub func_qualifier: INT,
    pub curr_typename: LPTSTR,
    pub curr_type_set: BOOL,
    pub enum_state: INT,
    pub curr_type_qualifier: INT,
    pub curr_sec_class_qualifier: LPTSTR,
    pub curr_var_global: BOOL,
    pub curr_var_external: BOOL,
    pub curr_var_block: INT,
    pub source_type: OB_SOURCE_BLK_TYPE,
    pub pbthis: *mut ::std::os::raw::c_void,
    pub curr_pcode: *mut ::std::os::raw::c_void,
    pub pcode_stack: PSH_GROWBLOCK,
    pub pcode_stack_pos: UINT,
    pub idents_have_dashes: BOOL,
    pub indir_attr_list: PSH_GROWBLOCK,
    pub indir_attr_pos: UINT,
    pub func_arginfo_list: PSH_GROWBLOCK,
    pub func_exprinfo_list: PSH_GROWBLOCK,
    pub func_arg_pos: UINT,
    pub end_lhs_assign_expr: BOOL,
    pub processing_indir_attrs: BOOL,
    pub arraylist_count: UINT,
    pub xform_table: *mut shhash,
    pub fStateFlags: ULONG,
    pub expr_pcode_start: UINT,
    pub pCurrPrototype: *mut ::std::os::raw::c_void,
    pub pcode_lines_on: BOOL,
    pub curr_eventtoken: OB_EVT_TOKEN_ID,
    pub obRoutType: OB_ROUT_TYPE,
    pub processing_indir_func: BOOL,
    pub pAliasList: *mut shhash,
    pub pszLastPBLName: LPTSTR,
    pub sql_free_refpak: BOOL,
    pub bShowWarnings: BOOL,
    pub inGroupScope: BOOL,
    pub inEvalContext: BOOL,
    pub curr_ext_class_id: OB_CLASS_ID,
    pub protoProperties: *mut ::std::os::raw::c_void,
    pub AncestorReturnValueId: OB_SYM_ID,
    pub isFirstPass: BOOL,
    pub isSecondPass: BOOL,
    pub bHyperlinkErrors: BOOL,
    pub pReplacedFuncList: *mut shlist,
    pub lPBExtensionName: OB_CONST_REF,
    pub log_file: HFILE,
    pub namespace_decl: LPTSTR,
    pub using_directive_list: *mut shlist,
    pub global_preprocessor_symbols: *mut shlist,
    pub ppcsthis: *mut ::std::os::raw::c_void,
    pub ppcs_debug_symbol_used: BOOL
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ot_lvalue_info {
    pub group_hndl: OB_GROUP_HNDL
}
pub type POT_LVALUE_INFO = *mut ot_lvalue_info;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OT_REFPAK_STYLE {
    OT_SIMPLE_REF = 0,
    OT_FIELD_REF = 1,
    OT_FIELD_ITEM_REF = 2
}
pub type OT_FIELDUPDATE_FUNC = ::std::option::Option<
    unsafe extern "C" fn(rtthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, index: ULONG) -> INT
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ot_ref_pak_simple_ref_tag {
    pub lvalue: POB_DATA
}
pub type ot_ref_pak_simple_ref = ot_ref_pak_simple_ref_tag;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ot_ref_pak_field_ref_tag {
    pub obinst: OB_INST_ID,
    pub field_id: UINT,
    pub field_update_func: OT_FIELDUPDATE_FUNC,
    pub item_index: ULONG
}
pub type ot_ref_pak_field_ref = ot_ref_pak_field_ref_tag;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ot_ref_tag_union {
    pub simple: ot_ref_pak_simple_ref,
    pub field: ot_ref_pak_field_ref
}
pub type OT_REF_TAG_UNION = ot_ref_tag_union;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ot_ref_pak {
    pub style: OT_REFPAK_STYLE,
    pub group_hndl: OB_GROUP_HNDL,
    pub type_: OB_CLASS_ID,
    pub flags: USHORT,
    pub ref_: OT_REF_TAG_UNION
}
pub type POT_REF_PAK = *mut ot_ref_pak;
pub type TIME_T = time_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OB_REQUEST {
    pub _address: u8
}
pub type POB_REQUEST = *mut OB_REQUEST;
pub type PPOB_REQUEST = *mut POB_REQUEST;
pub type POB_OBJECT = *mut OB_OBJECT;
pub type PPOB_OBJECT = *mut POB_OBJECT;
pub type POB_RUNTIME_CLASS = *mut OB_RUNTIME_CLASS;
#[repr(C, packed)]
pub struct OB_ISESSION {
    pub _base: IUnknown
}
pub type POB_ISESSION = *mut OB_ISESSION;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct OB_IREMOTE_REFERENCE {
    pub _base: IUnknown
}
pub type POB_IREMOTE_REFERENCE = *mut OB_IREMOTE_REFERENCE;
pub type PPOB_IREMOTE_REFERENCE = *mut POB_IREMOTE_REFERENCE;
#[repr(C, packed)]
pub struct OB_IREMOTE_SESSION {
    pub _base: OB_ISESSION
}
pub type POB_IREMOTE_SESSION = *mut OB_IREMOTE_SESSION;
pub type PPOB_IREMOTE_SESSION = *mut POB_IREMOTE_SESSION;
pub type POB_ILOCAL_SESSION = *mut OB_ILOCAL_SESSION;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_SESSION_STATE {
    OB_SESSION_ACTIVE = 0,
    OB_SESSION_SHUTTING_DOWN = 1,
    OB_SESSION_SHUTDOWN = 2
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_REMREF_TYPE {
    PB_TYPE = 0,
    JAG_TYPE = 1
}
#[repr(C, packed)]
pub struct OB_ILOCAL_SESSION {
    pub _base: OB_ISESSION
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct VTAB_INFO {
    pub vtab_index: USHORT,
    pub function: *mut ::std::os::raw::c_void
}
pub type PVTAB_INFO = *mut VTAB_INFO;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct VTAB_CLASS_INFO {
    pub classId: OB_CLASS_ID,
    pub numFuncs: USHORT,
    pub numEvents: USHORT,
    pub funcTableOffset: ::std::os::raw::c_long,
    pub eventTableOffset: ::std::os::raw::c_long,
    pub classVtableThunked: ::std::os::raw::c_long
}
pub type PVTAB_CLASS_INFO = *mut VTAB_CLASS_INFO;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct VTAB_GROUP_INFO {
    pub routineInfoTable: PVTAB_INFO,
    pub classInfoTable: PVTAB_CLASS_INFO
}
pub type PVTAB_GROUP_INFO = *mut VTAB_GROUP_INFO;
pub type PPVTAB_GROUP_INFO = *mut PVTAB_GROUP_INFO;
pub type OB_PVTAB_FUNC =
    ::std::option::Option<unsafe extern "C" fn(arg1: POB_THIS, arg2: OB_GROUP_ID, arg3: PPVTAB_GROUP_INFO)>;
pub type OB_EVENT_FUNC = ::std::option::Option<unsafe extern "C" fn(arg1: POB_THIS, arg2: UINT) -> INT>;
pub type OB_FUNC_FUNC = ::std::option::Option<unsafe extern "C" fn(arg1: POB_THIS, arg2: UINT) -> INT>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_runtime_vtable {
    pub func_ptr: OS_CALLC_FUNC
}
pub type OB_RUNTIME_VTABLE = ob_runtime_vtable;
pub type POB_RUNTIME_VTABLE = *mut ob_runtime_vtable;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_runtime_shared {
    pub dummy: INT
}
pub type OB_RUNTIME_SHARED = ob_runtime_shared;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_global_refs {
    pub dummy: INT
}
pub type OB_GLOBAL_REFS = ob_global_refs;
pub type PPOB_RUNTIME_CLASS = *mut POB_RUNTIME_CLASS;
pub type POB_PROTOTYPE = *mut ob_prototype;
pub type POB_CLASS_ENTRY = *mut ob_class_entry;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOB_PsppClass {
    _unused: [u8; 0]
}
#[repr(C, packed)]
pub struct OB_RUNTIME_CLASS {
    pub d_group: POB_GROUP,
    pub d_class_id: OB_CLASS_ID,
    pub d_inherit_level: USHORT,
    pub d_function_vtable: POB_RUNTIME_VTABLE,
    pub d_ancestor: POB_RUNTIME_CLASS,
    pub d_lpszClassName: LPTSTR,
    pub d_no_fields: ULONG,
    pub d_instance_image: POB_DATA,
    pub d_instance_image_ts: TIME_T,
    pub d_pspp_class: *mut IOB_PsppClass
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_MEMBER_ACCESS_TYPE {
    IGNORE_ACCESS_CHECK = 0,
    LOCAL_CLASS_ACCESS_CHECK = 1,
    ANC_CLASS_ACCESS_CHECK = 2,
    FOREIGN_CLASS_ACCESS_CHECK = 3
}
pub type POB_MEMBER_ACCESS_TYPE = *mut OB_MEMBER_ACCESS_TYPE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tag_OB_EVENT_LOOKUP_ITEM {
    pub token: OB_EVT_TOKEN_ID,
    pub vtable_id: OB_VTABLE_ID
}
pub type OB_EVENT_LOOKUP_ITEM = tag_OB_EVENT_LOOKUP_ITEM;
pub type POB_EVENT_LOOKUP_ITEM = *mut tag_OB_EVENT_LOOKUP_ITEM;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_FUNCPROTO_STYLE {
    OB_SYS_PROTOTYPE = 0,
    OB_USER_PROTOTYPE = 1,
    OB_FWD_PROTOTYPE = 2
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_perm_prototype {
    pub name: OB_CONST_REF,
    pub signature: OB_CONST_REF,
    pub args: OB_CONST_REF,
    pub aliasname: OB_CONST_REF,
    pub dllname: OB_CONST_REF,
    pub vtable_id: OB_VTABLE_ID,
    pub mod_id: OB_MODULE_ID,
    pub rout_id: OB_ROUT_ID,
    pub sys_func_id: OB_VTABLE_ID,
    pub type_: OB_CLASS_ID,
    pub info: USHORT,
    pub token: OB_EVT_TOKEN_ID,
    pub info2: USHORT,
    pub descriptor: OB_CONST_REF,
    pub help_id: ULONG,
    pub throws_ref: OB_CONST_REF
}
pub type OB_PERM_PROTOTYPE = ob_perm_prototype;
pub type POB_PERM_PROTOTYPE = *mut ob_perm_prototype;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_prototype {
    pub perm_entry: POB_PERM_PROTOTYPE,
    pub func_ptr: OS_CALLC_FUNC
}
pub type OB_PROTOTYPE = ob_prototype;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_FUNCTMPLT_ARG_TYPE {
    OB_FUNCTMPLT_VAR_ARG = 0,
    OB_FUNCTMPLT_NAME_ARG = 1,
    OB_FUNCTMPLT_ARGS_ARG = 2,
    OB_FUNCTMPLT_NARGS_ARG = 3,
    OB_FUNCTMPLT_VAL_ARG = 4,
    OB_FUNCTMPLT_EOSEQ_ARG = 5,
    OB_FUNCTMPLT_DIMS_ARG = 6
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_INDATTR_FUNC_TYPE {
    OB_INDATTR_GET_FUNC = 0,
    OB_INDATTR_SET_FUNC = 1,
    OB_INDATTR_GETITEM_FUNC = 2,
    OB_INDATTR_SETITEM_FUNC = 3,
    OB_INDATTR_INVMETHOD_FUNC = 4,
    OB_INDATTR_UPPERBOUND_FUNC = 5,
    OB_INDATTR_LOWERBOUND_FUNC = 6,
    OB_INDATTR_UNDEFINED_FUNC = 7
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_functmplt_argument {
    pub argname: OB_CONST_REF,
    pub argtype: USHORT,
    pub padding: SHORT
}
pub type OB_FUNCTMPLT_ARGUMENT = ob_functmplt_argument;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_indattr_functmplt {
    pub name: OB_CONST_REF,
    pub args: OB_CONST_REF,
    pub no_args: USHORT,
    pub func_type: USHORT,
    pub isDynamic: USHORT,
    pub padding: USHORT
}
pub type OB_INDATTR_FUNCTMPLT = ob_indattr_functmplt;
pub type POB_INDATTR_FUNCTMPLT = *mut ob_indattr_functmplt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_enumfield {
    pub name: OB_CONST_REF,
    pub val: SHORT,
    pub padding: SHORT
}
pub type OB_ENUMFIELD = ob_enumfield;
pub type POB_ENUMFIELD = *mut ob_enumfield;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_perm_virtual_node {
    pub vtable_id: OB_VTABLE_ID,
    pub proto_id: OB_PROTO_ID,
    pub class_id: OB_CLASS_ID
}
pub type OB_PERM_VIRTUAL_NODE = ob_perm_virtual_node;
pub type POB_PERM_VIRTUAL_NODE = *mut ob_perm_virtual_node;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_virtual_node {
    pub rout_id: OB_ROUT_ID,
    pub proto_id: OB_PROTO_ID,
    pub group_id: OB_GROUP_ID,
    pub class_id: OB_CLASS_ID
}
pub type OB_VIRTUAL_NODE = ob_virtual_node;
pub type POB_VIRTUAL_NODE = *mut ob_virtual_node;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OB_CLASS_STYLE {
    TYPE_CLASS = 0,
    TYPE_ENUM = 1,
    TYPE_INIT_SOURCE = 2,
    TYPE_INDIRECT = 3,
    TYPE_VAR_BLOCK = 4,
    TYPE_INHERITED = 5
}
pub type POB_CLASS_STYLE = *mut OB_CLASS_STYLE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_proto_arg {
    pub name: OB_CONST_REF,
    pub arrdef: OB_CONST_REF,
    pub datatype: OB_CLASS_ID,
    pub info: USHORT
}
pub type OB_PROTO_ARG = ob_proto_arg;
pub type POB_PROTO_ARG = *mut ob_proto_arg;
pub type OB_MOD_SYMTAB = OB_LOOK_SYMTAB;
pub type OB_ENUM_ID = UINT;
pub type OB_ENUM_SYMTAB = OB_LOOK_SYMTAB;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_class_def {
    pub parent_class: OB_CLASS_ID,
    pub nested_class: OB_CLASS_ID,
    pub no_protos: USHORT,
    pub no_func_protos: USHORT,
    pub no_event_protos: USHORT,
    pub no_local_event_protos: USHORT,
    pub curr_event: USHORT,
    pub curr_func: USHORT,
    pub no_vtab_lookup: USHORT,
    pub no_vtab_funcs: USHORT,
    pub no_vtab_events: USHORT,
    pub no_event_lookup: USHORT,
    pub no_perm_vtab_lookup: USHORT,
    pub no_indattr_functmplt: USHORT,
    pub no_instance_image: USHORT,
    pub no_system_fields: USHORT
}
pub type OB_CLASS_DEF = ob_class_def;
pub type POB_CLASS_DEF = *mut ob_class_def;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_perm_class_entry {
    pub info: USHORT,
    pub class_id: OB_CLASS_ID,
    pub __bindgen_anon_1: ob_perm_class_entry__bindgen_ty_1,
    pub descriptor: OB_CONST_REF,
    pub help_id: ULONG
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_perm_class_entry_struct_inh {
    pub sec_class: OB_CLASS_ID,
    pub sec_group_class: OB_CLASS_ID
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union ob_perm_class_entry__bindgen_ty_1 {
    pub no_enums: USHORT,
    pub def_slot: USHORT,
    pub inh: ob_perm_class_entry_struct_inh
}
pub type OB_PERM_CLASS_ENTRY = ob_perm_class_entry;
pub type POB_PERM_CLASS_ENTRY = *mut ob_perm_class_entry;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_class_entry {
    pub source: LPTSTR,
    pub len: UINT,
    pub perm_entry: POB_PERM_CLASS_ENTRY,
    pub def_entry: POB_CLASS_DEF,
    pub field_symtab: OB_LOOK_SYMTAB,
    pub perm_prototypes: POB_PERM_PROTOTYPE,
    pub prototypes: POB_PROTOTYPE,
    pub mod_symtab: OB_MOD_SYMTAB,
    pub routine_list: POB_ROUTLIST,
    pub rout_vtable: POB_VIRTUAL_NODE,
    pub event_lookup: POB_EVENT_LOOKUP_ITEM,
    pub perm_rout_vtable: POB_PERM_VIRTUAL_NODE,
    pub indattr_functmplt: POB_INDATTR_FUNCTMPLT,
    pub instance_image: POB_DATA,
    pub instvar_source: LPTSTR,
    pub instvar_source_len: UINT,
    pub prototype_source: LPTSTR,
    pub prototype_source_len: UINT,
    pub fwdproto_source: LPTSTR,
    pub fwdproto_source_len: UINT,
    pub field_sum: INT,
    pub cached_event_token1: OB_EVT_TOKEN_ID,
    pub cached_event_no1: UINT,
    pub cached_event_token2: OB_EVT_TOKEN_ID,
    pub cached_event_no2: UINT,
    pub cached_event_token3: OB_EVT_TOKEN_ID,
    pub cached_event_no3: UINT,
    pub enumlist: POB_ENUMFIELD,
    pub runtime_class: POB_RUNTIME_CLASS,
    pub propertyVariables: *mut ::std::os::raw::c_void,
    pub numPropertyVariables: ::std::os::raw::c_long,
    pub scriptList: *mut ::std::os::raw::c_void,
    pub numScripts: ::std::os::raw::c_long
}
pub type OB_CLASS_ENTRY = ob_class_entry;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct perm_type_descript {
    pub no_slots: USHORT,
    pub no_def_slots: USHORT
}
pub type OB_PERM_TYPE_DESCRIPT = perm_type_descript;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct temp_type_descript {
    pub perm_table: POB_PERM_CLASS_ENTRY,
    pub table: POB_CLASS_ENTRY,
    pub def_table: POB_CLASS_DEF,
    pub perm_alloc_size: UINT,
    pub perm_alloc_incr: UINT,
    pub alloc_size: UINT,
    pub alloc_incr: UINT,
    pub def_alloc_size: UINT,
    pub def_alloc_incr: UINT,
    pub slot_incr: UINT,
    pub next_free: UINT,
    pub next_free_def: UINT
}
pub type OB_TEMP_TYPE_DESCRIPT = temp_type_descript;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ob_type_descript {
    pub ps: OB_PERM_TYPE_DESCRIPT,
    pub ts: OB_TEMP_TYPE_DESCRIPT
}
pub type OB_TYPE_DESCRIPT = ob_type_descript;
pub type POB_TYPE_DESCRIPT = *mut ob_type_descript;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_typedef {
    pub type_symtab: OB_LOOK_SYMTAB,
    pub enum_symtab: OB_ENUM_SYMTAB,
    pub conpool: POB_CONPOOL,
    pub descript: POB_TYPE_DESCRIPT,
    pub arg_conpool: POB_CONPOOL
}
pub type OB_TYPEDEF = ob_typedef;
pub type POB_TYPEDEF = *mut ob_typedef;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_typeinfo {
    pub name: LPTSTR,
    pub data: OB_DATA,
    pub source_group_id: OB_GROUP_ID,
    pub group_id: OB_GROUP_ID,
    pub class_id: OB_CLASS_ID,
    pub arrdef: POB_ARRAYDEF,
    pub scope: OB_MEMBER_ACCESS,
    pub read_access: OB_MEMBER_ACCESS,
    pub write_access: OB_MEMBER_ACCESS,
    pub flags: UINT,
    pub set_func: LPTSTR,
    pub get_func: LPTSTR,
    pub array_set_func: LPTSTR,
    pub array_get_func: LPTSTR,
    pub array_upper_func: LPTSTR
}
pub type OB_TYPEINFO = ob_typeinfo;
pub type POB_TYPEINFO = *mut ob_typeinfo;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_act_arg {
    pub datatype: OB_CLASS_ID,
    pub group_id: OB_GROUP_ID,
    pub info: USHORT,
    pub num_dims: ULONG,
    pub dimensions: *mut ::std::os::raw::c_long
}
pub type OB_ACT_ARG = ob_act_arg;
pub type POB_ACT_ARG = *mut ob_act_arg;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_protoname {
    pub group_id: OB_GROUP_ID,
    pub class_id: OB_CLASS_ID,
    pub proto_id: OB_PROTO_ID,
    pub vtable_id: OB_VTABLE_ID,
    pub rout_type: OB_ROUT_TYPE,
    pub protoname: LPTSTR,
    pub classname: LPTSTR,
    pub is_a_dllfunc: BOOL,
    pub is_a_dbrpc: BOOL,
    pub token: OB_EVT_TOKEN_ID,
    pub type_: OB_CLASS_ID,
    pub args: POB_PROTO_ARG,
    pub no_args: UINT
}
pub type OB_PROTONAME = ob_protoname;
pub type POB_PROTONAME = *mut ob_protoname;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_class_nameid {
    pub name: OB_CONST_REF,
    pub class_id: OB_CLASS_ID,
    pub padding: SHORT
}
pub type OB_CLASS_NAMEID = ob_class_nameid;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_rout_nameid {
    pub id: USHORT,
    pub class_id: OB_CLASS_ID,
    pub name: OB_CONST_REF
}
pub type OB_ROUT_NAMEID = ob_rout_nameid;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_field_nameid {
    pub name: OB_CONST_REF,
    pub id: USHORT,
    pub type_: OB_CLASS_ID
}
pub type OB_FIELD_NAMEID = ob_field_nameid;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MONTHANDDAYNAMESSTRUCT_TAG {
    pub monAbbrev: *mut LPTSTR,
    pub monName: *mut LPTSTR,
    pub dayAbbrev: *mut LPTSTR,
    pub dayName: *mut LPTSTR
}
pub type LPMONTHANDDAYNAMESSTRUCT = *mut MONTHANDDAYNAMESSTRUCT_TAG;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_glob_symtype {
    OB_GLOB_VAR = 0,
    OB_GLOB_CLASS = 1,
    OB_GLOB_FUNC = 2,
    OB_GLOB_ANY_SYMTYPE = 3
}
pub use self::ob_glob_symtype as OB_GLOB_SYMTYPE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_glob_reftype {
    OB_GLOB_REF = 0,
    OB_GLOB_DECL = 1
}
pub use self::ob_glob_reftype as OB_GLOB_REFTYPE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_globsym_entry {
    pub name: OB_CONST_REF,
    pub group_id: OB_GROUP_ID,
    pub class_id: OB_CLASS_ID,
    pub id: OB_SYM_ID,
    pub info: USHORT
}
pub type OB_GLOBSYM_ENTRY = ob_globsym_entry;
pub type POB_GLOBSYM_ENTRY = *mut ob_globsym_entry;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_old_globsym_entry {
    pub name: USHORT,
    pub group_id: OB_GROUP_ID,
    pub class_id: OB_CLASS_ID,
    pub id: OB_SYM_ID,
    pub info: USHORT
}
pub type OB_OLD_GLOBSYM_ENTRY = ob_old_globsym_entry;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_perm_globsym {
    pub no_slots: USHORT
}
pub type OB_PERM_GLOBSYM = ob_perm_globsym;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_temp_globsym {
    pub table: POB_GLOBSYM_ENTRY,
    pub alloc_incr: UINT,
    pub alloc_size: UINT,
    pub slot_incr: UINT,
    pub next_free: UINT
}
pub type OB_TEMP_GLOBSYM = ob_temp_globsym;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ob_globsym {
    pub ps: OB_PERM_GLOBSYM,
    pub ts: OB_TEMP_GLOBSYM
}
pub type OB_GLOBSYM = ob_globsym;
pub type POB_GLOBSYM = *mut ob_globsym;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct rt_breakpt {
    pub obClassHndl: OB_CLASS_HNDL,
    pub obRoutineID: OB_VTABLE_ID,
    pub iLineNumber: UINT,
    pub boolean_expr: LPTSTR,
    pub n_times: UINT,
    pub nth_time: UINT,
    pub watchPoint: *mut ::std::os::raw::c_void,
    pub id: ::std::os::raw::c_long,
    pub obthis: POB_THIS
}
pub type PRT_BREAKPOINT = *mut rt_breakpt;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RT_EXEC_STATUS {
    RT_EXEC_SUCCESS = 0,
    RT_EXEC_NO_SCRIPT = 1,
    RT_EXEC_FAILURE = 2,
    RT_EXEC_BADTOKEN = 3,
    RT_EXEC_NO_MATCH = 4
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WATCHPOINT_TYPE {
    LOCAL_WATCH = 0,
    GLOBAL_WATCH = 1,
    SHARED_WATCH = 2,
    INSTANCE_WATCH = 3
}
#[doc = "In order not to include an extreme number of PB header files in ocx\nthe rt_error_struct structure which is defined here\nis redefined in pbrxctl.h\n\nIT MUST BE KEPT IN SYNC WITH THE VERSION WITHIN PBRXCTL.H\n"]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct rt_error_struct {
    pub rtthis: POB_THIS,
    pub message: LPTSTR,
    pub error_no: SHORT,
    pub line_no: USHORT,
    pub group_name: LPTSTR,
    pub class_name: LPTSTR,
    pub rout_name: LPTSTR
}
#[doc = "In order not to include an extreme number of PB header files in ocx\nthe rt_error_struct structure which is defined here\nis redefined in pbrxctl.h\n\nIT MUST BE KEPT IN SYNC WITH THE VERSION WITHIN PBRXCTL.H\n"]
pub type PRT_ERROR_STRUCT = *mut rt_error_struct;
pub type OB_ERROR_FUNC = ::std::option::Option<
    unsafe extern "C" fn(
        pErrorStruct: PRT_ERROR_STRUCT,
        bDisplayErr: *mut BOOL,
        RuntimeState: *mut ::std::os::raw::c_long
    )
>;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RT_CALL_TYPE {
    RT_INST_CALL = 0,
    RT_CLASS_CALL = 1,
    RT_CLASS_QUALIFIED_CALL = 2
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct rtClassInfo_tag {
    pub obClassHndl: OB_CLASS_HNDL,
    pub obInst: OB_INST_ID
}
pub type RT_CALL_TYPE_INFO = rtClassInfo_tag;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct rtCallInfo {
    pub rtClassInfo: RT_CALL_TYPE_INFO,
    pub enCallType: RT_CALL_TYPE,
    pub bDontTerminateRuntime: BOOL
}
pub type RT_CALL_INFO = rtCallInfo;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rtRoutineProtoInfo {
    pub obGroupId: OB_GROUP_ID,
    pub pchRoutineName: LPTSTR,
    pub pchAliasName: LPTSTR,
    pub pchDllName: LPTSTR,
    pub obdReturnType: OB_DATA,
    pub iNoArgs: INT,
    pub obRoutineType: OB_ROUT_TYPE,
    pub obMemberAccess: OB_MEMBER_ACCESS,
    pub pobdArgArray: POB_DATA,
    pub ppchArgNameArray: *mut LPTSTR,
    pub bVarArgs: BOOL,
    pub ppArrayDefs: *mut ::std::os::raw::c_void,
    pub pchSystemEventName: LPTSTR
}
pub type PRT_ROUTINE_PROTO_INFO = *mut rtRoutineProtoInfo;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RT_REFARG_TYPE {
    RT_SIMPLE = 0,
    RT_NOTIFY = 1
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct rtRefArgInfo {
    pub rtRefType: RT_REFARG_TYPE,
    pub pobdRefData: POB_DATA
}
pub type PRT_REFARG_INFO = *mut rtRefArgInfo;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct rtClassDescrip {
    pub pchClassName: LPTSTR,
    pub bIsStruct: BOOL,
    pub bIsGlobalStruct: BOOL
}
pub type PRT_CLASS_DESCRIP = *mut rtClassDescrip;
pub type POB_RUNTIME_INST = *mut OB_OBJECT;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_object_reference {
    pub d_backReference: PPOB_OBJECT,
    pub d_bIsExternal: BOOL
}
pub type OB_OBJECT_REFERENCE = ob_object_reference;
#[repr(C, packed)]
pub struct OB_OBJECT_REF_ARRAY {
    pub block: *mut OB_OBJECT_REFERENCE,
    pub numEntries: ::std::os::raw::c_long,
    pub numAlloced: ::std::os::raw::c_long,
    pub growBy: ::std::os::raw::c_long,
    pub anchor: ppbstg_anchor
}
#[repr(C, packed)]
pub struct OB_OBJECT {
    pub _base: IUnknown,
    pub d_reflist: OB_OBJECT_REF_ARRAY,
    pub d_refCount: UINT,
    pub d_externalRefCount: UINT,
    pub d_underlying_obj: *mut ::std::os::raw::c_void
}
pub type POB_GROUP_PROPERTIES = *mut ::std::os::raw::c_void;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_group_comp_state {
    OB_NEVER_COMPILED = 0,
    OB_TYPEDEFS_COMPILED = 1,
    OB_ALL_COMPILED = 2
}
pub use self::ob_group_comp_state as OB_GROUP_COMP_STATE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_group_header {
    pub version: ::std::os::raw::c_long,
    pub primary_system_class: OB_CLASS_ID,
    pub reserved: SHORT
}
pub type OB_GROUP_HEADER = ob_group_header;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_perm_group {
    pub name: OB_CONST_REF,
    pub modify_time: TIME_T,
    pub compile_time: TIME_T,
    pub info: USHORT,
    pub padding: SHORT
}
pub type OB_PERM_GROUP = ob_perm_group;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOB_PsppDll {
    _unused: [u8; 0]
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_temp_group {
    pub id: OB_GROUP_HNDL,
    pub shared_symtab: OB_LOOK_SYMTAB,
    pub conpool: POB_CONPOOL,
    pub typdef: POB_TYPEDEF,
    pub globsym: POB_GLOBSYM,
    pub subpool: OB_SUBPOOL,
    pub version: ::std::os::raw::c_long,
    pub copy_sym_table: BOOL,
    pub is_dirty: BOOL,
    pub primary_system_class: OB_CLASS_ID,
    pub group_properties: POB_GROUP_PROPERTIES,
    pub psppdll: *mut IOB_PsppDll,
    pub updated_preprocessor_symbols: BOOL,
    pub preprocessor_symbols: *mut shlist
}
pub type OB_TEMP_GROUP = ob_temp_group;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_group {
    pub ps: OB_PERM_GROUP,
    pub ts: OB_TEMP_GROUP,
    pub lpszGroupName: LPTSTR
}
pub type OB_GROUP = ob_group;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_group_binary_format {
    OB_GROUP_BINARY_FORMAT_ANSI = 0,
    OB_GROUP_BINARY_FORMAT_UNICODE = 1
}
pub use self::ob_group_binary_format as OB_GROUP_BINARY_FORMAT;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_old_perm_group {
    pub name: USHORT,
    pub modify_time: TIME_T,
    pub compile_time: TIME_T,
    pub info: USHORT
}
pub type OB_OLD_PERM_GROUP = ob_old_perm_group;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_old_perm_group_chg {
    pub name: OB_CONST_REF,
    pub modify_time: TIME_T,
    pub compile_time: TIME_T,
    pub info: USHORT
}
pub type OB_OLD_PERM_GROUP_CHG = ob_old_perm_group_chg;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_group_lock_state {
    OB_READLOCKED_GROUP = 0,
    OB_SYSTEM_GROUP = 1,
    OB_UNLOCKED_GROUP = 2,
    OB_WRITELOCKED_GROUP = 3
}
pub use self::ob_group_lock_state as OB_GROUP_LOCK_STATE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_group_load_state {
    OB_GROUP_NOT_LOADED = 0,
    OB_GROUP_GLOBSYM_LOADED = 1,
    OB_GROUP_TYPEDEFS_LOADED = 2,
    OB_GROUP_ALL_LOADED = 3
}
pub use self::ob_group_load_state as OB_GROUP_LOAD_STATE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_group_link_state {
    OB_NEVER_LINKED = 0,
    OB_LINKED = 1
}
pub use self::ob_group_link_state as OB_GROUP_LINK_STATE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_inst_list_node {
    pub rtinst: POB_RUNTIME_INST
}
pub type OB_INSTLIST_NODE = ob_inst_list_node;
pub type POB_INSTLIST_NODE = *mut ob_inst_list_node;
pub type OB_INST_LIST = SH_GROWBLOCK;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OB_GroupReference {
    _unused: [u8; 0]
}
pub type POB_GroupReference = *mut OB_GroupReference;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_grouplist {
    pub lpszGroupName: LPTSTR,
    pub m_GroupRef: POB_GroupReference,
    pub pLibListNode: *mut ::std::os::raw::c_void,
    pub modify_time: TIME_T,
    pub link_state: OB_GROUP_LINK_STATE,
    pub obGroupLockState: OB_GROUP_LOCK_STATE,
    pub hasSource: BOOL,
    pub iReadLockCount: UINT,
    pub isValid: BOOL,
    pub isDynamicLib: BOOL,
    pub temp_glbsym: POB_GLOBSYM_ENTRY,
    pub grp_shared_symtab: POB_LOOK_SYMTAB,
    pub inst_list: OB_INST_LIST,
    pub inst_free_list: POB_INSTLIST_NODE,
    pub no_insts: UINT,
    pub IsGroupBusy: BOOL
}
pub type OB_GROUPLIST = ob_grouplist;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_src_last_edit {
    pub pEntry: LPTSTR,
    pub usLastScript: USHORT
}
pub type OB_SRC_LAST_EDIT = ob_src_last_edit;
pub type POB_SRC_LAST_EDIT = *mut ob_src_last_edit;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union NUMERIC_VALS {
    pub double_buffer: f64,
    pub dec_buffer: SH_DEC,
    pub longlong_buffer: ::std::os::raw::c_longlong
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ot_eval_node {
    pub data: OB_DATA,
    pub v: NUMERIC_VALS,
    pub group_id: OB_GROUP_ID,
    pub lvalue_flags: SHORT
}
pub type OT_EVAL_NODE = ot_eval_node;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OT_TYPE_LOC {
    OT_IN_DATA_NODE = 0,
    OT_OUT_DATA_NODE = 1
}
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OT_TYPE_CHECK_ERROR {
    OT_TYPCHECK_SUCCESS = 0,
    OT_TYPCHECK_BAD_ARRAY_TYPES = 1,
    OT_TYPCHECK_BAD_TYPES = 2,
    OT_TYPCHECK_MIXED_GROUPING = 3,
    OT_TYPCHECK_UNDECLARED = 4
}
pub type PDBI_COMMAND = *mut DBI_Command;
pub type DBI_FUNC =
    ::std::option::Option<unsafe extern "C" fn(arg1: PDBI_COMMAND) -> *mut ::std::os::raw::c_void>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CPB_DBI_Connection {
    _unused: [u8; 0]
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct DBI_Bind {
    pub pDataLocation: *mut ::std::os::raw::c_void,
    pub pIndicatorLocation: *mut ::std::os::raw::c_void,
    pub lDataLength: ::std::os::raw::c_long,
    pub iDataType: UINT,
    pub iDBDataType: UINT,
    pub iCDataType: UINT,
    pub iDecPlaces: UINT,
    pub iLastDataTypeBound: UINT,
    pub lValueLength: ::std::os::raw::c_long,
    pub filler: [TCHAR; 32usize]
}
pub type PDBI_BIND = *mut DBI_Bind;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CacheList {
    pub pEntries: *mut shlist,
    pub cEntryLimit: ULONG,
    pub cHits: ULONG,
    pub cMisses: ULONG,
    pub filler: [TCHAR; 4usize]
}
pub type pCacheList = *mut CacheList;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DBI_TRANSEVTTYPE {
    DBI_TRANSEVT_DBNOTIF = 0,
    DBI_TRANSEVT_DBERROR = 1,
    DBI_TRANSEVT_SQLPREV = 2
}
pub type DBIPSCallback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut DBI_Command,
        arg2: *mut ::std::os::raw::c_void,
        arg3: DBI_TRANSEVTTYPE
    ) -> ::std::os::raw::c_long
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct DBI_Command {
    pub szReturnText: [TCHAR; 1024usize],
    pub iCommandType: INT,
    pub lReturnCode: ::std::os::raw::c_long,
    pub lTransactionID: ::std::os::raw::c_long,
    pub lReturnValue: ::std::os::raw::c_long,
    pub lpfnRouteProc: DBI_FUNC,
    pub hLibrary: HANDLE,
    pub szDbms: [TCHAR; 128usize],
    pub lpszLogId: LPTSTR,
    pub lpszServerName: LPTSTR,
    pub lpszLogPassWord: LPTSTR,
    pub lpszCommandBuffer: LPTSTR,
    pub lpszDatabaseName: LPTSTR,
    pub lpszPassWord: LPTSTR,
    pub lpszUserName: LPTSTR,
    pub lpszLock: LPTSTR,
    pub lpszDbParm: LPTSTR,
    pub wParam: WPARAM,
    pub lParam: ::std::os::raw::c_long,
    pub lpExtraParm1: LPTSTR,
    pub lpExtraParm2: LPTSTR,
    pub lExtraParm1: ::std::os::raw::c_long,
    pub lExtraParm2: ::std::os::raw::c_long,
    pub subpoolParm: pbstg_subpool,
    pub pDescribeColList: *mut shlist,
    pub iNumberOfVariables: INT,
    pub pBindBuffer: PDBI_BIND,
    pub lpszCursorName: LPTSTR,
    pub lpszTblOwner: LPTSTR,
    pub pFormatHash: *mut shhash,
    pub pValidHash: *mut shhash,
    pub pEditStyleHash: *mut shhash,
    pub lRuntimeCode: ::std::os::raw::c_long,
    pub stgThis: ppbstg_anchor,
    pub pExtBlock: *mut ::std::os::raw::c_void,
    pub pSyntaxList: *mut shlist,
    pub pErrorList: *mut shlist,
    pub iOperationMode: INT,
    pub bAsync: BOOL,
    pub bExportSameDB: BOOL,
    pub bNoDescribeDW: ::std::os::raw::c_uchar,
    pub bMoreErrorMsgs: ::std::os::raw::c_uchar,
    pub bPromptForAll: ::std::os::raw::c_uchar,
    pub bNoDelimitReserveWord: ::std::os::raw::c_uchar,
    pub bChildConnect: ::std::os::raw::c_uchar,
    pub bGuestConnect: ::std::os::raw::c_uchar,
    pub bNeedBLOBNRows: ::std::os::raw::c_uchar,
    pub bReadOnly: ::std::os::raw::c_uchar,
    pub bNoPBCatalog: ::std::os::raw::c_uchar,
    pub filler2: BOOL,
    pub lpszWorkData: LPTSTR,
    pub lpszDataSource: LPTSTR,
    pub dwFlags: DWORD,
    pub subpool: pbstg_subpool,
    pub pSelectCache: pCacheList,
    pub pParentDB: *mut DBI_Command,
    pub pDatatypeList: *mut shlist,
    pub bDescribeSpecial: ::std::os::raw::c_uchar,
    pub bUseQualifier: ::std::os::raw::c_uchar,
    pub bNoSyntaxLog: ::std::os::raw::c_uchar,
    pub bTableListCacheHasNonSystem: ::std::os::raw::c_uchar,
    pub bTableListCacheHasSystem: ::std::os::raw::c_uchar,
    pub bCommitOnDisconnect: ::std::os::raw::c_uchar,
    pub cQuote: TCHAR,
    pub lTableListAge: ULONG,
    pub lTableListLastRefresh: ULONG,
    pub lpszTableListFileName: LPTSTR,
    pub obThis: *mut ::std::os::raw::c_void,
    pub pPipeDB: *mut DBI_Command,
    pub pDefaultList: *mut shlist,
    pub hWinSyntax: HWND,
    pub nLogWidth: SHORT,
    pub cDecimalSeparator: TCHAR,
    pub lpszTempStrBuffer: LPTSTR,
    pub bGenEqualsNull: ::std::os::raw::c_uchar,
    pub bDontCommitAbort: BOOL,
    pub bUseContextObject: BOOL,
    pub bSupportStoredProc: BOOL,
    pub pTrace: *mut ::std::os::raw::c_void,
    pub iJagDBConnection: INT,
    pub lProvList: ::std::os::raw::c_long,
    pub bDoubleQuoted: BOOL,
    pub bInRuntime: BOOL,
    pub iType: INT,
    pub bDSI: BOOL,
    pub lWarningCode: ::std::os::raw::c_long,
    pub lpWarningMsg: LPTSTR,
    pub pDBIConnect: *mut CPB_DBI_Connection,
    pub fnPS_Callback: DBIPSCallback
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_objstk_node {
    pub rtinst: POB_RUNTIME_INST,
    pub return_value: OT_EVAL_NODE,
    pub local_vars: POB_DATA,
    pub num_vars: ULONG
}
pub type OB_OBJSTK_NODE = ob_objstk_node;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_timer_kind {
    OB_TIMER_NONE = 1,
    OB_TIMER_CLOCK = 2,
    OB_TIMER_PROCESS = 3,
    OB_TIMER_THREAD = 4
}
pub use self::ob_timer_kind as OB_TIMERKIND;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_trace_id {
    OB_TRACEID_ROUTINE = 1,
    OB_TRACEID_LINE = 2,
    OB_TRACEID_PCODE = 3,
    OB_TRACEID_ESQL = 4,
    OB_TRACEID_OBJECT_CREATE = 5,
    OB_TRACEID_OBJECT_DESTROY = 6,
    OB_TRACEID_USER = 7,
    OB_TRACEID_ERROR = 8,
    OB_TRACEID_BEGIN = 9,
    OB_TRACEID_GC = 10,
    OB_TRACEID_LAST = 11
}
pub use self::ob_trace_id as OB_TRACEID;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_trace_record_type {
    OB_TRACE_ENTRY = 1,
    OB_TRACE_EXIT = 2,
    OB_TRACE_ATOMIC = 3
}
pub use self::ob_trace_record_type as OB_TRACERECORDTYPE;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ob_error_return {
    ERR_RET_Success = 1,
    ERR_RET_TraceStartedError = 2,
    ERR_RET_TraceNotStartedError = 3,
    ERR_RET_FileCloseError = 4,
    ERR_RET_FileOpenError = 5,
    ERR_RET_FileReadError = 6,
    ERR_RET_FileWriteError = 7,
    ERR_RET_FileNotOpenError = 8,
    ERR_RET_FileAlreadyOpenError = 9,
    ERR_RET_NoMoreNodes = 10,
    ERR_RET_FileInvalidFormatError = 11,
    ERR_RET_ModelNotExistsError = 12,
    ERR_RET_ModelExistsError = 13,
    ERR_RET_GeneralError = 14,
    ERR_RET_FileNotSetError = 15,
    ERR_RET_EventNotExistError = 16,
    ERR_RET_EventWrongPrototypeError = 17,
    ERR_RET_FeatureNotSupportedError = 18,
    ERR_RET_SharedObjectNotExistsError = 19,
    ERR_RET_SharedObjectExistsError = 20,
    ERR_RET_MutexCreateError = 21,
    ERR_RET_SharedObjectCreateInstanceError = 22,
    ERR_RET_SharedObjectCreatePBSessionError = 23,
    ERR_RET_EnterpriseOnlyFeature = 24,
    ERR_RET_SourcePBLNotFound = 25
}
pub use self::ob_error_return as OB_ERROR_RETURN;
pub type OB_TRACETIME = ::std::os::raw::c_longlong;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_trace_runtime {
    pub tracefile: LPVOID,
    pub dwTraceEnable: DWORD,
    pub tskind: UINT,
    pub numActivities: ::std::os::raw::c_long,
    pub hProcess: HANDLE,
    pub hThread: HANDLE,
    pub tTotalSkew: OB_TRACETIME,
    pub tGetTimeCost: OB_TRACETIME
}
pub type OB_TRACE_RUNTIME = ob_trace_runtime;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ob_trace_header {
    pub magic: [TCHAR; 32usize],
    pub version: [TCHAR; 32usize],
    pub applicationName: [TCHAR; 64usize],
    pub platform: ULONG,
    pub group_table_offset: ::std::os::raw::c_long,
    pub library_list_offset: ::std::os::raw::c_long,
    pub library_count: ULONG,
    pub group_count: ULONG,
    pub group_max: ULONG,
    pub ticks_per_sec: OB_TRACETIME,
    pub total_skew: OB_TRACETIME,
    pub tskind: OB_TIMERKIND,
    pub numActivities: ::std::os::raw::c_long,
    pub unused1: ULONG,
    pub unused2: ULONG,
    pub unused3: ULONG,
    pub unused4: ULONG,
    pub unused5: ULONG,
    pub unused6: ULONG
}
pub type OB_TRACEHEADER = ob_trace_header;
pub struct Api {
    __library: PBLibrary,
    __version: u32,
    pbstg_begin: unsafe extern "stdcall" fn(buffer: USHORT) -> ppbstg_anchor,
    pbstg_begin_allocflags: unsafe extern "stdcall" fn(buffer: USHORT, lAllocFlags: UINT) -> ppbstg_anchor,
    pbstg_begin_nofast: unsafe extern "stdcall" fn(buffer: USHORT) -> ppbstg_anchor,
    pbstg_end: unsafe extern "stdcall" fn(pthis: ppbstg_anchor),
    pbstg_free_pool: unsafe extern "stdcall" fn(pthis: ppbstg_anchor, subPool: pbstg_subpool),
    pbstg_new_pool: unsafe extern "stdcall" fn(pthis: ppbstg_anchor) -> pbstg_subpool,
    pbstg_new_pool_nofast: unsafe extern "stdcall" fn(pthis: ppbstg_anchor) -> pbstg_subpool,
    pbstg_new_pool_with_size_nofast:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, page_size: USHORT) -> pbstg_subpool,
    pbstg_set_pool_name:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, subPool: pbstg_subpool, lpstrName: LPTSTR),
    pbstg_set_poolpagesize: unsafe extern "stdcall" fn(pthis: ppbstg_anchor, pagesize: ULONG) -> BOOL,
    pbstg_write_debug: unsafe extern "stdcall" fn(
        pthis: ppbstg_anchor,
        subpool: pbstg_subpool,
        lpFile: LPTSTR
    ) -> ::std::os::raw::c_short,
    pbstg_stat: unsafe extern "stdcall" fn(pthis: ppbstg_anchor, stat: *mut pbstg_statistics),
    pbstg_nextGeneration: unsafe extern "stdcall" fn() -> ::std::os::raw::c_long,
    pbstg_dumpLeaks: unsafe extern "stdcall" fn(generation: ::std::os::raw::c_long),
    pbstg_dumpHeap: unsafe extern "stdcall" fn(),
    pbstg_alloc: unsafe extern "stdcall" fn(
        pthis: ppbstg_anchor,
        iNumberOfBytes: ULONG,
        subPool: pbstg_subpool
    ) -> *mut ::std::os::raw::c_void,
    pbstg_free: unsafe extern "stdcall" fn(pthis: ppbstg_anchor, stg: *mut ::std::os::raw::c_void),
    pbstg_realloc: unsafe extern "stdcall" fn(
        pthis: ppbstg_anchor,
        pOldStorage: *mut ::std::os::raw::c_void,
        iLength: ULONG,
        subPool: pbstg_subpool
    ) -> *mut ::std::os::raw::c_void,
    pbstg_size: unsafe extern "stdcall" fn(pthis: ppbstg_anchor, pStg: *mut ::std::os::raw::c_void) -> ULONG,
    pbstg_fast_strlen: unsafe extern "stdcall" fn(s: LPTSTR) -> ULONG,
    pbstg_ansitoupper: unsafe extern "stdcall" fn(c: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    pbstg_ansitolower: unsafe extern "stdcall" fn(c: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    pbstg_strdup:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, string: LPCTSTR, subpool: pbstg_subpool) -> LPTSTR,
    pbstg_strdup_malloc: unsafe extern "stdcall" fn(lpstrString: LPTSTR) -> LPTSTR,
    pbstg_str_build: unsafe extern "stdcall" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: LPTSTR
    ),
    pbstg_str_build_char: unsafe extern "stdcall" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        c: TCHAR
    ),
    pbstg_str_build_huge: unsafe extern "stdcall" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: *mut TCHAR
    ),
    pbstg_str_remove_char: unsafe extern "stdcall" fn(string: LPTSTR, c: TCHAR) -> LPTSTR,
    pbstg_str_trim_left: unsafe extern "stdcall" fn(string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR,
    pbstg_str_trim_right: unsafe extern "stdcall" fn(string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR,
    pbstg_str_trim: unsafe extern "stdcall" fn(string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR,
    pbstg_str_wordcap: unsafe extern "stdcall" fn(s: LPTSTR) -> LPTSTR,
    pbstg_atoi_imp: unsafe extern "stdcall" fn(arg1: LPTSTR) -> INT,
    pbstg_atof_imp: unsafe extern "stdcall" fn(arg1: LPTSTR) -> f64,
    pbstg_strtod_imp: unsafe extern "stdcall" fn(pText: LPTSTR, endptr: *mut LPTSTR) -> f64,
    pbstg_atol_imp: unsafe extern "stdcall" fn(arg1: LPTSTR) -> ::std::os::raw::c_long,
    pbstg_strtol_imp: unsafe extern "stdcall" fn(
        arg1: LPTSTR,
        arg2: *mut LPTSTR,
        arg3: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pbstg_atou_imp: unsafe extern "stdcall" fn(arg1: LPTSTR) -> UINT,
    pbstg_atoul_imp: unsafe extern "stdcall" fn(arg1: LPTSTR) -> ULONG,
    pbstg_strtoul_imp:
        unsafe extern "stdcall" fn(arg1: LPTSTR, arg2: *mut LPTSTR, arg3: ::std::os::raw::c_int) -> ULONG,
    pbstg_remove_imp: unsafe extern "stdcall" fn(arg1: LPTSTR) -> INT,
    pbstg_dde_alloc:
        unsafe extern "stdcall" fn(iNumberOfBytes: ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_void,
    pbstg_dde_free: unsafe extern "stdcall" fn(arg1: *mut ::std::os::raw::c_void),
    pbstg_dde_get_handle: unsafe extern "stdcall" fn(arg1: *mut ::std::os::raw::c_void) -> GLOBALHANDLE,
    pbstg_dde_lock: unsafe extern "stdcall" fn(arg1: GLOBALHANDLE) -> *mut ::std::os::raw::c_void,
    pbstg_dde_unlock: unsafe extern "stdcall" fn(arg1: GLOBALHANDLE),
    pbstg_huge_memcmp: unsafe extern "stdcall" fn(
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> ::std::os::raw::c_short,
    pbstg_huge_memcpy: unsafe extern "stdcall" fn(
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void,
    pbstg_huge_memmove: unsafe extern "stdcall" fn(
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void,
    pbstg_huge_memset: unsafe extern "stdcall" fn(
        v1: *mut ::std::os::raw::c_void,
        c: ::std::os::raw::c_short,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void,
    pbstg_huge_strchr: unsafe extern "stdcall" fn(s: *mut TCHAR, c: TCHAR) -> *mut TCHAR,
    pbstg_huge_strcpy: unsafe extern "stdcall" fn(s: *mut TCHAR, s2: *mut TCHAR) -> *mut TCHAR,
    pbstg_huge_strlen: unsafe extern "stdcall" fn(s: *mut TCHAR) -> ULONG,
    pbstg_huge_strncpy: unsafe extern "stdcall" fn(s: *mut TCHAR, s2: *mut TCHAR, count: ULONG) -> *mut TCHAR,
    pbstg_huge_strstr: unsafe extern "stdcall" fn(s: *mut TCHAR, s2: *mut TCHAR) -> *mut TCHAR,
    pbstg_unicodestrdup:
        unsafe extern "stdcall" fn(sa: ppbstg_anchor, pwsz: LPCWSTR, subpool: pbstg_subpool) -> LPWSTR,
    pbstg_unicodestr_build: unsafe extern "stdcall" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPWSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: LPCWSTR
    ),
    pbstg_strtounicodedup:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, psz: LPCTSTR, subpool: pbstg_subpool) -> LPWSTR,
    pbstg_unicodetostrdup:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, pwsz: LPCWSTR, subpool: pbstg_subpool) -> LPTSTR,
    pbstg_strtoansidup:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, psz: LPCTSTR, subpool: pbstg_subpool) -> LPSTR,
    pbstg_ansitostrdup:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, pasz: LPCSTR, subpool: pbstg_subpool) -> LPTSTR,
    pbstg_strtoprintable: unsafe extern "stdcall" fn(dest: LPSTR, source: LPCTSTR) -> LPSTR,
    pbstg_strtoprintabledup:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, psz: LPCTSTR, subpool: pbstg_subpool) -> LPSTR,
    pbstg_printabletostr: unsafe extern "stdcall" fn(dest: LPTSTR, source: LPCSTR) -> LPTSTR,
    pbstg_printabletostrdup:
        unsafe extern "stdcall" fn(pthis: ppbstg_anchor, pasz: LPCSTR, subpool: pbstg_subpool) -> LPTSTR,
    pbstg_lchrcmp: unsafe extern "stdcall" fn(c1: TCHAR, c2: TCHAR) -> INT,
    pbstg_lchrcmpi: unsafe extern "stdcall" fn(c1: TCHAR, c2: TCHAR) -> INT,
    ob_set_session_icontext:
        unsafe extern "stdcall" fn(obthis: POB_THIS, pNewContext: *mut ::std::os::raw::c_void),
    rt_move_thread: unsafe extern "stdcall" fn(rtthis: POB_THIS) -> BOOL,
    rt_clear_thread: unsafe extern "stdcall" fn(),
    rt_get_current_this: unsafe extern "stdcall" fn() -> POB_THIS,
    rt_add_task: unsafe extern "stdcall" fn(rtthis: POB_THIS) -> BOOL,
    rt_free_task: unsafe extern "stdcall" fn() -> BOOL,
    rt_get_current_task_info: unsafe extern "stdcall" fn(info_pos: INT) -> *mut ::std::os::raw::c_void,
    rt_set_current_task_info:
        unsafe extern "stdcall" fn(info_pos: INT, user_info: *mut ::std::os::raw::c_void) -> BOOL,
    rt_get_free_task_slot: unsafe extern "stdcall" fn() -> INT,
    rt_is_running_exe: unsafe extern "stdcall" fn() -> BOOL,
    ob_add_const_data: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        conpool: POB_CONPOOL,
        data: *mut ::std::os::raw::c_void,
        item_type: OB_CONPOOL_ITEM_TYPE,
        nitems: UINT,
        len: ULONG
    ) -> OB_CONST_REF,
    ob_looksym_keyfunc: unsafe extern "stdcall" fn(
        pDataNode: *mut ::std::os::raw::c_void,
        tobthis: *mut ::std::os::raw::c_void
    ) -> LPTSTR,
    ob_looksym_reference:
        unsafe extern "stdcall" fn(obthis: POB_THIS, look_symtab: POB_LOOK_SYMTAB, name: LPTSTR) -> OB_SYM_ID,
    ob_looksym_delete: unsafe extern "stdcall" fn(obthis: POB_THIS, look_symtab: POB_LOOK_SYMTAB, slot: UINT),
    ob_dynarray_index: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        theArray: *mut tag_OB_DYNARRAY,
        index: ULONG,
        extend: BOOL
    ) -> *mut ::std::os::raw::c_void,
    ob_dynarray_grow: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        theArray: *mut tag_OB_DYNARRAY,
        limit: ULONG,
        initialize: BOOL
    ),
    ob_narray_create_static: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        subpool: OB_SUBPOOL,
        num_items: ULONG,
        elmtType: OB_CLASS_HNDL,
        elmtSize: USHORT,
        numDim: USHORT,
        boundsArray: *mut ::std::os::raw::c_long,
        userData: USHORT,
        useNulls: BOOL,
        freeData: BOOL,
        initFn: PNARRAY_INIT_FN
    ) -> *mut tag_OB_NARRAY,
    ob_narray_create_dynamic: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        subpool: OB_SUBPOOL,
        elmtType: OB_CLASS_HNDL,
        elmtSize: USHORT,
        userData: USHORT,
        useNulls: BOOL,
        freeData: BOOL,
        initFn: PNARRAY_INIT_FN
    ) -> *mut tag_OB_NARRAY,
    ob_set_arraydef: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        arraydef: POB_ARRAYDEF,
        no_dims: UINT,
        arr_style: OB_ARRAY_SYMBOL_STYLE,
        bounds: *mut ::std::os::raw::c_long
    ),
    ob_get_array_len: unsafe extern "stdcall" fn(obthis: POB_THIS, arraydef: POB_ARRAYDEF) -> ULONG,
    ob_array_item_init_callback: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        theArray: *mut tag_OB_NARRAY,
        theItem: *mut ::std::os::raw::c_void
    ) -> BOOL,
    ob_init_array: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        arrdef: POB_ARRAYDEF,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        init_data: BOOL
    ) -> POB_ARRAY_INST,
    ob_array_varinfo_nullval:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array_inst: POB_ARRAY_INST) -> BOOL,
    ob_array_set_varinfo_nullval:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array_inst: POB_ARRAY_INST, bNull: BOOL),
    ob_remove_array_data:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array_inst: POB_ARRAY_INST, IsNullVarInfor: BOOL),
    ob_init_pcode_blk: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        no_items: UINT,
        no_line_incr: UINT,
        subpool: OB_SUBPOOL
    ) -> POB_PCODE_BLK,
    ob_del_pcode_blk: unsafe extern "stdcall" fn(obthis: POB_THIS, pcode_blk: POB_PCODE_BLK),
    ob_reuse_routine: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        routlist: POB_ROUTLIST,
        rout_id: OB_ROUT_ID,
        proto_id: OB_PROTO_ID,
        subpool: OB_SUBPOOL,
        clear_pcode: BOOL
    ),
    shMaxDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    shMinDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    shCompareDec: unsafe extern "stdcall" fn(src1: PSH_DEC, src2: PSH_DEC) -> SHORT,
    shAbsDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: PSH_DEC) -> PSH_DEC,
    shNegateDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: PSH_DEC) -> PSH_DEC,
    shRoundDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: PSH_DEC, n: SHORT) -> PSH_DEC,
    shTruncDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: PSH_DEC, n: SHORT) -> PSH_DEC,
    shAddDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    shSubDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    shMultDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    shDivDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    shModDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    shExpDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    shIntToDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: SHORT) -> PSH_DEC,
    shDecToInt: unsafe extern "stdcall" fn(dst: *mut SHORT, src: PSH_DEC) -> *mut SHORT,
    shUintToDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: USHORT) -> PSH_DEC,
    shDecToUint: unsafe extern "stdcall" fn(dst: *mut USHORT, src: PSH_DEC) -> *mut USHORT,
    shByteToDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: ::std::os::raw::c_uchar) -> PSH_DEC,
    shDecToByte: unsafe extern "stdcall" fn(
        dst: *mut ::std::os::raw::c_uchar,
        src: PSH_DEC
    ) -> *mut ::std::os::raw::c_uchar,
    shLongToDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: ::std::os::raw::c_long) -> PSH_DEC,
    shDecToLong: unsafe extern "stdcall" fn(
        arg1: *mut ::std::os::raw::c_long,
        src: PSH_DEC
    ) -> *mut ::std::os::raw::c_long,
    shUlongToDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: ULONG) -> PSH_DEC,
    shDecToUlong: unsafe extern "stdcall" fn(dst: *mut ULONG, src: PSH_DEC) -> *mut ULONG,
    shLonglongToDec:
        unsafe extern "stdcall" fn(dst: PSH_DEC, src: *mut ::std::os::raw::c_longlong) -> PSH_DEC,
    shDecToLonglong: unsafe extern "stdcall" fn(
        dst: *mut ::std::os::raw::c_longlong,
        src: PSH_DEC
    ) -> *mut ::std::os::raw::c_longlong,
    shDecToFloat: unsafe extern "stdcall" fn(dst: *mut f32, src: PSH_DEC) -> *mut f32,
    shFloatToDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: *mut f32) -> PSH_DEC,
    shDoubleToDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: *mut f64) -> PSH_DEC,
    shDecToDouble: unsafe extern "stdcall" fn(dst: *mut f64, src: PSH_DEC) -> *mut f64,
    shDecToAscii: unsafe extern "stdcall" fn(dst: LPTSTR, src: PSH_DEC) -> LPTSTR,
    shAsciiToDec: unsafe extern "stdcall" fn(dst: PSH_DEC, src: LPTSTR) -> PSH_DEC,
    shAsciiToDecRnd: unsafe extern "stdcall" fn(dst: PSH_DEC, src: LPTSTR, n: SHORT) -> PSH_DEC,
    shSetDecFractions: unsafe extern "stdcall" fn(d: PSH_DEC, n: SHORT),
    shSetDecNegative: unsafe extern "stdcall" fn(d: PSH_DEC, n: BOOL),
    shDecSetOverflow: unsafe extern "stdcall" fn(dec: PSH_DEC, neg: BOOL) -> BOOL,
    ob_mgr_init: unsafe extern "stdcall" fn(dbgthis: *mut SH_DBG_THIS, stgthis: ppbstg_anchor) -> POB_THIS,
    ob_mgr_init_ex: unsafe extern "stdcall" fn(
        dbgthis: *mut SH_DBG_THIS,
        stgthis: ppbstg_anchor,
        lpstrTypdefPblName: LPTSTR
    ) -> POB_THIS,
    ob_mgr_restart: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_mgr_terminate: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_free_memory: unsafe extern "stdcall" fn(obthis: POB_THIS, data: *mut ::std::os::raw::c_void),
    ob_free_link_error_list: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_get_link_error_list: unsafe extern "stdcall" fn(obthis: POB_THIS) -> *mut ::std::os::raw::c_void,
    ob_enter_critical_section: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_leave_critical_section: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_alloc_string: unsafe extern "stdcall" fn(obthis: POB_THIS, len: ULONG) -> LPTSTR,
    ob_alloc_blob: unsafe extern "stdcall" fn(obthis: POB_THIS, len: ULONG) -> PSH_BINARY,
    ob_alloc_dec: unsafe extern "stdcall" fn(obthis: POB_THIS) -> PSH_DEC,
    ob_alloc_double: unsafe extern "stdcall" fn(obthis: POB_THIS) -> *mut f64,
    ob_alloc_longlong: unsafe extern "stdcall" fn(obthis: POB_THIS) -> *mut ::std::os::raw::c_longlong,
    ob_alloc_time: unsafe extern "stdcall" fn(obthis: POB_THIS) -> PSH_TIME,
    ob_realloc_string: unsafe extern "stdcall" fn(obthis: POB_THIS, string: LPTSTR, len: ULONG) -> LPTSTR,
    ob_realloc_blob: unsafe extern "stdcall" fn(obthis: POB_THIS, blob: PSH_BINARY, len: ULONG) -> PSH_BINARY,
    ob_dup_string: unsafe extern "stdcall" fn(obthis: POB_THIS, string: LPTSTR) -> LPTSTR,
    ob_dup_blob: unsafe extern "stdcall" fn(obthis: POB_THIS, blob: PSH_BINARY) -> PSH_BINARY,
    ob_dup_dec: unsafe extern "stdcall" fn(obthis: POB_THIS, dec_val: PSH_DEC) -> PSH_DEC,
    ob_dup_double: unsafe extern "stdcall" fn(obthis: POB_THIS, double_val: *mut f64) -> *mut f64,
    ob_dup_longlong: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        longlong_val: *mut ::std::os::raw::c_longlong
    ) -> *mut ::std::os::raw::c_longlong,
    ob_dup_time: unsafe extern "stdcall" fn(obthis: POB_THIS, time_val: PSH_TIME) -> PSH_TIME,
    ob_free_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data: *mut ::std::os::raw::c_void),
    ob_create_appl_report: unsafe extern "stdcall" fn(obthis: POB_THIS) -> POB_APPL_REPORT,
    ob_create_object_report: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        object_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> POB_APPL_REPORT,
    ob_free_appl_report: unsafe extern "stdcall" fn(obthis: POB_THIS, appl_report: POB_APPL_REPORT),
    ob_get_mode: unsafe extern "stdcall" fn(obthis: POB_THIS) -> OB_MODE,
    ob_set_mode: unsafe extern "stdcall" fn(obthis: POB_THIS, mode: OB_MODE) -> OB_MODE,
    ob_get_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        pData: POB_DATA
    ) -> POB_DATA,
    ob_set_field:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, pData: POB_DATA),
    ob_get_field_data:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> POB_DATA,
    ob_get_no_fields: unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG,
    ob_get_parent_obinst: unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_INST_ID,
    ob_get_first_user_field: unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG,
    ob_get_field_type:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> OB_CLASS_ID,
    ob_get_int_field: unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> INT,
    ob_get_uint_field:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> UINT,
    ob_get_byte_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> ::std::os::raw::c_uchar,
    ob_get_long_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> ::std::os::raw::c_long,
    ob_get_ulong_field:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> ULONG,
    ob_get_float_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        fl: *mut f32
    ) -> *mut f32,
    ob_get_ptr_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> *mut ::std::os::raw::c_void,
    ob_get_inst_field:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> OB_INST_ID,
    ob_get_array_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        no_items: *mut UINT
    ) -> *mut ::std::os::raw::c_void,
    ob_array_index: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        array_vals: *mut ::std::os::raw::c_void,
        index: ULONG,
        type_: POB_CLASS_ID
    ) -> *mut ::std::os::raw::c_void,
    ob_get_indirect_obdata:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obInst: OB_INST_ID, obInfo: POB_DATA_INFO) -> POB_DATA,
    ob_array_item: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        array_vals: *mut ::std::os::raw::c_void,
        index: ULONG
    ) -> POB_DATA,
    ob_array_get_index_from_subs: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        theArray: OB_ARRAY_ID,
        subs: *mut ::std::os::raw::c_long
    ) -> ULONG,
    ob_array_calc_index: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        numDims: UINT,
        bounds: *mut ::std::os::raw::c_long,
        subs: *mut ::std::os::raw::c_long
    ) -> ULONG,
    ob_set_int_field:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, int_val: INT),
    ob_set_uint_field:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, uint_val: UINT),
    ob_set_long_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        long_val: ::std::os::raw::c_long
    ),
    ob_set_ulong_field:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, ulong_val: ULONG),
    ob_set_float_field:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, flval: f32),
    ob_set_ptr_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        ptrval: *mut ::std::os::raw::c_void
    ),
    ob_set_array_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        pArray: *mut ::std::os::raw::c_void
    ),
    ob_set_obinst_field: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        obinstval: OB_INST_ID
    ),
    ob_set_underlying_object:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, obj: *mut ::std::os::raw::c_void),
    ob_get_underlying_object:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> *mut ::std::os::raw::c_void,
    ob_is_any_group_locked: unsafe extern "stdcall" fn(obthis: POB_THIS) -> BOOL,
    ob_get_group_lock_count: unsafe extern "stdcall" fn(obthis: POB_THIS) -> UINT,
    ob_is_group_locked: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> BOOL,
    ob_is_group_unlocked: unsafe extern "stdcall" fn(obthis: POB_THIS, obGroupHandle: OB_GROUP_HNDL) -> BOOL,
    ob_is_group_write_locked: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> BOOL,
    ob_lock_group:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, write_only: BOOL) -> INT,
    ob_unlock_group: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT,
    ob_clear_unlocked_groups: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_clear_all_other_unlocked_groups: unsafe extern "stdcall" fn(obthis: POB_THIS, obGroupId: OB_GROUP_ID),
    ob_is_ancestor_locked:
        unsafe extern "stdcall" fn(obthis: POB_THIS, groupid: OB_GROUP_ID, cReadWrite: TCHAR) -> BOOL,
    ob_is_descendent_locked:
        unsafe extern "stdcall" fn(obthis: POB_THIS, groupid: OB_GROUP_ID, cReadWrite: TCHAR) -> BOOL,
    ob_validate_liblist:
        unsafe extern "stdcall" fn(obThis: POB_THIS, pLibList: *mut LPTSTR, iNumberOfItems: UINT) -> INT,
    ob_set_liblist: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_list: *mut LPTSTR,
        no_items: UINT,
        bCreateNewLoader: BOOL
    ) -> INT,
    ob_get_liblist: unsafe extern "stdcall" fn(obthis: POB_THIS, no_items: *mut UINT) -> *mut LPTSTR,
    ob_set_default_appl:
        unsafe extern "stdcall" fn(obthis: POB_THIS, lib_name: LPTSTR, appl_group_name: LPTSTR),
    ob_load_appl_group: unsafe extern "stdcall" fn(obthis: POB_THIS) -> BOOL,
    ob_is_group_in_memory: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        qualified_name: LPTSTR
    ) -> OB_GROUP_HNDL,
    ob_group_declare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> OB_GROUP_HNDL,
    ob_group_reference: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> OB_GROUP_HNDL,
    ob_get_group_name: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> LPTSTR,
    ob_get_group_full_name: unsafe extern "stdcall" fn(obthis: POB_THIS, grouphndl: OB_GROUP_HNDL) -> LPTSTR,
    ob_group_save: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        lib_name: LPTSTR,
        comment: LPTSTR
    ) -> INT,
    ob_group_save_win: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        lib_name: LPTSTR,
        comment: LPTSTR,
        bSaveNormalize: BOOL
    ) -> INT,
    ob_load_group_source: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT,
    ob_rename_group:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, new_name: LPTSTR) -> INT,
    ob_move_group:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, lib_name: LPTSTR) -> INT,
    ob_move_group_with_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        qual_name: LPTSTR,
        oldlib: LPTSTR,
        newlib: LPTSTR
    ) -> INT,
    ob_copy_group_with_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        qual_name: LPTSTR,
        oldlib: LPTSTR,
        newlib: LPTSTR
    ) -> INT,
    ob_copy_group: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        new_name: LPTSTR,
        lib_name: LPTSTR
    ),
    ob_delete_group: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL),
    ob_delete_group_with_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> INT,
    ob_restore_group: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL),
    ob_save_working_group: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL),
    ob_delete_working_group: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_restore_working_group: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_open_group_id: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL),
    ob_close_group: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_get_group_lib: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> LPTSTR,
    ob_run_garbage_collection: unsafe extern "stdcall" fn(obthis: POB_THIS, force: BOOL) -> INT,
    ob_delete_instlist_shlist: unsafe extern "stdcall" fn(obthis: POB_THIS, instlist: *mut shlist),
    ob_get_group_instlist_as_shlist:
        unsafe extern "stdcall" fn(obthis: POB_THIS, groupId: OB_GROUP_ID) -> *mut shlist,
    ob_delete_groups_shlist: unsafe extern "stdcall" fn(obthis: POB_THIS, groups: *mut shlist),
    ob_get_groups_shlist: unsafe extern "stdcall" fn(obthis: POB_THIS) -> *mut shlist,
    ob_store_source:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, source: LPTSTR, len: UINT),
    ob_init_source: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT,
    ob_store_global_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT,
    ob_store_namespace_decl_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT,
    ob_store_shared_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT,
    ob_store_prototype_source:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, source: LPTSTR, len: UINT),
    ob_store_instvar_source:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, source: LPTSTR, len: UINT),
    ob_get_global_src:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, len: *mut UINT) -> LPTSTR,
    ob_get_namespace_decl_src:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, len: *mut UINT) -> LPTSTR,
    ob_get_shared_src:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, len: *mut UINT) -> LPTSTR,
    ob_get_prototype_source:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, len: *mut UINT) -> LPTSTR,
    ob_get_instvar_source:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, len: *mut UINT) -> LPTSTR,
    ob_get_routine_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> LPTSTR,
    ob_decl_and_store_routine_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        routname: LPTSTR,
        result_type: LPTSTR,
        no_args: INT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        source: LPTSTR,
        len: UINT
    ) -> INT,
    ob_store_routine_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID,
        source: LPTSTR,
        len: UINT
    ),
    ob_store_create_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT,
    ob_store_destroy_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT,
    ob_get_function_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> LPTSTR,
    ob_store_function_src: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID,
        source: LPTSTR,
        len: UINT
    ),
    ob_symbol_search_extended: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obClassHandle: OB_CLASS_HNDL,
        iCurrScope: INT,
        pszVarName: LPTSTR,
        bSkipVars: BOOL,
        bSkipTHIS: BOOL,
        puiLevel: *mut UINT,
        pbIsConstantField: *mut BOOL,
        pbIsPrivateMember: *mut BOOL,
        ppszFullName: *mut LPTSTR
    ) -> BOOL,
    ob_symbol_search: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obClassHandle: OB_CLASS_HNDL,
        iCurrScope: INT,
        pszVarName: LPTSTR,
        bSkipVars: BOOL,
        bSkipTHIS: BOOL,
        puiLevel: *mut UINT,
        pbIsConstantField: *mut BOOL
    ) -> BOOL,
    ob_class_declare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR,
        parent_class: OB_CLASS_ID,
        within_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL,
    ob_get_full_qualified_typename: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_id: OB_CLASS_ID
    ) -> LPTSTR,
    ob_class_declare_inh: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR,
        parent_class: OB_CLASS_HNDL,
        within_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL,
    ob_class_reference: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR
    ) -> OB_CLASS_HNDL,
    ob_class_name: unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> LPTSTR,
    ob_class_name_not_indirect:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> LPTSTR,
    ob_get_type_name: unsafe extern "stdcall" fn(obthis: POB_THIS, datanode: POB_DATA) -> LPTSTR,
    ob_classhndl_indirect:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL,
    ob_get_parent_class:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL,
    ob_get_within_class:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL,
    ob_class_delete: unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL),
    ob_class_rename:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, new_name: LPTSTR) -> INT,
    ob_is_a_system_class: unsafe extern "stdcall" fn(obthis: POB_THIS, class_name: LPTSTR) -> BOOL,
    ob_is_class_inherited: unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    ob_is_class_descendant: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        ancestor: OB_CLASS_HNDL,
        descendant: OB_CLASS_HNDL
    ) -> BOOL,
    ob_is_inh_from_user_class:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    ob_get_sec_class_ancestor:
        unsafe extern "stdcall" fn(obthis: POB_THIS, sec_class: OB_CLASS_HNDL) -> OB_CLASS_HNDL,
    ob_is_class_enum: unsafe extern "stdcall" fn(obThis: POB_THIS, obClassHndl: OB_CLASS_HNDL) -> BOOL,
    ob_new_event: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_name: LPTSTR,
        token_name: LPTSTR,
        result_type: LPTSTR,
        no_args: INT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        is_external_event: BOOL
    ) -> OB_VTABLE_ID,
    ob_update_event: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        event_name: LPTSTR,
        token_name: LPTSTR,
        result_type: LPTSTR,
        no_args: INT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        is_external_event: BOOL,
        no_throws: INT,
        throws_names: *mut LPTSTR
    ) -> INT,
    ob_delete_event:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, event_name: LPTSTR) -> INT,
    ob_has_events: unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    ob_get_event_token_id: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_name: LPTSTR,
        vtable_id: POB_VTABLE_ID
    ) -> OB_EVT_TOKEN_ID,
    ob_get_event_id_from_token: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_token: OB_EVT_TOKEN_ID
    ) -> OB_VTABLE_ID,
    ob_does_event_script_exist: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_id: OB_VTABLE_ID
    ) -> BOOL,
    ob_get_routine_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> LPTSTR,
    ob_delete_routine:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID),
    ob_get_curr_routine:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_VTABLE_ID,
    ob_get_curr_function:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_VTABLE_ID,
    ob_get_routid_from_vtable_id: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID
    ) -> OB_ROUT_ID,
    ob_is_valid_event_index:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> BOOL,
    ob_has_scripts: unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    ob_get_routine_type: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> OB_ROUT_TYPE,
    ob_get_function_vtable_ids: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        include_ancestors: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID,
    ob_get_function_vtable_ids_for_ide: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        include_ancestors: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID,
    ob_get_event_vtable_ids: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID,
    ob_get_function_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> LPTSTR,
    ob_delete_function:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> INT,
    ob_find_routine: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        routine_type: OB_ROUT_TYPE,
        routine_name: LPTSTR,
        result_type: LPTSTR,
        no_args: INT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        ov_error: POB_PROTO_OVERLOAD_ERROR,
        pproto_id: POB_PROTO_ID,
        pvtable_id: POB_VTABLE_ID
    ) -> BOOL,
    ob_get_vtable_id_from_proto_id: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        proto_id: OB_PROTO_ID
    ) -> OB_VTABLE_ID,
    ob_get_dll_func_names: unsafe extern "stdcall" fn(obthis: POB_THIS, no_funcs: *mut UINT) -> *mut LPTSTR,
    ob_get_global_func_names_in_lib:
        unsafe extern "stdcall" fn(obthis: POB_THIS, no_funcs: *mut UINT, lib_name: LPTSTR) -> *mut LPTSTR,
    ob_get_global_func_index: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        class_hndl: POB_CLASS_HNDL
    ) -> OB_VTABLE_ID,
    ob_get_func_index_in_lib: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        lib_name: LPTSTR,
        class_hndl: POB_CLASS_HNDL
    ) -> OB_VTABLE_ID,
    ob_get_proto_is_external_event:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> BOOL,
    ob_get_protoarg_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        no_items: *mut UINT,
        type_name: *mut LPTSTR,
        member_access: POB_MEMBER_ACCESS
    ) -> POB_ARG_INFO,
    ob_get_proto_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        no_args: *mut UINT,
        name: *mut LPTSTR,
        type_: *mut LPTSTR,
        member_access: POB_MEMBER_ACCESS,
        dll_lib_name: *mut LPTSTR,
        is_obsolete: *mut BOOL,
        token: POB_EVT_TOKEN_ID,
        rout_type: *mut OB_ROUT_TYPE,
        is_inherit: *mut BOOL
    ) -> POB_ARG_INFO,
    ob_get_method_signature: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID
    ) -> LPTSTR,
    ob_was_event_prototype_changed: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_id: OB_VTABLE_ID
    ) -> BOOL,
    ob_get_proto_name_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        func_id: OB_VTABLE_ID,
        name: *mut LPTSTR,
        is_obsolete: *mut BOOL
    ),
    ob_get_proto_throws_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        func_id: OB_VTABLE_ID,
        throws_list: *mut POB_CLASS_ID,
        no_throws: *mut UINT,
        group_id: POB_GROUP_ID
    ),
    ob_lookup_routine_by_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obInst: OB_INST_ID,
        lpstrRoutineName: LPTSTR,
        pVtableId: POB_VTABLE_ID,
        pNumRoutines: *mut UINT,
        pobRoutineType: *mut OB_ROUT_TYPE,
        pNoArgs: *mut UINT,
        ppobArgClassId: *mut POB_CLASS_ID,
        pbVarArgs: *mut BOOL
    ) -> HRESULT,
    ob_get_objnames_of_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_id: OB_CLASS_ID,
        no_items: *mut UINT
    ) -> *mut LPTSTR,
    ob_has_object_of_class: unsafe extern "stdcall" fn(obthis: POB_THIS, class_id: OB_CLASS_ID) -> BOOL,
    ob_get_obj_classhndls_of_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obClassID: OB_CLASS_ID,
        pNumberOfItems: *mut UINT
    ) -> POB_CLASS_HNDL,
    ob_get_objnames_of_class_in_lib: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_id: OB_CLASS_ID,
        no_items: *mut UINT,
        lib_name: LPTSTR
    ) -> *mut LPTSTR,
    ob_global_reference: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_name: LPTSTR,
        group_hndl: POB_GROUP_HNDL
    ) -> OB_CLASS_HNDL,
    ob_global_reference_in_lib: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_name: LPTSTR,
        lib_name: LPTSTR,
        group_hndl: POB_GROUP_HNDL,
        class_id: OB_CLASS_ID
    ) -> OB_CLASS_HNDL,
    ob_global_reference_of_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        grouphndl: POB_GROUP_HNDL,
        of_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL,
    ob_get_obinst_class_hndl:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_CLASS_HNDL,
    ob_is_a_typedef: unsafe extern "stdcall" fn(obthis: POB_THIS, data: POB_DATA) -> BOOL,
    ob_is_an_enum:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, data_node: POB_DATA) -> BOOL,
    ob_get_system_class:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_ID,
    ob_get_obinst_system_class:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_CLASS_ID,
    ob_get_obinst_group_hndl:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_GROUP_HNDL,
    ob_get_obinst_class_name: unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> LPTSTR,
    ob_fetch_fields_of_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        in_class: OB_CLASS_HNDL,
        of_class: OB_CLASS_ID,
        field_filter: OB_FIELD_FILTER,
        class_list: *mut POB_CLASS_ID,
        no_items: *mut UINT
    ) -> *mut LPTSTR,
    ob_get_fields_of_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        in_class: OB_CLASS_HNDL,
        of_class: OB_CLASS_ID,
        class_list: *mut POB_CLASS_ID,
        no_items: *mut UINT
    ) -> *mut LPTSTR,
    ob_get_local_var_info: unsafe extern "stdcall" fn(obthis: POB_THIS, no_items: *mut UINT) -> POB_DATA_INFO,
    ob_get_shared_vars_of_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        return_shared: BOOL,
        return_global: BOOL,
        of_class: OB_CLASS_ID,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    ob_get_shared_var_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        return_shared: BOOL,
        return_global: BOOL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    ob_get_global_vars_of_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        of_class: OB_CLASS_ID,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    ob_get_class_field_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    ob_get_enum_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_enums: *mut UINT
    ) -> POB_ENUM_INFO,
    ob_get_class_event_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_events: *mut UINT
    ) -> POB_EVENT_INFO,
    ob_get_instance_field_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    ob_get_obinst_field_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        no_items: *mut UINT,
        field_filter: BOOL
    ) -> POB_DATA_INFO,
    ob_get_obinst_all_field_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        no_items: *mut UINT,
        field_filter: BOOL
    ) -> POB_DATA_INFO,
    ob_get_classes_within_group: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        of_class_id: OB_CLASS_ID,
        total_items: *mut UINT
    ) -> POB_CLASS_INFO,
    ob_get_enums_within_group: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        total_items: *mut UINT
    ) -> POB_CLASS_INFO,
    ob_get_global_var_data: unsafe extern "stdcall" fn(obthis: POB_THIS, var_name: LPTSTR) -> POB_DATA,
    ob_object_reference_count: unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG,
    ob_named_global_var_info: unsafe extern "stdcall" fn(obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO,
    ob_named_shared_var_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        varname: LPTSTR
    ) -> POB_DATA_INFO,
    ob_named_special_var_info: unsafe extern "stdcall" fn(obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO,
    ob_named_local_var_info: unsafe extern "stdcall" fn(obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO,
    ob_named_field_info:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID, fieldname: LPTSTR) -> POB_DATA_INFO,
    ob_get_array_info: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> POB_ARRAY_INFO,
    ob_get_array_bounds_string: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        data_node: POB_DATA
    ) -> LPTSTR,
    ob_get_array_bounds_string_from_field_info:
        unsafe extern "stdcall" fn(obthis: POB_THIS, fieldinfo: POB_DATA_INFO) -> LPTSTR,
    ob_get_info_watchpoint:
        unsafe extern "stdcall" fn(obthis: POB_THIS, info: POB_DATA_INFO) -> *mut ::std::os::raw::c_void,
    ob_compile_lib_entry: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR,
        write_source: BOOL
    ) -> INT,
    ob_compile_lib_typedefs: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR,
        bUpdateModifyTime: BOOL
    ) -> INT,
    ob_compile_lib_entry_3_pass:
        unsafe extern "stdcall" fn(obThis: POB_THIS, lpszLibraryName: LPTSTR, lpszEntryName: LPTSTR) -> BOOL,
    ob_compile_lib_scripts:
        unsafe extern "stdcall" fn(obthis: POB_THIS, lib_name: LPTSTR, entry_name: LPTSTR) -> INT,
    ob_func_search: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        argtypes: POB_CLASS_ID,
        no_args: UINT,
        type_: POB_CLASS_ID,
        error: POB_PROTOREF_ERROR
    ) -> POB_FUNCCALL_INFO,
    ob_del_funccall_info: unsafe extern "stdcall" fn(obthis: POB_THIS, funccall_info: POB_FUNCCALL_INFO),
    ob_link_project: unsafe extern "stdcall" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT,
    ob_check_for_locked_menu: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_create_obinst: unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_INST_ID,
    ob_instantiate_child_object: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        parent_obinst: OB_INST_ID,
        child_type: OB_CLASS_ID,
        field_id: UINT,
        child_obinst: POB_INST_ID
    ) -> INT,
    ob_instantiate_system_object:
        unsafe extern "stdcall" fn(obthis: POB_THIS, object_type: OB_CLASS_ID, pObint: POB_INST_ID) -> INT,
    ob_destroy_obinst: unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> INT,
    ob_set_runtime: unsafe extern "stdcall" fn(obthis: POB_THIS, bInRuntime: BOOL),
    ob_create_executable: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        pExecBlock: POB_EXEC,
        bFreeGroups: BOOL,
        pManifestInfo: LPTSTR
    ) -> INT,
    ob_create_library: unsafe extern "stdcall" fn(obthis: POB_THIS, pExecBlock: POB_EXEC) -> INT,
    ob_create_consolidated_library: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        pTargetLibrary: LPTSTR,
        nSourceLibs: ULONG,
        pSourceLibs: *mut LPTSTR,
        pIncludeLibType: POB_LIB_INCLUDE_TYPE,
        nComponents: ULONG,
        pComponents: *mut LPTSTR,
        pPBRFile: LPTSTR,
        pPBDArray: *mut PBD_ARRAY,
        pNumPBD: *mut ULONG,
        ppErrorMessage: *mut LPTSTR
    ) -> INT,
    ob_create_interface_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        hSourceClass: OB_CLASS_HNDL,
        lpstrDestClassName: LPTSTR,
        lpstrDestLibrary: LPTSTR,
        lpstrComments: LPTSTR,
        lpstrSourceClassName: LPTSTR
    ) -> HRESULT,
    ob_init_executable:
        unsafe extern "stdcall" fn(obthis: POB_THIS, executable_name: LPTSTR) -> OB_CLASS_HNDL,
    ob_scan_source_blocks: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        source: POB_SOURCE_BLOCK,
        src_len: ULONG,
        srcloc: *mut *mut ::std::os::raw::c_void,
        no_blocks: *mut UINT,
        subpool: OB_SUBPOOL
    ) -> *mut LPTSTR,
    ob_create_launcher: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        pExecBlock: POB_EXEC,
        pObjectList: *mut ::std::os::raw::c_void
    ) -> INT,
    ob_sanitize_pb_name: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        lpszDestName: LPTSTR,
        destLength: ::std::os::raw::c_long,
        lpszNameToSanitize: LPTSTR
    ),
    ob_validate_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_CONFLICT_LIST,
    ob_get_orphaned_classes: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        total_items: *mut UINT
    ) -> POB_CLASS_HNDL,
    ob_validate_type_name:
        unsafe extern "stdcall" fn(obThis: POB_THIS, obGroupHndl: OB_GROUP_HNDL, TypeName: LPTSTR) -> BOOL,
    ob_convert_to_ver2_source:
        unsafe extern "stdcall" fn(obthis: POB_THIS, lib_name: LPTSTR, entry_name: LPTSTR) -> INT,
    ob_is_vers2_obj: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR,
        error: *mut INT
    ) -> BOOL,
    ob_build_ordered_compile_list: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        list_type: OB_COMPILE_LIST_TYPE,
        no_items: *mut UINT,
        inconsistency: POB_INCONSISTENCY_TYPE
    ) -> POB_COMPILE_LIST,
    ob_free_ordered_compile_list:
        unsafe extern "stdcall" fn(obthis: POB_THIS, compile_list: POB_COMPILE_LIST, no_items: UINT),
    ob_build_hierarchy_list: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        no_items: *mut UINT,
        type_: OB_CLASS_ID
    ) -> POB_HIERARCHY_LIST,
    ob_free_hierarchy_list:
        unsafe extern "stdcall" fn(obthis: POB_THIS, hierarchy_list: POB_HIERARCHY_LIST, no_items: UINT),
    ob_clear_instance_ref:
        unsafe extern "stdcall" fn(obthis: POB_THIS, back_ptr: *mut ::std::os::raw::c_void),
    ob_insert_inst_ref_dbg: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        ref_addr: *mut ::std::os::raw::c_void,
        fileName: LPTSTR,
        lineNo: UINT
    ),
    ob_open_typedef_group: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszGroupName: LPTSTR,
        bCreateIfNotFound: BOOL
    ) -> INT,
    ob_save_dll_to_pbd: unsafe extern "stdcall" fn(argc: ::std::os::raw::c_int, argv: *mut LPTSTR) -> INT,
    ob_convert_pbx_to_native_groups:
        unsafe extern "stdcall" fn(obthis: POB_THIS, pbl_name: LPCTSTR, dll_name: LPCTSTR) -> INT,
    ob_share_typedef_group: unsafe extern "stdcall" fn(destObThis: POB_THIS, srcObThis: POB_THIS) -> INT,
    ob_unshare_typedef_group: unsafe extern "stdcall" fn(obThis: POB_THIS) -> INT,
    ob_cm_evaluate_expression:
        unsafe extern "stdcall" fn(obthis: POB_THIS, text: LPTSTR, result_data_node: POB_DATA) -> INT,
    ob_entryInheritsFromClass: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszTypeName: LPTSTR,
        lpszEntryName: LPTSTR
    ) -> BOOL,
    ob_get_class_from_name: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        lpszClassName: LPTSTR,
        pbIsEnum: *mut BOOL
    ) -> OB_CLASS_HNDL,
    ob_local_global_lv:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID) -> POB_DATA,
    ob_local_global_refpkt: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        group: POB_GROUP,
        var_id: OB_SYM_ID
    ),
    ob_shared_global_lv:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID) -> POB_DATA,
    ob_shared_global_refpkt: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        group: POB_GROUP,
        var_id: OB_SYM_ID
    ),
    ob_shared_lv:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID) -> POB_DATA,
    ob_shared_refpkt: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        group: POB_GROUP,
        var_id: OB_SYM_ID
    ),
    ob_convert_chararray_to_string: unsafe extern "stdcall" fn(obthis: POB_THIS, data: POB_DATA) -> BOOL,
    ob_class_delete_and_withinclass:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, class_id: OB_CLASS_ID),
    ob_find_orphan_class: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszEntryName: LPTSTR,
        bFoundAncestor: BOOL
    ) -> INT,
    ob_nuke_orphan_class:
        unsafe extern "stdcall" fn(obThis: POB_THIS, lpszLibraryName: LPTSTR, lpszEntryName: LPTSTR) -> BOOL,
    ob_is_ancestor_class_modified:
        unsafe extern "stdcall" fn(obThis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    ob_rebuild_instance_image: unsafe extern "stdcall" fn(obThis: POB_THIS, class_hndl: OB_CLASS_HNDL),
    ob_build_compile_list:
        unsafe extern "stdcall" fn(obthis: POB_THIS, no_items: *mut UINT) -> POB_COMPILE_LIST,
    ot_get_next_evaled_arg: unsafe extern "stdcall" fn(obthis: POB_THIS) -> POB_DATA,
    ot_get_next_evaled_arg_no_convert: unsafe extern "stdcall" fn(obthis: POB_THIS) -> POB_DATA,
    ot_get_next_lvalue_arg:
        unsafe extern "stdcall" fn(obthis: POB_THIS, str_: *mut POT_LVALUE_INFO) -> POB_DATA,
    ot_get_simple_intarg: unsafe extern "stdcall" fn(obthis: POB_THIS, null: *mut BOOL) -> INT,
    ot_get_simple_longarg:
        unsafe extern "stdcall" fn(obthis: POB_THIS, null: *mut BOOL) -> ::std::os::raw::c_long,
    ot_get_intarg: unsafe extern "stdcall" fn(obthis: POB_THIS, null: *mut BOOL) -> INT,
    ot_get_uintarg: unsafe extern "stdcall" fn(obthis: POB_THIS, null: *mut BOOL) -> UINT,
    ot_get_longarg: unsafe extern "stdcall" fn(obthis: POB_THIS, null: *mut BOOL) -> ::std::os::raw::c_long,
    ot_get_ulongarg: unsafe extern "stdcall" fn(obthis: POB_THIS, null: *mut BOOL) -> ULONG,
    ot_get_decarg: unsafe extern "stdcall" fn(obthis: POB_THIS, null: *mut BOOL) -> PSH_DEC,
    ot_get_floatarg: unsafe extern "stdcall" fn(obthis: POB_THIS, fl: *mut f32, null: *mut BOOL) -> *mut f32,
    ot_get_doublearg:
        unsafe extern "stdcall" fn(obthis: POB_THIS, doub: *mut f64, null: *mut BOOL) -> *mut f64,
    ot_get_longlongarg: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        longlong_val: *mut ::std::os::raw::c_longlong,
        null: *mut BOOL
    ) -> *mut ::std::os::raw::c_longlong,
    ot_get_obinstarg:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: POB_INST_ID, null: *mut BOOL) -> POB_INST_ID,
    ot_get_valptr_arg:
        unsafe extern "stdcall" fn(obthis: POB_THIS, null: *mut BOOL) -> *mut ::std::os::raw::c_void,
    ot_init_arglist: unsafe extern "stdcall" fn(obthis: POB_THIS, nargs: UINT) -> UINT,
    ot_get_valptr:
        unsafe extern "stdcall" fn(obthis: POB_THIS, data: POB_DATA) -> *mut ::std::os::raw::c_void,
    ot_type_srch: unsafe extern "stdcall" fn(name: LPTSTR) -> INT,
    ot_type_attr: unsafe extern "stdcall" fn(type_: OB_CLASS_ID) -> INT,
    ot_get_class_name:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> LPTSTR,
    ot_is_array_eq: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        array_id1: OB_ARRAY_ID,
        array_id2: OB_ARRAY_ID,
        nullval: *mut BOOL
    ) -> BOOL,
    ot_is_struct_eq: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        data_node1: POB_DATA,
        data_node2: POB_DATA,
        nullval: *mut BOOL
    ) -> BOOL,
    ot_create_obinst_with_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        lvalue_info: POT_LVALUE_INFO,
        class_name: LPTSTR,
        nested_obinst: OB_INST_ID
    ) -> OB_INST_ID,
    ot_create_obinst_at_lval: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        lvalue_info: POT_LVALUE_INFO,
        nested_obinst: OB_INST_ID
    ) -> OB_INST_ID,
    ot_get_curr_obinst_expr: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obinst_buf: POB_INST_ID,
        nullval: *mut BOOL
    ) -> POB_INST_ID,
    ot_func_call: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        funccall_info: POB_FUNCCALL_INFO,
        actual_args: *mut *mut ::std::os::raw::c_void
    ) -> POB_DATA,
    ot_set_return_val: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA),
    ot_set_return_double: unsafe extern "stdcall" fn(obthis: POB_THIS, doub_val: *mut f64, null_val: BOOL),
    ot_set_return_longlong: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        longl_val: *mut ::std::os::raw::c_longlong,
        null_val: BOOL
    ),
    ot_set_return_dec: unsafe extern "stdcall" fn(obthis: POB_THIS, dec_val: PSH_DEC, null_val: BOOL),
    ot_no_return_val: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ot_assign_lvalue_dec:
        unsafe extern "stdcall" fn(obthis: POB_THIS, lvalue_data: POB_DATA, val: PSH_DEC, nullval: BOOL),
    ot_assign_lvalue_double:
        unsafe extern "stdcall" fn(obthis: POB_THIS, lvalue_data: POB_DATA, val: f64, nullval: BOOL),
    ot_assign_lvalue_longlong: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        val: ::std::os::raw::c_longlong,
        nullval: BOOL
    ),
    ot_assign_lvalue_blob:
        unsafe extern "stdcall" fn(obthis: POB_THIS, lvalue_data: POB_DATA, val: PSH_BINARY, nullval: BOOL),
    ot_assign_lvalue_obinst:
        unsafe extern "stdcall" fn(obthis: POB_THIS, lvalue_data: POB_DATA, val: OB_INST_ID, nullval: BOOL),
    ot_assign_lvalue_array: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        rvalue_array: OB_ARRAY_ID,
        nullval: BOOL
    ),
    ot_assign_lvalue_any: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        rvalue_data: POB_DATA,
        rhs_class_id: OB_CLASS_ID
    ),
    ot_set_local_var:
        unsafe extern "stdcall" fn(ths: POB_THIS, sym_id: OB_SYM_ID, data_node: POB_DATA) -> INT,
    ot_set_shared_var: unsafe extern "stdcall" fn(
        ths: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        sym_id: OB_SYM_ID,
        data_node: POB_DATA
    ) -> INT,
    ot_set_obinst_var: unsafe extern "stdcall" fn(
        ths: POB_THIS,
        ob_inst_id: OB_INST_ID,
        field_id: UINT,
        data_node: POB_DATA
    ) -> INT,
    ot_set_local_array_item:
        unsafe extern "stdcall" fn(ths: POB_THIS, sym_id: OB_SYM_ID, index: UINT, data_node: POB_DATA) -> INT,
    ot_set_shared_array_item: unsafe extern "stdcall" fn(
        ths: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        sym_id: OB_SYM_ID,
        index: UINT,
        data_node: POB_DATA
    ) -> INT,
    ot_set_obinst_array_item: unsafe extern "stdcall" fn(
        ths: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        index: ULONG,
        new_data: POB_DATA
    ) -> INT,
    ot_get_array_values: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        arraynode: POB_DATA,
        nitems: *mut UINT
    ) -> *mut ::std::os::raw::c_void,
    ot_reset_array: unsafe extern "stdcall" fn(obthis: POB_THIS, array_node: POB_DATA, nitems: ULONG) -> INT,
    ot_get_local_var:
        unsafe extern "stdcall" fn(obthis: POB_THIS, grphndl: OB_GROUP_HNDL, sym_id: OB_SYM_ID) -> POB_DATA,
    ot_get_shared_var:
        unsafe extern "stdcall" fn(obthis: POB_THIS, grphndl: OB_GROUP_HNDL, sym_id: OB_SYM_ID) -> POB_DATA,
    ot_math_type_convert:
        unsafe extern "stdcall" fn(class_id1: OB_CLASS_ID, class_id2: OB_CLASS_ID) -> OB_CLASS_ID,
    ot_get_int_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> INT,
    ot_get_uint_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> UINT,
    ot_get_byte_value:
        unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_uchar,
    ot_get_long_value:
        unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_long,
    ot_get_ulong_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> ULONG,
    ot_get_dec_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> PSH_DEC,
    ot_get_float_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> f32,
    ot_get_double_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> f64,
    ot_get_longlong_value:
        unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_longlong,
    ot_free_val_ptr: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA),
    ot_free_array: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA),
    ot_convert_to_int: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> INT,
    ot_convert_to_uint: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> UINT,
    ot_convert_to_byte:
        unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_uchar,
    ot_convert_to_long:
        unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_long,
    ot_convert_to_ulong: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> ULONG,
    ot_convert_to_dec: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> PSH_DEC,
    ot_convert_to_float: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> f32,
    ot_convert_to_double: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> f64,
    ot_convert_to_longlong:
        unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_longlong,
    ot_ansi_lower: unsafe extern "stdcall" fn(obthis: POB_THIS, string: LPTSTR) -> LPTSTR,
    ot_ansi_upper: unsafe extern "stdcall" fn(obthis: POB_THIS, string: LPTSTR) -> LPTSTR,
    ot_ansi_strcmp: unsafe extern "stdcall" fn(obthis: POB_THIS, stringOne: LPTSTR, stringTwo: LPTSTR) -> INT,
    ot_get_field_lv:
        unsafe extern "stdcall" fn(obthis: POB_THIS, obInst: OB_INST_ID, fieldId: UINT) -> POB_DATA,
    ot_get_field_item_lv: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        obInst: OB_INST_ID,
        fieldId: UINT,
        item_index: ULONG
    ) -> POB_DATA,
    ot_assign_ref_int:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: INT, nullval: BOOL),
    ot_assign_ref_uint:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: UINT, nullval: BOOL),
    ot_assign_ref_byte: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_uchar,
        nullval: BOOL
    ),
    ot_assign_ref_long: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_long,
        nullval: BOOL
    ),
    ot_assign_ref_ulong:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: ULONG, nullval: BOOL),
    ot_assign_ref_dec:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_DEC, nullval: BOOL),
    ot_assign_ref_float:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: f32, nullval: BOOL),
    ot_assign_ref_double:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: f64, nullval: BOOL),
    ot_assign_ref_longlong: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_longlong,
        nullval: BOOL
    ),
    ot_assign_ref_string:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: LPTSTR, nullval: BOOL),
    ot_assign_ref_bool:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: BOOL, nullval: BOOL),
    ot_assign_ref_char:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: TCHAR, nullval: BOOL),
    ot_assign_ref_blob:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_BINARY, nullval: BOOL),
    ot_assign_ref_time:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_TIME, nullval: BOOL),
    ot_assign_ref_date:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_TIME, nullval: BOOL),
    ot_assign_ref_datetime:
        unsafe extern "stdcall" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_TIME, nullval: BOOL),
    ot_assign_ref_obinst: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: OB_INST_ID,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ) -> INT,
    ot_assign_ref_enum: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: INT,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ),
    ot_assign_ref_array: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: OB_ARRAY_ID,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ),
    ot_assign_ref_any: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        rvalue_data: POB_DATA,
        rhs_class_id: OB_CLASS_ID
    ),
    ot_get_nested_obinst: unsafe extern "stdcall" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_INST_ID,
    ot_array_create_bounded: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        num_items: ULONG,
        elmtType: OB_CLASS_HNDL,
        varInfo: USHORT,
        numDim: USHORT,
        boundsArray: *mut ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void,
    ot_array_create_unbounded: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        elmtType: OB_CLASS_HNDL,
        varInfo: USHORT
    ) -> *mut ::std::os::raw::c_void,
    ot_array_index: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        array: *mut ::std::os::raw::c_void,
        index: ULONG
    ) -> POB_DATA,
    ot_array_set_free_data:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void, newValue: BOOL),
    ot_array_free_data:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> BOOL,
    ot_array_class_id:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> OB_CLASS_ID,
    ot_array_class_hndl:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> OB_CLASS_HNDL,
    ot_array_num_dimensions:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> USHORT,
    ot_array_num_items:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> ULONG,
    ot_is_array_unbounded:
        unsafe extern "stdcall" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> BOOL,
    ot_get_arraydef_no_dims:
        unsafe extern "stdcall" fn(obthis: POB_THIS, arrdef: *mut ::std::os::raw::c_void) -> USHORT,
    ot_get_arraydef_style: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void
    ) -> OB_ARRAY_SYMBOL_STYLE,
    ot_get_arraydef_bounds: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_long,
    ot_get_arraydef_varinfo:
        unsafe extern "stdcall" fn(obthis: POB_THIS, arrdef: *mut ::std::os::raw::c_void) -> OB_INFO_FLAGS,
    ot_get_arraydef_upper_bound: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void,
        dimension: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long,
    ot_get_arraydef_lower_bound: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void,
        dimension: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long,
    ot_randomize: unsafe extern "stdcall" fn(obthis: POB_THIS, iSeed: UINT),
    ot_rand: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lLimit: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long,
    ot_class_compare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        classHndl1: OB_CLASS_HNDL,
        classHndl2: OB_CLASS_HNDL
    ) -> BOOL,
    ot_assign_global_var_obinst:
        unsafe extern "stdcall" fn(obthis: POB_THIS, szName: LPTSTR, obInst: OB_INST_ID) -> INT,
    ob_class_indirect:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: *mut POB_GROUP, class_id: POB_CLASS_ID) -> INT,
    ob_add_external_class_ref: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        local_group: POB_GROUP,
        ext_group_id: OB_GROUP_ID,
        ext_class_id: OB_CLASS_ID,
        refstyle: OB_GLOB_REFSTYLE,
        error: *mut INT
    ) -> OB_CLASS_ID,
    ob_get_local_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        refstyle: OB_GLOB_REFSTYLE,
        error: *mut INT
    ) -> OB_CLASS_ID,
    ob_get_primary_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> OB_CLASS_ID,
    ob_build_qual_sec_class_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        primary_class_name: LPTSTR,
        sec_class_name: LPTSTR
    ) -> LPTSTR,
    ob_decl_indirect_sec_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        target_group: POB_GROUP,
        prim_class_name: LPTSTR,
        sec_class_name: LPTSTR,
        error: *mut INT
    ) -> OB_CLASS_ID,
    ob_update_class_ref: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        refstyle: OB_GLOB_REFSTYLE,
        is_prim_parent: BOOL
    ),
    ob_update_glob_class_instflag: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        is_instance: BOOL
    ),
    ob_is_class_member_accessable: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        member_access: OB_MEMBER_ACCESS,
        access_check_type: OB_MEMBER_ACCESS_TYPE,
        inheritance_level: UINT,
        in_system_routine: BOOL
    ) -> BOOL,
    ob_get_system_func_class: unsafe extern "stdcall" fn(obthis: POB_THIS) -> POB_RUNTIME_CLASS,
    ob_get_global_func_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        pGroup: POB_GROUP,
        classId: OB_CLASS_ID,
        module_id: OB_MODULE_ID
    ) -> POB_RUNTIME_CLASS,
    ob_type_declare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        type_name: LPTSTR,
        type_kind: OB_TYPE_KIND,
        style: OB_CLASS_STYLE,
        parent_type: OB_CLASS_ID,
        nested_type: OB_CLASS_ID,
        autoinstantiate: BOOL,
        error: *mut INT
    ) -> OB_CLASS_ID,
    ob_type_declare_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        type_name: LPTSTR,
        type_kind: OB_TYPE_KIND,
        class_style: OB_CLASS_STYLE,
        parent_class: OB_CLASS_ID,
        nested_class: OB_CLASS_ID,
        autoinstantiate: BOOL,
        error: *mut INT
    ) -> OB_CLASS_ID,
    ob_type_declare_vtab: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        parent_class: OB_CLASS_ID,
        error: *mut INT
    ),
    ob_type_reference:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, type_name: LPTSTR) -> OB_CLASS_ID,
    ob_get_first_type: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: POB_CLASS_ID,
        style: POB_CLASS_STYLE
    ) -> LPTSTR,
    ob_get_next_type: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_id: POB_CLASS_ID,
        style: POB_CLASS_STYLE
    ) -> LPTSTR,
    ob_type_init_process: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        class_style: OB_CLASS_STYLE
    ),
    ob_type_decl_process:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID),
    ob_get_nested_class:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> OB_CLASS_ID,
    ob_get_class_entry: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> POB_CLASS_ENTRY,
    ob_is_class_indirect:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> BOOL,
    ob_fetch_routine: unsafe extern "stdcall" fn(
        class_entry: POB_CLASS_ENTRY,
        rout_id: OB_ROUT_ID,
        type_: *mut OB_ROUT_TYPE
    ) -> POB_ROUTNODE,
    ob_type_proto_decl: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        type_: OB_CLASS_ID,
        mod_id: OB_MODULE_ID,
        args: POB_PROTO_ARG,
        no_args: UINT,
        func_type: OB_FUNC_TYPE,
        dllname: LPTSTR,
        aliasname: LPTSTR,
        sys_func_id: OB_VTABLE_ID,
        proto_style: OB_FUNCPROTO_STYLE,
        member_access: OB_MEMBER_ACCESS,
        is_obsolete: BOOL,
        is_local_decl: BOOL,
        token_id: OB_EVT_TOKEN_ID,
        is_event_external: BOOL,
        throws_list: POB_CLASS_ID,
        no_throws: UINT,
        error: *mut INT
    ) -> OB_PROTO_ID,
    ob_type_proto_ref: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        funcname: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        access_type: OB_MEMBER_ACCESS_TYPE,
        funcargs: *mut POB_ACT_ARG,
        no_args: UINT,
        ret_type: POB_CLASS_ID,
        func_type: POB_FUNC_TYPE,
        dllname: *mut LPTSTR,
        proto_id: POB_PROTO_ID,
        vtable_id: POB_VTABLE_ID,
        error: POB_PROTOREF_ERROR,
        bound_exact_match: BOOL
    ) -> OB_MODULE_ID,
    ob_proto_error_upgrade: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        currerror: OB_PROTOREF_ERROR,
        newerror: OB_PROTOREF_ERROR
    ) -> OB_PROTOREF_ERROR,
    ob_get_proto_access_type: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        curr_group: POB_GROUP,
        curr_class_id: OB_CLASS_ID,
        formal_arg_group: POB_GROUP,
        formal_arg_class_id: OB_CLASS_ID
    ) -> OB_MEMBER_ACCESS_TYPE,
    ob_type_process_protos:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID),
    ob_type_reprocess_protos: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        delete_proto_name: LPTSTR,
        delete_proto_rout_type: OB_ROUT_TYPE,
        delete_proto_args: POB_PROTO_ARG,
        delete_proto_no_args: UINT,
        filter_userprotos: BOOL
    ) -> INT,
    ob_get_type_proto_names: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        rout_type: OB_ROUT_TYPE,
        include_ancestors: BOOL,
        local_protos_only: BOOL,
        nprotos: *mut UINT,
        error: *mut BOOL
    ) -> POB_PROTONAME,
    ob_declare_external_event_type: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        parent_class: OB_CLASS_ID,
        error: *mut INT
    ),
    ob_get_type_proto_names_for_ide: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        rout_type: OB_ROUT_TYPE,
        include_ancestors: BOOL,
        local_protos_only: BOOL,
        nprotos: *mut UINT,
        error: *mut BOOL
    ) -> POB_PROTONAME,
    ob_type_vtable_module_srch: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        vtable_id: OB_VTABLE_ID
    ) -> OB_PROTO_ID,
    ob_get_prototype: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        curr_group: *mut POB_GROUP,
        curr_class_id: POB_CLASS_ID,
        vtable_id: OB_VTABLE_ID
    ) -> POB_PROTOTYPE,
    ob_update_proto_mod_id:
        unsafe extern "stdcall" fn(obthis: POB_THIS, proto_id: OB_PROTO_ID, mod_id: OB_MODULE_ID),
    ob_update_proto_rout_id:
        unsafe extern "stdcall" fn(obthis: POB_THIS, proto_id: OB_PROTO_ID, rout_id: OB_ROUT_ID),
    ob_protolist_read: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_entry: POB_CLASS_ENTRY,
        subpool: OB_SUBPOOL
    ) -> INT,
    ob_protolist_write:
        unsafe extern "stdcall" fn(obthis: POB_THIS, class_entry: POB_CLASS_ENTRY) -> OB_ERROR,
    ob_prototype_match_for_event: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        proto: POB_PROTOTYPE,
        proto_group: POB_GROUP,
        result_type: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT
    ) -> BOOL,
    ob_prototype_search: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        proto_list: POB_PROTOTYPE,
        no_proto_list: UINT,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        proto_group: POB_GROUP,
        result_type: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT,
        error: POB_PROTO_OVERLOAD_ERROR
    ) -> OB_PROTO_ID,
    ob_proto_overload_search: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        type_: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT
    ) -> OB_PROTO_OVERLOAD_ERROR,
    ob_event_module_name: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        mod_id: OB_MODULE_ID
    ) -> LPTSTR,
    ob_find_first_event: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_hndl: POB_CLASS_HNDL,
        event_name: LPTSTR
    ) -> OB_VTABLE_ID,
    ob_type_event_script_srch: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        name: LPTSTR,
        error: *mut INT
    ) -> OB_MODULE_ID,
    ob_build_proto_vtable:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_entry: POB_CLASS_ENTRY) -> INT,
    ob_type_field_decl: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        target_class_id: OB_CLASS_ID,
        name: LPTSTR,
        info: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        class_id: OB_CLASS_ID,
        arrdef: POB_ARRAYDEF,
        dup_field_type: POB_FIELD_TYPE,
        error: *mut INT
    ) -> OB_SYM_ID,
    ob_type_field_search: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        fieldtype: POB_CLASS_ID,
        actual_field_id: POB_SYM_ID
    ) -> OB_SYM_ID,
    ob_type_field_ref: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        name: LPTSTR,
        curr_group: POB_GROUP,
        curr_class_id: OB_CLASS_ID,
        field_type: POB_CLASS_ID,
        grouping: POB_GROUPTYPE,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        rel_field_id: POB_SYM_ID,
        access_check_type: POB_MEMBER_ACCESS_TYPE,
        level: *mut UINT,
        error: *mut INT
    ) -> OB_SYM_ID,
    ob_get_type_field_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        nfields: *mut UINT,
        error: *mut INT,
        filter_fields: BOOL
    ) -> POB_TYPEINFO,
    ob_set_field_init_value: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        value: OB_CONST_REF
    ),
    ob_get_field_init_value: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID
    ) -> POB_DATA,
    ob_type_field_clear_instvars:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID),
    ob_convert_fields_to_const:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> INT,
    ob_build_instance_image:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> INT,
    ob_field_decl_indattr_funcs: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        func_templates: POB_INDATTR_FUNCTMPLT,
        no_template_items: UINT
    ),
    ob_field_get_indattr_funcs: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        no_tmplts: *mut UINT
    ) -> POB_INDATTR_FUNCTMPLT,
    ob_field_requires_update_notification: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID
    ) -> BOOL,
    ob_get_field_symtab: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> POB_LOOK_SYMTAB,
    ob_enum_entry_decl: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        has_val: BOOL,
        value: INT
    ) -> INT,
    ob_enum_decl_process:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID),
    ob_enum_reference: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        enumname: LPTSTR,
        enum_val: *mut INT,
        class_id: POB_CLASS_ID,
        group_id: POB_GROUP_ID
    ) -> INT,
    ob_get_type_enum_info: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        nenums: *mut UINT
    ) -> POB_ENUM_INFO,
    ob_is_type_enum:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> BOOL,
    ob_type_indattr_search: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        no_tmplts: *mut UINT
    ) -> POB_INDATTR_FUNCTMPLT,
    ob_type_decl_indattr_funcs: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        func_templates: POB_INDATTR_FUNCTMPLT,
        no_func_templates: UINT
    ),
    ob_is_an_ancestor: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        of_group: POB_GROUP,
        of_class_id: OB_CLASS_ID,
        ret: *mut INT
    ) -> BOOL,
    ob_is_an_ancestor_excl: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        of_group: POB_GROUP,
        of_class_id: OB_CLASS_ID,
        ret: *mut INT
    ) -> BOOL,
    ob_find_type_ancestor: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group1: POB_GROUP,
        class_id1: OB_CLASS_ID,
        group2: POB_GROUP,
        class_id2: OB_CLASS_ID
    ) -> INT,
    ob_find_common_ancestor: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group1: POB_GROUP,
        class_id1: OB_CLASS_ID,
        group2: *mut POB_GROUP,
        class_id2: OB_CLASS_ID
    ) -> OB_CLASS_ID,
    ob_get_ancestor_system_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> OB_CLASS_ID,
    ob_get_runtime_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> POB_RUNTIME_CLASS,
    ob_get_func_vtable_entry: unsafe extern "stdcall" fn(obinst: OB_INST_ID, offset: ULONG) -> OB_FUNC_FUNC,
    ob_rout_declare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        routname: LPTSTR,
        qual_routname: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        func_type: OB_FUNC_TYPE,
        proto_id: OB_PROTO_ID,
        glob_id: OB_SYM_ID,
        rout_id: POB_ROUT_ID,
        subpool: OB_SUBPOOL,
        clear_routine: BOOL,
        error: *mut INT
    ) -> OB_MODULE_ID,
    ob_open_routine: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        class_entry: POB_CLASS_ENTRY,
        module_id: OB_MODULE_ID
    ) -> POB_ROUTNODE,
    ob_close_routine: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_func_indirect: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_entry: *mut POB_CLASS_ENTRY,
        mod_id: POB_MODULE_ID
    ) -> INT,
    ob_local_var_declare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        error: *mut INT
    ) -> OB_SYM_ID,
    ob_local_array_declare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        arrdef: POB_ARRAYDEF,
        error: *mut INT
    ) -> OB_SYM_ID,
    ob_local_var_reference: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        varname: LPTSTR,
        type_: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> OB_SYM_ID,
    ob_local_set_var: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        var_id: OB_SYM_ID,
        value: OB_CONST_REF
    ),
    ob_local_set_id_var:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID, value: UINT),
    ob_set_const: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        value: *mut ::std::os::raw::c_void,
        item_type: OB_CONPOOL_ITEM_TYPE,
        nitems: UINT,
        len: ULONG
    ) -> OB_CONST_REF,
    ob_get_const: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        const_ref: OB_CONST_REF
    ) -> *mut ::std::os::raw::c_void,
    ob_convert_vars_to_const: unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP) -> INT,
    ob_clear_group_objects: unsafe extern "stdcall" fn(obthis: POB_THIS, pGroup: POB_GROUP) -> BOOL,
    ob_init_group_objects: unsafe extern "stdcall" fn(obthis: POB_THIS, pGroup: POB_GROUP),
    shformatDateTimeWeb: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: PSH_TIME,
        flags: ::std::os::raw::c_int,
        cultureInfo: LPMONTHANDDAYNAMESSTRUCT
    ) -> ::std::os::raw::c_long,
    shformatDateTime: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: PSH_TIME,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    shformatDecimal: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prFmt: LPTSTR,
        value: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    shformatDecimalWeb: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prFmt: LPTSTR,
        value: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long,
    shformatDouble: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: f64,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    shformatDoubleWeb: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: f64,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long,
    shformatLonglong: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: ::std::os::raw::c_longlong,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    shformatLonglongWeb: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: ::std::os::raw::c_longlong,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long,
    shformatReal: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        pValue: *mut f32,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    shformatRealWeb: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        pValue: *mut f32,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long,
    shformatString: unsafe extern "stdcall" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: LPTSTR,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    shformatCmplDateTimeMask: unsafe extern "stdcall" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    shformatCmplDateTimeMaskWeb: unsafe extern "stdcall" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int,
    shformatCmplNumericMask: unsafe extern "stdcall" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    shformatCmplNumericMaskWeb: unsafe extern "stdcall" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int,
    shformatCmplNumericMaskWebCommasPos: unsafe extern "stdcall" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int,
    shformatCmplStringMask: unsafe extern "stdcall" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    shformatErrorString: unsafe extern "stdcall" fn(errMsg: LPTSTR, err: ::std::os::raw::c_int),
    ob_add_glbsym_var: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        class_id: OB_CLASS_ID,
        id: OB_SYM_ID
    ) -> OB_SYM_ID,
    ob_add_glbsym_class: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        refstyle: OB_GLOB_REFSTYLE,
        group_id: OB_GROUP_ID,
        class_id: OB_CLASS_ID,
        sys_class_id: OB_CLASS_ID
    ) -> OB_SYM_ID,
    ob_add_glbsym_func: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        class_id: OB_CLASS_ID,
        mod_id: OB_MODULE_ID
    ) -> OB_SYM_ID,
    rt_set_class_handle:
        unsafe extern "stdcall" fn(rtthis: POB_THIS, appclasshndl: OB_CLASS_HNDL, appinst: OB_INST_ID),
    rt_init: unsafe extern "stdcall" fn(obthis: POB_THIS, stgthis: ppbstg_anchor) -> POB_THIS,
    rt_start_debug: unsafe extern "stdcall" fn(
        rtthis: POB_THIS,
        rtBreakCallback: *mut RT_BREAK_PROC,
        pUserData: *mut ::std::os::raw::c_void
    ) -> INT,
    rt_stop_debug: unsafe extern "stdcall" fn(rtthis: POB_THIS) -> INT,
    rt_set_pcode_to_line: unsafe extern "stdcall" fn(obthis: POB_THIS, line_no: UINT) -> INT,
    rt_breakpoint: unsafe extern "stdcall" fn(
        rtthis: POB_THIS,
        bSet: BOOL,
        obClassHndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        iLineNumber: UINT,
        n_times: UINT,
        condition: LPTSTR,
        id: ::std::os::raw::c_long
    ) -> PRT_BREAKPOINT,
    rt_create_watchpoint: unsafe extern "stdcall" fn(
        rtthis: POB_THIS,
        pdata_info: POB_DATA_INFO,
        watch_type: WATCHPOINT_TYPE,
        item_scope: ::std::os::raw::c_uchar,
        id: ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void,
    rt_find_watchpoint_for_watchid: unsafe extern "stdcall" fn(
        rtthis: POB_THIS,
        watchId: ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void,
    rt_delete_watchpoint: unsafe extern "stdcall" fn(rtthis: POB_THIS, watchpt: *mut ::std::os::raw::c_void),
    rt_is_line_executable: unsafe extern "stdcall" fn(
        rtthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        line_no: UINT
    ) -> BOOL,
    rt_closest_executable_line: unsafe extern "stdcall" fn(
        rtthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        line_no: UINT
    ) -> UINT,
    rt_start_run: unsafe extern "stdcall" fn(rtthis: POB_THIS) -> INT,
    rt_stop_run: unsafe extern "stdcall" fn(rtthis: POB_THIS) -> INT,
    rt_create_obinst: unsafe extern "stdcall" fn(rtthis: POB_THIS, name: LPTSTR, obinst: POB_INST_ID) -> INT,
    rtReturnValGet: unsafe extern "stdcall" fn(rtThis: POB_THIS) -> POB_DATA,
    rtReturnValFree: unsafe extern "stdcall" fn(rtThis: POB_THIS),
    rt_error: unsafe extern "stdcall" fn(rtthis: POB_THIS, iMessageID: INT) -> INT,
    rt_free_error_struct: unsafe extern "stdcall" fn(rtthis: POB_THIS, error_struct: PRT_ERROR_STRUCT),
    rt_error_using_struct: unsafe extern "stdcall" fn(
        rtthis: POB_THIS,
        error_struct: PRT_ERROR_STRUCT,
        exceptionClassName: LPTSTR
    ) -> INT,
    rt_normalize_error_id: unsafe extern "stdcall" fn(obthis: POB_THIS, iMessageID: INT) -> INT,
    ot_handle_exception: unsafe extern "stdcall" fn(
        rtthis: POB_THIS,
        pException_Stack: *mut ::std::os::raw::c_void,
        currDepth: USHORT
    ) -> INT,
    ob_dbg_pop_call_stack_ntimes: unsafe extern "stdcall" fn(obthis: POB_THIS, n: UINT) -> INT,
    ob_dbg_push_call_stack_ntimes: unsafe extern "stdcall" fn(obthis: POB_THIS, n: UINT) -> INT,
    ob_get_current_stack_location: unsafe extern "stdcall" fn(obthis: POB_THIS) -> PRT_BREAKPOINT,
    rtRoutineSearch: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pchRoutineName: LPTSTR,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineType: OB_ROUT_TYPE,
        pobRoutineId: POB_VTABLE_ID
    ) -> INT,
    rtRoutineExec: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineId: OB_VTABLE_ID,
        obRoutineType: OB_ROUT_TYPE,
        bConvert: BOOL
    ) -> RT_EXEC_STATUS,
    rtRoutineExecByName: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        pchRoutineName: LPTSTR,
        rtCallInfo: RT_CALL_INFO,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineType: OB_ROUT_TYPE,
        bConvert: BOOL
    ) -> RT_EXEC_STATUS,
    rtRoutineExecPosted: unsafe extern "stdcall" fn(pData: *mut ::std::os::raw::c_void) -> RT_EXEC_STATUS,
    rtRoutineInfo: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        obRoutineId: OB_VTABLE_ID,
        pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO
    ) -> INT,
    rtInitializeInfoForCall:
        unsafe extern "stdcall" fn(obThis: POB_THIS, pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO) -> INT,
    rtCleanupInfoAfterCall:
        unsafe extern "stdcall" fn(obThis: POB_THIS, pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO) -> INT,
    rtRoutineCount: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pusRoutineTotal: *mut USHORT,
        pusFuncTotal: *mut USHORT,
        pusEventTotal: *mut USHORT
    ) -> INT,
    rtReferenceArgCreate: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        pobdRefArg: POB_DATA,
        prtRefArgInfo: PRT_REFARG_INFO
    ) -> INT,
    rtReferenceArgFree: unsafe extern "stdcall" fn(obThis: POB_THIS, pobdRefArg: POB_DATA) -> INT,
    rtGetClassDescrip: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        obClassHndl: OB_CLASS_HNDL,
        prtClassDescrip: PRT_CLASS_DESCRIP,
        pobClassIdSystem: POB_CLASS_ID
    ) -> INT,
    rtDataFree: unsafe extern "stdcall" fn(pobThis: POB_THIS, pobdVal: POB_DATA),
    rtDataCopy: unsafe extern "stdcall" fn(
        pobThis: POB_THIS,
        pobdDest: POB_DATA,
        pobdSrc: POB_DATA,
        AddReference: BOOL
    ),
    rt_hit_level_0: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_create_object: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        p_group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> INT,
    ob_create_object_using: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        context: POB_RUNTIME_INST,
        class_name: LPTSTR
    ) -> HRESULT,
    ob_copy_rtinst:
        unsafe extern "stdcall" fn(obthis: POB_THIS, from_rtinst: POB_RUNTIME_INST) -> POB_RUNTIME_INST,
    ob_destroy_rtinst: unsafe extern "stdcall" fn(obthis: POB_THIS, rtinst: POB_RUNTIME_INST) -> INT,
    ob_get_primary_rtinst:
        unsafe extern "stdcall" fn(obthis: POB_THIS, rtinst: POB_RUNTIME_INST) -> POB_RUNTIME_INST,
    ob_is_rtinst_autoinstantiate:
        unsafe extern "stdcall" fn(obthis: POB_THIS, rtinst: POB_RUNTIME_INST) -> BOOL,
    ob_object_compare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        rtinst1: POB_RUNTIME_INST,
        rtinst2: POB_RUNTIME_INST
    ) -> BOOL,
    ob_invoke_static: unsafe extern "stdcall" fn(
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        numArgs: USHORT,
        args: POB_DATA,
        result: POB_DATA
    ) -> HRESULT,
    ob_invoke_dynamic: unsafe extern "stdcall" fn(
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        routType: OB_ROUT_TYPE,
        name: LPTSTR,
        numArgs: USHORT,
        args: POB_DATA,
        result: POB_DATA
    ) -> HRESULT,
    ob_invoke_staticAsync: unsafe extern "stdcall" fn(
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        numArgs: USHORT,
        args: POB_DATA
    ) -> HRESULT,
    ob_invoke_dynamicAsync: unsafe extern "stdcall" fn(
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        routType: OB_ROUT_TYPE,
        name: LPTSTR,
        numArgs: USHORT,
        args: POB_DATA
    ) -> HRESULT,
    ob_instance_lv: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        current_inst: POB_RUNTIME_INST,
        var_id: OB_SYM_ID
    ) -> POB_DATA,
    ob_instance_fldupdate_refpkt: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        var_id: OB_SYM_ID
    ),
    ob_instance_flditemupdate_refpkt: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        group_id: OB_GROUP_ID,
        var_id: OB_SYM_ID,
        lvalue: POB_DATA,
        item_index: ULONG
    ),
    ob_instance_simple_refpkt: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        var_id: OB_SYM_ID
    ),
    ob_get_group_load_state:
        unsafe extern "stdcall" fn(pGroupReference: *mut ::std::os::raw::c_void) -> OB_GROUP_LOAD_STATE,
    ob_get_groupref_group:
        unsafe extern "stdcall" fn(pGroupReference: *mut ::std::os::raw::c_void) -> POB_GROUP,
    ob_group_get_next_index: unsafe extern "stdcall" fn(obthis: POB_THIS) -> ULONG,
    ob_close_typedef_group: unsafe extern "stdcall" fn(obThis: POB_THIS),
    ob_create_group_structure:
        unsafe extern "stdcall" fn(obThis: POB_THIS, lpszGroupName: LPTSTR) -> POB_GROUP,
    ob_new_group: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        qual_group_name: LPTSTR,
        group_lock_state: OB_GROUP_LOCK_STATE,
        group_load_state: OB_GROUP_LOAD_STATE
    ) -> POB_GROUP,
    ob_del_group_structure: unsafe extern "stdcall" fn(obThis: POB_THIS, pGroup: POB_GROUP),
    ob_group_data_srch:
        unsafe extern "stdcall" fn(obThis: POB_THIS, obGroupHandle: OB_GROUP_HNDL) -> POB_GROUP,
    ob_replace_group:
        unsafe extern "stdcall" fn(obThis: POB_THIS, obGroupID: OB_GROUP_ID, pNewGroup: POB_GROUP),
    ob_copy_group_shrsym_data: unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP),
    ob_get_qualified_name_with_namespace:
        unsafe extern "stdcall" fn(obThis: POB_THIS, pGroup: POB_GROUP, lpszNamespace: LPTSTR) -> LPTSTR,
    ob_get_source_from_group: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        src_type: *mut POB_SOURCE_BLK_TYPE,
        no_blocks: *mut UINT,
        subpool: OB_SUBPOOL,
        ppSrcLastEdit: *mut POB_SRC_LAST_EDIT,
        pNoSrcLastEdit: *mut UINT
    ) -> *mut LPTSTR,
    ob_get_var: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        look_symtab: POB_LOOK_SYMTAB,
        var_id: OB_SYM_ID
    ) -> POB_DATA,
    ob_init_var_data: unsafe extern "stdcall" fn(obthis: POB_THIS, var_data: POB_DATA, group: POB_GROUP),
    ob_global_indirect:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: *mut POB_GROUP, glob_id: POB_SYM_ID) -> POB_DATA,
    ob_global_var_declare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        error: *mut INT
    ) -> OB_SYM_ID,
    ob_global_array_declare: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        var: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        arrdef: POB_ARRAYDEF,
        error: *mut INT
    ) -> OB_SYM_ID,
    ob_shared_var_reference: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        type_: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> OB_SYM_ID,
    ob_global_set_var: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        var_id: OB_SYM_ID,
        value: OB_CONST_REF
    ),
    ob_global_set_id_var:
        unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID, value: UINT),
    ob_get_local_symtab: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        var_id: POB_SYM_ID
    ) -> POB_LOOK_SYMTAB,
    ob_get_unconverted_var: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        var: OB_SYM_ID,
        level: UINT
    ) -> POB_DATA,
    ob_lookup_shared_var_info: unsafe extern "stdcall" fn(
        obThis: POB_THIS,
        iGroupID: OB_GROUP_ID,
        iSymbolID: OB_SYM_ID,
        pType: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> INT,
    ob_clear_shared_vars: unsafe extern "stdcall" fn(obthis: POB_THIS, group: POB_GROUP, level: INT),
    ot_eval_expr: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        pcode_blk: POB_PCODE_BLK,
        expr_result_buf: POB_DATA
    ) -> POB_DATA,
    ot_dbg_funccall: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        call_label: LPTSTR,
        group: POB_GROUP,
        class_entry: OB_CLASS_ID,
        name: LPTSTR
    ),
    ot_run_dllfunccall: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        funcname: LPTSTR,
        evaled_arglist: POB_DATA,
        no_args: UINT,
        funcproto: POB_PROTOTYPE
    ) -> INT,
    ot_run_rpcfunccall: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        rtinst: POB_RUNTIME_INST,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        funcname: LPTSTR,
        evaled_arglist: POB_DATA,
        no_args: UINT,
        funcproto: POB_PROTOTYPE,
        rpc_funcname: LPTSTR
    ) -> INT,
    ot_get_dll_funcptr_by_name:
        unsafe extern "stdcall" fn(obthis: POB_THIS, dllname: LPTSTR, funcname: LPTSTR) -> OS_CALLC_FUNC,
    ot_post_call: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        pRuntimeClass: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        uiNoArgs: UINT,
        args: POB_DATA
    ) -> INT,
    ot_check_types: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group1: POB_GROUP,
        type1: OB_CLASS_ID,
        grouping1: OB_GROUPTYPE,
        group2: POB_GROUP,
        type2: OB_CLASS_ID,
        grouping2: OB_GROUPTYPE,
        ancestor_flag: *mut UINT
    ) -> OT_TYPE_CHECK_ERROR,
    ot_type_loc: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> OT_TYPE_LOC,
    ot_init_data_node: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        data_node: POB_DATA,
        type_: OB_CLASS_ID,
        varinfo: OB_INFO_FLAGS
    ),
    ot_set_lvalue: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_id: OB_GROUP_ID,
        lvalue_data: POB_DATA,
        rvalue_data: POB_DATA,
        do_error_check: BOOL
    ) -> INT,
    ot_free_out_node: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA),
    ot_free_inv_meth_args:
        unsafe extern "stdcall" fn(obthis: POB_THIS, pArrayDataNode: POB_DATA, pFreeFlags: LPTSTR) -> INT,
    ot_copy_array:
        unsafe extern "stdcall" fn(obthis: POB_THIS, old_array_inst: POB_ARRAY_INST) -> POB_ARRAY_INST,
    ot_get_string_from_chararray:
        unsafe extern "stdcall" fn(obthis: POB_THIS, arrayinst: POB_ARRAY_INST) -> LPTSTR,
    ot_create_chararray_from_string: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        string_val: LPTSTR,
        target_data_node: POB_DATA
    ) -> POB_DATA,
    ot_create_bounded_chararray_from_string: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        string_val: LPTSTR,
        bounds: *mut ::std::os::raw::c_long,
        target_data_node: POB_DATA
    ) -> POB_DATA,
    ot_get_char_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> TCHAR,
    ot_get_string_value: unsafe extern "stdcall" fn(obthis: POB_THIS, data_node: POB_DATA) -> LPTSTR,
    ot_get_string_from_char: unsafe extern "stdcall" fn(obthis: POB_THIS, char_val: TCHAR) -> LPTSTR,
    ot_string_cat: unsafe extern "stdcall" fn(obthis: POB_THIS, string1: LPTSTR, string2: LPTSTR) -> LPTSTR,
    ot_binary_cat:
        unsafe extern "stdcall" fn(obthis: POB_THIS, bin1: PSH_BINARY, bin2: PSH_BINARY) -> PSH_BINARY,
    ot_halt: unsafe extern "stdcall" fn(obthis: POB_THIS, send_close_event: BOOL) -> INT,
    ot_convert_bounded_to_bounded: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long,
        free_old_array: BOOL
    ) -> POB_ARRAY_INST,
    ot_convert_bounded_to_unbounded: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST,
    ot_convert_unbounded_to_bounded: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long
    ) -> POB_ARRAY_INST,
    ot_convert_unbounded_to_unbounded: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST,
    ot_convert_any_to_bounded: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        any_node: POB_DATA,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long
    ) -> POB_ARRAY_INST,
    ot_convert_any_to_unbounded: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        any_node: POB_DATA,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST,
    ot_convert_array_to_object: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        any_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_RUNTIME_INST,
    ot_build_simple_refpak: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        group_id: OB_GROUP_ID
    ) -> POT_REF_PAK,
    ot_build_field_refpak: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        group_id: OB_GROUP_ID,
        rtinst: POB_RUNTIME_INST,
        field_id: UINT,
        item_index: ULONG,
        bTriggerFieldUpdate: BOOL
    ) -> POT_REF_PAK,
    ot_add_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_sub_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_mul_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_div_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_pow_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_neg_any: unsafe extern "stdcall" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT,
    ot_eq_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_ne_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_gt_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_lt_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_ge_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_le_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_and_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_or_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_not_any: unsafe extern "stdcall" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny: POB_DATA) -> INT,
    ot_incr_any: unsafe extern "stdcall" fn(prtThis: POB_THIS, pAny: POB_DATA) -> INT,
    ot_decr_any: unsafe extern "stdcall" fn(prtThis: POB_THIS, pAny: POB_DATA) -> INT,
    ot_mod_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_min_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_max_any: unsafe extern "stdcall" fn(
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT,
    ot_check_any_exact_type:
        unsafe extern "stdcall" fn(obthis: POB_THIS, any_var: POB_DATA, expected_type: OB_CLASS_ID) -> INT,
    ot_check_any_string_type:
        unsafe extern "stdcall" fn(obthis: POB_THIS, any_var: POB_DATA, expected_type: OB_CLASS_ID) -> INT,
    ot_check_any_binary_type:
        unsafe extern "stdcall" fn(obthis: POB_THIS, any_var: POB_DATA, expected_type: OB_CLASS_ID) -> INT,
    ot_check_any_math_type:
        unsafe extern "stdcall" fn(obthis: POB_THIS, any_var: POB_DATA, expected_type: OB_CLASS_ID) -> INT,
    ot_check_any_enum_type: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        any_var: POB_DATA,
        current_group: POB_GROUP,
        expected_type: OB_CLASS_ID
    ) -> INT,
    ot_check_any_object_type: unsafe extern "stdcall" fn(
        obthis: POB_THIS,
        any_var: POB_DATA,
        current_group: POB_GROUP,
        expected_type: OB_CLASS_ID
    ) -> INT,
    ot_duplicate_any: unsafe extern "stdcall" fn(pobThis: POB_THIS, pAny: POB_DATA) -> INT,
    ot_abs_any: unsafe extern "stdcall" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT,
    ot_ceiling_any: unsafe extern "stdcall" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT,
    ot_string_to_binary: unsafe extern "stdcall" fn(
        rtThis: POB_THIS,
        lpStr: LPTSTR,
        EncodingType: ::std::os::raw::c_int,
        bNullTerminated: BOOL
    ) -> PSH_BINARY,
    ot_bytearray_to_binary:
        unsafe extern "stdcall" fn(rtThis: POB_THIS, array_inst: POB_ARRAY_INST) -> PSH_BINARY,
    ot_any_to_binary: unsafe extern "stdcall" fn(rtThis: POB_THIS, obData: POB_DATA) -> PSH_BINARY,
    ob_set_curr_rtinst_and_return: unsafe extern "stdcall" fn(obthis: POB_THIS, new_rtinst: POB_RUNTIME_INST),
    ob_unset_curr_rtinst_and_return: unsafe extern "stdcall" fn(obthis: POB_THIS),
    ob_open_trace:
        unsafe extern "stdcall" fn(obthis: POB_THIS, filename: LPTSTR, kind: OB_TIMERKIND) -> OB_ERROR_RETURN,
    ob_close_trace: unsafe extern "stdcall" fn(obthis: POB_THIS) -> OB_ERROR_RETURN,
    ob_begin_trace: unsafe extern "stdcall" fn(obthis: POB_THIS, message: LPTSTR) -> OB_ERROR_RETURN,
    ob_end_trace: unsafe extern "stdcall" fn(obthis: POB_THIS) -> OB_ERROR_RETURN,
    ob_enable_event_trace: unsafe extern "stdcall" fn(obthis: POB_THIS, event: OB_TRACEID) -> OB_ERROR_RETURN,
    ob_disable_event_trace:
        unsafe extern "stdcall" fn(obthis: POB_THIS, event: OB_TRACEID) -> OB_ERROR_RETURN
}
impl Api {
    pub unsafe fn load() -> Result<Self, PBLibraryError> {
        let __library = PBLibrary::load()?;
        let __version = __library.version();
        let pbstg_begin = __library.get(b"pbstg_begin\0").map(|sym| *sym)?;
        let pbstg_begin_allocflags = __library.get(b"pbstg_begin_allocflags\0").map(|sym| *sym)?;
        let pbstg_begin_nofast = __library.get(b"pbstg_begin_nofast\0").map(|sym| *sym)?;
        let pbstg_end = __library.get(b"pbstg_end\0").map(|sym| *sym)?;
        let pbstg_free_pool = __library.get(b"pbstg_free_pool\0").map(|sym| *sym)?;
        let pbstg_new_pool = __library.get(b"pbstg_new_pool\0").map(|sym| *sym)?;
        let pbstg_new_pool_nofast = __library.get(b"pbstg_new_pool_nofast\0").map(|sym| *sym)?;
        let pbstg_new_pool_with_size_nofast =
            __library.get(b"pbstg_new_pool_with_size_nofast\0").map(|sym| *sym)?;
        let pbstg_set_pool_name = __library.get(b"pbstg_set_pool_name\0").map(|sym| *sym)?;
        let pbstg_set_poolpagesize = __library.get(b"pbstg_set_poolpagesize\0").map(|sym| *sym)?;
        let pbstg_write_debug = __library.get(b"pbstg_write_debug\0").map(|sym| *sym)?;
        let pbstg_stat = __library.get(b"pbstg_stat\0").map(|sym| *sym)?;
        let pbstg_nextGeneration = __library.get(b"pbstg_nextGeneration\0").map(|sym| *sym)?;
        let pbstg_dumpLeaks = __library.get(b"pbstg_dumpLeaks\0").map(|sym| *sym)?;
        let pbstg_dumpHeap = __library.get(b"pbstg_dumpHeap\0").map(|sym| *sym)?;
        let pbstg_alloc = __library.get(b"pbstg_alc\0").map(|sym| *sym)?;
        let pbstg_free = __library.get(b"pbstg_fee\0").map(|sym| *sym)?;
        let pbstg_realloc = __library.get(b"pbstg_realc\0").map(|sym| *sym)?;
        let pbstg_size = __library.get(b"pbstg_sz\0").map(|sym| *sym)?;
        let pbstg_fast_strlen = __library.get(b"pbstg_fast_strlen\0").map(|sym| *sym)?;
        let pbstg_ansitoupper = __library.get(b"pbstg_ansitoupper\0").map(|sym| *sym)?;
        let pbstg_ansitolower = __library.get(b"pbstg_ansitolower\0").map(|sym| *sym)?;
        let pbstg_strdup = __library.get(b"pbstg_strdup\0").map(|sym| *sym)?;
        let pbstg_strdup_malloc = __library.get(b"pbstg_strdup_malloc\0").map(|sym| *sym)?;
        let pbstg_str_build = __library.get(b"pbstg_str_build\0").map(|sym| *sym)?;
        let pbstg_str_build_char = __library.get(b"pbstg_str_build_char\0").map(|sym| *sym)?;
        let pbstg_str_build_huge = __library.get(b"pbstg_str_build_huge\0").map(|sym| *sym)?;
        let pbstg_str_remove_char = __library.get(b"pbstg_str_remove_char\0").map(|sym| *sym)?;
        let pbstg_str_trim_left = __library.get(b"pbstg_str_trim_left\0").map(|sym| *sym)?;
        let pbstg_str_trim_right = __library.get(b"pbstg_str_trim_right\0").map(|sym| *sym)?;
        let pbstg_str_trim = __library.get(b"pbstg_str_trim\0").map(|sym| *sym)?;
        let pbstg_str_wordcap = __library.get(b"pbstg_str_wordcap\0").map(|sym| *sym)?;
        let pbstg_atoi_imp = __library.get(b"pbstg_atoi_imp\0").map(|sym| *sym)?;
        let pbstg_atof_imp = __library.get(b"pbstg_atof_imp\0").map(|sym| *sym)?;
        let pbstg_strtod_imp = __library.get(b"pbstg_strtod_imp\0").map(|sym| *sym)?;
        let pbstg_atol_imp = __library.get(b"pbstg_atol_imp\0").map(|sym| *sym)?;
        let pbstg_strtol_imp = __library.get(b"pbstg_strtol_imp\0").map(|sym| *sym)?;
        let pbstg_atou_imp = __library.get(b"pbstg_atou_imp\0").map(|sym| *sym)?;
        let pbstg_atoul_imp = __library.get(b"pbstg_atoul_imp\0").map(|sym| *sym)?;
        let pbstg_strtoul_imp = __library.get(b"pbstg_strtoul_imp\0").map(|sym| *sym)?;
        let pbstg_remove_imp = __library.get(b"pbstg_remove_imp\0").map(|sym| *sym)?;
        let pbstg_dde_alloc = __library.get(b"pbstg_dde_alloc\0").map(|sym| *sym)?;
        let pbstg_dde_free = __library.get(b"pbstg_dde_free\0").map(|sym| *sym)?;
        let pbstg_dde_get_handle = __library.get(b"pbstg_dde_get_handle\0").map(|sym| *sym)?;
        let pbstg_dde_lock = __library.get(b"pbstg_dde_lock\0").map(|sym| *sym)?;
        let pbstg_dde_unlock = __library.get(b"pbstg_dde_unlock\0").map(|sym| *sym)?;
        let pbstg_huge_memcmp = __library.get(b"pbstg_huge_memcmp\0").map(|sym| *sym)?;
        let pbstg_huge_memcpy = __library.get(b"pbstg_huge_memcpy\0").map(|sym| *sym)?;
        let pbstg_huge_memmove = __library.get(b"pbstg_huge_memmove\0").map(|sym| *sym)?;
        let pbstg_huge_memset = __library.get(b"pbstg_huge_memset\0").map(|sym| *sym)?;
        let pbstg_huge_strchr = __library.get(b"pbstg_huge_strchr\0").map(|sym| *sym)?;
        let pbstg_huge_strcpy = __library.get(b"pbstg_huge_strcpy\0").map(|sym| *sym)?;
        let pbstg_huge_strlen = __library.get(b"pbstg_huge_strlen\0").map(|sym| *sym)?;
        let pbstg_huge_strncpy = __library.get(b"pbstg_huge_strncpy\0").map(|sym| *sym)?;
        let pbstg_huge_strstr = __library.get(b"pbstg_huge_strstr\0").map(|sym| *sym)?;
        let pbstg_unicodestrdup = __library.get(b"pbstg_unicodestrdup\0").map(|sym| *sym)?;
        let pbstg_unicodestr_build = __library.get(b"pbstg_unicodestr_build\0").map(|sym| *sym)?;
        let pbstg_strtounicodedup = __library.get(b"pbstg_strtounicodedup\0").map(|sym| *sym)?;
        let pbstg_unicodetostrdup = __library.get(b"pbstg_unicodetostrdup\0").map(|sym| *sym)?;
        let pbstg_strtoansidup = __library.get(b"pbstg_strtoansidup\0").map(|sym| *sym)?;
        let pbstg_ansitostrdup = __library.get(b"pbstg_ansitostrdup\0").map(|sym| *sym)?;
        let pbstg_strtoprintable = __library.get(b"pbstg_strtoprintable\0").map(|sym| *sym)?;
        let pbstg_strtoprintabledup = __library.get(b"pbstg_strtoprintabledup\0").map(|sym| *sym)?;
        let pbstg_printabletostr = __library.get(b"pbstg_printabletostr\0").map(|sym| *sym)?;
        let pbstg_printabletostrdup = __library.get(b"pbstg_printabletostrdup\0").map(|sym| *sym)?;
        let pbstg_lchrcmp = __library.get(b"pbstg_lchrcmp\0").map(|sym| *sym)?;
        let pbstg_lchrcmpi = __library.get(b"pbstg_lchrcmpi\0").map(|sym| *sym)?;
        let ob_set_session_icontext = __library.get(b"ob_set_session_icontext\0").map(|sym| *sym)?;
        let rt_move_thread = __library.get(b"rt_move_thread\0").map(|sym| *sym)?;
        let rt_clear_thread = __library.get(b"rt_clear_thread\0").map(|sym| *sym)?;
        let rt_get_current_this = __library.get(b"rt_get_current_this\0").map(|sym| *sym)?;
        let rt_add_task = __library.get(b"rt_add_task\0").map(|sym| *sym)?;
        let rt_free_task = __library.get(b"rt_free_task\0").map(|sym| *sym)?;
        let rt_get_current_task_info = __library.get(b"rt_get_current_task_info\0").map(|sym| *sym)?;
        let rt_set_current_task_info = __library.get(b"rt_set_current_task_info\0").map(|sym| *sym)?;
        let rt_get_free_task_slot = __library.get(b"rt_get_free_task_slot\0").map(|sym| *sym)?;
        let rt_is_running_exe = __library.get(b"rt_is_running_exe\0").map(|sym| *sym)?;
        let ob_add_const_data = __library.get(b"ob_add_const_data\0").map(|sym| *sym)?;
        let ob_looksym_keyfunc = __library.get(b"ob_looksym_keyfunc\0").map(|sym| *sym)?;
        let ob_looksym_reference = __library.get(b"ob_looksym_reference\0").map(|sym| *sym)?;
        let ob_looksym_delete = __library.get(b"ob_looksym_delete\0").map(|sym| *sym)?;
        let ob_dynarray_index = __library.get(b"ob_dynarray_index\0").map(|sym| *sym)?;
        let ob_dynarray_grow = __library.get(b"ob_dynarray_grow\0").map(|sym| *sym)?;
        let ob_narray_create_static = __library.get(b"ob_narray_create_static\0").map(|sym| *sym)?;
        let ob_narray_create_dynamic = __library.get(b"ob_narray_create_dynamic\0").map(|sym| *sym)?;
        let ob_set_arraydef = __library.get(b"ob_set_arraydef\0").map(|sym| *sym)?;
        let ob_get_array_len = __library.get(b"ob_get_array_len\0").map(|sym| *sym)?;
        let ob_array_item_init_callback = __library.get(b"ob_array_item_init_callback\0").map(|sym| *sym)?;
        let ob_init_array = __library.get(b"ob_init_array\0").map(|sym| *sym)?;
        let ob_array_varinfo_nullval = __library.get(b"ob_array_varinfo_nullval\0").map(|sym| *sym)?;
        let ob_array_set_varinfo_nullval =
            __library.get(b"ob_array_set_varinfo_nullval\0").map(|sym| *sym)?;
        let ob_remove_array_data = __library.get(b"ob_remove_array_data\0").map(|sym| *sym)?;
        let ob_init_pcode_blk = __library.get(b"ob_init_pcode_blk\0").map(|sym| *sym)?;
        let ob_del_pcode_blk = __library.get(b"ob_del_pcode_blk\0").map(|sym| *sym)?;
        let ob_reuse_routine = __library.get(b"ob_reuse_routine\0").map(|sym| *sym)?;
        let shMaxDec = __library.get(b"shMaxDec\0").map(|sym| *sym)?;
        let shMinDec = __library.get(b"shMinDec\0").map(|sym| *sym)?;
        let shCompareDec = __library.get(b"shCompareDec\0").map(|sym| *sym)?;
        let shAbsDec = __library.get(b"shAbsDec\0").map(|sym| *sym)?;
        let shNegateDec = __library.get(b"shNegateDec\0").map(|sym| *sym)?;
        let shRoundDec = __library.get(b"shRoundDec\0").map(|sym| *sym)?;
        let shTruncDec = __library.get(b"shTruncDec\0").map(|sym| *sym)?;
        let shAddDec = __library.get(b"shAddDec\0").map(|sym| *sym)?;
        let shSubDec = __library.get(b"shSubDec\0").map(|sym| *sym)?;
        let shMultDec = __library.get(b"shMultDec\0").map(|sym| *sym)?;
        let shDivDec = __library.get(b"shDivDec\0").map(|sym| *sym)?;
        let shModDec = __library.get(b"shModDec\0").map(|sym| *sym)?;
        let shExpDec = __library.get(b"shExpDec\0").map(|sym| *sym)?;
        let shIntToDec = __library.get(b"shIntToDec\0").map(|sym| *sym)?;
        let shDecToInt = __library.get(b"shDecToInt\0").map(|sym| *sym)?;
        let shUintToDec = __library.get(b"shUintToDec\0").map(|sym| *sym)?;
        let shDecToUint = __library.get(b"shDecToUint\0").map(|sym| *sym)?;
        let shByteToDec = __library.get(b"shByteToDec\0").map(|sym| *sym)?;
        let shDecToByte = __library.get(b"shDecToByte\0").map(|sym| *sym)?;
        let shLongToDec = __library.get(b"shLongToDec\0").map(|sym| *sym)?;
        let shDecToLong = __library.get(b"shDecToLong\0").map(|sym| *sym)?;
        let shUlongToDec = __library.get(b"shUlongToDec\0").map(|sym| *sym)?;
        let shDecToUlong = __library.get(b"shDecToUlong\0").map(|sym| *sym)?;
        let shLonglongToDec = __library.get(b"shLonglongToDec\0").map(|sym| *sym)?;
        let shDecToLonglong = __library.get(b"shDecToLonglong\0").map(|sym| *sym)?;
        let shDecToFloat = __library.get(b"shDecToFloat\0").map(|sym| *sym)?;
        let shFloatToDec = __library.get(b"shFloatToDec\0").map(|sym| *sym)?;
        let shDoubleToDec = __library.get(b"shDoubleToDec\0").map(|sym| *sym)?;
        let shDecToDouble = __library.get(b"shDecToDouble\0").map(|sym| *sym)?;
        let shDecToAscii = __library.get(b"shDecToAscii\0").map(|sym| *sym)?;
        let shAsciiToDec = __library.get(b"shAsciiToDec\0").map(|sym| *sym)?;
        let shAsciiToDecRnd = __library.get(b"shAsciiToDecRnd\0").map(|sym| *sym)?;
        let shSetDecFractions = __library.get(b"shSetDecFractions\0").map(|sym| *sym)?;
        let shSetDecNegative = __library.get(b"shSetDecNegative\0").map(|sym| *sym)?;
        let shDecSetOverflow = __library.get(b"shDecSetOverflow\0").map(|sym| *sym)?;
        let ob_mgr_init = __library.get(b"ob_mgr_init\0").map(|sym| *sym)?;
        let ob_mgr_init_ex = __library.get(b"ob_mgr_init_ex\0").map(|sym| *sym)?;
        let ob_mgr_restart = __library.get(b"ob_mgr_restart\0").map(|sym| *sym)?;
        let ob_mgr_terminate = __library.get(b"ob_mgr_terminate\0").map(|sym| *sym)?;
        let ob_free_memory = __library.get(b"ob_free_memory\0").map(|sym| *sym)?;
        let ob_free_link_error_list = __library.get(b"ob_free_link_error_list\0").map(|sym| *sym)?;
        let ob_get_link_error_list = __library.get(b"ob_get_link_error_list\0").map(|sym| *sym)?;
        let ob_enter_critical_section = __library.get(b"ob_enter_critical_section\0").map(|sym| *sym)?;
        let ob_leave_critical_section = __library.get(b"ob_leave_critical_section\0").map(|sym| *sym)?;
        let ob_alloc_string = __library.get(b"ob_alloc_string\0").map(|sym| *sym)?;
        let ob_alloc_blob = __library.get(b"ob_alloc_blob\0").map(|sym| *sym)?;
        let ob_alloc_dec = __library.get(b"ob_alloc_dec\0").map(|sym| *sym)?;
        let ob_alloc_double = __library.get(b"ob_alloc_double\0").map(|sym| *sym)?;
        let ob_alloc_longlong = __library.get(b"ob_alloc_longlong\0").map(|sym| *sym)?;
        let ob_alloc_time = __library.get(b"ob_alloc_time\0").map(|sym| *sym)?;
        let ob_realloc_string = __library.get(b"ob_realloc_string\0").map(|sym| *sym)?;
        let ob_realloc_blob = __library.get(b"ob_realloc_blob\0").map(|sym| *sym)?;
        let ob_dup_string = __library.get(b"ob_dup_string\0").map(|sym| *sym)?;
        let ob_dup_blob = __library.get(b"ob_dup_blob\0").map(|sym| *sym)?;
        let ob_dup_dec = __library.get(b"ob_dup_dec\0").map(|sym| *sym)?;
        let ob_dup_double = __library.get(b"ob_dup_double\0").map(|sym| *sym)?;
        let ob_dup_longlong = __library.get(b"ob_dup_longlong\0").map(|sym| *sym)?;
        let ob_dup_time = __library.get(b"ob_dup_time\0").map(|sym| *sym)?;
        let ob_free_value = __library.get(b"ob_free_value\0").map(|sym| *sym)?;
        let ob_create_appl_report = __library.get(b"ob_create_appl_report\0").map(|sym| *sym)?;
        let ob_create_object_report = __library.get(b"ob_create_object_report\0").map(|sym| *sym)?;
        let ob_free_appl_report = __library.get(b"ob_free_appl_report\0").map(|sym| *sym)?;
        let ob_get_mode = __library.get(b"ob_get_mode\0").map(|sym| *sym)?;
        let ob_set_mode = __library.get(b"ob_set_mode\0").map(|sym| *sym)?;
        let ob_get_field = __library.get(b"ob_get_field\0").map(|sym| *sym)?;
        let ob_set_field = __library.get(b"ob_set_field\0").map(|sym| *sym)?;
        let ob_get_field_data = __library.get(b"ob_get_field_data\0").map(|sym| *sym)?;
        let ob_get_no_fields = __library.get(b"ob_get_no_fields\0").map(|sym| *sym)?;
        let ob_get_parent_obinst = __library.get(b"ob_get_parent_obinst\0").map(|sym| *sym)?;
        let ob_get_first_user_field = __library.get(b"ob_get_first_user_field\0").map(|sym| *sym)?;
        let ob_get_field_type = __library.get(b"ob_get_field_type\0").map(|sym| *sym)?;
        let ob_get_int_field = __library.get(b"ob_get_int_field\0").map(|sym| *sym)?;
        let ob_get_uint_field = __library.get(b"ob_get_uint_field\0").map(|sym| *sym)?;
        let ob_get_byte_field = __library.get(b"ob_get_byte_field\0").map(|sym| *sym)?;
        let ob_get_long_field = __library.get(b"ob_get_long_field\0").map(|sym| *sym)?;
        let ob_get_ulong_field = __library.get(b"ob_get_ulong_field\0").map(|sym| *sym)?;
        let ob_get_float_field = __library.get(b"ob_get_float_field\0").map(|sym| *sym)?;
        let ob_get_ptr_field = __library.get(b"ob_get_ptr_field\0").map(|sym| *sym)?;
        let ob_get_inst_field = __library.get(b"ob_get_inst_field\0").map(|sym| *sym)?;
        let ob_get_array_field = __library.get(b"ob_get_array_field\0").map(|sym| *sym)?;
        let ob_array_index = __library.get(b"ob_array_index\0").map(|sym| *sym)?;
        let ob_get_indirect_obdata = __library.get(b"ob_get_indirect_obdata\0").map(|sym| *sym)?;
        let ob_array_item = __library.get(b"ob_array_item\0").map(|sym| *sym)?;
        let ob_array_get_index_from_subs =
            __library.get(b"ob_array_get_index_from_subs\0").map(|sym| *sym)?;
        let ob_array_calc_index = __library.get(b"ob_array_calc_index\0").map(|sym| *sym)?;
        let ob_set_int_field = __library.get(b"ob_set_int_field\0").map(|sym| *sym)?;
        let ob_set_uint_field = __library.get(b"ob_set_uint_field\0").map(|sym| *sym)?;
        let ob_set_long_field = __library.get(b"ob_set_long_field\0").map(|sym| *sym)?;
        let ob_set_ulong_field = __library.get(b"ob_set_ulong_field\0").map(|sym| *sym)?;
        let ob_set_float_field = __library.get(b"ob_set_float_field\0").map(|sym| *sym)?;
        let ob_set_ptr_field = __library.get(b"ob_set_ptr_field\0").map(|sym| *sym)?;
        let ob_set_array_field = __library.get(b"ob_set_array_field\0").map(|sym| *sym)?;
        let ob_set_obinst_field = __library.get(b"ob_set_obinst_field\0").map(|sym| *sym)?;
        let ob_set_underlying_object = __library.get(b"ob_set_underlying_object\0").map(|sym| *sym)?;
        let ob_get_underlying_object = __library.get(b"ob_get_underlying_object\0").map(|sym| *sym)?;
        let ob_is_any_group_locked = __library.get(b"ob_is_any_group_locked\0").map(|sym| *sym)?;
        let ob_get_group_lock_count = __library.get(b"ob_get_group_lock_count\0").map(|sym| *sym)?;
        let ob_is_group_locked = __library.get(b"ob_is_group_locked\0").map(|sym| *sym)?;
        let ob_is_group_unlocked = __library.get(b"ob_is_group_unlocked\0").map(|sym| *sym)?;
        let ob_is_group_write_locked = __library.get(b"ob_is_group_write_locked\0").map(|sym| *sym)?;
        let ob_lock_group = __library.get(b"ob_lock_group\0").map(|sym| *sym)?;
        let ob_unlock_group = __library.get(b"ob_unlock_group\0").map(|sym| *sym)?;
        let ob_clear_unlocked_groups = __library.get(b"ob_clear_unlocked_groups\0").map(|sym| *sym)?;
        let ob_clear_all_other_unlocked_groups =
            __library.get(b"ob_clear_all_other_unlocked_groups\0").map(|sym| *sym)?;
        let ob_is_ancestor_locked = __library.get(b"ob_is_ancestor_locked\0").map(|sym| *sym)?;
        let ob_is_descendent_locked = __library.get(b"ob_is_descendent_locked\0").map(|sym| *sym)?;
        let ob_validate_liblist = __library.get(b"ob_validate_liblist\0").map(|sym| *sym)?;
        let ob_set_liblist = __library.get(b"ob_set_liblist\0").map(|sym| *sym)?;
        let ob_get_liblist = __library.get(b"ob_get_liblist\0").map(|sym| *sym)?;
        let ob_set_default_appl = __library.get(b"ob_set_default_appl\0").map(|sym| *sym)?;
        let ob_load_appl_group = __library.get(b"ob_load_appl_group\0").map(|sym| *sym)?;
        let ob_is_group_in_memory = __library.get(b"ob_is_group_in_memory\0").map(|sym| *sym)?;
        let ob_group_declare = __library.get(b"ob_group_declare\0").map(|sym| *sym)?;
        let ob_group_reference = __library.get(b"ob_group_reference\0").map(|sym| *sym)?;
        let ob_get_group_name = __library.get(b"ob_get_group_name\0").map(|sym| *sym)?;
        let ob_get_group_full_name = __library.get(b"ob_get_group_full_name\0").map(|sym| *sym)?;
        let ob_group_save = __library.get(b"ob_group_save\0").map(|sym| *sym)?;
        let ob_group_save_win = __library.get(b"ob_group_save_win\0").map(|sym| *sym)?;
        let ob_load_group_source = __library.get(b"ob_load_group_source\0").map(|sym| *sym)?;
        let ob_rename_group = __library.get(b"ob_rename_group\0").map(|sym| *sym)?;
        let ob_move_group = __library.get(b"ob_move_group\0").map(|sym| *sym)?;
        let ob_move_group_with_name = __library.get(b"ob_move_group_with_name\0").map(|sym| *sym)?;
        let ob_copy_group_with_name = __library.get(b"ob_copy_group_with_name\0").map(|sym| *sym)?;
        let ob_copy_group = __library.get(b"ob_copy_group\0").map(|sym| *sym)?;
        let ob_delete_group = __library.get(b"ob_delete_group\0").map(|sym| *sym)?;
        let ob_delete_group_with_name = __library.get(b"ob_delete_group_with_name\0").map(|sym| *sym)?;
        let ob_restore_group = __library.get(b"ob_restore_group\0").map(|sym| *sym)?;
        let ob_save_working_group = __library.get(b"ob_save_working_group\0").map(|sym| *sym)?;
        let ob_delete_working_group = __library.get(b"ob_delete_working_group\0").map(|sym| *sym)?;
        let ob_restore_working_group = __library.get(b"ob_restore_working_group\0").map(|sym| *sym)?;
        let ob_open_group_id = __library.get(b"ob_open_group_id\0").map(|sym| *sym)?;
        let ob_close_group = __library.get(b"ob_close_group\0").map(|sym| *sym)?;
        let ob_get_group_lib = __library.get(b"ob_get_group_lib\0").map(|sym| *sym)?;
        let ob_run_garbage_collection = __library.get(b"ob_run_garbage_collection\0").map(|sym| *sym)?;
        let ob_delete_instlist_shlist = __library.get(b"ob_delete_instlist_shlist\0").map(|sym| *sym)?;
        let ob_get_group_instlist_as_shlist =
            __library.get(b"ob_get_group_instlist_as_shlist\0").map(|sym| *sym)?;
        let ob_delete_groups_shlist = __library.get(b"ob_delete_groups_shlist\0").map(|sym| *sym)?;
        let ob_get_groups_shlist = __library.get(b"ob_get_groups_shlist\0").map(|sym| *sym)?;
        let ob_store_source = __library.get(b"ob_store_source\0").map(|sym| *sym)?;
        let ob_init_source = __library.get(b"ob_init_source\0").map(|sym| *sym)?;
        let ob_store_global_src = __library.get(b"ob_store_global_src\0").map(|sym| *sym)?;
        let ob_store_namespace_decl_src = __library.get(b"ob_store_namespace_decl_src\0").map(|sym| *sym)?;
        let ob_store_shared_src = __library.get(b"ob_store_shared_src\0").map(|sym| *sym)?;
        let ob_store_prototype_source = __library.get(b"ob_store_prototype_source\0").map(|sym| *sym)?;
        let ob_store_instvar_source = __library.get(b"ob_store_instvar_source\0").map(|sym| *sym)?;
        let ob_get_global_src = __library.get(b"ob_get_global_src\0").map(|sym| *sym)?;
        let ob_get_namespace_decl_src = __library.get(b"ob_get_namespace_decl_src\0").map(|sym| *sym)?;
        let ob_get_shared_src = __library.get(b"ob_get_shared_src\0").map(|sym| *sym)?;
        let ob_get_prototype_source = __library.get(b"ob_get_prototype_source\0").map(|sym| *sym)?;
        let ob_get_instvar_source = __library.get(b"ob_get_instvar_source\0").map(|sym| *sym)?;
        let ob_get_routine_src = __library.get(b"ob_get_routine_src\0").map(|sym| *sym)?;
        let ob_decl_and_store_routine_src =
            __library.get(b"ob_decl_and_store_routine_src\0").map(|sym| *sym)?;
        let ob_store_routine_src = __library.get(b"ob_store_routine_src\0").map(|sym| *sym)?;
        let ob_store_create_src = __library.get(b"ob_store_create_src\0").map(|sym| *sym)?;
        let ob_store_destroy_src = __library.get(b"ob_store_destroy_src\0").map(|sym| *sym)?;
        let ob_get_function_src = __library.get(b"ob_get_function_src\0").map(|sym| *sym)?;
        let ob_store_function_src = __library.get(b"ob_store_function_src\0").map(|sym| *sym)?;
        let ob_symbol_search_extended = __library.get(b"ob_symbol_search_extended\0").map(|sym| *sym)?;
        let ob_symbol_search = __library.get(b"ob_symbol_search\0").map(|sym| *sym)?;
        let ob_class_declare = __library.get(b"ob_class_declare\0").map(|sym| *sym)?;
        let ob_get_full_qualified_typename =
            __library.get(b"ob_get_full_qualified_typename\0").map(|sym| *sym)?;
        let ob_class_declare_inh = __library.get(b"ob_class_declare_inh\0").map(|sym| *sym)?;
        let ob_class_reference = __library.get(b"ob_class_reference\0").map(|sym| *sym)?;
        let ob_class_name = __library.get(b"ob_class_name\0").map(|sym| *sym)?;
        let ob_class_name_not_indirect = __library.get(b"ob_class_name_not_indirect\0").map(|sym| *sym)?;
        let ob_get_type_name = __library.get(b"ob_get_type_name\0").map(|sym| *sym)?;
        let ob_classhndl_indirect = __library.get(b"ob_classhndl_indirect\0").map(|sym| *sym)?;
        let ob_get_parent_class = __library.get(b"ob_get_parent_class\0").map(|sym| *sym)?;
        let ob_get_within_class = __library.get(b"ob_get_within_class\0").map(|sym| *sym)?;
        let ob_class_delete = __library.get(b"ob_class_delete\0").map(|sym| *sym)?;
        let ob_class_rename = __library.get(b"ob_class_rename\0").map(|sym| *sym)?;
        let ob_is_a_system_class = __library.get(b"ob_is_a_system_class\0").map(|sym| *sym)?;
        let ob_is_class_inherited = __library.get(b"ob_is_class_inherited\0").map(|sym| *sym)?;
        let ob_is_class_descendant = __library.get(b"ob_is_class_descendant\0").map(|sym| *sym)?;
        let ob_is_inh_from_user_class = __library.get(b"ob_is_inh_from_user_class\0").map(|sym| *sym)?;
        let ob_get_sec_class_ancestor = __library.get(b"ob_get_sec_class_ancestor\0").map(|sym| *sym)?;
        let ob_is_class_enum = __library.get(b"ob_is_class_enum\0").map(|sym| *sym)?;
        let ob_new_event = __library.get(b"ob_new_event\0").map(|sym| *sym)?;
        let ob_update_event = __library.get(b"ob_update_event\0").map(|sym| *sym)?;
        let ob_delete_event = __library.get(b"ob_delete_event\0").map(|sym| *sym)?;
        let ob_has_events = __library.get(b"ob_has_events\0").map(|sym| *sym)?;
        let ob_get_event_token_id = __library.get(b"ob_get_event_token_id\0").map(|sym| *sym)?;
        let ob_get_event_id_from_token = __library.get(b"ob_get_event_id_from_token\0").map(|sym| *sym)?;
        let ob_does_event_script_exist = __library.get(b"ob_does_event_script_exist\0").map(|sym| *sym)?;
        let ob_get_routine_name = __library.get(b"ob_get_routine_name\0").map(|sym| *sym)?;
        let ob_delete_routine = __library.get(b"ob_delete_routine\0").map(|sym| *sym)?;
        let ob_get_curr_routine = __library.get(b"ob_get_curr_routine\0").map(|sym| *sym)?;
        let ob_get_curr_function = __library.get(b"ob_get_curr_function\0").map(|sym| *sym)?;
        let ob_get_routid_from_vtable_id =
            __library.get(b"ob_get_routid_from_vtable_id\0").map(|sym| *sym)?;
        let ob_is_valid_event_index = __library.get(b"ob_is_valid_event_index\0").map(|sym| *sym)?;
        let ob_has_scripts = __library.get(b"ob_has_scripts\0").map(|sym| *sym)?;
        let ob_get_routine_type = __library.get(b"ob_get_routine_type\0").map(|sym| *sym)?;
        let ob_get_function_vtable_ids = __library.get(b"ob_get_function_vtable_ids\0").map(|sym| *sym)?;
        let ob_get_function_vtable_ids_for_ide =
            __library.get(b"ob_get_function_vtable_ids_for_ide\0").map(|sym| *sym)?;
        let ob_get_event_vtable_ids = __library.get(b"ob_get_event_vtable_ids\0").map(|sym| *sym)?;
        let ob_get_function_name = __library.get(b"ob_get_function_name\0").map(|sym| *sym)?;
        let ob_delete_function = __library.get(b"ob_delete_function\0").map(|sym| *sym)?;
        let ob_find_routine = __library.get(b"ob_find_routine\0").map(|sym| *sym)?;
        let ob_get_vtable_id_from_proto_id =
            __library.get(b"ob_get_vtable_id_from_proto_id\0").map(|sym| *sym)?;
        let ob_get_dll_func_names = __library.get(b"ob_get_dll_func_names\0").map(|sym| *sym)?;
        let ob_get_global_func_names_in_lib =
            __library.get(b"ob_get_global_func_names_in_lib\0").map(|sym| *sym)?;
        let ob_get_global_func_index = __library.get(b"ob_get_global_func_index\0").map(|sym| *sym)?;
        let ob_get_func_index_in_lib = __library.get(b"ob_get_func_index_in_lib\0").map(|sym| *sym)?;
        let ob_get_proto_is_external_event =
            __library.get(b"ob_get_proto_is_external_event\0").map(|sym| *sym)?;
        let ob_get_protoarg_info = __library.get(b"ob_get_protoarg_info\0").map(|sym| *sym)?;
        let ob_get_proto_info = __library.get(b"ob_get_proto_info\0").map(|sym| *sym)?;
        let ob_get_method_signature = __library.get(b"ob_get_method_signature\0").map(|sym| *sym)?;
        let ob_was_event_prototype_changed =
            __library.get(b"ob_was_event_prototype_changed\0").map(|sym| *sym)?;
        let ob_get_proto_name_info = __library.get(b"ob_get_proto_name_info\0").map(|sym| *sym)?;
        let ob_get_proto_throws_info = __library.get(b"ob_get_proto_throws_info\0").map(|sym| *sym)?;
        let ob_lookup_routine_by_name = __library.get(b"ob_lookup_routine_by_name\0").map(|sym| *sym)?;
        let ob_get_objnames_of_class = __library.get(b"ob_get_objnames_of_class\0").map(|sym| *sym)?;
        let ob_has_object_of_class = __library.get(b"ob_has_object_of_class\0").map(|sym| *sym)?;
        let ob_get_obj_classhndls_of_class =
            __library.get(b"ob_get_obj_classhndls_of_class\0").map(|sym| *sym)?;
        let ob_get_objnames_of_class_in_lib =
            __library.get(b"ob_get_objnames_of_class_in_lib\0").map(|sym| *sym)?;
        let ob_global_reference = __library.get(b"ob_global_reference\0").map(|sym| *sym)?;
        let ob_global_reference_in_lib = __library.get(b"ob_global_reference_in_lib\0").map(|sym| *sym)?;
        let ob_global_reference_of_class =
            __library.get(b"ob_global_reference_of_class\0").map(|sym| *sym)?;
        let ob_get_obinst_class_hndl = __library.get(b"ob_get_obinst_class_hndl\0").map(|sym| *sym)?;
        let ob_is_a_typedef = __library.get(b"ob_is_a_typedef\0").map(|sym| *sym)?;
        let ob_is_an_enum = __library.get(b"ob_is_an_enum\0").map(|sym| *sym)?;
        let ob_get_system_class = __library.get(b"ob_get_system_class\0").map(|sym| *sym)?;
        let ob_get_obinst_system_class = __library.get(b"ob_get_obinst_system_class\0").map(|sym| *sym)?;
        let ob_get_obinst_group_hndl = __library.get(b"ob_get_obinst_group_hndl\0").map(|sym| *sym)?;
        let ob_get_obinst_class_name = __library.get(b"ob_get_obinst_class_name\0").map(|sym| *sym)?;
        let ob_fetch_fields_of_class = __library.get(b"ob_fetch_fields_of_class\0").map(|sym| *sym)?;
        let ob_get_fields_of_class = __library.get(b"ob_get_fields_of_class\0").map(|sym| *sym)?;
        let ob_get_local_var_info = __library.get(b"ob_get_local_var_info\0").map(|sym| *sym)?;
        let ob_get_shared_vars_of_class = __library.get(b"ob_get_shared_vars_of_class\0").map(|sym| *sym)?;
        let ob_get_shared_var_info = __library.get(b"ob_get_shared_var_info\0").map(|sym| *sym)?;
        let ob_get_global_vars_of_class = __library.get(b"ob_get_global_vars_of_class\0").map(|sym| *sym)?;
        let ob_get_class_field_info = __library.get(b"ob_get_class_field_info\0").map(|sym| *sym)?;
        let ob_get_enum_info = __library.get(b"ob_get_enum_info\0").map(|sym| *sym)?;
        let ob_get_class_event_info = __library.get(b"ob_get_class_event_info\0").map(|sym| *sym)?;
        let ob_get_instance_field_info = __library.get(b"ob_get_instance_field_info\0").map(|sym| *sym)?;
        let ob_get_obinst_field_info = __library.get(b"ob_get_obinst_field_info\0").map(|sym| *sym)?;
        let ob_get_obinst_all_field_info =
            __library.get(b"ob_get_obinst_all_field_info\0").map(|sym| *sym)?;
        let ob_get_classes_within_group = __library.get(b"ob_get_classes_within_group\0").map(|sym| *sym)?;
        let ob_get_enums_within_group = __library.get(b"ob_get_enums_within_group\0").map(|sym| *sym)?;
        let ob_get_global_var_data = __library.get(b"ob_get_global_var_data\0").map(|sym| *sym)?;
        let ob_object_reference_count = __library.get(b"ob_object_reference_count\0").map(|sym| *sym)?;
        let ob_named_global_var_info = __library.get(b"ob_named_global_var_info\0").map(|sym| *sym)?;
        let ob_named_shared_var_info = __library.get(b"ob_named_shared_var_info\0").map(|sym| *sym)?;
        let ob_named_special_var_info = __library.get(b"ob_named_special_var_info\0").map(|sym| *sym)?;
        let ob_named_local_var_info = __library.get(b"ob_named_local_var_info\0").map(|sym| *sym)?;
        let ob_named_field_info = __library.get(b"ob_named_field_info\0").map(|sym| *sym)?;
        let ob_get_array_info = __library.get(b"ob_get_array_info\0").map(|sym| *sym)?;
        let ob_get_array_bounds_string = __library.get(b"ob_get_array_bounds_string\0").map(|sym| *sym)?;
        let ob_get_array_bounds_string_from_field_info =
            __library.get(b"ob_get_array_bounds_string_from_field_info\0").map(|sym| *sym)?;
        let ob_get_info_watchpoint = __library.get(b"ob_get_info_watchpoint\0").map(|sym| *sym)?;
        let ob_compile_lib_entry = __library.get(b"ob_compile_lib_entry\0").map(|sym| *sym)?;
        let ob_compile_lib_typedefs = __library.get(b"ob_compile_lib_typedefs\0").map(|sym| *sym)?;
        let ob_compile_lib_entry_3_pass = __library.get(b"ob_compile_lib_entry_3_pass\0").map(|sym| *sym)?;
        let ob_compile_lib_scripts = __library.get(b"ob_compile_lib_scripts\0").map(|sym| *sym)?;
        let ob_func_search = __library.get(b"ob_func_search\0").map(|sym| *sym)?;
        let ob_del_funccall_info = __library.get(b"ob_del_funccall_info\0").map(|sym| *sym)?;
        let ob_link_project = __library.get(b"ob_link_project\0").map(|sym| *sym)?;
        let ob_check_for_locked_menu = __library.get(b"ob_check_for_locked_menu\0").map(|sym| *sym)?;
        let ob_create_obinst = __library.get(b"ob_create_obinst\0").map(|sym| *sym)?;
        let ob_instantiate_child_object = __library.get(b"ob_instantiate_child_object\0").map(|sym| *sym)?;
        let ob_instantiate_system_object =
            __library.get(b"ob_instantiate_system_object\0").map(|sym| *sym)?;
        let ob_destroy_obinst = __library.get(b"ob_destroy_obinst\0").map(|sym| *sym)?;
        let ob_set_runtime = __library.get(b"ob_set_runtime\0").map(|sym| *sym)?;
        let ob_create_executable = __library.get(b"ob_create_executable\0").map(|sym| *sym)?;
        let ob_create_library = __library.get(b"ob_create_library\0").map(|sym| *sym)?;
        let ob_create_consolidated_library =
            __library.get(b"ob_create_consolidated_library\0").map(|sym| *sym)?;
        let ob_create_interface_class = __library.get(b"ob_create_interface_class\0").map(|sym| *sym)?;
        let ob_init_executable = __library.get(b"ob_init_executable\0").map(|sym| *sym)?;
        let ob_scan_source_blocks = __library.get(b"ob_scan_source_blocks\0").map(|sym| *sym)?;
        let ob_create_launcher = __library.get(b"ob_create_launcher\0").map(|sym| *sym)?;
        let ob_sanitize_pb_name = __library.get(b"ob_sanitize_pb_name\0").map(|sym| *sym)?;
        let ob_validate_class = __library.get(b"ob_validate_class\0").map(|sym| *sym)?;
        let ob_get_orphaned_classes = __library.get(b"ob_get_orphaned_classes\0").map(|sym| *sym)?;
        let ob_validate_type_name = __library.get(b"ob_validate_type_name\0").map(|sym| *sym)?;
        let ob_convert_to_ver2_source = __library.get(b"ob_convert_to_ver2_source\0").map(|sym| *sym)?;
        let ob_is_vers2_obj = __library.get(b"ob_is_vers2_obj\0").map(|sym| *sym)?;
        let ob_build_ordered_compile_list =
            __library.get(b"ob_build_ordered_compile_list\0").map(|sym| *sym)?;
        let ob_free_ordered_compile_list =
            __library.get(b"ob_free_ordered_compile_list\0").map(|sym| *sym)?;
        let ob_build_hierarchy_list = __library.get(b"ob_build_hierarchy_list\0").map(|sym| *sym)?;
        let ob_free_hierarchy_list = __library.get(b"ob_free_hierarchy_list\0").map(|sym| *sym)?;
        let ob_clear_instance_ref = __library.get(b"ob_clear_instance_ref\0").map(|sym| *sym)?;
        let ob_insert_inst_ref_dbg = __library.get(b"ob_insert_inst_ref_dbg\0").map(|sym| *sym)?;
        let ob_open_typedef_group = __library.get(b"ob_open_typedef_group\0").map(|sym| *sym)?;
        let ob_save_dll_to_pbd = __library.get(b"ob_save_dll_to_pbd\0").map(|sym| *sym)?;
        let ob_convert_pbx_to_native_groups =
            __library.get(b"ob_convert_pbx_to_native_groups\0").map(|sym| *sym)?;
        let ob_share_typedef_group = __library.get(b"ob_share_typedef_group\0").map(|sym| *sym)?;
        let ob_unshare_typedef_group = __library.get(b"ob_unshare_typedef_group\0").map(|sym| *sym)?;
        let ob_cm_evaluate_expression = __library.get(b"ob_cm_evaluate_expression\0").map(|sym| *sym)?;
        let ob_entryInheritsFromClass = __library.get(b"ob_entryInheritsFromClass\0").map(|sym| *sym)?;
        let ob_get_class_from_name = __library.get(b"ob_get_class_from_name\0").map(|sym| *sym)?;
        let ob_local_global_lv = __library.get(b"ob_local_global_lv\0").map(|sym| *sym)?;
        let ob_local_global_refpkt = __library.get(b"ob_local_global_refpkt\0").map(|sym| *sym)?;
        let ob_shared_global_lv = __library.get(b"ob_shared_global_lv\0").map(|sym| *sym)?;
        let ob_shared_global_refpkt = __library.get(b"ob_shared_global_refpkt\0").map(|sym| *sym)?;
        let ob_shared_lv = __library.get(b"ob_shared_lv\0").map(|sym| *sym)?;
        let ob_shared_refpkt = __library.get(b"ob_shared_refpkt\0").map(|sym| *sym)?;
        let ob_convert_chararray_to_string =
            __library.get(b"ob_convert_chararray_to_string\0").map(|sym| *sym)?;
        let ob_class_delete_and_withinclass =
            __library.get(b"ob_class_delete_and_withinclass\0").map(|sym| *sym)?;
        let ob_find_orphan_class = __library.get(b"ob_find_orphan_class\0").map(|sym| *sym)?;
        let ob_nuke_orphan_class = __library.get(b"ob_nuke_orphan_class\0").map(|sym| *sym)?;
        let ob_is_ancestor_class_modified =
            __library.get(b"ob_is_ancestor_class_modified\0").map(|sym| *sym)?;
        let ob_rebuild_instance_image = __library.get(b"ob_rebuild_instance_image\0").map(|sym| *sym)?;
        let ob_build_compile_list = __library.get(b"ob_build_compile_list\0").map(|sym| *sym)?;
        let ot_get_next_evaled_arg = __library.get(b"ot_get_next_evaled_arg\0").map(|sym| *sym)?;
        let ot_get_next_evaled_arg_no_convert =
            __library.get(b"ot_get_next_evaled_arg_no_convert\0").map(|sym| *sym)?;
        let ot_get_next_lvalue_arg = __library.get(b"ot_get_next_lvalue_arg\0").map(|sym| *sym)?;
        let ot_get_simple_intarg = __library.get(b"ot_get_simple_intarg\0").map(|sym| *sym)?;
        let ot_get_simple_longarg = __library.get(b"ot_get_simple_longarg\0").map(|sym| *sym)?;
        let ot_get_intarg = __library.get(b"ot_get_intarg\0").map(|sym| *sym)?;
        let ot_get_uintarg = __library.get(b"ot_get_uintarg\0").map(|sym| *sym)?;
        let ot_get_longarg = __library.get(b"ot_get_longarg\0").map(|sym| *sym)?;
        let ot_get_ulongarg = __library.get(b"ot_get_ulongarg\0").map(|sym| *sym)?;
        let ot_get_decarg = __library.get(b"ot_get_decarg\0").map(|sym| *sym)?;
        let ot_get_floatarg = __library.get(b"ot_get_floatarg\0").map(|sym| *sym)?;
        let ot_get_doublearg = __library.get(b"ot_get_doublearg\0").map(|sym| *sym)?;
        let ot_get_longlongarg = __library.get(b"ot_get_longlongarg\0").map(|sym| *sym)?;
        let ot_get_obinstarg = __library.get(b"ot_get_obinstarg\0").map(|sym| *sym)?;
        let ot_get_valptr_arg = __library.get(b"ot_get_valptr_arg\0").map(|sym| *sym)?;
        let ot_init_arglist = __library.get(b"ot_init_arglist\0").map(|sym| *sym)?;
        let ot_get_valptr = __library.get(b"ot_get_valptr\0").map(|sym| *sym)?;
        let ot_type_srch = __library.get(b"ot_type_srch\0").map(|sym| *sym)?;
        let ot_type_attr = __library.get(b"ot_type_attr\0").map(|sym| *sym)?;
        let ot_get_class_name = __library.get(b"ot_get_class_name\0").map(|sym| *sym)?;
        let ot_is_array_eq = __library.get(b"ot_is_array_eq\0").map(|sym| *sym)?;
        let ot_is_struct_eq = __library.get(b"ot_is_struct_eq\0").map(|sym| *sym)?;
        let ot_create_obinst_with_name = __library.get(b"ot_create_obinst_with_name\0").map(|sym| *sym)?;
        let ot_create_obinst_at_lval = __library.get(b"ot_create_obinst_at_lval\0").map(|sym| *sym)?;
        let ot_get_curr_obinst_expr = __library.get(b"ot_get_curr_obinst_expr\0").map(|sym| *sym)?;
        let ot_func_call = __library.get(b"ot_func_call\0").map(|sym| *sym)?;
        let ot_set_return_val = __library.get(b"ot_set_return_val\0").map(|sym| *sym)?;
        let ot_set_return_double = __library.get(b"ot_set_return_double\0").map(|sym| *sym)?;
        let ot_set_return_longlong = __library.get(b"ot_set_return_longlong\0").map(|sym| *sym)?;
        let ot_set_return_dec = __library.get(b"ot_set_return_dec\0").map(|sym| *sym)?;
        let ot_no_return_val = __library.get(b"ot_no_return_val\0").map(|sym| *sym)?;
        let ot_assign_lvalue_dec = __library.get(b"ot_assign_lvalue_dec\0").map(|sym| *sym)?;
        let ot_assign_lvalue_double = __library.get(b"ot_assign_lvalue_double\0").map(|sym| *sym)?;
        let ot_assign_lvalue_longlong = __library.get(b"ot_assign_lvalue_longlong\0").map(|sym| *sym)?;
        let ot_assign_lvalue_blob = __library.get(b"ot_assign_lvalue_blob\0").map(|sym| *sym)?;
        let ot_assign_lvalue_obinst = __library.get(b"ot_assign_lvalue_obinst\0").map(|sym| *sym)?;
        let ot_assign_lvalue_array = __library.get(b"ot_assign_lvalue_array\0").map(|sym| *sym)?;
        let ot_assign_lvalue_any = __library.get(b"ot_assign_lvalue_any\0").map(|sym| *sym)?;
        let ot_set_local_var = __library.get(b"ot_set_local_var\0").map(|sym| *sym)?;
        let ot_set_shared_var = __library.get(b"ot_set_shared_var\0").map(|sym| *sym)?;
        let ot_set_obinst_var = __library.get(b"ot_set_obinst_var\0").map(|sym| *sym)?;
        let ot_set_local_array_item = __library.get(b"ot_set_local_array_item\0").map(|sym| *sym)?;
        let ot_set_shared_array_item = __library.get(b"ot_set_shared_array_item\0").map(|sym| *sym)?;
        let ot_set_obinst_array_item = __library.get(b"ot_set_obinst_array_item\0").map(|sym| *sym)?;
        let ot_get_array_values = __library.get(b"ot_get_array_values\0").map(|sym| *sym)?;
        let ot_reset_array = __library.get(b"ot_reset_array\0").map(|sym| *sym)?;
        let ot_get_local_var = __library.get(b"ot_get_local_var\0").map(|sym| *sym)?;
        let ot_get_shared_var = __library.get(b"ot_get_shared_var\0").map(|sym| *sym)?;
        let ot_math_type_convert = __library.get(b"ot_math_type_convert\0").map(|sym| *sym)?;
        let ot_get_int_value = __library.get(b"ot_get_int_value\0").map(|sym| *sym)?;
        let ot_get_uint_value = __library.get(b"ot_get_uint_value\0").map(|sym| *sym)?;
        let ot_get_byte_value = __library.get(b"ot_get_byte_value\0").map(|sym| *sym)?;
        let ot_get_long_value = __library.get(b"ot_get_long_value\0").map(|sym| *sym)?;
        let ot_get_ulong_value = __library.get(b"ot_get_ulong_value\0").map(|sym| *sym)?;
        let ot_get_dec_value = __library.get(b"ot_get_dec_value\0").map(|sym| *sym)?;
        let ot_get_float_value = __library.get(b"ot_get_float_value\0").map(|sym| *sym)?;
        let ot_get_double_value = __library.get(b"ot_get_double_value\0").map(|sym| *sym)?;
        let ot_get_longlong_value = __library.get(b"ot_get_longlong_value\0").map(|sym| *sym)?;
        let ot_free_val_ptr = __library.get(b"ot_free_val_ptr\0").map(|sym| *sym)?;
        let ot_free_array = __library.get(b"ot_free_array\0").map(|sym| *sym)?;
        let ot_convert_to_int = __library.get(b"ot_convert_to_int\0").map(|sym| *sym)?;
        let ot_convert_to_uint = __library.get(b"ot_convert_to_uint\0").map(|sym| *sym)?;
        let ot_convert_to_byte = __library.get(b"ot_convert_to_byte\0").map(|sym| *sym)?;
        let ot_convert_to_long = __library.get(b"ot_convert_to_long\0").map(|sym| *sym)?;
        let ot_convert_to_ulong = __library.get(b"ot_convert_to_ulong\0").map(|sym| *sym)?;
        let ot_convert_to_dec = __library.get(b"ot_convert_to_dec\0").map(|sym| *sym)?;
        let ot_convert_to_float = __library.get(b"ot_convert_to_float\0").map(|sym| *sym)?;
        let ot_convert_to_double = __library.get(b"ot_convert_to_double\0").map(|sym| *sym)?;
        let ot_convert_to_longlong = __library.get(b"ot_convert_to_longlong\0").map(|sym| *sym)?;
        let ot_ansi_lower = __library.get(b"ot_ansi_lower\0").map(|sym| *sym)?;
        let ot_ansi_upper = __library.get(b"ot_ansi_upper\0").map(|sym| *sym)?;
        let ot_ansi_strcmp = __library.get(b"ot_ansi_strcmp\0").map(|sym| *sym)?;
        let ot_get_field_lv = __library.get(b"ot_get_field_lv\0").map(|sym| *sym)?;
        let ot_get_field_item_lv = __library.get(b"ot_get_field_item_lv\0").map(|sym| *sym)?;
        let ot_assign_ref_int = __library.get(b"ot_assign_ref_int\0").map(|sym| *sym)?;
        let ot_assign_ref_uint = __library.get(b"ot_assign_ref_uint\0").map(|sym| *sym)?;
        let ot_assign_ref_byte = __library.get(b"ot_assign_ref_byte\0").map(|sym| *sym)?;
        let ot_assign_ref_long = __library.get(b"ot_assign_ref_long\0").map(|sym| *sym)?;
        let ot_assign_ref_ulong = __library.get(b"ot_assign_ref_ulong\0").map(|sym| *sym)?;
        let ot_assign_ref_dec = __library.get(b"ot_assign_ref_dec\0").map(|sym| *sym)?;
        let ot_assign_ref_float = __library.get(b"ot_assign_ref_float\0").map(|sym| *sym)?;
        let ot_assign_ref_double = __library.get(b"ot_assign_ref_double\0").map(|sym| *sym)?;
        let ot_assign_ref_longlong = __library.get(b"ot_assign_ref_longlong\0").map(|sym| *sym)?;
        let ot_assign_ref_string = __library.get(b"ot_assign_ref_string\0").map(|sym| *sym)?;
        let ot_assign_ref_bool = __library.get(b"ot_assign_ref_bool\0").map(|sym| *sym)?;
        let ot_assign_ref_char = __library.get(b"ot_assign_ref_char\0").map(|sym| *sym)?;
        let ot_assign_ref_blob = __library.get(b"ot_assign_ref_blob\0").map(|sym| *sym)?;
        let ot_assign_ref_time = __library.get(b"ot_assign_ref_time\0").map(|sym| *sym)?;
        let ot_assign_ref_date = __library.get(b"ot_assign_ref_date\0").map(|sym| *sym)?;
        let ot_assign_ref_datetime = __library.get(b"ot_assign_ref_datetime\0").map(|sym| *sym)?;
        let ot_assign_ref_obinst = __library.get(b"ot_assign_ref_obinst\0").map(|sym| *sym)?;
        let ot_assign_ref_enum = __library.get(b"ot_assign_ref_enum\0").map(|sym| *sym)?;
        let ot_assign_ref_array = __library.get(b"ot_assign_ref_array\0").map(|sym| *sym)?;
        let ot_assign_ref_any = __library.get(b"ot_assign_ref_any\0").map(|sym| *sym)?;
        let ot_get_nested_obinst = __library.get(b"ot_get_nested_obinst\0").map(|sym| *sym)?;
        let ot_array_create_bounded = __library.get(b"ot_array_create_bounded\0").map(|sym| *sym)?;
        let ot_array_create_unbounded = __library.get(b"ot_array_create_unbounded\0").map(|sym| *sym)?;
        let ot_array_index = __library.get(b"ot_array_index\0").map(|sym| *sym)?;
        let ot_array_set_free_data = __library.get(b"ot_array_set_free_data\0").map(|sym| *sym)?;
        let ot_array_free_data = __library.get(b"ot_array_free_data\0").map(|sym| *sym)?;
        let ot_array_class_id = __library.get(b"ot_array_class_id\0").map(|sym| *sym)?;
        let ot_array_class_hndl = __library.get(b"ot_array_class_hndl\0").map(|sym| *sym)?;
        let ot_array_num_dimensions = __library.get(b"ot_array_num_dimensions\0").map(|sym| *sym)?;
        let ot_array_num_items = __library.get(b"ot_array_num_items\0").map(|sym| *sym)?;
        let ot_is_array_unbounded = __library.get(b"ot_is_array_unbounded\0").map(|sym| *sym)?;
        let ot_get_arraydef_no_dims = __library.get(b"ot_get_arraydef_no_dims\0").map(|sym| *sym)?;
        let ot_get_arraydef_style = __library.get(b"ot_get_arraydef_style\0").map(|sym| *sym)?;
        let ot_get_arraydef_bounds = __library.get(b"ot_get_arraydef_bounds\0").map(|sym| *sym)?;
        let ot_get_arraydef_varinfo = __library.get(b"ot_get_arraydef_varinfo\0").map(|sym| *sym)?;
        let ot_get_arraydef_upper_bound = __library.get(b"ot_get_arraydef_upper_bound\0").map(|sym| *sym)?;
        let ot_get_arraydef_lower_bound = __library.get(b"ot_get_arraydef_lower_bound\0").map(|sym| *sym)?;
        let ot_randomize = __library.get(b"ot_randomize\0").map(|sym| *sym)?;
        let ot_rand = __library.get(b"ot_rand\0").map(|sym| *sym)?;
        let ot_class_compare = __library.get(b"ot_class_compare\0").map(|sym| *sym)?;
        let ot_assign_global_var_obinst = __library.get(b"ot_assign_global_var_obinst\0").map(|sym| *sym)?;
        let ob_class_indirect = __library.get(b"ob_class_indirect\0").map(|sym| *sym)?;
        let ob_add_external_class_ref = __library.get(b"ob_add_external_class_ref\0").map(|sym| *sym)?;
        let ob_get_local_class = __library.get(b"ob_get_local_class\0").map(|sym| *sym)?;
        let ob_get_primary_class = __library.get(b"ob_get_primary_class\0").map(|sym| *sym)?;
        let ob_build_qual_sec_class_name =
            __library.get(b"ob_build_qual_sec_class_name\0").map(|sym| *sym)?;
        let ob_decl_indirect_sec_class = __library.get(b"ob_decl_indirect_sec_class\0").map(|sym| *sym)?;
        let ob_update_class_ref = __library.get(b"ob_update_class_ref\0").map(|sym| *sym)?;
        let ob_update_glob_class_instflag =
            __library.get(b"ob_update_glob_class_instflag\0").map(|sym| *sym)?;
        let ob_is_class_member_accessable =
            __library.get(b"ob_is_class_member_accessable\0").map(|sym| *sym)?;
        let ob_get_system_func_class = __library.get(b"ob_get_system_func_class\0").map(|sym| *sym)?;
        let ob_get_global_func_class = __library.get(b"ob_get_global_func_class\0").map(|sym| *sym)?;
        let ob_type_declare = __library.get(b"ob_type_declare\0").map(|sym| *sym)?;
        let ob_type_declare_class = __library.get(b"ob_type_declare_class\0").map(|sym| *sym)?;
        let ob_type_declare_vtab = __library.get(b"ob_type_declare_vtab\0").map(|sym| *sym)?;
        let ob_type_reference = __library.get(b"ob_type_reference\0").map(|sym| *sym)?;
        let ob_get_first_type = __library.get(b"ob_get_first_type\0").map(|sym| *sym)?;
        let ob_get_next_type = __library.get(b"ob_get_next_type\0").map(|sym| *sym)?;
        let ob_type_init_process = __library.get(b"ob_type_init_process\0").map(|sym| *sym)?;
        let ob_type_decl_process = __library.get(b"ob_type_decl_process\0").map(|sym| *sym)?;
        let ob_get_nested_class = __library.get(b"ob_get_nested_class\0").map(|sym| *sym)?;
        let ob_get_class_entry = __library.get(b"ob_get_class_entry\0").map(|sym| *sym)?;
        let ob_is_class_indirect = __library.get(b"ob_is_class_indirect\0").map(|sym| *sym)?;
        let ob_fetch_routine = __library.get(b"ob_fetch_routine\0").map(|sym| *sym)?;
        let ob_type_proto_decl = __library.get(b"ob_type_proto_decl\0").map(|sym| *sym)?;
        let ob_type_proto_ref = __library.get(b"ob_type_proto_ref\0").map(|sym| *sym)?;
        let ob_proto_error_upgrade = __library.get(b"ob_proto_error_upgrade\0").map(|sym| *sym)?;
        let ob_get_proto_access_type = __library.get(b"ob_get_proto_access_type\0").map(|sym| *sym)?;
        let ob_type_process_protos = __library.get(b"ob_type_process_protos\0").map(|sym| *sym)?;
        let ob_type_reprocess_protos = __library.get(b"ob_type_reprocess_protos\0").map(|sym| *sym)?;
        let ob_get_type_proto_names = __library.get(b"ob_get_type_proto_names\0").map(|sym| *sym)?;
        let ob_declare_external_event_type =
            __library.get(b"ob_declare_external_event_type\0").map(|sym| *sym)?;
        let ob_get_type_proto_names_for_ide =
            __library.get(b"ob_get_type_proto_names_for_ide\0").map(|sym| *sym)?;
        let ob_type_vtable_module_srch = __library.get(b"ob_type_vtable_module_srch\0").map(|sym| *sym)?;
        let ob_get_prototype = __library.get(b"ob_get_prototype\0").map(|sym| *sym)?;
        let ob_update_proto_mod_id = __library.get(b"ob_update_proto_mod_id\0").map(|sym| *sym)?;
        let ob_update_proto_rout_id = __library.get(b"ob_update_proto_rout_id\0").map(|sym| *sym)?;
        let ob_protolist_read = __library.get(b"ob_protolist_read\0").map(|sym| *sym)?;
        let ob_protolist_write = __library.get(b"ob_protolist_write\0").map(|sym| *sym)?;
        let ob_prototype_match_for_event =
            __library.get(b"ob_prototype_match_for_event\0").map(|sym| *sym)?;
        let ob_prototype_search = __library.get(b"ob_prototype_search\0").map(|sym| *sym)?;
        let ob_proto_overload_search = __library.get(b"ob_proto_overload_search\0").map(|sym| *sym)?;
        let ob_event_module_name = __library.get(b"ob_event_module_name\0").map(|sym| *sym)?;
        let ob_find_first_event = __library.get(b"ob_find_first_event\0").map(|sym| *sym)?;
        let ob_type_event_script_srch = __library.get(b"ob_type_event_script_srch\0").map(|sym| *sym)?;
        let ob_build_proto_vtable = __library.get(b"ob_build_proto_vtable\0").map(|sym| *sym)?;
        let ob_type_field_decl = __library.get(b"ob_type_field_decl\0").map(|sym| *sym)?;
        let ob_type_field_search = __library.get(b"ob_type_field_search\0").map(|sym| *sym)?;
        let ob_type_field_ref = __library.get(b"ob_type_field_ref\0").map(|sym| *sym)?;
        let ob_get_type_field_info = __library.get(b"ob_get_type_field_info\0").map(|sym| *sym)?;
        let ob_set_field_init_value = __library.get(b"ob_set_field_init_value\0").map(|sym| *sym)?;
        let ob_get_field_init_value = __library.get(b"ob_get_field_init_value\0").map(|sym| *sym)?;
        let ob_type_field_clear_instvars =
            __library.get(b"ob_type_field_clear_instvars\0").map(|sym| *sym)?;
        let ob_convert_fields_to_const = __library.get(b"ob_convert_fields_to_const\0").map(|sym| *sym)?;
        let ob_build_instance_image = __library.get(b"ob_build_instance_image\0").map(|sym| *sym)?;
        let ob_field_decl_indattr_funcs = __library.get(b"ob_field_decl_indattr_funcs\0").map(|sym| *sym)?;
        let ob_field_get_indattr_funcs = __library.get(b"ob_field_get_indattr_funcs\0").map(|sym| *sym)?;
        let ob_field_requires_update_notification =
            __library.get(b"ob_field_requires_update_notification\0").map(|sym| *sym)?;
        let ob_get_field_symtab = __library.get(b"ob_get_field_symtab\0").map(|sym| *sym)?;
        let ob_enum_entry_decl = __library.get(b"ob_enum_entry_decl\0").map(|sym| *sym)?;
        let ob_enum_decl_process = __library.get(b"ob_enum_decl_process\0").map(|sym| *sym)?;
        let ob_enum_reference = __library.get(b"ob_enum_reference\0").map(|sym| *sym)?;
        let ob_get_type_enum_info = __library.get(b"ob_get_type_enum_info\0").map(|sym| *sym)?;
        let ob_is_type_enum = __library.get(b"ob_is_type_enum\0").map(|sym| *sym)?;
        let ob_type_indattr_search = __library.get(b"ob_type_indattr_search\0").map(|sym| *sym)?;
        let ob_type_decl_indattr_funcs = __library.get(b"ob_type_decl_indattr_funcs\0").map(|sym| *sym)?;
        let ob_is_an_ancestor = __library.get(b"ob_is_an_ancestor\0").map(|sym| *sym)?;
        let ob_is_an_ancestor_excl = __library.get(b"ob_is_an_ancestor_excl\0").map(|sym| *sym)?;
        let ob_find_type_ancestor = __library.get(b"ob_find_type_ancestor\0").map(|sym| *sym)?;
        let ob_find_common_ancestor = __library.get(b"ob_find_common_ancestor\0").map(|sym| *sym)?;
        let ob_get_ancestor_system_class =
            __library.get(b"ob_get_ancestor_system_class\0").map(|sym| *sym)?;
        let ob_get_runtime_class = __library.get(b"ob_get_runtime_class\0").map(|sym| *sym)?;
        let ob_get_func_vtable_entry = __library.get(b"ob_get_func_vtable_entry\0").map(|sym| *sym)?;
        let ob_rout_declare = __library.get(b"ob_rout_declare\0").map(|sym| *sym)?;
        let ob_open_routine = __library.get(b"ob_open_routine\0").map(|sym| *sym)?;
        let ob_close_routine = __library.get(b"ob_close_routine\0").map(|sym| *sym)?;
        let ob_func_indirect = __library.get(b"ob_func_indirect\0").map(|sym| *sym)?;
        let ob_local_var_declare = __library.get(b"ob_local_var_declare\0").map(|sym| *sym)?;
        let ob_local_array_declare = __library.get(b"ob_local_array_declare\0").map(|sym| *sym)?;
        let ob_local_var_reference = __library.get(b"ob_local_var_reference\0").map(|sym| *sym)?;
        let ob_local_set_var = __library.get(b"ob_local_set_var\0").map(|sym| *sym)?;
        let ob_local_set_id_var = __library.get(b"ob_local_set_id_var\0").map(|sym| *sym)?;
        let ob_set_const = __library.get(b"ob_set_const\0").map(|sym| *sym)?;
        let ob_get_const = __library.get(b"ob_get_const\0").map(|sym| *sym)?;
        let ob_convert_vars_to_const = __library.get(b"ob_convert_vars_to_const\0").map(|sym| *sym)?;
        let ob_clear_group_objects = __library.get(b"ob_clear_group_objects\0").map(|sym| *sym)?;
        let ob_init_group_objects = __library.get(b"ob_init_group_objects\0").map(|sym| *sym)?;
        let shformatDateTimeWeb = __library.get(b"shformatDateTimeWeb\0").map(|sym| *sym)?;
        let shformatDateTime = __library.get(b"shformatDateTime\0").map(|sym| *sym)?;
        let shformatDecimal = __library.get(b"shformatDecimal\0").map(|sym| *sym)?;
        let shformatDecimalWeb = __library.get(b"shformatDecimalWeb\0").map(|sym| *sym)?;
        let shformatDouble = __library.get(b"shformatDouble\0").map(|sym| *sym)?;
        let shformatDoubleWeb = __library.get(b"shformatDoubleWeb\0").map(|sym| *sym)?;
        let shformatLonglong = __library.get(b"shformatLonglong\0").map(|sym| *sym)?;
        let shformatLonglongWeb = __library.get(b"shformatLonglongWeb\0").map(|sym| *sym)?;
        let shformatReal = __library.get(b"shformatReal\0").map(|sym| *sym)?;
        let shformatRealWeb = __library.get(b"shformatRealWeb\0").map(|sym| *sym)?;
        let shformatString = __library.get(b"shformatString\0").map(|sym| *sym)?;
        let shformatCmplDateTimeMask = __library.get(b"shformatCmplDateTimeMask\0").map(|sym| *sym)?;
        let shformatCmplDateTimeMaskWeb = __library.get(b"shformatCmplDateTimeMaskWeb\0").map(|sym| *sym)?;
        let shformatCmplNumericMask = __library.get(b"shformatCmplNumericMask\0").map(|sym| *sym)?;
        let shformatCmplNumericMaskWeb = __library.get(b"shformatCmplNumericMaskWeb\0").map(|sym| *sym)?;
        let shformatCmplNumericMaskWebCommasPos =
            __library.get(b"shformatCmplNumericMaskWebCommasPos\0").map(|sym| *sym)?;
        let shformatCmplStringMask = __library.get(b"shformatCmplStringMask\0").map(|sym| *sym)?;
        let shformatErrorString = __library.get(b"shformatErrorString\0").map(|sym| *sym)?;
        let ob_add_glbsym_var = __library.get(b"ob_add_glbsym_var\0").map(|sym| *sym)?;
        let ob_add_glbsym_class = __library.get(b"ob_add_glbsym_class\0").map(|sym| *sym)?;
        let ob_add_glbsym_func = __library.get(b"ob_add_glbsym_func\0").map(|sym| *sym)?;
        let rt_set_class_handle = __library.get(b"rt_set_class_handle\0").map(|sym| *sym)?;
        let rt_init = __library.get(b"rt_init\0").map(|sym| *sym)?;
        let rt_start_debug = __library.get(b"rt_start_debug\0").map(|sym| *sym)?;
        let rt_stop_debug = __library.get(b"rt_stop_debug\0").map(|sym| *sym)?;
        let rt_set_pcode_to_line = __library.get(b"rt_set_pcode_to_line\0").map(|sym| *sym)?;
        let rt_breakpoint = __library.get(b"rt_breakpoint\0").map(|sym| *sym)?;
        let rt_create_watchpoint = __library.get(b"rt_create_watchpoint\0").map(|sym| *sym)?;
        let rt_find_watchpoint_for_watchid =
            __library.get(b"rt_find_watchpoint_for_watchid\0").map(|sym| *sym)?;
        let rt_delete_watchpoint = __library.get(b"rt_delete_watchpoint\0").map(|sym| *sym)?;
        let rt_is_line_executable = __library.get(b"rt_is_line_executable\0").map(|sym| *sym)?;
        let rt_closest_executable_line = __library.get(b"rt_closest_executable_line\0").map(|sym| *sym)?;
        let rt_start_run = __library.get(b"rt_start_run\0").map(|sym| *sym)?;
        let rt_stop_run = __library.get(b"rt_stop_run\0").map(|sym| *sym)?;
        let rt_create_obinst = __library.get(b"rt_create_obinst\0").map(|sym| *sym)?;
        let rtReturnValGet = __library.get(b"rtReturnValGet\0").map(|sym| *sym)?;
        let rtReturnValFree = __library.get(b"rtReturnValFree\0").map(|sym| *sym)?;
        let rt_error = __library.get(b"rt_error\0").map(|sym| *sym)?;
        let rt_free_error_struct = __library.get(b"rt_free_error_struct\0").map(|sym| *sym)?;
        let rt_error_using_struct = __library.get(b"rt_error_using_struct\0").map(|sym| *sym)?;
        let rt_normalize_error_id = __library.get(b"rt_normalize_error_id\0").map(|sym| *sym)?;
        let ot_handle_exception = __library.get(b"ot_handle_exception\0").map(|sym| *sym)?;
        let ob_dbg_pop_call_stack_ntimes =
            __library.get(b"ob_dbg_pop_call_stack_ntimes\0").map(|sym| *sym)?;
        let ob_dbg_push_call_stack_ntimes =
            __library.get(b"ob_dbg_push_call_stack_ntimes\0").map(|sym| *sym)?;
        let ob_get_current_stack_location =
            __library.get(b"ob_get_current_stack_location\0").map(|sym| *sym)?;
        let rtRoutineSearch = __library.get(b"rtRoutineSearch\0").map(|sym| *sym)?;
        let rtRoutineExec = __library.get(b"rtRoutineExec\0").map(|sym| *sym)?;
        let rtRoutineExecByName = __library.get(b"rtRoutineExecByName\0").map(|sym| *sym)?;
        let rtRoutineExecPosted = __library.get(b"rtRoutineExecPosted\0").map(|sym| *sym)?;
        let rtRoutineInfo = __library.get(b"rtRoutineInfo\0").map(|sym| *sym)?;
        let rtInitializeInfoForCall = __library.get(b"rtInitializeInfoForCall\0").map(|sym| *sym)?;
        let rtCleanupInfoAfterCall = __library.get(b"rtCleanupInfoAfterCall\0").map(|sym| *sym)?;
        let rtRoutineCount = __library.get(b"rtRoutineCount\0").map(|sym| *sym)?;
        let rtReferenceArgCreate = __library.get(b"rtReferenceArgCreate\0").map(|sym| *sym)?;
        let rtReferenceArgFree = __library.get(b"rtReferenceArgFree\0").map(|sym| *sym)?;
        let rtGetClassDescrip = __library.get(b"rtGetClassDescrip\0").map(|sym| *sym)?;
        let rtDataFree = __library.get(b"rtDataFree\0").map(|sym| *sym)?;
        let rtDataCopy = __library.get(b"rtDataCopy\0").map(|sym| *sym)?;
        let rt_hit_level_0 = __library.get(b"rt_hit_level_0\0").map(|sym| *sym)?;
        let ob_create_object = __library.get(b"ob_create_object\0").map(|sym| *sym)?;
        let ob_create_object_using = __library.get(b"ob_create_object_using\0").map(|sym| *sym)?;
        let ob_copy_rtinst = __library.get(b"ob_copy_rtinst\0").map(|sym| *sym)?;
        let ob_destroy_rtinst = __library.get(b"ob_destroy_rtinst\0").map(|sym| *sym)?;
        let ob_get_primary_rtinst = __library.get(b"ob_get_primary_rtinst\0").map(|sym| *sym)?;
        let ob_is_rtinst_autoinstantiate =
            __library.get(b"ob_is_rtinst_autoinstantiate\0").map(|sym| *sym)?;
        let ob_object_compare = __library.get(b"ob_object_compare\0").map(|sym| *sym)?;
        let ob_invoke_static = __library.get(b"ob_invoke_static\0").map(|sym| *sym)?;
        let ob_invoke_dynamic = __library.get(b"ob_invoke_dynamic\0").map(|sym| *sym)?;
        let ob_invoke_staticAsync = __library.get(b"ob_invoke_staticAsync\0").map(|sym| *sym)?;
        let ob_invoke_dynamicAsync = __library.get(b"ob_invoke_dynamicAsync\0").map(|sym| *sym)?;
        let ob_instance_lv = __library.get(b"ob_instance_lv\0").map(|sym| *sym)?;
        let ob_instance_fldupdate_refpkt =
            __library.get(b"ob_instance_fldupdate_refpkt\0").map(|sym| *sym)?;
        let ob_instance_flditemupdate_refpkt =
            __library.get(b"ob_instance_flditemupdate_refpkt\0").map(|sym| *sym)?;
        let ob_instance_simple_refpkt = __library.get(b"ob_instance_simple_refpkt\0").map(|sym| *sym)?;
        let ob_get_group_load_state = __library.get(b"ob_get_group_load_state\0").map(|sym| *sym)?;
        let ob_get_groupref_group = __library.get(b"ob_get_groupref_group\0").map(|sym| *sym)?;
        let ob_group_get_next_index = __library.get(b"ob_group_get_next_index\0").map(|sym| *sym)?;
        let ob_close_typedef_group = __library.get(b"ob_close_typedef_group\0").map(|sym| *sym)?;
        let ob_create_group_structure = __library.get(b"ob_create_group_structure\0").map(|sym| *sym)?;
        let ob_new_group = __library.get(b"ob_new_group\0").map(|sym| *sym)?;
        let ob_del_group_structure = __library.get(b"ob_del_group_structure\0").map(|sym| *sym)?;
        let ob_group_data_srch = __library.get(b"ob_group_data_srch\0").map(|sym| *sym)?;
        let ob_replace_group = __library.get(b"ob_replace_group\0").map(|sym| *sym)?;
        let ob_copy_group_shrsym_data = __library.get(b"ob_copy_group_shrsym_data\0").map(|sym| *sym)?;
        let ob_get_qualified_name_with_namespace =
            __library.get(b"ob_get_qualified_name_with_namespace\0").map(|sym| *sym)?;
        let ob_get_source_from_group = __library.get(b"ob_get_source_from_group\0").map(|sym| *sym)?;
        let ob_get_var = __library.get(b"ob_get_var\0").map(|sym| *sym)?;
        let ob_init_var_data = __library.get(b"ob_init_var_data\0").map(|sym| *sym)?;
        let ob_global_indirect = __library.get(b"ob_global_indirect\0").map(|sym| *sym)?;
        let ob_global_var_declare = __library.get(b"ob_global_var_declare\0").map(|sym| *sym)?;
        let ob_global_array_declare = __library.get(b"ob_global_array_declare\0").map(|sym| *sym)?;
        let ob_shared_var_reference = __library.get(b"ob_shared_var_reference\0").map(|sym| *sym)?;
        let ob_global_set_var = __library.get(b"ob_global_set_var\0").map(|sym| *sym)?;
        let ob_global_set_id_var = __library.get(b"ob_global_set_id_var\0").map(|sym| *sym)?;
        let ob_get_local_symtab = __library.get(b"ob_get_local_symtab\0").map(|sym| *sym)?;
        let ob_get_unconverted_var = __library.get(b"ob_get_unconverted_var\0").map(|sym| *sym)?;
        let ob_lookup_shared_var_info = __library.get(b"ob_lookup_shared_var_info\0").map(|sym| *sym)?;
        let ob_clear_shared_vars = __library.get(b"ob_clear_shared_vars\0").map(|sym| *sym)?;
        let ot_eval_expr = __library.get(b"ot_eval_expr\0").map(|sym| *sym)?;
        let ot_dbg_funccall = __library.get(b"ot_dbg_funccall\0").map(|sym| *sym)?;
        let ot_run_dllfunccall = __library.get(b"ot_run_dllfunccall\0").map(|sym| *sym)?;
        let ot_run_rpcfunccall = __library.get(b"ot_run_rpcfunccall\0").map(|sym| *sym)?;
        let ot_get_dll_funcptr_by_name = __library.get(b"ot_get_dll_funcptr_by_name\0").map(|sym| *sym)?;
        let ot_post_call = __library.get(b"ot_post_call\0").map(|sym| *sym)?;
        let ot_check_types = __library.get(b"ot_check_types\0").map(|sym| *sym)?;
        let ot_type_loc = __library.get(b"ot_type_loc\0").map(|sym| *sym)?;
        let ot_init_data_node = __library.get(b"ot_init_data_node\0").map(|sym| *sym)?;
        let ot_set_lvalue = __library.get(b"ot_set_lvalue\0").map(|sym| *sym)?;
        let ot_free_out_node = __library.get(b"ot_free_out_node\0").map(|sym| *sym)?;
        let ot_free_inv_meth_args = __library.get(b"ot_free_inv_meth_args\0").map(|sym| *sym)?;
        let ot_copy_array = __library.get(b"ot_copy_array\0").map(|sym| *sym)?;
        let ot_get_string_from_chararray =
            __library.get(b"ot_get_string_from_chararray\0").map(|sym| *sym)?;
        let ot_create_chararray_from_string =
            __library.get(b"ot_create_chararray_from_string\0").map(|sym| *sym)?;
        let ot_create_bounded_chararray_from_string =
            __library.get(b"ot_create_bounded_chararray_from_string\0").map(|sym| *sym)?;
        let ot_get_char_value = __library.get(b"ot_get_char_value\0").map(|sym| *sym)?;
        let ot_get_string_value = __library.get(b"ot_get_string_value\0").map(|sym| *sym)?;
        let ot_get_string_from_char = __library.get(b"ot_get_string_from_char\0").map(|sym| *sym)?;
        let ot_string_cat = __library.get(b"ot_string_cat\0").map(|sym| *sym)?;
        let ot_binary_cat = __library.get(b"ot_binary_cat\0").map(|sym| *sym)?;
        let ot_halt = __library.get(b"ot_halt\0").map(|sym| *sym)?;
        let ot_convert_bounded_to_bounded =
            __library.get(b"ot_convert_bounded_to_bounded\0").map(|sym| *sym)?;
        let ot_convert_bounded_to_unbounded =
            __library.get(b"ot_convert_bounded_to_unbounded\0").map(|sym| *sym)?;
        let ot_convert_unbounded_to_bounded =
            __library.get(b"ot_convert_unbounded_to_bounded\0").map(|sym| *sym)?;
        let ot_convert_unbounded_to_unbounded =
            __library.get(b"ot_convert_unbounded_to_unbounded\0").map(|sym| *sym)?;
        let ot_convert_any_to_bounded = __library.get(b"ot_convert_any_to_bounded\0").map(|sym| *sym)?;
        let ot_convert_any_to_unbounded = __library.get(b"ot_convert_any_to_unbounded\0").map(|sym| *sym)?;
        let ot_convert_array_to_object = __library.get(b"ot_convert_array_to_object\0").map(|sym| *sym)?;
        let ot_build_simple_refpak = __library.get(b"ot_build_simple_refpak\0").map(|sym| *sym)?;
        let ot_build_field_refpak = __library.get(b"ot_build_field_refpak\0").map(|sym| *sym)?;
        let ot_add_any = __library.get(b"ot_add_any\0").map(|sym| *sym)?;
        let ot_sub_any = __library.get(b"ot_sub_any\0").map(|sym| *sym)?;
        let ot_mul_any = __library.get(b"ot_mul_any\0").map(|sym| *sym)?;
        let ot_div_any = __library.get(b"ot_div_any\0").map(|sym| *sym)?;
        let ot_pow_any = __library.get(b"ot_pow_any\0").map(|sym| *sym)?;
        let ot_neg_any = __library.get(b"ot_neg_any\0").map(|sym| *sym)?;
        let ot_eq_any = __library.get(b"ot_eq_any\0").map(|sym| *sym)?;
        let ot_ne_any = __library.get(b"ot_ne_any\0").map(|sym| *sym)?;
        let ot_gt_any = __library.get(b"ot_gt_any\0").map(|sym| *sym)?;
        let ot_lt_any = __library.get(b"ot_lt_any\0").map(|sym| *sym)?;
        let ot_ge_any = __library.get(b"ot_ge_any\0").map(|sym| *sym)?;
        let ot_le_any = __library.get(b"ot_le_any\0").map(|sym| *sym)?;
        let ot_and_any = __library.get(b"ot_and_any\0").map(|sym| *sym)?;
        let ot_or_any = __library.get(b"ot_or_any\0").map(|sym| *sym)?;
        let ot_not_any = __library.get(b"ot_not_any\0").map(|sym| *sym)?;
        let ot_incr_any = __library.get(b"ot_incr_any\0").map(|sym| *sym)?;
        let ot_decr_any = __library.get(b"ot_decr_any\0").map(|sym| *sym)?;
        let ot_mod_any = __library.get(b"ot_mod_any\0").map(|sym| *sym)?;
        let ot_min_any = __library.get(b"ot_min_any\0").map(|sym| *sym)?;
        let ot_max_any = __library.get(b"ot_max_any\0").map(|sym| *sym)?;
        let ot_check_any_exact_type = __library.get(b"ot_check_any_exact_type\0").map(|sym| *sym)?;
        let ot_check_any_string_type = __library.get(b"ot_check_any_string_type\0").map(|sym| *sym)?;
        let ot_check_any_binary_type = __library.get(b"ot_check_any_binary_type\0").map(|sym| *sym)?;
        let ot_check_any_math_type = __library.get(b"ot_check_any_math_type\0").map(|sym| *sym)?;
        let ot_check_any_enum_type = __library.get(b"ot_check_any_enum_type\0").map(|sym| *sym)?;
        let ot_check_any_object_type = __library.get(b"ot_check_any_object_type\0").map(|sym| *sym)?;
        let ot_duplicate_any = __library.get(b"ot_duplicate_any\0").map(|sym| *sym)?;
        let ot_abs_any = __library.get(b"ot_abs_any\0").map(|sym| *sym)?;
        let ot_ceiling_any = __library.get(b"ot_ceiling_any\0").map(|sym| *sym)?;
        let ot_string_to_binary = __library.get(b"ot_string_to_binary\0").map(|sym| *sym)?;
        let ot_bytearray_to_binary = __library.get(b"ot_bytearray_to_binary\0").map(|sym| *sym)?;
        let ot_any_to_binary = __library.get(b"ot_any_to_binary\0").map(|sym| *sym)?;
        let ob_set_curr_rtinst_and_return =
            __library.get(b"ob_set_curr_rtinst_and_return\0").map(|sym| *sym)?;
        let ob_unset_curr_rtinst_and_return =
            __library.get(b"ob_unset_curr_rtinst_and_return\0").map(|sym| *sym)?;
        let ob_open_trace = __library.get(b"ob_open_trace\0").map(|sym| *sym)?;
        let ob_close_trace = __library.get(b"ob_close_trace\0").map(|sym| *sym)?;
        let ob_begin_trace = __library.get(b"ob_begin_trace\0").map(|sym| *sym)?;
        let ob_end_trace = __library.get(b"ob_end_trace\0").map(|sym| *sym)?;
        let ob_enable_event_trace = __library.get(b"ob_enable_event_trace\0").map(|sym| *sym)?;
        let ob_disable_event_trace = __library.get(b"ob_disable_event_trace\0").map(|sym| *sym)?;
        Ok(Api {
            __library,
            __version,
            pbstg_begin,
            pbstg_begin_allocflags,
            pbstg_begin_nofast,
            pbstg_end,
            pbstg_free_pool,
            pbstg_new_pool,
            pbstg_new_pool_nofast,
            pbstg_new_pool_with_size_nofast,
            pbstg_set_pool_name,
            pbstg_set_poolpagesize,
            pbstg_write_debug,
            pbstg_stat,
            pbstg_nextGeneration,
            pbstg_dumpLeaks,
            pbstg_dumpHeap,
            pbstg_alloc,
            pbstg_free,
            pbstg_realloc,
            pbstg_size,
            pbstg_fast_strlen,
            pbstg_ansitoupper,
            pbstg_ansitolower,
            pbstg_strdup,
            pbstg_strdup_malloc,
            pbstg_str_build,
            pbstg_str_build_char,
            pbstg_str_build_huge,
            pbstg_str_remove_char,
            pbstg_str_trim_left,
            pbstg_str_trim_right,
            pbstg_str_trim,
            pbstg_str_wordcap,
            pbstg_atoi_imp,
            pbstg_atof_imp,
            pbstg_strtod_imp,
            pbstg_atol_imp,
            pbstg_strtol_imp,
            pbstg_atou_imp,
            pbstg_atoul_imp,
            pbstg_strtoul_imp,
            pbstg_remove_imp,
            pbstg_dde_alloc,
            pbstg_dde_free,
            pbstg_dde_get_handle,
            pbstg_dde_lock,
            pbstg_dde_unlock,
            pbstg_huge_memcmp,
            pbstg_huge_memcpy,
            pbstg_huge_memmove,
            pbstg_huge_memset,
            pbstg_huge_strchr,
            pbstg_huge_strcpy,
            pbstg_huge_strlen,
            pbstg_huge_strncpy,
            pbstg_huge_strstr,
            pbstg_unicodestrdup,
            pbstg_unicodestr_build,
            pbstg_strtounicodedup,
            pbstg_unicodetostrdup,
            pbstg_strtoansidup,
            pbstg_ansitostrdup,
            pbstg_strtoprintable,
            pbstg_strtoprintabledup,
            pbstg_printabletostr,
            pbstg_printabletostrdup,
            pbstg_lchrcmp,
            pbstg_lchrcmpi,
            ob_set_session_icontext,
            rt_move_thread,
            rt_clear_thread,
            rt_get_current_this,
            rt_add_task,
            rt_free_task,
            rt_get_current_task_info,
            rt_set_current_task_info,
            rt_get_free_task_slot,
            rt_is_running_exe,
            ob_add_const_data,
            ob_looksym_keyfunc,
            ob_looksym_reference,
            ob_looksym_delete,
            ob_dynarray_index,
            ob_dynarray_grow,
            ob_narray_create_static,
            ob_narray_create_dynamic,
            ob_set_arraydef,
            ob_get_array_len,
            ob_array_item_init_callback,
            ob_init_array,
            ob_array_varinfo_nullval,
            ob_array_set_varinfo_nullval,
            ob_remove_array_data,
            ob_init_pcode_blk,
            ob_del_pcode_blk,
            ob_reuse_routine,
            shMaxDec,
            shMinDec,
            shCompareDec,
            shAbsDec,
            shNegateDec,
            shRoundDec,
            shTruncDec,
            shAddDec,
            shSubDec,
            shMultDec,
            shDivDec,
            shModDec,
            shExpDec,
            shIntToDec,
            shDecToInt,
            shUintToDec,
            shDecToUint,
            shByteToDec,
            shDecToByte,
            shLongToDec,
            shDecToLong,
            shUlongToDec,
            shDecToUlong,
            shLonglongToDec,
            shDecToLonglong,
            shDecToFloat,
            shFloatToDec,
            shDoubleToDec,
            shDecToDouble,
            shDecToAscii,
            shAsciiToDec,
            shAsciiToDecRnd,
            shSetDecFractions,
            shSetDecNegative,
            shDecSetOverflow,
            ob_mgr_init,
            ob_mgr_init_ex,
            ob_mgr_restart,
            ob_mgr_terminate,
            ob_free_memory,
            ob_free_link_error_list,
            ob_get_link_error_list,
            ob_enter_critical_section,
            ob_leave_critical_section,
            ob_alloc_string,
            ob_alloc_blob,
            ob_alloc_dec,
            ob_alloc_double,
            ob_alloc_longlong,
            ob_alloc_time,
            ob_realloc_string,
            ob_realloc_blob,
            ob_dup_string,
            ob_dup_blob,
            ob_dup_dec,
            ob_dup_double,
            ob_dup_longlong,
            ob_dup_time,
            ob_free_value,
            ob_create_appl_report,
            ob_create_object_report,
            ob_free_appl_report,
            ob_get_mode,
            ob_set_mode,
            ob_get_field,
            ob_set_field,
            ob_get_field_data,
            ob_get_no_fields,
            ob_get_parent_obinst,
            ob_get_first_user_field,
            ob_get_field_type,
            ob_get_int_field,
            ob_get_uint_field,
            ob_get_byte_field,
            ob_get_long_field,
            ob_get_ulong_field,
            ob_get_float_field,
            ob_get_ptr_field,
            ob_get_inst_field,
            ob_get_array_field,
            ob_array_index,
            ob_get_indirect_obdata,
            ob_array_item,
            ob_array_get_index_from_subs,
            ob_array_calc_index,
            ob_set_int_field,
            ob_set_uint_field,
            ob_set_long_field,
            ob_set_ulong_field,
            ob_set_float_field,
            ob_set_ptr_field,
            ob_set_array_field,
            ob_set_obinst_field,
            ob_set_underlying_object,
            ob_get_underlying_object,
            ob_is_any_group_locked,
            ob_get_group_lock_count,
            ob_is_group_locked,
            ob_is_group_unlocked,
            ob_is_group_write_locked,
            ob_lock_group,
            ob_unlock_group,
            ob_clear_unlocked_groups,
            ob_clear_all_other_unlocked_groups,
            ob_is_ancestor_locked,
            ob_is_descendent_locked,
            ob_validate_liblist,
            ob_set_liblist,
            ob_get_liblist,
            ob_set_default_appl,
            ob_load_appl_group,
            ob_is_group_in_memory,
            ob_group_declare,
            ob_group_reference,
            ob_get_group_name,
            ob_get_group_full_name,
            ob_group_save,
            ob_group_save_win,
            ob_load_group_source,
            ob_rename_group,
            ob_move_group,
            ob_move_group_with_name,
            ob_copy_group_with_name,
            ob_copy_group,
            ob_delete_group,
            ob_delete_group_with_name,
            ob_restore_group,
            ob_save_working_group,
            ob_delete_working_group,
            ob_restore_working_group,
            ob_open_group_id,
            ob_close_group,
            ob_get_group_lib,
            ob_run_garbage_collection,
            ob_delete_instlist_shlist,
            ob_get_group_instlist_as_shlist,
            ob_delete_groups_shlist,
            ob_get_groups_shlist,
            ob_store_source,
            ob_init_source,
            ob_store_global_src,
            ob_store_namespace_decl_src,
            ob_store_shared_src,
            ob_store_prototype_source,
            ob_store_instvar_source,
            ob_get_global_src,
            ob_get_namespace_decl_src,
            ob_get_shared_src,
            ob_get_prototype_source,
            ob_get_instvar_source,
            ob_get_routine_src,
            ob_decl_and_store_routine_src,
            ob_store_routine_src,
            ob_store_create_src,
            ob_store_destroy_src,
            ob_get_function_src,
            ob_store_function_src,
            ob_symbol_search_extended,
            ob_symbol_search,
            ob_class_declare,
            ob_get_full_qualified_typename,
            ob_class_declare_inh,
            ob_class_reference,
            ob_class_name,
            ob_class_name_not_indirect,
            ob_get_type_name,
            ob_classhndl_indirect,
            ob_get_parent_class,
            ob_get_within_class,
            ob_class_delete,
            ob_class_rename,
            ob_is_a_system_class,
            ob_is_class_inherited,
            ob_is_class_descendant,
            ob_is_inh_from_user_class,
            ob_get_sec_class_ancestor,
            ob_is_class_enum,
            ob_new_event,
            ob_update_event,
            ob_delete_event,
            ob_has_events,
            ob_get_event_token_id,
            ob_get_event_id_from_token,
            ob_does_event_script_exist,
            ob_get_routine_name,
            ob_delete_routine,
            ob_get_curr_routine,
            ob_get_curr_function,
            ob_get_routid_from_vtable_id,
            ob_is_valid_event_index,
            ob_has_scripts,
            ob_get_routine_type,
            ob_get_function_vtable_ids,
            ob_get_function_vtable_ids_for_ide,
            ob_get_event_vtable_ids,
            ob_get_function_name,
            ob_delete_function,
            ob_find_routine,
            ob_get_vtable_id_from_proto_id,
            ob_get_dll_func_names,
            ob_get_global_func_names_in_lib,
            ob_get_global_func_index,
            ob_get_func_index_in_lib,
            ob_get_proto_is_external_event,
            ob_get_protoarg_info,
            ob_get_proto_info,
            ob_get_method_signature,
            ob_was_event_prototype_changed,
            ob_get_proto_name_info,
            ob_get_proto_throws_info,
            ob_lookup_routine_by_name,
            ob_get_objnames_of_class,
            ob_has_object_of_class,
            ob_get_obj_classhndls_of_class,
            ob_get_objnames_of_class_in_lib,
            ob_global_reference,
            ob_global_reference_in_lib,
            ob_global_reference_of_class,
            ob_get_obinst_class_hndl,
            ob_is_a_typedef,
            ob_is_an_enum,
            ob_get_system_class,
            ob_get_obinst_system_class,
            ob_get_obinst_group_hndl,
            ob_get_obinst_class_name,
            ob_fetch_fields_of_class,
            ob_get_fields_of_class,
            ob_get_local_var_info,
            ob_get_shared_vars_of_class,
            ob_get_shared_var_info,
            ob_get_global_vars_of_class,
            ob_get_class_field_info,
            ob_get_enum_info,
            ob_get_class_event_info,
            ob_get_instance_field_info,
            ob_get_obinst_field_info,
            ob_get_obinst_all_field_info,
            ob_get_classes_within_group,
            ob_get_enums_within_group,
            ob_get_global_var_data,
            ob_object_reference_count,
            ob_named_global_var_info,
            ob_named_shared_var_info,
            ob_named_special_var_info,
            ob_named_local_var_info,
            ob_named_field_info,
            ob_get_array_info,
            ob_get_array_bounds_string,
            ob_get_array_bounds_string_from_field_info,
            ob_get_info_watchpoint,
            ob_compile_lib_entry,
            ob_compile_lib_typedefs,
            ob_compile_lib_entry_3_pass,
            ob_compile_lib_scripts,
            ob_func_search,
            ob_del_funccall_info,
            ob_link_project,
            ob_check_for_locked_menu,
            ob_create_obinst,
            ob_instantiate_child_object,
            ob_instantiate_system_object,
            ob_destroy_obinst,
            ob_set_runtime,
            ob_create_executable,
            ob_create_library,
            ob_create_consolidated_library,
            ob_create_interface_class,
            ob_init_executable,
            ob_scan_source_blocks,
            ob_create_launcher,
            ob_sanitize_pb_name,
            ob_validate_class,
            ob_get_orphaned_classes,
            ob_validate_type_name,
            ob_convert_to_ver2_source,
            ob_is_vers2_obj,
            ob_build_ordered_compile_list,
            ob_free_ordered_compile_list,
            ob_build_hierarchy_list,
            ob_free_hierarchy_list,
            ob_clear_instance_ref,
            ob_insert_inst_ref_dbg,
            ob_open_typedef_group,
            ob_save_dll_to_pbd,
            ob_convert_pbx_to_native_groups,
            ob_share_typedef_group,
            ob_unshare_typedef_group,
            ob_cm_evaluate_expression,
            ob_entryInheritsFromClass,
            ob_get_class_from_name,
            ob_local_global_lv,
            ob_local_global_refpkt,
            ob_shared_global_lv,
            ob_shared_global_refpkt,
            ob_shared_lv,
            ob_shared_refpkt,
            ob_convert_chararray_to_string,
            ob_class_delete_and_withinclass,
            ob_find_orphan_class,
            ob_nuke_orphan_class,
            ob_is_ancestor_class_modified,
            ob_rebuild_instance_image,
            ob_build_compile_list,
            ot_get_next_evaled_arg,
            ot_get_next_evaled_arg_no_convert,
            ot_get_next_lvalue_arg,
            ot_get_simple_intarg,
            ot_get_simple_longarg,
            ot_get_intarg,
            ot_get_uintarg,
            ot_get_longarg,
            ot_get_ulongarg,
            ot_get_decarg,
            ot_get_floatarg,
            ot_get_doublearg,
            ot_get_longlongarg,
            ot_get_obinstarg,
            ot_get_valptr_arg,
            ot_init_arglist,
            ot_get_valptr,
            ot_type_srch,
            ot_type_attr,
            ot_get_class_name,
            ot_is_array_eq,
            ot_is_struct_eq,
            ot_create_obinst_with_name,
            ot_create_obinst_at_lval,
            ot_get_curr_obinst_expr,
            ot_func_call,
            ot_set_return_val,
            ot_set_return_double,
            ot_set_return_longlong,
            ot_set_return_dec,
            ot_no_return_val,
            ot_assign_lvalue_dec,
            ot_assign_lvalue_double,
            ot_assign_lvalue_longlong,
            ot_assign_lvalue_blob,
            ot_assign_lvalue_obinst,
            ot_assign_lvalue_array,
            ot_assign_lvalue_any,
            ot_set_local_var,
            ot_set_shared_var,
            ot_set_obinst_var,
            ot_set_local_array_item,
            ot_set_shared_array_item,
            ot_set_obinst_array_item,
            ot_get_array_values,
            ot_reset_array,
            ot_get_local_var,
            ot_get_shared_var,
            ot_math_type_convert,
            ot_get_int_value,
            ot_get_uint_value,
            ot_get_byte_value,
            ot_get_long_value,
            ot_get_ulong_value,
            ot_get_dec_value,
            ot_get_float_value,
            ot_get_double_value,
            ot_get_longlong_value,
            ot_free_val_ptr,
            ot_free_array,
            ot_convert_to_int,
            ot_convert_to_uint,
            ot_convert_to_byte,
            ot_convert_to_long,
            ot_convert_to_ulong,
            ot_convert_to_dec,
            ot_convert_to_float,
            ot_convert_to_double,
            ot_convert_to_longlong,
            ot_ansi_lower,
            ot_ansi_upper,
            ot_ansi_strcmp,
            ot_get_field_lv,
            ot_get_field_item_lv,
            ot_assign_ref_int,
            ot_assign_ref_uint,
            ot_assign_ref_byte,
            ot_assign_ref_long,
            ot_assign_ref_ulong,
            ot_assign_ref_dec,
            ot_assign_ref_float,
            ot_assign_ref_double,
            ot_assign_ref_longlong,
            ot_assign_ref_string,
            ot_assign_ref_bool,
            ot_assign_ref_char,
            ot_assign_ref_blob,
            ot_assign_ref_time,
            ot_assign_ref_date,
            ot_assign_ref_datetime,
            ot_assign_ref_obinst,
            ot_assign_ref_enum,
            ot_assign_ref_array,
            ot_assign_ref_any,
            ot_get_nested_obinst,
            ot_array_create_bounded,
            ot_array_create_unbounded,
            ot_array_index,
            ot_array_set_free_data,
            ot_array_free_data,
            ot_array_class_id,
            ot_array_class_hndl,
            ot_array_num_dimensions,
            ot_array_num_items,
            ot_is_array_unbounded,
            ot_get_arraydef_no_dims,
            ot_get_arraydef_style,
            ot_get_arraydef_bounds,
            ot_get_arraydef_varinfo,
            ot_get_arraydef_upper_bound,
            ot_get_arraydef_lower_bound,
            ot_randomize,
            ot_rand,
            ot_class_compare,
            ot_assign_global_var_obinst,
            ob_class_indirect,
            ob_add_external_class_ref,
            ob_get_local_class,
            ob_get_primary_class,
            ob_build_qual_sec_class_name,
            ob_decl_indirect_sec_class,
            ob_update_class_ref,
            ob_update_glob_class_instflag,
            ob_is_class_member_accessable,
            ob_get_system_func_class,
            ob_get_global_func_class,
            ob_type_declare,
            ob_type_declare_class,
            ob_type_declare_vtab,
            ob_type_reference,
            ob_get_first_type,
            ob_get_next_type,
            ob_type_init_process,
            ob_type_decl_process,
            ob_get_nested_class,
            ob_get_class_entry,
            ob_is_class_indirect,
            ob_fetch_routine,
            ob_type_proto_decl,
            ob_type_proto_ref,
            ob_proto_error_upgrade,
            ob_get_proto_access_type,
            ob_type_process_protos,
            ob_type_reprocess_protos,
            ob_get_type_proto_names,
            ob_declare_external_event_type,
            ob_get_type_proto_names_for_ide,
            ob_type_vtable_module_srch,
            ob_get_prototype,
            ob_update_proto_mod_id,
            ob_update_proto_rout_id,
            ob_protolist_read,
            ob_protolist_write,
            ob_prototype_match_for_event,
            ob_prototype_search,
            ob_proto_overload_search,
            ob_event_module_name,
            ob_find_first_event,
            ob_type_event_script_srch,
            ob_build_proto_vtable,
            ob_type_field_decl,
            ob_type_field_search,
            ob_type_field_ref,
            ob_get_type_field_info,
            ob_set_field_init_value,
            ob_get_field_init_value,
            ob_type_field_clear_instvars,
            ob_convert_fields_to_const,
            ob_build_instance_image,
            ob_field_decl_indattr_funcs,
            ob_field_get_indattr_funcs,
            ob_field_requires_update_notification,
            ob_get_field_symtab,
            ob_enum_entry_decl,
            ob_enum_decl_process,
            ob_enum_reference,
            ob_get_type_enum_info,
            ob_is_type_enum,
            ob_type_indattr_search,
            ob_type_decl_indattr_funcs,
            ob_is_an_ancestor,
            ob_is_an_ancestor_excl,
            ob_find_type_ancestor,
            ob_find_common_ancestor,
            ob_get_ancestor_system_class,
            ob_get_runtime_class,
            ob_get_func_vtable_entry,
            ob_rout_declare,
            ob_open_routine,
            ob_close_routine,
            ob_func_indirect,
            ob_local_var_declare,
            ob_local_array_declare,
            ob_local_var_reference,
            ob_local_set_var,
            ob_local_set_id_var,
            ob_set_const,
            ob_get_const,
            ob_convert_vars_to_const,
            ob_clear_group_objects,
            ob_init_group_objects,
            shformatDateTimeWeb,
            shformatDateTime,
            shformatDecimal,
            shformatDecimalWeb,
            shformatDouble,
            shformatDoubleWeb,
            shformatLonglong,
            shformatLonglongWeb,
            shformatReal,
            shformatRealWeb,
            shformatString,
            shformatCmplDateTimeMask,
            shformatCmplDateTimeMaskWeb,
            shformatCmplNumericMask,
            shformatCmplNumericMaskWeb,
            shformatCmplNumericMaskWebCommasPos,
            shformatCmplStringMask,
            shformatErrorString,
            ob_add_glbsym_var,
            ob_add_glbsym_class,
            ob_add_glbsym_func,
            rt_set_class_handle,
            rt_init,
            rt_start_debug,
            rt_stop_debug,
            rt_set_pcode_to_line,
            rt_breakpoint,
            rt_create_watchpoint,
            rt_find_watchpoint_for_watchid,
            rt_delete_watchpoint,
            rt_is_line_executable,
            rt_closest_executable_line,
            rt_start_run,
            rt_stop_run,
            rt_create_obinst,
            rtReturnValGet,
            rtReturnValFree,
            rt_error,
            rt_free_error_struct,
            rt_error_using_struct,
            rt_normalize_error_id,
            ot_handle_exception,
            ob_dbg_pop_call_stack_ntimes,
            ob_dbg_push_call_stack_ntimes,
            ob_get_current_stack_location,
            rtRoutineSearch,
            rtRoutineExec,
            rtRoutineExecByName,
            rtRoutineExecPosted,
            rtRoutineInfo,
            rtInitializeInfoForCall,
            rtCleanupInfoAfterCall,
            rtRoutineCount,
            rtReferenceArgCreate,
            rtReferenceArgFree,
            rtGetClassDescrip,
            rtDataFree,
            rtDataCopy,
            rt_hit_level_0,
            ob_create_object,
            ob_create_object_using,
            ob_copy_rtinst,
            ob_destroy_rtinst,
            ob_get_primary_rtinst,
            ob_is_rtinst_autoinstantiate,
            ob_object_compare,
            ob_invoke_static,
            ob_invoke_dynamic,
            ob_invoke_staticAsync,
            ob_invoke_dynamicAsync,
            ob_instance_lv,
            ob_instance_fldupdate_refpkt,
            ob_instance_flditemupdate_refpkt,
            ob_instance_simple_refpkt,
            ob_get_group_load_state,
            ob_get_groupref_group,
            ob_group_get_next_index,
            ob_close_typedef_group,
            ob_create_group_structure,
            ob_new_group,
            ob_del_group_structure,
            ob_group_data_srch,
            ob_replace_group,
            ob_copy_group_shrsym_data,
            ob_get_qualified_name_with_namespace,
            ob_get_source_from_group,
            ob_get_var,
            ob_init_var_data,
            ob_global_indirect,
            ob_global_var_declare,
            ob_global_array_declare,
            ob_shared_var_reference,
            ob_global_set_var,
            ob_global_set_id_var,
            ob_get_local_symtab,
            ob_get_unconverted_var,
            ob_lookup_shared_var_info,
            ob_clear_shared_vars,
            ot_eval_expr,
            ot_dbg_funccall,
            ot_run_dllfunccall,
            ot_run_rpcfunccall,
            ot_get_dll_funcptr_by_name,
            ot_post_call,
            ot_check_types,
            ot_type_loc,
            ot_init_data_node,
            ot_set_lvalue,
            ot_free_out_node,
            ot_free_inv_meth_args,
            ot_copy_array,
            ot_get_string_from_chararray,
            ot_create_chararray_from_string,
            ot_create_bounded_chararray_from_string,
            ot_get_char_value,
            ot_get_string_value,
            ot_get_string_from_char,
            ot_string_cat,
            ot_binary_cat,
            ot_halt,
            ot_convert_bounded_to_bounded,
            ot_convert_bounded_to_unbounded,
            ot_convert_unbounded_to_bounded,
            ot_convert_unbounded_to_unbounded,
            ot_convert_any_to_bounded,
            ot_convert_any_to_unbounded,
            ot_convert_array_to_object,
            ot_build_simple_refpak,
            ot_build_field_refpak,
            ot_add_any,
            ot_sub_any,
            ot_mul_any,
            ot_div_any,
            ot_pow_any,
            ot_neg_any,
            ot_eq_any,
            ot_ne_any,
            ot_gt_any,
            ot_lt_any,
            ot_ge_any,
            ot_le_any,
            ot_and_any,
            ot_or_any,
            ot_not_any,
            ot_incr_any,
            ot_decr_any,
            ot_mod_any,
            ot_min_any,
            ot_max_any,
            ot_check_any_exact_type,
            ot_check_any_string_type,
            ot_check_any_binary_type,
            ot_check_any_math_type,
            ot_check_any_enum_type,
            ot_check_any_object_type,
            ot_duplicate_any,
            ot_abs_any,
            ot_ceiling_any,
            ot_string_to_binary,
            ot_bytearray_to_binary,
            ot_any_to_binary,
            ob_set_curr_rtinst_and_return,
            ob_unset_curr_rtinst_and_return,
            ob_open_trace,
            ob_close_trace,
            ob_begin_trace,
            ob_end_trace,
            ob_enable_event_trace,
            ob_disable_event_trace
        })
    }
    pub fn version(&self) -> u32 { self.__version }
    pub unsafe fn pbstg_begin(&self, buffer: USHORT) -> ppbstg_anchor { (self.pbstg_begin)(buffer) }
    pub unsafe fn pbstg_begin_allocflags(&self, buffer: USHORT, lAllocFlags: UINT) -> ppbstg_anchor {
        (self.pbstg_begin_allocflags)(buffer, lAllocFlags)
    }
    pub unsafe fn pbstg_begin_nofast(&self, buffer: USHORT) -> ppbstg_anchor {
        (self.pbstg_begin_nofast)(buffer)
    }
    pub unsafe fn pbstg_end(&self, pthis: ppbstg_anchor) -> () { (self.pbstg_end)(pthis) }
    pub unsafe fn pbstg_free_pool(&self, pthis: ppbstg_anchor, subPool: pbstg_subpool) -> () {
        (self.pbstg_free_pool)(pthis, subPool)
    }
    pub unsafe fn pbstg_new_pool(&self, pthis: ppbstg_anchor) -> pbstg_subpool {
        (self.pbstg_new_pool)(pthis)
    }
    pub unsafe fn pbstg_new_pool_nofast(&self, pthis: ppbstg_anchor) -> pbstg_subpool {
        (self.pbstg_new_pool_nofast)(pthis)
    }
    pub unsafe fn pbstg_new_pool_with_size_nofast(
        &self,
        pthis: ppbstg_anchor,
        page_size: USHORT
    ) -> pbstg_subpool {
        (self.pbstg_new_pool_with_size_nofast)(pthis, page_size)
    }
    pub unsafe fn pbstg_set_pool_name(
        &self,
        pthis: ppbstg_anchor,
        subPool: pbstg_subpool,
        lpstrName: LPTSTR
    ) -> () {
        (self.pbstg_set_pool_name)(pthis, subPool, lpstrName)
    }
    pub unsafe fn pbstg_set_poolpagesize(&self, pthis: ppbstg_anchor, pagesize: ULONG) -> BOOL {
        (self.pbstg_set_poolpagesize)(pthis, pagesize)
    }
    pub unsafe fn pbstg_write_debug(
        &self,
        pthis: ppbstg_anchor,
        subpool: pbstg_subpool,
        lpFile: LPTSTR
    ) -> ::std::os::raw::c_short {
        (self.pbstg_write_debug)(pthis, subpool, lpFile)
    }
    pub unsafe fn pbstg_stat(&self, pthis: ppbstg_anchor, stat: *mut pbstg_statistics) -> () {
        (self.pbstg_stat)(pthis, stat)
    }
    pub unsafe fn pbstg_nextGeneration(&self) -> ::std::os::raw::c_long { (self.pbstg_nextGeneration)() }
    pub unsafe fn pbstg_dumpLeaks(&self, generation: ::std::os::raw::c_long) -> () {
        (self.pbstg_dumpLeaks)(generation)
    }
    pub unsafe fn pbstg_dumpHeap(&self) -> () { (self.pbstg_dumpHeap)() }
    pub unsafe fn pbstg_alloc(
        &self,
        pthis: ppbstg_anchor,
        iNumberOfBytes: ULONG,
        subPool: pbstg_subpool
    ) -> *mut ::std::os::raw::c_void {
        (self.pbstg_alloc)(pthis, iNumberOfBytes, subPool)
    }
    pub unsafe fn pbstg_free(&self, pthis: ppbstg_anchor, stg: *mut ::std::os::raw::c_void) -> () {
        (self.pbstg_free)(pthis, stg)
    }
    pub unsafe fn pbstg_realloc(
        &self,
        pthis: ppbstg_anchor,
        pOldStorage: *mut ::std::os::raw::c_void,
        iLength: ULONG,
        subPool: pbstg_subpool
    ) -> *mut ::std::os::raw::c_void {
        (self.pbstg_realloc)(pthis, pOldStorage, iLength, subPool)
    }
    pub unsafe fn pbstg_size(&self, pthis: ppbstg_anchor, pStg: *mut ::std::os::raw::c_void) -> ULONG {
        (self.pbstg_size)(pthis, pStg)
    }
    pub unsafe fn pbstg_fast_strlen(&self, s: LPTSTR) -> ULONG { (self.pbstg_fast_strlen)(s) }
    pub unsafe fn pbstg_ansitoupper(&self, c: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        (self.pbstg_ansitoupper)(c)
    }
    pub unsafe fn pbstg_ansitolower(&self, c: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        (self.pbstg_ansitolower)(c)
    }
    pub unsafe fn pbstg_strdup(
        &self,
        pthis: ppbstg_anchor,
        string: LPCTSTR,
        subpool: pbstg_subpool
    ) -> LPTSTR {
        (self.pbstg_strdup)(pthis, string, subpool)
    }
    pub unsafe fn pbstg_strdup_malloc(&self, lpstrString: LPTSTR) -> LPTSTR {
        (self.pbstg_strdup_malloc)(lpstrString)
    }
    pub unsafe fn pbstg_str_build(
        &self,
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: LPTSTR
    ) -> () {
        (self.pbstg_str_build)(sa, subpool, syn, synLen, synOff, string)
    }
    pub unsafe fn pbstg_str_build_char(
        &self,
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        c: TCHAR
    ) -> () {
        (self.pbstg_str_build_char)(sa, subpool, syn, synLen, synOff, c)
    }
    pub unsafe fn pbstg_str_build_huge(
        &self,
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: *mut TCHAR
    ) -> () {
        (self.pbstg_str_build_huge)(sa, subpool, syn, synLen, synOff, string)
    }
    pub unsafe fn pbstg_str_remove_char(&self, string: LPTSTR, c: TCHAR) -> LPTSTR {
        (self.pbstg_str_remove_char)(string, c)
    }
    pub unsafe fn pbstg_str_trim_left(&self, string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR {
        (self.pbstg_str_trim_left)(string, IncludeAllSpaceTypes)
    }
    pub unsafe fn pbstg_str_trim_right(&self, string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR {
        (self.pbstg_str_trim_right)(string, IncludeAllSpaceTypes)
    }
    pub unsafe fn pbstg_str_trim(&self, string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR {
        (self.pbstg_str_trim)(string, IncludeAllSpaceTypes)
    }
    pub unsafe fn pbstg_str_wordcap(&self, s: LPTSTR) -> LPTSTR { (self.pbstg_str_wordcap)(s) }
    pub unsafe fn pbstg_atoi_imp(&self, arg1: LPTSTR) -> INT { (self.pbstg_atoi_imp)(arg1) }
    pub unsafe fn pbstg_atof_imp(&self, arg1: LPTSTR) -> f64 { (self.pbstg_atof_imp)(arg1) }
    pub unsafe fn pbstg_strtod_imp(&self, pText: LPTSTR, endptr: *mut LPTSTR) -> f64 {
        (self.pbstg_strtod_imp)(pText, endptr)
    }
    pub unsafe fn pbstg_atol_imp(&self, arg1: LPTSTR) -> ::std::os::raw::c_long {
        (self.pbstg_atol_imp)(arg1)
    }
    pub unsafe fn pbstg_strtol_imp(
        &self,
        arg1: LPTSTR,
        arg2: *mut LPTSTR,
        arg3: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long {
        (self.pbstg_strtol_imp)(arg1, arg2, arg3)
    }
    pub unsafe fn pbstg_atou_imp(&self, arg1: LPTSTR) -> UINT { (self.pbstg_atou_imp)(arg1) }
    pub unsafe fn pbstg_atoul_imp(&self, arg1: LPTSTR) -> ULONG { (self.pbstg_atoul_imp)(arg1) }
    pub unsafe fn pbstg_strtoul_imp(
        &self,
        arg1: LPTSTR,
        arg2: *mut LPTSTR,
        arg3: ::std::os::raw::c_int
    ) -> ULONG {
        (self.pbstg_strtoul_imp)(arg1, arg2, arg3)
    }
    pub unsafe fn pbstg_remove_imp(&self, arg1: LPTSTR) -> INT { (self.pbstg_remove_imp)(arg1) }
    pub unsafe fn pbstg_dde_alloc(
        &self,
        iNumberOfBytes: ::std::os::raw::c_ushort
    ) -> *mut ::std::os::raw::c_void {
        (self.pbstg_dde_alloc)(iNumberOfBytes)
    }
    pub unsafe fn pbstg_dde_free(&self, arg1: *mut ::std::os::raw::c_void) -> () {
        (self.pbstg_dde_free)(arg1)
    }
    pub unsafe fn pbstg_dde_get_handle(&self, arg1: *mut ::std::os::raw::c_void) -> GLOBALHANDLE {
        (self.pbstg_dde_get_handle)(arg1)
    }
    pub unsafe fn pbstg_dde_lock(&self, arg1: GLOBALHANDLE) -> *mut ::std::os::raw::c_void {
        (self.pbstg_dde_lock)(arg1)
    }
    pub unsafe fn pbstg_dde_unlock(&self, arg1: GLOBALHANDLE) -> () { (self.pbstg_dde_unlock)(arg1) }
    pub unsafe fn pbstg_huge_memcmp(
        &self,
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> ::std::os::raw::c_short {
        (self.pbstg_huge_memcmp)(v1, v2, count)
    }
    pub unsafe fn pbstg_huge_memcpy(
        &self,
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void {
        (self.pbstg_huge_memcpy)(v1, v2, count)
    }
    pub unsafe fn pbstg_huge_memmove(
        &self,
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void {
        (self.pbstg_huge_memmove)(v1, v2, count)
    }
    pub unsafe fn pbstg_huge_memset(
        &self,
        v1: *mut ::std::os::raw::c_void,
        c: ::std::os::raw::c_short,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void {
        (self.pbstg_huge_memset)(v1, c, count)
    }
    pub unsafe fn pbstg_huge_strchr(&self, s: *mut TCHAR, c: TCHAR) -> *mut TCHAR {
        (self.pbstg_huge_strchr)(s, c)
    }
    pub unsafe fn pbstg_huge_strcpy(&self, s: *mut TCHAR, s2: *mut TCHAR) -> *mut TCHAR {
        (self.pbstg_huge_strcpy)(s, s2)
    }
    pub unsafe fn pbstg_huge_strlen(&self, s: *mut TCHAR) -> ULONG { (self.pbstg_huge_strlen)(s) }
    pub unsafe fn pbstg_huge_strncpy(&self, s: *mut TCHAR, s2: *mut TCHAR, count: ULONG) -> *mut TCHAR {
        (self.pbstg_huge_strncpy)(s, s2, count)
    }
    pub unsafe fn pbstg_huge_strstr(&self, s: *mut TCHAR, s2: *mut TCHAR) -> *mut TCHAR {
        (self.pbstg_huge_strstr)(s, s2)
    }
    pub unsafe fn pbstg_unicodestrdup(
        &self,
        sa: ppbstg_anchor,
        pwsz: LPCWSTR,
        subpool: pbstg_subpool
    ) -> LPWSTR {
        (self.pbstg_unicodestrdup)(sa, pwsz, subpool)
    }
    pub unsafe fn pbstg_unicodestr_build(
        &self,
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPWSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: LPCWSTR
    ) -> () {
        (self.pbstg_unicodestr_build)(sa, subpool, syn, synLen, synOff, string)
    }
    pub unsafe fn pbstg_strtounicodedup(
        &self,
        pthis: ppbstg_anchor,
        psz: LPCTSTR,
        subpool: pbstg_subpool
    ) -> LPWSTR {
        (self.pbstg_strtounicodedup)(pthis, psz, subpool)
    }
    pub unsafe fn pbstg_unicodetostrdup(
        &self,
        pthis: ppbstg_anchor,
        pwsz: LPCWSTR,
        subpool: pbstg_subpool
    ) -> LPTSTR {
        (self.pbstg_unicodetostrdup)(pthis, pwsz, subpool)
    }
    pub unsafe fn pbstg_strtoansidup(
        &self,
        pthis: ppbstg_anchor,
        psz: LPCTSTR,
        subpool: pbstg_subpool
    ) -> LPSTR {
        (self.pbstg_strtoansidup)(pthis, psz, subpool)
    }
    pub unsafe fn pbstg_ansitostrdup(
        &self,
        pthis: ppbstg_anchor,
        pasz: LPCSTR,
        subpool: pbstg_subpool
    ) -> LPTSTR {
        (self.pbstg_ansitostrdup)(pthis, pasz, subpool)
    }
    pub unsafe fn pbstg_strtoprintable(&self, dest: LPSTR, source: LPCTSTR) -> LPSTR {
        (self.pbstg_strtoprintable)(dest, source)
    }
    pub unsafe fn pbstg_strtoprintabledup(
        &self,
        pthis: ppbstg_anchor,
        psz: LPCTSTR,
        subpool: pbstg_subpool
    ) -> LPSTR {
        (self.pbstg_strtoprintabledup)(pthis, psz, subpool)
    }
    pub unsafe fn pbstg_printabletostr(&self, dest: LPTSTR, source: LPCSTR) -> LPTSTR {
        (self.pbstg_printabletostr)(dest, source)
    }
    pub unsafe fn pbstg_printabletostrdup(
        &self,
        pthis: ppbstg_anchor,
        pasz: LPCSTR,
        subpool: pbstg_subpool
    ) -> LPTSTR {
        (self.pbstg_printabletostrdup)(pthis, pasz, subpool)
    }
    pub unsafe fn pbstg_lchrcmp(&self, c1: TCHAR, c2: TCHAR) -> INT { (self.pbstg_lchrcmp)(c1, c2) }
    pub unsafe fn pbstg_lchrcmpi(&self, c1: TCHAR, c2: TCHAR) -> INT { (self.pbstg_lchrcmpi)(c1, c2) }
    pub unsafe fn ob_set_session_icontext(
        &self,
        obthis: POB_THIS,
        pNewContext: *mut ::std::os::raw::c_void
    ) -> () {
        (self.ob_set_session_icontext)(obthis, pNewContext)
    }
    pub unsafe fn rt_move_thread(&self, rtthis: POB_THIS) -> BOOL { (self.rt_move_thread)(rtthis) }
    pub unsafe fn rt_clear_thread(&self) -> () { (self.rt_clear_thread)() }
    pub unsafe fn rt_get_current_this(&self) -> POB_THIS { (self.rt_get_current_this)() }
    pub unsafe fn rt_add_task(&self, rtthis: POB_THIS) -> BOOL { (self.rt_add_task)(rtthis) }
    pub unsafe fn rt_free_task(&self) -> BOOL { (self.rt_free_task)() }
    pub unsafe fn rt_get_current_task_info(&self, info_pos: INT) -> *mut ::std::os::raw::c_void {
        (self.rt_get_current_task_info)(info_pos)
    }
    pub unsafe fn rt_set_current_task_info(
        &self,
        info_pos: INT,
        user_info: *mut ::std::os::raw::c_void
    ) -> BOOL {
        (self.rt_set_current_task_info)(info_pos, user_info)
    }
    pub unsafe fn rt_get_free_task_slot(&self) -> INT { (self.rt_get_free_task_slot)() }
    pub unsafe fn rt_is_running_exe(&self) -> BOOL { (self.rt_is_running_exe)() }
    pub unsafe fn ob_add_const_data(
        &self,
        obthis: POB_THIS,
        conpool: POB_CONPOOL,
        data: *mut ::std::os::raw::c_void,
        item_type: OB_CONPOOL_ITEM_TYPE,
        nitems: UINT,
        len: ULONG
    ) -> OB_CONST_REF {
        (self.ob_add_const_data)(obthis, conpool, data, item_type, nitems, len)
    }
    pub unsafe fn ob_looksym_keyfunc(
        &self,
        pDataNode: *mut ::std::os::raw::c_void,
        tobthis: *mut ::std::os::raw::c_void
    ) -> LPTSTR {
        (self.ob_looksym_keyfunc)(pDataNode, tobthis)
    }
    pub unsafe fn ob_looksym_reference(
        &self,
        obthis: POB_THIS,
        look_symtab: POB_LOOK_SYMTAB,
        name: LPTSTR
    ) -> OB_SYM_ID {
        (self.ob_looksym_reference)(obthis, look_symtab, name)
    }
    pub unsafe fn ob_looksym_delete(&self, obthis: POB_THIS, look_symtab: POB_LOOK_SYMTAB, slot: UINT) -> () {
        (self.ob_looksym_delete)(obthis, look_symtab, slot)
    }
    pub unsafe fn ob_dynarray_index(
        &self,
        obthis: POB_THIS,
        theArray: *mut tag_OB_DYNARRAY,
        index: ULONG,
        extend: BOOL
    ) -> *mut ::std::os::raw::c_void {
        (self.ob_dynarray_index)(obthis, theArray, index, extend)
    }
    pub unsafe fn ob_dynarray_grow(
        &self,
        obthis: POB_THIS,
        theArray: *mut tag_OB_DYNARRAY,
        limit: ULONG,
        initialize: BOOL
    ) -> () {
        (self.ob_dynarray_grow)(obthis, theArray, limit, initialize)
    }
    pub unsafe fn ob_narray_create_static(
        &self,
        obthis: POB_THIS,
        subpool: OB_SUBPOOL,
        num_items: ULONG,
        elmtType: OB_CLASS_HNDL,
        elmtSize: USHORT,
        numDim: USHORT,
        boundsArray: *mut ::std::os::raw::c_long,
        userData: USHORT,
        useNulls: BOOL,
        freeData: BOOL,
        initFn: PNARRAY_INIT_FN
    ) -> *mut tag_OB_NARRAY {
        (self.ob_narray_create_static)(
            obthis,
            subpool,
            num_items,
            elmtType,
            elmtSize,
            numDim,
            boundsArray,
            userData,
            useNulls,
            freeData,
            initFn
        )
    }
    pub unsafe fn ob_narray_create_dynamic(
        &self,
        obthis: POB_THIS,
        subpool: OB_SUBPOOL,
        elmtType: OB_CLASS_HNDL,
        elmtSize: USHORT,
        userData: USHORT,
        useNulls: BOOL,
        freeData: BOOL,
        initFn: PNARRAY_INIT_FN
    ) -> *mut tag_OB_NARRAY {
        (self.ob_narray_create_dynamic)(
            obthis, subpool, elmtType, elmtSize, userData, useNulls, freeData, initFn
        )
    }
    pub unsafe fn ob_set_arraydef(
        &self,
        obthis: POB_THIS,
        arraydef: POB_ARRAYDEF,
        no_dims: UINT,
        arr_style: OB_ARRAY_SYMBOL_STYLE,
        bounds: *mut ::std::os::raw::c_long
    ) -> () {
        (self.ob_set_arraydef)(obthis, arraydef, no_dims, arr_style, bounds)
    }
    pub unsafe fn ob_get_array_len(&self, obthis: POB_THIS, arraydef: POB_ARRAYDEF) -> ULONG {
        (self.ob_get_array_len)(obthis, arraydef)
    }
    pub unsafe fn ob_array_item_init_callback(
        &self,
        obthis: POB_THIS,
        theArray: *mut tag_OB_NARRAY,
        theItem: *mut ::std::os::raw::c_void
    ) -> BOOL {
        (self.ob_array_item_init_callback)(obthis, theArray, theItem)
    }
    pub unsafe fn ob_init_array(
        &self,
        obthis: POB_THIS,
        arrdef: POB_ARRAYDEF,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        init_data: BOOL
    ) -> POB_ARRAY_INST {
        (self.ob_init_array)(obthis, arrdef, group, class_id, init_data)
    }
    pub unsafe fn ob_array_varinfo_nullval(&self, obthis: POB_THIS, array_inst: POB_ARRAY_INST) -> BOOL {
        (self.ob_array_varinfo_nullval)(obthis, array_inst)
    }
    pub unsafe fn ob_array_set_varinfo_nullval(
        &self,
        obthis: POB_THIS,
        array_inst: POB_ARRAY_INST,
        bNull: BOOL
    ) -> () {
        (self.ob_array_set_varinfo_nullval)(obthis, array_inst, bNull)
    }
    pub unsafe fn ob_remove_array_data(
        &self,
        obthis: POB_THIS,
        array_inst: POB_ARRAY_INST,
        IsNullVarInfor: BOOL
    ) -> () {
        (self.ob_remove_array_data)(obthis, array_inst, IsNullVarInfor)
    }
    pub unsafe fn ob_init_pcode_blk(
        &self,
        obthis: POB_THIS,
        no_items: UINT,
        no_line_incr: UINT,
        subpool: OB_SUBPOOL
    ) -> POB_PCODE_BLK {
        (self.ob_init_pcode_blk)(obthis, no_items, no_line_incr, subpool)
    }
    pub unsafe fn ob_del_pcode_blk(&self, obthis: POB_THIS, pcode_blk: POB_PCODE_BLK) -> () {
        (self.ob_del_pcode_blk)(obthis, pcode_blk)
    }
    pub unsafe fn ob_reuse_routine(
        &self,
        obthis: POB_THIS,
        routlist: POB_ROUTLIST,
        rout_id: OB_ROUT_ID,
        proto_id: OB_PROTO_ID,
        subpool: OB_SUBPOOL,
        clear_pcode: BOOL
    ) -> () {
        (self.ob_reuse_routine)(obthis, routlist, rout_id, proto_id, subpool, clear_pcode)
    }
    pub unsafe fn shMaxDec(&self, dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC {
        (self.shMaxDec)(dst, src1, src2)
    }
    pub unsafe fn shMinDec(&self, dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC {
        (self.shMinDec)(dst, src1, src2)
    }
    pub unsafe fn shCompareDec(&self, src1: PSH_DEC, src2: PSH_DEC) -> SHORT {
        (self.shCompareDec)(src1, src2)
    }
    pub unsafe fn shAbsDec(&self, dst: PSH_DEC, src: PSH_DEC) -> PSH_DEC { (self.shAbsDec)(dst, src) }
    pub unsafe fn shNegateDec(&self, dst: PSH_DEC, src: PSH_DEC) -> PSH_DEC { (self.shNegateDec)(dst, src) }
    pub unsafe fn shRoundDec(&self, dst: PSH_DEC, src: PSH_DEC, n: SHORT) -> PSH_DEC {
        (self.shRoundDec)(dst, src, n)
    }
    pub unsafe fn shTruncDec(&self, dst: PSH_DEC, src: PSH_DEC, n: SHORT) -> PSH_DEC {
        (self.shTruncDec)(dst, src, n)
    }
    pub unsafe fn shAddDec(&self, dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC {
        (self.shAddDec)(dst, src1, src2)
    }
    pub unsafe fn shSubDec(&self, dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC {
        (self.shSubDec)(dst, src1, src2)
    }
    pub unsafe fn shMultDec(&self, dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC {
        (self.shMultDec)(dst, src1, src2)
    }
    pub unsafe fn shDivDec(&self, dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC {
        (self.shDivDec)(dst, src1, src2)
    }
    pub unsafe fn shModDec(&self, dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC {
        (self.shModDec)(dst, src1, src2)
    }
    pub unsafe fn shExpDec(&self, dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC {
        (self.shExpDec)(dst, src1, src2)
    }
    pub unsafe fn shIntToDec(&self, dst: PSH_DEC, src: SHORT) -> PSH_DEC { (self.shIntToDec)(dst, src) }
    pub unsafe fn shDecToInt(&self, dst: *mut SHORT, src: PSH_DEC) -> *mut SHORT {
        (self.shDecToInt)(dst, src)
    }
    pub unsafe fn shUintToDec(&self, dst: PSH_DEC, src: USHORT) -> PSH_DEC { (self.shUintToDec)(dst, src) }
    pub unsafe fn shDecToUint(&self, dst: *mut USHORT, src: PSH_DEC) -> *mut USHORT {
        (self.shDecToUint)(dst, src)
    }
    pub unsafe fn shByteToDec(&self, dst: PSH_DEC, src: ::std::os::raw::c_uchar) -> PSH_DEC {
        (self.shByteToDec)(dst, src)
    }
    pub unsafe fn shDecToByte(
        &self,
        dst: *mut ::std::os::raw::c_uchar,
        src: PSH_DEC
    ) -> *mut ::std::os::raw::c_uchar {
        (self.shDecToByte)(dst, src)
    }
    pub unsafe fn shLongToDec(&self, dst: PSH_DEC, src: ::std::os::raw::c_long) -> PSH_DEC {
        (self.shLongToDec)(dst, src)
    }
    pub unsafe fn shDecToLong(
        &self,
        arg1: *mut ::std::os::raw::c_long,
        src: PSH_DEC
    ) -> *mut ::std::os::raw::c_long {
        (self.shDecToLong)(arg1, src)
    }
    pub unsafe fn shUlongToDec(&self, dst: PSH_DEC, src: ULONG) -> PSH_DEC { (self.shUlongToDec)(dst, src) }
    pub unsafe fn shDecToUlong(&self, dst: *mut ULONG, src: PSH_DEC) -> *mut ULONG {
        (self.shDecToUlong)(dst, src)
    }
    pub unsafe fn shLonglongToDec(&self, dst: PSH_DEC, src: *mut ::std::os::raw::c_longlong) -> PSH_DEC {
        (self.shLonglongToDec)(dst, src)
    }
    pub unsafe fn shDecToLonglong(
        &self,
        dst: *mut ::std::os::raw::c_longlong,
        src: PSH_DEC
    ) -> *mut ::std::os::raw::c_longlong {
        (self.shDecToLonglong)(dst, src)
    }
    pub unsafe fn shDecToFloat(&self, dst: *mut f32, src: PSH_DEC) -> *mut f32 {
        (self.shDecToFloat)(dst, src)
    }
    pub unsafe fn shFloatToDec(&self, dst: PSH_DEC, src: *mut f32) -> PSH_DEC {
        (self.shFloatToDec)(dst, src)
    }
    pub unsafe fn shDoubleToDec(&self, dst: PSH_DEC, src: *mut f64) -> PSH_DEC {
        (self.shDoubleToDec)(dst, src)
    }
    pub unsafe fn shDecToDouble(&self, dst: *mut f64, src: PSH_DEC) -> *mut f64 {
        (self.shDecToDouble)(dst, src)
    }
    pub unsafe fn shDecToAscii(&self, dst: LPTSTR, src: PSH_DEC) -> LPTSTR { (self.shDecToAscii)(dst, src) }
    pub unsafe fn shAsciiToDec(&self, dst: PSH_DEC, src: LPTSTR) -> PSH_DEC { (self.shAsciiToDec)(dst, src) }
    pub unsafe fn shAsciiToDecRnd(&self, dst: PSH_DEC, src: LPTSTR, n: SHORT) -> PSH_DEC {
        (self.shAsciiToDecRnd)(dst, src, n)
    }
    pub unsafe fn shSetDecFractions(&self, d: PSH_DEC, n: SHORT) -> () { (self.shSetDecFractions)(d, n) }
    pub unsafe fn shSetDecNegative(&self, d: PSH_DEC, n: BOOL) -> () { (self.shSetDecNegative)(d, n) }
    pub unsafe fn shDecSetOverflow(&self, dec: PSH_DEC, neg: BOOL) -> BOOL {
        (self.shDecSetOverflow)(dec, neg)
    }
    pub unsafe fn ob_mgr_init(&self, dbgthis: *mut SH_DBG_THIS, stgthis: ppbstg_anchor) -> POB_THIS {
        (self.ob_mgr_init)(dbgthis, stgthis)
    }
    pub unsafe fn ob_mgr_init_ex(
        &self,
        dbgthis: *mut SH_DBG_THIS,
        stgthis: ppbstg_anchor,
        lpstrTypdefPblName: LPTSTR
    ) -> POB_THIS {
        (self.ob_mgr_init_ex)(dbgthis, stgthis, lpstrTypdefPblName)
    }
    pub unsafe fn ob_mgr_restart(&self, obthis: POB_THIS) -> () { (self.ob_mgr_restart)(obthis) }
    pub unsafe fn ob_mgr_terminate(&self, obthis: POB_THIS) -> () { (self.ob_mgr_terminate)(obthis) }
    pub unsafe fn ob_free_memory(&self, obthis: POB_THIS, data: *mut ::std::os::raw::c_void) -> () {
        (self.ob_free_memory)(obthis, data)
    }
    pub unsafe fn ob_free_link_error_list(&self, obthis: POB_THIS) -> () {
        (self.ob_free_link_error_list)(obthis)
    }
    pub unsafe fn ob_get_link_error_list(&self, obthis: POB_THIS) -> *mut ::std::os::raw::c_void {
        (self.ob_get_link_error_list)(obthis)
    }
    pub unsafe fn ob_enter_critical_section(&self, obthis: POB_THIS) -> () {
        (self.ob_enter_critical_section)(obthis)
    }
    pub unsafe fn ob_leave_critical_section(&self, obthis: POB_THIS) -> () {
        (self.ob_leave_critical_section)(obthis)
    }
    pub unsafe fn ob_alloc_string(&self, obthis: POB_THIS, len: ULONG) -> LPTSTR {
        (self.ob_alloc_string)(obthis, len)
    }
    pub unsafe fn ob_alloc_blob(&self, obthis: POB_THIS, len: ULONG) -> PSH_BINARY {
        (self.ob_alloc_blob)(obthis, len)
    }
    pub unsafe fn ob_alloc_dec(&self, obthis: POB_THIS) -> PSH_DEC { (self.ob_alloc_dec)(obthis) }
    pub unsafe fn ob_alloc_double(&self, obthis: POB_THIS) -> *mut f64 { (self.ob_alloc_double)(obthis) }
    pub unsafe fn ob_alloc_longlong(&self, obthis: POB_THIS) -> *mut ::std::os::raw::c_longlong {
        (self.ob_alloc_longlong)(obthis)
    }
    pub unsafe fn ob_alloc_time(&self, obthis: POB_THIS) -> PSH_TIME { (self.ob_alloc_time)(obthis) }
    pub unsafe fn ob_realloc_string(&self, obthis: POB_THIS, string: LPTSTR, len: ULONG) -> LPTSTR {
        (self.ob_realloc_string)(obthis, string, len)
    }
    pub unsafe fn ob_realloc_blob(&self, obthis: POB_THIS, blob: PSH_BINARY, len: ULONG) -> PSH_BINARY {
        (self.ob_realloc_blob)(obthis, blob, len)
    }
    pub unsafe fn ob_dup_string(&self, obthis: POB_THIS, string: LPTSTR) -> LPTSTR {
        (self.ob_dup_string)(obthis, string)
    }
    pub unsafe fn ob_dup_blob(&self, obthis: POB_THIS, blob: PSH_BINARY) -> PSH_BINARY {
        (self.ob_dup_blob)(obthis, blob)
    }
    pub unsafe fn ob_dup_dec(&self, obthis: POB_THIS, dec_val: PSH_DEC) -> PSH_DEC {
        (self.ob_dup_dec)(obthis, dec_val)
    }
    pub unsafe fn ob_dup_double(&self, obthis: POB_THIS, double_val: *mut f64) -> *mut f64 {
        (self.ob_dup_double)(obthis, double_val)
    }
    pub unsafe fn ob_dup_longlong(
        &self,
        obthis: POB_THIS,
        longlong_val: *mut ::std::os::raw::c_longlong
    ) -> *mut ::std::os::raw::c_longlong {
        (self.ob_dup_longlong)(obthis, longlong_val)
    }
    pub unsafe fn ob_dup_time(&self, obthis: POB_THIS, time_val: PSH_TIME) -> PSH_TIME {
        (self.ob_dup_time)(obthis, time_val)
    }
    pub unsafe fn ob_free_value(&self, obthis: POB_THIS, data: *mut ::std::os::raw::c_void) -> () {
        (self.ob_free_value)(obthis, data)
    }
    pub unsafe fn ob_create_appl_report(&self, obthis: POB_THIS) -> POB_APPL_REPORT {
        (self.ob_create_appl_report)(obthis)
    }
    pub unsafe fn ob_create_object_report(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        object_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> POB_APPL_REPORT {
        (self.ob_create_object_report)(obthis, lib_name, object_name, class_id)
    }
    pub unsafe fn ob_free_appl_report(&self, obthis: POB_THIS, appl_report: POB_APPL_REPORT) -> () {
        (self.ob_free_appl_report)(obthis, appl_report)
    }
    pub unsafe fn ob_get_mode(&self, obthis: POB_THIS) -> OB_MODE { (self.ob_get_mode)(obthis) }
    pub unsafe fn ob_set_mode(&self, obthis: POB_THIS, mode: OB_MODE) -> OB_MODE {
        (self.ob_set_mode)(obthis, mode)
    }
    pub unsafe fn ob_get_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        pData: POB_DATA
    ) -> POB_DATA {
        (self.ob_get_field)(obthis, obinst, field_id, pData)
    }
    pub unsafe fn ob_set_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        pData: POB_DATA
    ) -> () {
        (self.ob_set_field)(obthis, obinst, field_id, pData)
    }
    pub unsafe fn ob_get_field_data(&self, obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> POB_DATA {
        (self.ob_get_field_data)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_no_fields(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG {
        (self.ob_get_no_fields)(obthis, obinst)
    }
    pub unsafe fn ob_get_parent_obinst(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> OB_INST_ID {
        (self.ob_get_parent_obinst)(obthis, obinst)
    }
    pub unsafe fn ob_get_first_user_field(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG {
        (self.ob_get_first_user_field)(obthis, obinst)
    }
    pub unsafe fn ob_get_field_type(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> OB_CLASS_ID {
        (self.ob_get_field_type)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_int_field(&self, obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> INT {
        (self.ob_get_int_field)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_uint_field(&self, obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> UINT {
        (self.ob_get_uint_field)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_byte_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> ::std::os::raw::c_uchar {
        (self.ob_get_byte_field)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_long_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> ::std::os::raw::c_long {
        (self.ob_get_long_field)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_ulong_field(&self, obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> ULONG {
        (self.ob_get_ulong_field)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_float_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        fl: *mut f32
    ) -> *mut f32 {
        (self.ob_get_float_field)(obthis, obinst, field_id, fl)
    }
    pub unsafe fn ob_get_ptr_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> *mut ::std::os::raw::c_void {
        (self.ob_get_ptr_field)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_inst_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> OB_INST_ID {
        (self.ob_get_inst_field)(obthis, obinst, field_id)
    }
    pub unsafe fn ob_get_array_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        no_items: *mut UINT
    ) -> *mut ::std::os::raw::c_void {
        (self.ob_get_array_field)(obthis, obinst, field_id, no_items)
    }
    pub unsafe fn ob_array_index(
        &self,
        obthis: POB_THIS,
        array_vals: *mut ::std::os::raw::c_void,
        index: ULONG,
        type_: POB_CLASS_ID
    ) -> *mut ::std::os::raw::c_void {
        (self.ob_array_index)(obthis, array_vals, index, type_)
    }
    pub unsafe fn ob_get_indirect_obdata(
        &self,
        obthis: POB_THIS,
        obInst: OB_INST_ID,
        obInfo: POB_DATA_INFO
    ) -> POB_DATA {
        (self.ob_get_indirect_obdata)(obthis, obInst, obInfo)
    }
    pub unsafe fn ob_array_item(
        &self,
        obthis: POB_THIS,
        array_vals: *mut ::std::os::raw::c_void,
        index: ULONG
    ) -> POB_DATA {
        (self.ob_array_item)(obthis, array_vals, index)
    }
    pub unsafe fn ob_array_get_index_from_subs(
        &self,
        obthis: POB_THIS,
        theArray: OB_ARRAY_ID,
        subs: *mut ::std::os::raw::c_long
    ) -> ULONG {
        (self.ob_array_get_index_from_subs)(obthis, theArray, subs)
    }
    pub unsafe fn ob_array_calc_index(
        &self,
        obthis: POB_THIS,
        numDims: UINT,
        bounds: *mut ::std::os::raw::c_long,
        subs: *mut ::std::os::raw::c_long
    ) -> ULONG {
        (self.ob_array_calc_index)(obthis, numDims, bounds, subs)
    }
    pub unsafe fn ob_set_int_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        int_val: INT
    ) -> () {
        (self.ob_set_int_field)(obthis, obinst, field_id, int_val)
    }
    pub unsafe fn ob_set_uint_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        uint_val: UINT
    ) -> () {
        (self.ob_set_uint_field)(obthis, obinst, field_id, uint_val)
    }
    pub unsafe fn ob_set_long_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        long_val: ::std::os::raw::c_long
    ) -> () {
        (self.ob_set_long_field)(obthis, obinst, field_id, long_val)
    }
    pub unsafe fn ob_set_ulong_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        ulong_val: ULONG
    ) -> () {
        (self.ob_set_ulong_field)(obthis, obinst, field_id, ulong_val)
    }
    pub unsafe fn ob_set_float_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        flval: f32
    ) -> () {
        (self.ob_set_float_field)(obthis, obinst, field_id, flval)
    }
    pub unsafe fn ob_set_ptr_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        ptrval: *mut ::std::os::raw::c_void
    ) -> () {
        (self.ob_set_ptr_field)(obthis, obinst, field_id, ptrval)
    }
    pub unsafe fn ob_set_array_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        pArray: *mut ::std::os::raw::c_void
    ) -> () {
        (self.ob_set_array_field)(obthis, obinst, field_id, pArray)
    }
    pub unsafe fn ob_set_obinst_field(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        obinstval: OB_INST_ID
    ) -> () {
        (self.ob_set_obinst_field)(obthis, obinst, field_id, obinstval)
    }
    pub unsafe fn ob_set_underlying_object(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        obj: *mut ::std::os::raw::c_void
    ) -> () {
        (self.ob_set_underlying_object)(obthis, obinst, obj)
    }
    pub unsafe fn ob_get_underlying_object(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID
    ) -> *mut ::std::os::raw::c_void {
        (self.ob_get_underlying_object)(obthis, obinst)
    }
    pub unsafe fn ob_is_any_group_locked(&self, obthis: POB_THIS) -> BOOL {
        (self.ob_is_any_group_locked)(obthis)
    }
    pub unsafe fn ob_get_group_lock_count(&self, obthis: POB_THIS) -> UINT {
        (self.ob_get_group_lock_count)(obthis)
    }
    pub unsafe fn ob_is_group_locked(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> BOOL {
        (self.ob_is_group_locked)(obthis, group_hndl)
    }
    pub unsafe fn ob_is_group_unlocked(&self, obthis: POB_THIS, obGroupHandle: OB_GROUP_HNDL) -> BOOL {
        (self.ob_is_group_unlocked)(obthis, obGroupHandle)
    }
    pub unsafe fn ob_is_group_write_locked(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> BOOL {
        (self.ob_is_group_write_locked)(obthis, group_hndl)
    }
    pub unsafe fn ob_lock_group(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, write_only: BOOL) -> INT {
        (self.ob_lock_group)(obthis, group_hndl, write_only)
    }
    pub unsafe fn ob_unlock_group(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT {
        (self.ob_unlock_group)(obthis, group_hndl)
    }
    pub unsafe fn ob_clear_unlocked_groups(&self, obthis: POB_THIS) -> () {
        (self.ob_clear_unlocked_groups)(obthis)
    }
    pub unsafe fn ob_clear_all_other_unlocked_groups(&self, obthis: POB_THIS, obGroupId: OB_GROUP_ID) -> () {
        (self.ob_clear_all_other_unlocked_groups)(obthis, obGroupId)
    }
    pub unsafe fn ob_is_ancestor_locked(
        &self,
        obthis: POB_THIS,
        groupid: OB_GROUP_ID,
        cReadWrite: TCHAR
    ) -> BOOL {
        (self.ob_is_ancestor_locked)(obthis, groupid, cReadWrite)
    }
    pub unsafe fn ob_is_descendent_locked(
        &self,
        obthis: POB_THIS,
        groupid: OB_GROUP_ID,
        cReadWrite: TCHAR
    ) -> BOOL {
        (self.ob_is_descendent_locked)(obthis, groupid, cReadWrite)
    }
    pub unsafe fn ob_validate_liblist(
        &self,
        obThis: POB_THIS,
        pLibList: *mut LPTSTR,
        iNumberOfItems: UINT
    ) -> INT {
        (self.ob_validate_liblist)(obThis, pLibList, iNumberOfItems)
    }
    pub unsafe fn ob_set_liblist(
        &self,
        obthis: POB_THIS,
        lib_list: *mut LPTSTR,
        no_items: UINT,
        bCreateNewLoader: BOOL
    ) -> INT {
        (self.ob_set_liblist)(obthis, lib_list, no_items, bCreateNewLoader)
    }
    pub unsafe fn ob_get_liblist(&self, obthis: POB_THIS, no_items: *mut UINT) -> *mut LPTSTR {
        (self.ob_get_liblist)(obthis, no_items)
    }
    pub unsafe fn ob_set_default_appl(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        appl_group_name: LPTSTR
    ) -> () {
        (self.ob_set_default_appl)(obthis, lib_name, appl_group_name)
    }
    pub unsafe fn ob_load_appl_group(&self, obthis: POB_THIS) -> BOOL { (self.ob_load_appl_group)(obthis) }
    pub unsafe fn ob_is_group_in_memory(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        qualified_name: LPTSTR
    ) -> OB_GROUP_HNDL {
        (self.ob_is_group_in_memory)(obthis, lib_name, qualified_name)
    }
    pub unsafe fn ob_group_declare(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> OB_GROUP_HNDL {
        (self.ob_group_declare)(obthis, lib_name, group_name, class_id)
    }
    pub unsafe fn ob_group_reference(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> OB_GROUP_HNDL {
        (self.ob_group_reference)(obthis, lib_name, group_name, class_id)
    }
    pub unsafe fn ob_get_group_name(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> LPTSTR {
        (self.ob_get_group_name)(obthis, group_hndl)
    }
    pub unsafe fn ob_get_group_full_name(&self, obthis: POB_THIS, grouphndl: OB_GROUP_HNDL) -> LPTSTR {
        (self.ob_get_group_full_name)(obthis, grouphndl)
    }
    pub unsafe fn ob_group_save(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        lib_name: LPTSTR,
        comment: LPTSTR
    ) -> INT {
        (self.ob_group_save)(obthis, group_hndl, lib_name, comment)
    }
    pub unsafe fn ob_group_save_win(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        lib_name: LPTSTR,
        comment: LPTSTR,
        bSaveNormalize: BOOL
    ) -> INT {
        (self.ob_group_save_win)(obthis, group_hndl, lib_name, comment, bSaveNormalize)
    }
    pub unsafe fn ob_load_group_source(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT {
        (self.ob_load_group_source)(obthis, group_hndl)
    }
    pub unsafe fn ob_rename_group(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        new_name: LPTSTR
    ) -> INT {
        (self.ob_rename_group)(obthis, group_hndl, new_name)
    }
    pub unsafe fn ob_move_group(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, lib_name: LPTSTR) -> INT {
        (self.ob_move_group)(obthis, group_hndl, lib_name)
    }
    pub unsafe fn ob_move_group_with_name(
        &self,
        obthis: POB_THIS,
        qual_name: LPTSTR,
        oldlib: LPTSTR,
        newlib: LPTSTR
    ) -> INT {
        (self.ob_move_group_with_name)(obthis, qual_name, oldlib, newlib)
    }
    pub unsafe fn ob_copy_group_with_name(
        &self,
        obthis: POB_THIS,
        qual_name: LPTSTR,
        oldlib: LPTSTR,
        newlib: LPTSTR
    ) -> INT {
        (self.ob_copy_group_with_name)(obthis, qual_name, oldlib, newlib)
    }
    pub unsafe fn ob_copy_group(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        new_name: LPTSTR,
        lib_name: LPTSTR
    ) -> () {
        (self.ob_copy_group)(obthis, group_hndl, new_name, lib_name)
    }
    pub unsafe fn ob_delete_group(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> () {
        (self.ob_delete_group)(obthis, group_hndl)
    }
    pub unsafe fn ob_delete_group_with_name(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> INT {
        (self.ob_delete_group_with_name)(obthis, lib_name, group_name, class_id)
    }
    pub unsafe fn ob_restore_group(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> () {
        (self.ob_restore_group)(obthis, group_hndl)
    }
    pub unsafe fn ob_save_working_group(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> () {
        (self.ob_save_working_group)(obthis, group_hndl)
    }
    pub unsafe fn ob_delete_working_group(&self, obthis: POB_THIS) -> () {
        (self.ob_delete_working_group)(obthis)
    }
    pub unsafe fn ob_restore_working_group(&self, obthis: POB_THIS) -> () {
        (self.ob_restore_working_group)(obthis)
    }
    pub unsafe fn ob_open_group_id(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> () {
        (self.ob_open_group_id)(obthis, group_hndl)
    }
    pub unsafe fn ob_close_group(&self, obthis: POB_THIS) -> () { (self.ob_close_group)(obthis) }
    pub unsafe fn ob_get_group_lib(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> LPTSTR {
        (self.ob_get_group_lib)(obthis, group_hndl)
    }
    pub unsafe fn ob_run_garbage_collection(&self, obthis: POB_THIS, force: BOOL) -> INT {
        (self.ob_run_garbage_collection)(obthis, force)
    }
    pub unsafe fn ob_delete_instlist_shlist(&self, obthis: POB_THIS, instlist: *mut shlist) -> () {
        (self.ob_delete_instlist_shlist)(obthis, instlist)
    }
    pub unsafe fn ob_get_group_instlist_as_shlist(
        &self,
        obthis: POB_THIS,
        groupId: OB_GROUP_ID
    ) -> *mut shlist {
        (self.ob_get_group_instlist_as_shlist)(obthis, groupId)
    }
    pub unsafe fn ob_delete_groups_shlist(&self, obthis: POB_THIS, groups: *mut shlist) -> () {
        (self.ob_delete_groups_shlist)(obthis, groups)
    }
    pub unsafe fn ob_get_groups_shlist(&self, obthis: POB_THIS) -> *mut shlist {
        (self.ob_get_groups_shlist)(obthis)
    }
    pub unsafe fn ob_store_source(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> () {
        (self.ob_store_source)(obthis, class_hndl, source, len)
    }
    pub unsafe fn ob_init_source(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT {
        (self.ob_init_source)(obthis, group_hndl, source, len)
    }
    pub unsafe fn ob_store_global_src(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT {
        (self.ob_store_global_src)(obthis, group_hndl, source, len)
    }
    pub unsafe fn ob_store_namespace_decl_src(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT {
        (self.ob_store_namespace_decl_src)(obthis, group_hndl, source, len)
    }
    pub unsafe fn ob_store_shared_src(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT {
        (self.ob_store_shared_src)(obthis, group_hndl, source, len)
    }
    pub unsafe fn ob_store_prototype_source(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> () {
        (self.ob_store_prototype_source)(obthis, class_hndl, source, len)
    }
    pub unsafe fn ob_store_instvar_source(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> () {
        (self.ob_store_instvar_source)(obthis, class_hndl, source, len)
    }
    pub unsafe fn ob_get_global_src(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        len: *mut UINT
    ) -> LPTSTR {
        (self.ob_get_global_src)(obthis, group_hndl, len)
    }
    pub unsafe fn ob_get_namespace_decl_src(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        len: *mut UINT
    ) -> LPTSTR {
        (self.ob_get_namespace_decl_src)(obthis, group_hndl, len)
    }
    pub unsafe fn ob_get_shared_src(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        len: *mut UINT
    ) -> LPTSTR {
        (self.ob_get_shared_src)(obthis, group_hndl, len)
    }
    pub unsafe fn ob_get_prototype_source(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        len: *mut UINT
    ) -> LPTSTR {
        (self.ob_get_prototype_source)(obthis, class_hndl, len)
    }
    pub unsafe fn ob_get_instvar_source(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        len: *mut UINT
    ) -> LPTSTR {
        (self.ob_get_instvar_source)(obthis, class_hndl, len)
    }
    pub unsafe fn ob_get_routine_src(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> LPTSTR {
        (self.ob_get_routine_src)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_decl_and_store_routine_src(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        routname: LPTSTR,
        result_type: LPTSTR,
        no_args: INT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        source: LPTSTR,
        len: UINT
    ) -> INT {
        (self.ob_decl_and_store_routine_src)(
            obthis,
            class_hndl,
            routname,
            result_type,
            no_args,
            arg_pass_style,
            arg_type_names,
            arg_names,
            arg_grouping,
            source,
            len
        )
    }
    pub unsafe fn ob_store_routine_src(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID,
        source: LPTSTR,
        len: UINT
    ) -> () {
        (self.ob_store_routine_src)(obthis, class_hndl, index, source, len)
    }
    pub unsafe fn ob_store_create_src(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT {
        (self.ob_store_create_src)(obthis, class_hndl, source, len)
    }
    pub unsafe fn ob_store_destroy_src(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        source: LPTSTR,
        len: UINT
    ) -> INT {
        (self.ob_store_destroy_src)(obthis, class_hndl, source, len)
    }
    pub unsafe fn ob_get_function_src(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> LPTSTR {
        (self.ob_get_function_src)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_store_function_src(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID,
        source: LPTSTR,
        len: UINT
    ) -> () {
        (self.ob_store_function_src)(obthis, class_hndl, index, source, len)
    }
    pub unsafe fn ob_symbol_search_extended(
        &self,
        obthis: POB_THIS,
        obClassHandle: OB_CLASS_HNDL,
        iCurrScope: INT,
        pszVarName: LPTSTR,
        bSkipVars: BOOL,
        bSkipTHIS: BOOL,
        puiLevel: *mut UINT,
        pbIsConstantField: *mut BOOL,
        pbIsPrivateMember: *mut BOOL,
        ppszFullName: *mut LPTSTR
    ) -> BOOL {
        (self.ob_symbol_search_extended)(
            obthis,
            obClassHandle,
            iCurrScope,
            pszVarName,
            bSkipVars,
            bSkipTHIS,
            puiLevel,
            pbIsConstantField,
            pbIsPrivateMember,
            ppszFullName
        )
    }
    pub unsafe fn ob_symbol_search(
        &self,
        obthis: POB_THIS,
        obClassHandle: OB_CLASS_HNDL,
        iCurrScope: INT,
        pszVarName: LPTSTR,
        bSkipVars: BOOL,
        bSkipTHIS: BOOL,
        puiLevel: *mut UINT,
        pbIsConstantField: *mut BOOL
    ) -> BOOL {
        (self.ob_symbol_search)(
            obthis,
            obClassHandle,
            iCurrScope,
            pszVarName,
            bSkipVars,
            bSkipTHIS,
            puiLevel,
            pbIsConstantField
        )
    }
    pub unsafe fn ob_class_declare(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR,
        parent_class: OB_CLASS_ID,
        within_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL {
        (self.ob_class_declare)(obthis, group_hndl, class_name, parent_class, within_class)
    }
    pub unsafe fn ob_get_full_qualified_typename(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_id: OB_CLASS_ID
    ) -> LPTSTR {
        (self.ob_get_full_qualified_typename)(obthis, group_hndl, class_id)
    }
    pub unsafe fn ob_class_declare_inh(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR,
        parent_class: OB_CLASS_HNDL,
        within_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL {
        (self.ob_class_declare_inh)(obthis, group_hndl, class_name, parent_class, within_class)
    }
    pub unsafe fn ob_class_reference(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR
    ) -> OB_CLASS_HNDL {
        (self.ob_class_reference)(obthis, group_hndl, class_name)
    }
    pub unsafe fn ob_class_name(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> LPTSTR {
        (self.ob_class_name)(obthis, class_hndl)
    }
    pub unsafe fn ob_class_name_not_indirect(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> LPTSTR {
        (self.ob_class_name_not_indirect)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_type_name(&self, obthis: POB_THIS, datanode: POB_DATA) -> LPTSTR {
        (self.ob_get_type_name)(obthis, datanode)
    }
    pub unsafe fn ob_classhndl_indirect(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL {
        (self.ob_classhndl_indirect)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_parent_class(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL {
        (self.ob_get_parent_class)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_within_class(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL {
        (self.ob_get_within_class)(obthis, class_hndl)
    }
    pub unsafe fn ob_class_delete(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> () {
        (self.ob_class_delete)(obthis, class_hndl)
    }
    pub unsafe fn ob_class_rename(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        new_name: LPTSTR
    ) -> INT {
        (self.ob_class_rename)(obthis, class_hndl, new_name)
    }
    pub unsafe fn ob_is_a_system_class(&self, obthis: POB_THIS, class_name: LPTSTR) -> BOOL {
        (self.ob_is_a_system_class)(obthis, class_name)
    }
    pub unsafe fn ob_is_class_inherited(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL {
        (self.ob_is_class_inherited)(obthis, class_hndl)
    }
    pub unsafe fn ob_is_class_descendant(
        &self,
        obthis: POB_THIS,
        ancestor: OB_CLASS_HNDL,
        descendant: OB_CLASS_HNDL
    ) -> BOOL {
        (self.ob_is_class_descendant)(obthis, ancestor, descendant)
    }
    pub unsafe fn ob_is_inh_from_user_class(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL {
        (self.ob_is_inh_from_user_class)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_sec_class_ancestor(
        &self,
        obthis: POB_THIS,
        sec_class: OB_CLASS_HNDL
    ) -> OB_CLASS_HNDL {
        (self.ob_get_sec_class_ancestor)(obthis, sec_class)
    }
    pub unsafe fn ob_is_class_enum(&self, obThis: POB_THIS, obClassHndl: OB_CLASS_HNDL) -> BOOL {
        (self.ob_is_class_enum)(obThis, obClassHndl)
    }
    pub unsafe fn ob_new_event(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_name: LPTSTR,
        token_name: LPTSTR,
        result_type: LPTSTR,
        no_args: INT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        is_external_event: BOOL
    ) -> OB_VTABLE_ID {
        (self.ob_new_event)(
            obthis,
            class_hndl,
            event_name,
            token_name,
            result_type,
            no_args,
            arg_pass_style,
            arg_type_names,
            arg_names,
            arg_grouping,
            is_external_event
        )
    }
    pub unsafe fn ob_update_event(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        event_name: LPTSTR,
        token_name: LPTSTR,
        result_type: LPTSTR,
        no_args: INT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        is_external_event: BOOL,
        no_throws: INT,
        throws_names: *mut LPTSTR
    ) -> INT {
        (self.ob_update_event)(
            obthis,
            class_hndl,
            vtable_id,
            event_name,
            token_name,
            result_type,
            no_args,
            arg_pass_style,
            arg_type_names,
            arg_names,
            arg_grouping,
            is_external_event,
            no_throws,
            throws_names
        )
    }
    pub unsafe fn ob_delete_event(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_name: LPTSTR
    ) -> INT {
        (self.ob_delete_event)(obthis, class_hndl, event_name)
    }
    pub unsafe fn ob_has_events(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL {
        (self.ob_has_events)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_event_token_id(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_name: LPTSTR,
        vtable_id: POB_VTABLE_ID
    ) -> OB_EVT_TOKEN_ID {
        (self.ob_get_event_token_id)(obthis, class_hndl, event_name, vtable_id)
    }
    pub unsafe fn ob_get_event_id_from_token(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_token: OB_EVT_TOKEN_ID
    ) -> OB_VTABLE_ID {
        (self.ob_get_event_id_from_token)(obthis, class_hndl, event_token)
    }
    pub unsafe fn ob_does_event_script_exist(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_id: OB_VTABLE_ID
    ) -> BOOL {
        (self.ob_does_event_script_exist)(obthis, class_hndl, event_id)
    }
    pub unsafe fn ob_get_routine_name(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> LPTSTR {
        (self.ob_get_routine_name)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_delete_routine(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> () {
        (self.ob_delete_routine)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_get_curr_routine(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_VTABLE_ID {
        (self.ob_get_curr_routine)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_curr_function(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_VTABLE_ID {
        (self.ob_get_curr_function)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_routid_from_vtable_id(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID
    ) -> OB_ROUT_ID {
        (self.ob_get_routid_from_vtable_id)(obthis, class_hndl, vtable_id)
    }
    pub unsafe fn ob_is_valid_event_index(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> BOOL {
        (self.ob_is_valid_event_index)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_has_scripts(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL {
        (self.ob_has_scripts)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_routine_type(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> OB_ROUT_TYPE {
        (self.ob_get_routine_type)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_get_function_vtable_ids(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        include_ancestors: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID {
        (self.ob_get_function_vtable_ids)(obthis, class_hndl, include_dlls, include_ancestors, no_items)
    }
    pub unsafe fn ob_get_function_vtable_ids_for_ide(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        include_ancestors: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID {
        (self.ob_get_function_vtable_ids_for_ide)(
            obthis,
            class_hndl,
            include_dlls,
            include_ancestors,
            no_items
        )
    }
    pub unsafe fn ob_get_event_vtable_ids(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID {
        (self.ob_get_event_vtable_ids)(obthis, class_hndl, include_dlls, no_items)
    }
    pub unsafe fn ob_get_function_name(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> LPTSTR {
        (self.ob_get_function_name)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_delete_function(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> INT {
        (self.ob_delete_function)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_find_routine(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        routine_type: OB_ROUT_TYPE,
        routine_name: LPTSTR,
        result_type: LPTSTR,
        no_args: INT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        ov_error: POB_PROTO_OVERLOAD_ERROR,
        pproto_id: POB_PROTO_ID,
        pvtable_id: POB_VTABLE_ID
    ) -> BOOL {
        (self.ob_find_routine)(
            obthis,
            class_hndl,
            routine_type,
            routine_name,
            result_type,
            no_args,
            arg_pass_style,
            arg_type_names,
            arg_grouping,
            ov_error,
            pproto_id,
            pvtable_id
        )
    }
    pub unsafe fn ob_get_vtable_id_from_proto_id(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        proto_id: OB_PROTO_ID
    ) -> OB_VTABLE_ID {
        (self.ob_get_vtable_id_from_proto_id)(obthis, class_hndl, proto_id)
    }
    pub unsafe fn ob_get_dll_func_names(&self, obthis: POB_THIS, no_funcs: *mut UINT) -> *mut LPTSTR {
        (self.ob_get_dll_func_names)(obthis, no_funcs)
    }
    pub unsafe fn ob_get_global_func_names_in_lib(
        &self,
        obthis: POB_THIS,
        no_funcs: *mut UINT,
        lib_name: LPTSTR
    ) -> *mut LPTSTR {
        (self.ob_get_global_func_names_in_lib)(obthis, no_funcs, lib_name)
    }
    pub unsafe fn ob_get_global_func_index(
        &self,
        obthis: POB_THIS,
        name: LPTSTR,
        class_hndl: POB_CLASS_HNDL
    ) -> OB_VTABLE_ID {
        (self.ob_get_global_func_index)(obthis, name, class_hndl)
    }
    pub unsafe fn ob_get_func_index_in_lib(
        &self,
        obthis: POB_THIS,
        name: LPTSTR,
        lib_name: LPTSTR,
        class_hndl: POB_CLASS_HNDL
    ) -> OB_VTABLE_ID {
        (self.ob_get_func_index_in_lib)(obthis, name, lib_name, class_hndl)
    }
    pub unsafe fn ob_get_proto_is_external_event(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> BOOL {
        (self.ob_get_proto_is_external_event)(obthis, class_hndl, index)
    }
    pub unsafe fn ob_get_protoarg_info(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        no_items: *mut UINT,
        type_name: *mut LPTSTR,
        member_access: POB_MEMBER_ACCESS
    ) -> POB_ARG_INFO {
        (self.ob_get_protoarg_info)(obthis, class_hndl, vtable_id, no_items, type_name, member_access)
    }
    pub unsafe fn ob_get_proto_info(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        no_args: *mut UINT,
        name: *mut LPTSTR,
        type_: *mut LPTSTR,
        member_access: POB_MEMBER_ACCESS,
        dll_lib_name: *mut LPTSTR,
        is_obsolete: *mut BOOL,
        token: POB_EVT_TOKEN_ID,
        rout_type: *mut OB_ROUT_TYPE,
        is_inherit: *mut BOOL
    ) -> POB_ARG_INFO {
        (self.ob_get_proto_info)(
            obthis,
            class_hndl,
            vtable_id,
            no_args,
            name,
            type_,
            member_access,
            dll_lib_name,
            is_obsolete,
            token,
            rout_type,
            is_inherit
        )
    }
    pub unsafe fn ob_get_method_signature(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID
    ) -> LPTSTR {
        (self.ob_get_method_signature)(obthis, class_hndl, vtable_id)
    }
    pub unsafe fn ob_was_event_prototype_changed(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_id: OB_VTABLE_ID
    ) -> BOOL {
        (self.ob_was_event_prototype_changed)(obthis, class_hndl, event_id)
    }
    pub unsafe fn ob_get_proto_name_info(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        func_id: OB_VTABLE_ID,
        name: *mut LPTSTR,
        is_obsolete: *mut BOOL
    ) -> () {
        (self.ob_get_proto_name_info)(obthis, class_hndl, func_id, name, is_obsolete)
    }
    pub unsafe fn ob_get_proto_throws_info(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        func_id: OB_VTABLE_ID,
        throws_list: *mut POB_CLASS_ID,
        no_throws: *mut UINT,
        group_id: POB_GROUP_ID
    ) -> () {
        (self.ob_get_proto_throws_info)(obthis, class_hndl, func_id, throws_list, no_throws, group_id)
    }
    pub unsafe fn ob_lookup_routine_by_name(
        &self,
        obthis: POB_THIS,
        obInst: OB_INST_ID,
        lpstrRoutineName: LPTSTR,
        pVtableId: POB_VTABLE_ID,
        pNumRoutines: *mut UINT,
        pobRoutineType: *mut OB_ROUT_TYPE,
        pNoArgs: *mut UINT,
        ppobArgClassId: *mut POB_CLASS_ID,
        pbVarArgs: *mut BOOL
    ) -> HRESULT {
        (self.ob_lookup_routine_by_name)(
            obthis,
            obInst,
            lpstrRoutineName,
            pVtableId,
            pNumRoutines,
            pobRoutineType,
            pNoArgs,
            ppobArgClassId,
            pbVarArgs
        )
    }
    pub unsafe fn ob_get_objnames_of_class(
        &self,
        obthis: POB_THIS,
        class_id: OB_CLASS_ID,
        no_items: *mut UINT
    ) -> *mut LPTSTR {
        (self.ob_get_objnames_of_class)(obthis, class_id, no_items)
    }
    pub unsafe fn ob_has_object_of_class(&self, obthis: POB_THIS, class_id: OB_CLASS_ID) -> BOOL {
        (self.ob_has_object_of_class)(obthis, class_id)
    }
    pub unsafe fn ob_get_obj_classhndls_of_class(
        &self,
        obthis: POB_THIS,
        obClassID: OB_CLASS_ID,
        pNumberOfItems: *mut UINT
    ) -> POB_CLASS_HNDL {
        (self.ob_get_obj_classhndls_of_class)(obthis, obClassID, pNumberOfItems)
    }
    pub unsafe fn ob_get_objnames_of_class_in_lib(
        &self,
        obthis: POB_THIS,
        class_id: OB_CLASS_ID,
        no_items: *mut UINT,
        lib_name: LPTSTR
    ) -> *mut LPTSTR {
        (self.ob_get_objnames_of_class_in_lib)(obthis, class_id, no_items, lib_name)
    }
    pub unsafe fn ob_global_reference(
        &self,
        obthis: POB_THIS,
        class_name: LPTSTR,
        group_hndl: POB_GROUP_HNDL
    ) -> OB_CLASS_HNDL {
        (self.ob_global_reference)(obthis, class_name, group_hndl)
    }
    pub unsafe fn ob_global_reference_in_lib(
        &self,
        obthis: POB_THIS,
        class_name: LPTSTR,
        lib_name: LPTSTR,
        group_hndl: POB_GROUP_HNDL,
        class_id: OB_CLASS_ID
    ) -> OB_CLASS_HNDL {
        (self.ob_global_reference_in_lib)(obthis, class_name, lib_name, group_hndl, class_id)
    }
    pub unsafe fn ob_global_reference_of_class(
        &self,
        obthis: POB_THIS,
        name: LPTSTR,
        grouphndl: POB_GROUP_HNDL,
        of_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL {
        (self.ob_global_reference_of_class)(obthis, name, grouphndl, of_class)
    }
    pub unsafe fn ob_get_obinst_class_hndl(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> OB_CLASS_HNDL {
        (self.ob_get_obinst_class_hndl)(obthis, obinst)
    }
    pub unsafe fn ob_is_a_typedef(&self, obthis: POB_THIS, data: POB_DATA) -> BOOL {
        (self.ob_is_a_typedef)(obthis, data)
    }
    pub unsafe fn ob_is_an_enum(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        data_node: POB_DATA
    ) -> BOOL {
        (self.ob_is_an_enum)(obthis, group_hndl, data_node)
    }
    pub unsafe fn ob_get_system_class(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_ID {
        (self.ob_get_system_class)(obthis, class_hndl)
    }
    pub unsafe fn ob_get_obinst_system_class(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> OB_CLASS_ID {
        (self.ob_get_obinst_system_class)(obthis, obinst)
    }
    pub unsafe fn ob_get_obinst_group_hndl(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> OB_GROUP_HNDL {
        (self.ob_get_obinst_group_hndl)(obthis, obinst)
    }
    pub unsafe fn ob_get_obinst_class_name(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> LPTSTR {
        (self.ob_get_obinst_class_name)(obthis, obinst)
    }
    pub unsafe fn ob_fetch_fields_of_class(
        &self,
        obthis: POB_THIS,
        in_class: OB_CLASS_HNDL,
        of_class: OB_CLASS_ID,
        field_filter: OB_FIELD_FILTER,
        class_list: *mut POB_CLASS_ID,
        no_items: *mut UINT
    ) -> *mut LPTSTR {
        (self.ob_fetch_fields_of_class)(obthis, in_class, of_class, field_filter, class_list, no_items)
    }
    pub unsafe fn ob_get_fields_of_class(
        &self,
        obthis: POB_THIS,
        in_class: OB_CLASS_HNDL,
        of_class: OB_CLASS_ID,
        class_list: *mut POB_CLASS_ID,
        no_items: *mut UINT
    ) -> *mut LPTSTR {
        (self.ob_get_fields_of_class)(obthis, in_class, of_class, class_list, no_items)
    }
    pub unsafe fn ob_get_local_var_info(&self, obthis: POB_THIS, no_items: *mut UINT) -> POB_DATA_INFO {
        (self.ob_get_local_var_info)(obthis, no_items)
    }
    pub unsafe fn ob_get_shared_vars_of_class(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        return_shared: BOOL,
        return_global: BOOL,
        of_class: OB_CLASS_ID,
        no_items: *mut UINT
    ) -> POB_DATA_INFO {
        (self.ob_get_shared_vars_of_class)(
            obthis,
            group_hndl,
            return_shared,
            return_global,
            of_class,
            no_items
        )
    }
    pub unsafe fn ob_get_shared_var_info(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        return_shared: BOOL,
        return_global: BOOL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO {
        (self.ob_get_shared_var_info)(obthis, group_hndl, return_shared, return_global, no_items)
    }
    pub unsafe fn ob_get_global_vars_of_class(
        &self,
        obthis: POB_THIS,
        of_class: OB_CLASS_ID,
        no_items: *mut UINT
    ) -> POB_DATA_INFO {
        (self.ob_get_global_vars_of_class)(obthis, of_class, no_items)
    }
    pub unsafe fn ob_get_class_field_info(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO {
        (self.ob_get_class_field_info)(obthis, class_hndl, no_items)
    }
    pub unsafe fn ob_get_enum_info(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_enums: *mut UINT
    ) -> POB_ENUM_INFO {
        (self.ob_get_enum_info)(obthis, class_hndl, no_enums)
    }
    pub unsafe fn ob_get_class_event_info(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_events: *mut UINT
    ) -> POB_EVENT_INFO {
        (self.ob_get_class_event_info)(obthis, class_hndl, no_events)
    }
    pub unsafe fn ob_get_instance_field_info(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO {
        (self.ob_get_instance_field_info)(obthis, class_hndl, no_items)
    }
    pub unsafe fn ob_get_obinst_field_info(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        no_items: *mut UINT,
        field_filter: BOOL
    ) -> POB_DATA_INFO {
        (self.ob_get_obinst_field_info)(obthis, obinst, no_items, field_filter)
    }
    pub unsafe fn ob_get_obinst_all_field_info(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        no_items: *mut UINT,
        field_filter: BOOL
    ) -> POB_DATA_INFO {
        (self.ob_get_obinst_all_field_info)(obthis, obinst, no_items, field_filter)
    }
    pub unsafe fn ob_get_classes_within_group(
        &self,
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        of_class_id: OB_CLASS_ID,
        total_items: *mut UINT
    ) -> POB_CLASS_INFO {
        (self.ob_get_classes_within_group)(obthis, in_group_hndl, of_class_id, total_items)
    }
    pub unsafe fn ob_get_enums_within_group(
        &self,
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        total_items: *mut UINT
    ) -> POB_CLASS_INFO {
        (self.ob_get_enums_within_group)(obthis, in_group_hndl, total_items)
    }
    pub unsafe fn ob_get_global_var_data(&self, obthis: POB_THIS, var_name: LPTSTR) -> POB_DATA {
        (self.ob_get_global_var_data)(obthis, var_name)
    }
    pub unsafe fn ob_object_reference_count(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG {
        (self.ob_object_reference_count)(obthis, obinst)
    }
    pub unsafe fn ob_named_global_var_info(&self, obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO {
        (self.ob_named_global_var_info)(obthis, varname)
    }
    pub unsafe fn ob_named_shared_var_info(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        varname: LPTSTR
    ) -> POB_DATA_INFO {
        (self.ob_named_shared_var_info)(obthis, group_hndl, varname)
    }
    pub unsafe fn ob_named_special_var_info(&self, obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO {
        (self.ob_named_special_var_info)(obthis, varname)
    }
    pub unsafe fn ob_named_local_var_info(&self, obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO {
        (self.ob_named_local_var_info)(obthis, varname)
    }
    pub unsafe fn ob_named_field_info(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        fieldname: LPTSTR
    ) -> POB_DATA_INFO {
        (self.ob_named_field_info)(obthis, obinst, fieldname)
    }
    pub unsafe fn ob_get_array_info(&self, obthis: POB_THIS, data_node: POB_DATA) -> POB_ARRAY_INFO {
        (self.ob_get_array_info)(obthis, data_node)
    }
    pub unsafe fn ob_get_array_bounds_string(
        &self,
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        data_node: POB_DATA
    ) -> LPTSTR {
        (self.ob_get_array_bounds_string)(obthis, group_hndl, data_node)
    }
    pub unsafe fn ob_get_array_bounds_string_from_field_info(
        &self,
        obthis: POB_THIS,
        fieldinfo: POB_DATA_INFO
    ) -> LPTSTR {
        (self.ob_get_array_bounds_string_from_field_info)(obthis, fieldinfo)
    }
    pub unsafe fn ob_get_info_watchpoint(
        &self,
        obthis: POB_THIS,
        info: POB_DATA_INFO
    ) -> *mut ::std::os::raw::c_void {
        (self.ob_get_info_watchpoint)(obthis, info)
    }
    pub unsafe fn ob_compile_lib_entry(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR,
        write_source: BOOL
    ) -> INT {
        (self.ob_compile_lib_entry)(obthis, lib_name, entry_name, write_source)
    }
    pub unsafe fn ob_compile_lib_typedefs(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR,
        bUpdateModifyTime: BOOL
    ) -> INT {
        (self.ob_compile_lib_typedefs)(obthis, lib_name, entry_name, bUpdateModifyTime)
    }
    pub unsafe fn ob_compile_lib_entry_3_pass(
        &self,
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszEntryName: LPTSTR
    ) -> BOOL {
        (self.ob_compile_lib_entry_3_pass)(obThis, lpszLibraryName, lpszEntryName)
    }
    pub unsafe fn ob_compile_lib_scripts(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR
    ) -> INT {
        (self.ob_compile_lib_scripts)(obthis, lib_name, entry_name)
    }
    pub unsafe fn ob_func_search(
        &self,
        obthis: POB_THIS,
        name: LPTSTR,
        argtypes: POB_CLASS_ID,
        no_args: UINT,
        type_: POB_CLASS_ID,
        error: POB_PROTOREF_ERROR
    ) -> POB_FUNCCALL_INFO {
        (self.ob_func_search)(obthis, name, argtypes, no_args, type_, error)
    }
    pub unsafe fn ob_del_funccall_info(&self, obthis: POB_THIS, funccall_info: POB_FUNCCALL_INFO) -> () {
        (self.ob_del_funccall_info)(obthis, funccall_info)
    }
    pub unsafe fn ob_link_project(&self, obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT {
        (self.ob_link_project)(obthis, group_hndl)
    }
    pub unsafe fn ob_check_for_locked_menu(&self, obthis: POB_THIS) -> () {
        (self.ob_check_for_locked_menu)(obthis)
    }
    pub unsafe fn ob_create_obinst(&self, obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_INST_ID {
        (self.ob_create_obinst)(obthis, class_hndl)
    }
    pub unsafe fn ob_instantiate_child_object(
        &self,
        obthis: POB_THIS,
        parent_obinst: OB_INST_ID,
        child_type: OB_CLASS_ID,
        field_id: UINT,
        child_obinst: POB_INST_ID
    ) -> INT {
        (self.ob_instantiate_child_object)(obthis, parent_obinst, child_type, field_id, child_obinst)
    }
    pub unsafe fn ob_instantiate_system_object(
        &self,
        obthis: POB_THIS,
        object_type: OB_CLASS_ID,
        pObint: POB_INST_ID
    ) -> INT {
        (self.ob_instantiate_system_object)(obthis, object_type, pObint)
    }
    pub unsafe fn ob_destroy_obinst(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> INT {
        (self.ob_destroy_obinst)(obthis, obinst)
    }
    pub unsafe fn ob_set_runtime(&self, obthis: POB_THIS, bInRuntime: BOOL) -> () {
        (self.ob_set_runtime)(obthis, bInRuntime)
    }
    pub unsafe fn ob_create_executable(
        &self,
        obthis: POB_THIS,
        pExecBlock: POB_EXEC,
        bFreeGroups: BOOL,
        pManifestInfo: LPTSTR
    ) -> INT {
        (self.ob_create_executable)(obthis, pExecBlock, bFreeGroups, pManifestInfo)
    }
    pub unsafe fn ob_create_library(&self, obthis: POB_THIS, pExecBlock: POB_EXEC) -> INT {
        (self.ob_create_library)(obthis, pExecBlock)
    }
    pub unsafe fn ob_create_consolidated_library(
        &self,
        obthis: POB_THIS,
        pTargetLibrary: LPTSTR,
        nSourceLibs: ULONG,
        pSourceLibs: *mut LPTSTR,
        pIncludeLibType: POB_LIB_INCLUDE_TYPE,
        nComponents: ULONG,
        pComponents: *mut LPTSTR,
        pPBRFile: LPTSTR,
        pPBDArray: *mut PBD_ARRAY,
        pNumPBD: *mut ULONG,
        ppErrorMessage: *mut LPTSTR
    ) -> INT {
        (self.ob_create_consolidated_library)(
            obthis,
            pTargetLibrary,
            nSourceLibs,
            pSourceLibs,
            pIncludeLibType,
            nComponents,
            pComponents,
            pPBRFile,
            pPBDArray,
            pNumPBD,
            ppErrorMessage
        )
    }
    pub unsafe fn ob_create_interface_class(
        &self,
        obthis: POB_THIS,
        hSourceClass: OB_CLASS_HNDL,
        lpstrDestClassName: LPTSTR,
        lpstrDestLibrary: LPTSTR,
        lpstrComments: LPTSTR,
        lpstrSourceClassName: LPTSTR
    ) -> HRESULT {
        (self.ob_create_interface_class)(
            obthis,
            hSourceClass,
            lpstrDestClassName,
            lpstrDestLibrary,
            lpstrComments,
            lpstrSourceClassName
        )
    }
    pub unsafe fn ob_init_executable(&self, obthis: POB_THIS, executable_name: LPTSTR) -> OB_CLASS_HNDL {
        (self.ob_init_executable)(obthis, executable_name)
    }
    pub unsafe fn ob_scan_source_blocks(
        &self,
        obthis: POB_THIS,
        source: POB_SOURCE_BLOCK,
        src_len: ULONG,
        srcloc: *mut *mut ::std::os::raw::c_void,
        no_blocks: *mut UINT,
        subpool: OB_SUBPOOL
    ) -> *mut LPTSTR {
        (self.ob_scan_source_blocks)(obthis, source, src_len, srcloc, no_blocks, subpool)
    }
    pub unsafe fn ob_create_launcher(
        &self,
        obThis: POB_THIS,
        pExecBlock: POB_EXEC,
        pObjectList: *mut ::std::os::raw::c_void
    ) -> INT {
        (self.ob_create_launcher)(obThis, pExecBlock, pObjectList)
    }
    pub unsafe fn ob_sanitize_pb_name(
        &self,
        obThis: POB_THIS,
        lpszDestName: LPTSTR,
        destLength: ::std::os::raw::c_long,
        lpszNameToSanitize: LPTSTR
    ) -> () {
        (self.ob_sanitize_pb_name)(obThis, lpszDestName, destLength, lpszNameToSanitize)
    }
    pub unsafe fn ob_validate_class(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_CONFLICT_LIST {
        (self.ob_validate_class)(obthis, class_hndl, no_items)
    }
    pub unsafe fn ob_get_orphaned_classes(
        &self,
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        total_items: *mut UINT
    ) -> POB_CLASS_HNDL {
        (self.ob_get_orphaned_classes)(obthis, in_group_hndl, total_items)
    }
    pub unsafe fn ob_validate_type_name(
        &self,
        obThis: POB_THIS,
        obGroupHndl: OB_GROUP_HNDL,
        TypeName: LPTSTR
    ) -> BOOL {
        (self.ob_validate_type_name)(obThis, obGroupHndl, TypeName)
    }
    pub unsafe fn ob_convert_to_ver2_source(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR
    ) -> INT {
        (self.ob_convert_to_ver2_source)(obthis, lib_name, entry_name)
    }
    pub unsafe fn ob_is_vers2_obj(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR,
        error: *mut INT
    ) -> BOOL {
        (self.ob_is_vers2_obj)(obthis, lib_name, entry_name, error)
    }
    pub unsafe fn ob_build_ordered_compile_list(
        &self,
        obthis: POB_THIS,
        list_type: OB_COMPILE_LIST_TYPE,
        no_items: *mut UINT,
        inconsistency: POB_INCONSISTENCY_TYPE
    ) -> POB_COMPILE_LIST {
        (self.ob_build_ordered_compile_list)(obthis, list_type, no_items, inconsistency)
    }
    pub unsafe fn ob_free_ordered_compile_list(
        &self,
        obthis: POB_THIS,
        compile_list: POB_COMPILE_LIST,
        no_items: UINT
    ) -> () {
        (self.ob_free_ordered_compile_list)(obthis, compile_list, no_items)
    }
    pub unsafe fn ob_build_hierarchy_list(
        &self,
        obthis: POB_THIS,
        no_items: *mut UINT,
        type_: OB_CLASS_ID
    ) -> POB_HIERARCHY_LIST {
        (self.ob_build_hierarchy_list)(obthis, no_items, type_)
    }
    pub unsafe fn ob_free_hierarchy_list(
        &self,
        obthis: POB_THIS,
        hierarchy_list: POB_HIERARCHY_LIST,
        no_items: UINT
    ) -> () {
        (self.ob_free_hierarchy_list)(obthis, hierarchy_list, no_items)
    }
    pub unsafe fn ob_clear_instance_ref(
        &self,
        obthis: POB_THIS,
        back_ptr: *mut ::std::os::raw::c_void
    ) -> () {
        (self.ob_clear_instance_ref)(obthis, back_ptr)
    }
    pub unsafe fn ob_insert_inst_ref_dbg(
        &self,
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        ref_addr: *mut ::std::os::raw::c_void,
        fileName: LPTSTR,
        lineNo: UINT
    ) -> () {
        (self.ob_insert_inst_ref_dbg)(obthis, obinst, ref_addr, fileName, lineNo)
    }
    pub unsafe fn ob_open_typedef_group(
        &self,
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszGroupName: LPTSTR,
        bCreateIfNotFound: BOOL
    ) -> INT {
        (self.ob_open_typedef_group)(obThis, lpszLibraryName, lpszGroupName, bCreateIfNotFound)
    }
    pub unsafe fn ob_save_dll_to_pbd(&self, argc: ::std::os::raw::c_int, argv: *mut LPTSTR) -> INT {
        (self.ob_save_dll_to_pbd)(argc, argv)
    }
    pub unsafe fn ob_convert_pbx_to_native_groups(
        &self,
        obthis: POB_THIS,
        pbl_name: LPCTSTR,
        dll_name: LPCTSTR
    ) -> INT {
        (self.ob_convert_pbx_to_native_groups)(obthis, pbl_name, dll_name)
    }
    pub unsafe fn ob_share_typedef_group(&self, destObThis: POB_THIS, srcObThis: POB_THIS) -> INT {
        (self.ob_share_typedef_group)(destObThis, srcObThis)
    }
    pub unsafe fn ob_unshare_typedef_group(&self, obThis: POB_THIS) -> INT {
        (self.ob_unshare_typedef_group)(obThis)
    }
    pub unsafe fn ob_cm_evaluate_expression(
        &self,
        obthis: POB_THIS,
        text: LPTSTR,
        result_data_node: POB_DATA
    ) -> INT {
        (self.ob_cm_evaluate_expression)(obthis, text, result_data_node)
    }
    pub unsafe fn ob_entryInheritsFromClass(
        &self,
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszTypeName: LPTSTR,
        lpszEntryName: LPTSTR
    ) -> BOOL {
        (self.ob_entryInheritsFromClass)(obThis, lpszLibraryName, lpszTypeName, lpszEntryName)
    }
    pub unsafe fn ob_get_class_from_name(
        &self,
        obThis: POB_THIS,
        lpszClassName: LPTSTR,
        pbIsEnum: *mut BOOL
    ) -> OB_CLASS_HNDL {
        (self.ob_get_class_from_name)(obThis, lpszClassName, pbIsEnum)
    }
    pub unsafe fn ob_local_global_lv(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        var_id: OB_SYM_ID
    ) -> POB_DATA {
        (self.ob_local_global_lv)(obthis, group, var_id)
    }
    pub unsafe fn ob_local_global_refpkt(
        &self,
        obthis: POB_THIS,
        destination: POB_DATA,
        group: POB_GROUP,
        var_id: OB_SYM_ID
    ) -> () {
        (self.ob_local_global_refpkt)(obthis, destination, group, var_id)
    }
    pub unsafe fn ob_shared_global_lv(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        var_id: OB_SYM_ID
    ) -> POB_DATA {
        (self.ob_shared_global_lv)(obthis, group, var_id)
    }
    pub unsafe fn ob_shared_global_refpkt(
        &self,
        obthis: POB_THIS,
        destination: POB_DATA,
        group: POB_GROUP,
        var_id: OB_SYM_ID
    ) -> () {
        (self.ob_shared_global_refpkt)(obthis, destination, group, var_id)
    }
    pub unsafe fn ob_shared_lv(&self, obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID) -> POB_DATA {
        (self.ob_shared_lv)(obthis, group, var_id)
    }
    pub unsafe fn ob_shared_refpkt(
        &self,
        obthis: POB_THIS,
        destination: POB_DATA,
        group: POB_GROUP,
        var_id: OB_SYM_ID
    ) -> () {
        (self.ob_shared_refpkt)(obthis, destination, group, var_id)
    }
    pub unsafe fn ob_convert_chararray_to_string(&self, obthis: POB_THIS, data: POB_DATA) -> BOOL {
        (self.ob_convert_chararray_to_string)(obthis, data)
    }
    pub unsafe fn ob_class_delete_and_withinclass(
        &self,
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        class_id: OB_CLASS_ID
    ) -> () {
        (self.ob_class_delete_and_withinclass)(obthis, class_hndl, class_id)
    }
    pub unsafe fn ob_find_orphan_class(
        &self,
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszEntryName: LPTSTR,
        bFoundAncestor: BOOL
    ) -> INT {
        (self.ob_find_orphan_class)(obThis, lpszLibraryName, lpszEntryName, bFoundAncestor)
    }
    pub unsafe fn ob_nuke_orphan_class(
        &self,
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszEntryName: LPTSTR
    ) -> BOOL {
        (self.ob_nuke_orphan_class)(obThis, lpszLibraryName, lpszEntryName)
    }
    pub unsafe fn ob_is_ancestor_class_modified(&self, obThis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL {
        (self.ob_is_ancestor_class_modified)(obThis, class_hndl)
    }
    pub unsafe fn ob_rebuild_instance_image(&self, obThis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> () {
        (self.ob_rebuild_instance_image)(obThis, class_hndl)
    }
    pub unsafe fn ob_build_compile_list(&self, obthis: POB_THIS, no_items: *mut UINT) -> POB_COMPILE_LIST {
        (self.ob_build_compile_list)(obthis, no_items)
    }
    pub unsafe fn ot_get_next_evaled_arg(&self, obthis: POB_THIS) -> POB_DATA {
        (self.ot_get_next_evaled_arg)(obthis)
    }
    pub unsafe fn ot_get_next_evaled_arg_no_convert(&self, obthis: POB_THIS) -> POB_DATA {
        (self.ot_get_next_evaled_arg_no_convert)(obthis)
    }
    pub unsafe fn ot_get_next_lvalue_arg(&self, obthis: POB_THIS, str_: *mut POT_LVALUE_INFO) -> POB_DATA {
        (self.ot_get_next_lvalue_arg)(obthis, str_)
    }
    pub unsafe fn ot_get_simple_intarg(&self, obthis: POB_THIS, null: *mut BOOL) -> INT {
        (self.ot_get_simple_intarg)(obthis, null)
    }
    pub unsafe fn ot_get_simple_longarg(&self, obthis: POB_THIS, null: *mut BOOL) -> ::std::os::raw::c_long {
        (self.ot_get_simple_longarg)(obthis, null)
    }
    pub unsafe fn ot_get_intarg(&self, obthis: POB_THIS, null: *mut BOOL) -> INT {
        (self.ot_get_intarg)(obthis, null)
    }
    pub unsafe fn ot_get_uintarg(&self, obthis: POB_THIS, null: *mut BOOL) -> UINT {
        (self.ot_get_uintarg)(obthis, null)
    }
    pub unsafe fn ot_get_longarg(&self, obthis: POB_THIS, null: *mut BOOL) -> ::std::os::raw::c_long {
        (self.ot_get_longarg)(obthis, null)
    }
    pub unsafe fn ot_get_ulongarg(&self, obthis: POB_THIS, null: *mut BOOL) -> ULONG {
        (self.ot_get_ulongarg)(obthis, null)
    }
    pub unsafe fn ot_get_decarg(&self, obthis: POB_THIS, null: *mut BOOL) -> PSH_DEC {
        (self.ot_get_decarg)(obthis, null)
    }
    pub unsafe fn ot_get_floatarg(&self, obthis: POB_THIS, fl: *mut f32, null: *mut BOOL) -> *mut f32 {
        (self.ot_get_floatarg)(obthis, fl, null)
    }
    pub unsafe fn ot_get_doublearg(&self, obthis: POB_THIS, doub: *mut f64, null: *mut BOOL) -> *mut f64 {
        (self.ot_get_doublearg)(obthis, doub, null)
    }
    pub unsafe fn ot_get_longlongarg(
        &self,
        obthis: POB_THIS,
        longlong_val: *mut ::std::os::raw::c_longlong,
        null: *mut BOOL
    ) -> *mut ::std::os::raw::c_longlong {
        (self.ot_get_longlongarg)(obthis, longlong_val, null)
    }
    pub unsafe fn ot_get_obinstarg(
        &self,
        obthis: POB_THIS,
        obinst: POB_INST_ID,
        null: *mut BOOL
    ) -> POB_INST_ID {
        (self.ot_get_obinstarg)(obthis, obinst, null)
    }
    pub unsafe fn ot_get_valptr_arg(&self, obthis: POB_THIS, null: *mut BOOL) -> *mut ::std::os::raw::c_void {
        (self.ot_get_valptr_arg)(obthis, null)
    }
    pub unsafe fn ot_init_arglist(&self, obthis: POB_THIS, nargs: UINT) -> UINT {
        (self.ot_init_arglist)(obthis, nargs)
    }
    pub unsafe fn ot_get_valptr(&self, obthis: POB_THIS, data: POB_DATA) -> *mut ::std::os::raw::c_void {
        (self.ot_get_valptr)(obthis, data)
    }
    pub unsafe fn ot_type_srch(&self, name: LPTSTR) -> INT { (self.ot_type_srch)(name) }
    pub unsafe fn ot_type_attr(&self, type_: OB_CLASS_ID) -> INT { (self.ot_type_attr)(type_) }
    pub unsafe fn ot_get_class_name(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> LPTSTR {
        (self.ot_get_class_name)(obthis, group, class_id)
    }
    pub unsafe fn ot_is_array_eq(
        &self,
        obthis: POB_THIS,
        array_id1: OB_ARRAY_ID,
        array_id2: OB_ARRAY_ID,
        nullval: *mut BOOL
    ) -> BOOL {
        (self.ot_is_array_eq)(obthis, array_id1, array_id2, nullval)
    }
    pub unsafe fn ot_is_struct_eq(
        &self,
        obthis: POB_THIS,
        data_node1: POB_DATA,
        data_node2: POB_DATA,
        nullval: *mut BOOL
    ) -> BOOL {
        (self.ot_is_struct_eq)(obthis, data_node1, data_node2, nullval)
    }
    pub unsafe fn ot_create_obinst_with_name(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        lvalue_info: POT_LVALUE_INFO,
        class_name: LPTSTR,
        nested_obinst: OB_INST_ID
    ) -> OB_INST_ID {
        (self.ot_create_obinst_with_name)(obthis, lvalue_data, lvalue_info, class_name, nested_obinst)
    }
    pub unsafe fn ot_create_obinst_at_lval(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        lvalue_info: POT_LVALUE_INFO,
        nested_obinst: OB_INST_ID
    ) -> OB_INST_ID {
        (self.ot_create_obinst_at_lval)(obthis, lvalue_data, lvalue_info, nested_obinst)
    }
    pub unsafe fn ot_get_curr_obinst_expr(
        &self,
        obthis: POB_THIS,
        obinst_buf: POB_INST_ID,
        nullval: *mut BOOL
    ) -> POB_INST_ID {
        (self.ot_get_curr_obinst_expr)(obthis, obinst_buf, nullval)
    }
    pub unsafe fn ot_func_call(
        &self,
        obthis: POB_THIS,
        funccall_info: POB_FUNCCALL_INFO,
        actual_args: *mut *mut ::std::os::raw::c_void
    ) -> POB_DATA {
        (self.ot_func_call)(obthis, funccall_info, actual_args)
    }
    pub unsafe fn ot_set_return_val(&self, obthis: POB_THIS, data_node: POB_DATA) -> () {
        (self.ot_set_return_val)(obthis, data_node)
    }
    pub unsafe fn ot_set_return_double(&self, obthis: POB_THIS, doub_val: *mut f64, null_val: BOOL) -> () {
        (self.ot_set_return_double)(obthis, doub_val, null_val)
    }
    pub unsafe fn ot_set_return_longlong(
        &self,
        obthis: POB_THIS,
        longl_val: *mut ::std::os::raw::c_longlong,
        null_val: BOOL
    ) -> () {
        (self.ot_set_return_longlong)(obthis, longl_val, null_val)
    }
    pub unsafe fn ot_set_return_dec(&self, obthis: POB_THIS, dec_val: PSH_DEC, null_val: BOOL) -> () {
        (self.ot_set_return_dec)(obthis, dec_val, null_val)
    }
    pub unsafe fn ot_no_return_val(&self, obthis: POB_THIS) -> () { (self.ot_no_return_val)(obthis) }
    pub unsafe fn ot_assign_lvalue_dec(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        val: PSH_DEC,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_lvalue_dec)(obthis, lvalue_data, val, nullval)
    }
    pub unsafe fn ot_assign_lvalue_double(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        val: f64,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_lvalue_double)(obthis, lvalue_data, val, nullval)
    }
    pub unsafe fn ot_assign_lvalue_longlong(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        val: ::std::os::raw::c_longlong,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_lvalue_longlong)(obthis, lvalue_data, val, nullval)
    }
    pub unsafe fn ot_assign_lvalue_blob(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        val: PSH_BINARY,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_lvalue_blob)(obthis, lvalue_data, val, nullval)
    }
    pub unsafe fn ot_assign_lvalue_obinst(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        val: OB_INST_ID,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_lvalue_obinst)(obthis, lvalue_data, val, nullval)
    }
    pub unsafe fn ot_assign_lvalue_array(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        rvalue_array: OB_ARRAY_ID,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_lvalue_array)(obthis, lvalue_data, rvalue_array, nullval)
    }
    pub unsafe fn ot_assign_lvalue_any(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        rvalue_data: POB_DATA,
        rhs_class_id: OB_CLASS_ID
    ) -> () {
        (self.ot_assign_lvalue_any)(obthis, lvalue_data, rvalue_data, rhs_class_id)
    }
    pub unsafe fn ot_set_local_var(&self, ths: POB_THIS, sym_id: OB_SYM_ID, data_node: POB_DATA) -> INT {
        (self.ot_set_local_var)(ths, sym_id, data_node)
    }
    pub unsafe fn ot_set_shared_var(
        &self,
        ths: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        sym_id: OB_SYM_ID,
        data_node: POB_DATA
    ) -> INT {
        (self.ot_set_shared_var)(ths, group_hndl, sym_id, data_node)
    }
    pub unsafe fn ot_set_obinst_var(
        &self,
        ths: POB_THIS,
        ob_inst_id: OB_INST_ID,
        field_id: UINT,
        data_node: POB_DATA
    ) -> INT {
        (self.ot_set_obinst_var)(ths, ob_inst_id, field_id, data_node)
    }
    pub unsafe fn ot_set_local_array_item(
        &self,
        ths: POB_THIS,
        sym_id: OB_SYM_ID,
        index: UINT,
        data_node: POB_DATA
    ) -> INT {
        (self.ot_set_local_array_item)(ths, sym_id, index, data_node)
    }
    pub unsafe fn ot_set_shared_array_item(
        &self,
        ths: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        sym_id: OB_SYM_ID,
        index: UINT,
        data_node: POB_DATA
    ) -> INT {
        (self.ot_set_shared_array_item)(ths, group_hndl, sym_id, index, data_node)
    }
    pub unsafe fn ot_set_obinst_array_item(
        &self,
        ths: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        index: ULONG,
        new_data: POB_DATA
    ) -> INT {
        (self.ot_set_obinst_array_item)(ths, obinst, field_id, index, new_data)
    }
    pub unsafe fn ot_get_array_values(
        &self,
        obthis: POB_THIS,
        arraynode: POB_DATA,
        nitems: *mut UINT
    ) -> *mut ::std::os::raw::c_void {
        (self.ot_get_array_values)(obthis, arraynode, nitems)
    }
    pub unsafe fn ot_reset_array(&self, obthis: POB_THIS, array_node: POB_DATA, nitems: ULONG) -> INT {
        (self.ot_reset_array)(obthis, array_node, nitems)
    }
    pub unsafe fn ot_get_local_var(
        &self,
        obthis: POB_THIS,
        grphndl: OB_GROUP_HNDL,
        sym_id: OB_SYM_ID
    ) -> POB_DATA {
        (self.ot_get_local_var)(obthis, grphndl, sym_id)
    }
    pub unsafe fn ot_get_shared_var(
        &self,
        obthis: POB_THIS,
        grphndl: OB_GROUP_HNDL,
        sym_id: OB_SYM_ID
    ) -> POB_DATA {
        (self.ot_get_shared_var)(obthis, grphndl, sym_id)
    }
    pub unsafe fn ot_math_type_convert(&self, class_id1: OB_CLASS_ID, class_id2: OB_CLASS_ID) -> OB_CLASS_ID {
        (self.ot_math_type_convert)(class_id1, class_id2)
    }
    pub unsafe fn ot_get_int_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> INT {
        (self.ot_get_int_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_uint_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> UINT {
        (self.ot_get_uint_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_byte_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_uchar {
        (self.ot_get_byte_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_long_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_long {
        (self.ot_get_long_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_ulong_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> ULONG {
        (self.ot_get_ulong_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_dec_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> PSH_DEC {
        (self.ot_get_dec_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_float_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> f32 {
        (self.ot_get_float_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_double_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> f64 {
        (self.ot_get_double_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_longlong_value(
        &self,
        obthis: POB_THIS,
        data_node: POB_DATA
    ) -> ::std::os::raw::c_longlong {
        (self.ot_get_longlong_value)(obthis, data_node)
    }
    pub unsafe fn ot_free_val_ptr(&self, obthis: POB_THIS, data_node: POB_DATA) -> () {
        (self.ot_free_val_ptr)(obthis, data_node)
    }
    pub unsafe fn ot_free_array(&self, obthis: POB_THIS, data_node: POB_DATA) -> () {
        (self.ot_free_array)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_int(&self, obthis: POB_THIS, data_node: POB_DATA) -> INT {
        (self.ot_convert_to_int)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_uint(&self, obthis: POB_THIS, data_node: POB_DATA) -> UINT {
        (self.ot_convert_to_uint)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_byte(
        &self,
        obthis: POB_THIS,
        data_node: POB_DATA
    ) -> ::std::os::raw::c_uchar {
        (self.ot_convert_to_byte)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_long(&self, obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_long {
        (self.ot_convert_to_long)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_ulong(&self, obthis: POB_THIS, data_node: POB_DATA) -> ULONG {
        (self.ot_convert_to_ulong)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_dec(&self, obthis: POB_THIS, data_node: POB_DATA) -> PSH_DEC {
        (self.ot_convert_to_dec)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_float(&self, obthis: POB_THIS, data_node: POB_DATA) -> f32 {
        (self.ot_convert_to_float)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_double(&self, obthis: POB_THIS, data_node: POB_DATA) -> f64 {
        (self.ot_convert_to_double)(obthis, data_node)
    }
    pub unsafe fn ot_convert_to_longlong(
        &self,
        obthis: POB_THIS,
        data_node: POB_DATA
    ) -> ::std::os::raw::c_longlong {
        (self.ot_convert_to_longlong)(obthis, data_node)
    }
    pub unsafe fn ot_ansi_lower(&self, obthis: POB_THIS, string: LPTSTR) -> LPTSTR {
        (self.ot_ansi_lower)(obthis, string)
    }
    pub unsafe fn ot_ansi_upper(&self, obthis: POB_THIS, string: LPTSTR) -> LPTSTR {
        (self.ot_ansi_upper)(obthis, string)
    }
    pub unsafe fn ot_ansi_strcmp(&self, obthis: POB_THIS, stringOne: LPTSTR, stringTwo: LPTSTR) -> INT {
        (self.ot_ansi_strcmp)(obthis, stringOne, stringTwo)
    }
    pub unsafe fn ot_get_field_lv(&self, obthis: POB_THIS, obInst: OB_INST_ID, fieldId: UINT) -> POB_DATA {
        (self.ot_get_field_lv)(obthis, obInst, fieldId)
    }
    pub unsafe fn ot_get_field_item_lv(
        &self,
        obthis: POB_THIS,
        obInst: OB_INST_ID,
        fieldId: UINT,
        item_index: ULONG
    ) -> POB_DATA {
        (self.ot_get_field_item_lv)(obthis, obInst, fieldId, item_index)
    }
    pub unsafe fn ot_assign_ref_int(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: INT,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_int)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_uint(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: UINT,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_uint)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_byte(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_uchar,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_byte)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_long(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_long,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_long)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_ulong(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ULONG,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_ulong)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_dec(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: PSH_DEC,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_dec)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_float(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: f32,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_float)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_double(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: f64,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_double)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_longlong(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_longlong,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_longlong)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_string(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: LPTSTR,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_string)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_bool(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: BOOL,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_bool)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_char(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: TCHAR,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_char)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_blob(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: PSH_BINARY,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_blob)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_time(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: PSH_TIME,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_time)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_date(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: PSH_TIME,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_date)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_datetime(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: PSH_TIME,
        nullval: BOOL
    ) -> () {
        (self.ot_assign_ref_datetime)(obthis, refpak, value, nullval)
    }
    pub unsafe fn ot_assign_ref_obinst(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: OB_INST_ID,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ) -> INT {
        (self.ot_assign_ref_obinst)(obthis, refpak, value, nullval, type_)
    }
    pub unsafe fn ot_assign_ref_enum(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: INT,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ) -> () {
        (self.ot_assign_ref_enum)(obthis, refpak, value, nullval, type_)
    }
    pub unsafe fn ot_assign_ref_array(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: OB_ARRAY_ID,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ) -> () {
        (self.ot_assign_ref_array)(obthis, refpak, value, nullval, type_)
    }
    pub unsafe fn ot_assign_ref_any(
        &self,
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        rvalue_data: POB_DATA,
        rhs_class_id: OB_CLASS_ID
    ) -> () {
        (self.ot_assign_ref_any)(obthis, refpak, rvalue_data, rhs_class_id)
    }
    pub unsafe fn ot_get_nested_obinst(&self, obthis: POB_THIS, obinst: OB_INST_ID) -> OB_INST_ID {
        (self.ot_get_nested_obinst)(obthis, obinst)
    }
    pub unsafe fn ot_array_create_bounded(
        &self,
        obthis: POB_THIS,
        num_items: ULONG,
        elmtType: OB_CLASS_HNDL,
        varInfo: USHORT,
        numDim: USHORT,
        boundsArray: *mut ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void {
        (self.ot_array_create_bounded)(obthis, num_items, elmtType, varInfo, numDim, boundsArray)
    }
    pub unsafe fn ot_array_create_unbounded(
        &self,
        obthis: POB_THIS,
        elmtType: OB_CLASS_HNDL,
        varInfo: USHORT
    ) -> *mut ::std::os::raw::c_void {
        (self.ot_array_create_unbounded)(obthis, elmtType, varInfo)
    }
    pub unsafe fn ot_array_index(
        &self,
        obthis: POB_THIS,
        array: *mut ::std::os::raw::c_void,
        index: ULONG
    ) -> POB_DATA {
        (self.ot_array_index)(obthis, array, index)
    }
    pub unsafe fn ot_array_set_free_data(
        &self,
        obthis: POB_THIS,
        array: *mut ::std::os::raw::c_void,
        newValue: BOOL
    ) -> () {
        (self.ot_array_set_free_data)(obthis, array, newValue)
    }
    pub unsafe fn ot_array_free_data(&self, obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> BOOL {
        (self.ot_array_free_data)(obthis, array)
    }
    pub unsafe fn ot_array_class_id(
        &self,
        obthis: POB_THIS,
        array: *mut ::std::os::raw::c_void
    ) -> OB_CLASS_ID {
        (self.ot_array_class_id)(obthis, array)
    }
    pub unsafe fn ot_array_class_hndl(
        &self,
        obthis: POB_THIS,
        array: *mut ::std::os::raw::c_void
    ) -> OB_CLASS_HNDL {
        (self.ot_array_class_hndl)(obthis, array)
    }
    pub unsafe fn ot_array_num_dimensions(
        &self,
        obthis: POB_THIS,
        array: *mut ::std::os::raw::c_void
    ) -> USHORT {
        (self.ot_array_num_dimensions)(obthis, array)
    }
    pub unsafe fn ot_array_num_items(&self, obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> ULONG {
        (self.ot_array_num_items)(obthis, array)
    }
    pub unsafe fn ot_is_array_unbounded(&self, obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> BOOL {
        (self.ot_is_array_unbounded)(obthis, array)
    }
    pub unsafe fn ot_get_arraydef_no_dims(
        &self,
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void
    ) -> USHORT {
        (self.ot_get_arraydef_no_dims)(obthis, arrdef)
    }
    pub unsafe fn ot_get_arraydef_style(
        &self,
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void
    ) -> OB_ARRAY_SYMBOL_STYLE {
        (self.ot_get_arraydef_style)(obthis, arrdef)
    }
    pub unsafe fn ot_get_arraydef_bounds(
        &self,
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_long {
        (self.ot_get_arraydef_bounds)(obthis, arrdef)
    }
    pub unsafe fn ot_get_arraydef_varinfo(
        &self,
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void
    ) -> OB_INFO_FLAGS {
        (self.ot_get_arraydef_varinfo)(obthis, arrdef)
    }
    pub unsafe fn ot_get_arraydef_upper_bound(
        &self,
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void,
        dimension: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long {
        (self.ot_get_arraydef_upper_bound)(obthis, arrdef, dimension)
    }
    pub unsafe fn ot_get_arraydef_lower_bound(
        &self,
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void,
        dimension: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long {
        (self.ot_get_arraydef_lower_bound)(obthis, arrdef, dimension)
    }
    pub unsafe fn ot_randomize(&self, obthis: POB_THIS, iSeed: UINT) -> () {
        (self.ot_randomize)(obthis, iSeed)
    }
    pub unsafe fn ot_rand(&self, obthis: POB_THIS, lLimit: ::std::os::raw::c_long) -> ::std::os::raw::c_long {
        (self.ot_rand)(obthis, lLimit)
    }
    pub unsafe fn ot_class_compare(
        &self,
        obthis: POB_THIS,
        classHndl1: OB_CLASS_HNDL,
        classHndl2: OB_CLASS_HNDL
    ) -> BOOL {
        (self.ot_class_compare)(obthis, classHndl1, classHndl2)
    }
    pub unsafe fn ot_assign_global_var_obinst(
        &self,
        obthis: POB_THIS,
        szName: LPTSTR,
        obInst: OB_INST_ID
    ) -> INT {
        (self.ot_assign_global_var_obinst)(obthis, szName, obInst)
    }
    pub unsafe fn ob_class_indirect(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID
    ) -> INT {
        (self.ob_class_indirect)(obthis, group, class_id)
    }
    pub unsafe fn ob_add_external_class_ref(
        &self,
        obthis: POB_THIS,
        name: LPTSTR,
        local_group: POB_GROUP,
        ext_group_id: OB_GROUP_ID,
        ext_class_id: OB_CLASS_ID,
        refstyle: OB_GLOB_REFSTYLE,
        error: *mut INT
    ) -> OB_CLASS_ID {
        (self.ob_add_external_class_ref)(
            obthis,
            name,
            local_group,
            ext_group_id,
            ext_class_id,
            refstyle,
            error
        )
    }
    pub unsafe fn ob_get_local_class(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        refstyle: OB_GLOB_REFSTYLE,
        error: *mut INT
    ) -> OB_CLASS_ID {
        (self.ob_get_local_class)(obthis, group, name, refstyle, error)
    }
    pub unsafe fn ob_get_primary_class(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> OB_CLASS_ID {
        (self.ob_get_primary_class)(obthis, group, class_id)
    }
    pub unsafe fn ob_build_qual_sec_class_name(
        &self,
        obthis: POB_THIS,
        primary_class_name: LPTSTR,
        sec_class_name: LPTSTR
    ) -> LPTSTR {
        (self.ob_build_qual_sec_class_name)(obthis, primary_class_name, sec_class_name)
    }
    pub unsafe fn ob_decl_indirect_sec_class(
        &self,
        obthis: POB_THIS,
        target_group: POB_GROUP,
        prim_class_name: LPTSTR,
        sec_class_name: LPTSTR,
        error: *mut INT
    ) -> OB_CLASS_ID {
        (self.ob_decl_indirect_sec_class)(obthis, target_group, prim_class_name, sec_class_name, error)
    }
    pub unsafe fn ob_update_class_ref(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        refstyle: OB_GLOB_REFSTYLE,
        is_prim_parent: BOOL
    ) -> () {
        (self.ob_update_class_ref)(obthis, group, class_id, refstyle, is_prim_parent)
    }
    pub unsafe fn ob_update_glob_class_instflag(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        is_instance: BOOL
    ) -> () {
        (self.ob_update_glob_class_instflag)(obthis, group, class_id, is_instance)
    }
    pub unsafe fn ob_is_class_member_accessable(
        &self,
        obthis: POB_THIS,
        member_access: OB_MEMBER_ACCESS,
        access_check_type: OB_MEMBER_ACCESS_TYPE,
        inheritance_level: UINT,
        in_system_routine: BOOL
    ) -> BOOL {
        (self.ob_is_class_member_accessable)(
            obthis,
            member_access,
            access_check_type,
            inheritance_level,
            in_system_routine
        )
    }
    pub unsafe fn ob_get_system_func_class(&self, obthis: POB_THIS) -> POB_RUNTIME_CLASS {
        (self.ob_get_system_func_class)(obthis)
    }
    pub unsafe fn ob_get_global_func_class(
        &self,
        obthis: POB_THIS,
        pGroup: POB_GROUP,
        classId: OB_CLASS_ID,
        module_id: OB_MODULE_ID
    ) -> POB_RUNTIME_CLASS {
        (self.ob_get_global_func_class)(obthis, pGroup, classId, module_id)
    }
    pub unsafe fn ob_type_declare(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        type_name: LPTSTR,
        type_kind: OB_TYPE_KIND,
        style: OB_CLASS_STYLE,
        parent_type: OB_CLASS_ID,
        nested_type: OB_CLASS_ID,
        autoinstantiate: BOOL,
        error: *mut INT
    ) -> OB_CLASS_ID {
        (self.ob_type_declare)(
            obthis,
            group,
            type_name,
            type_kind,
            style,
            parent_type,
            nested_type,
            autoinstantiate,
            error
        )
    }
    pub unsafe fn ob_type_declare_class(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        type_name: LPTSTR,
        type_kind: OB_TYPE_KIND,
        class_style: OB_CLASS_STYLE,
        parent_class: OB_CLASS_ID,
        nested_class: OB_CLASS_ID,
        autoinstantiate: BOOL,
        error: *mut INT
    ) -> OB_CLASS_ID {
        (self.ob_type_declare_class)(
            obthis,
            group,
            type_name,
            type_kind,
            class_style,
            parent_class,
            nested_class,
            autoinstantiate,
            error
        )
    }
    pub unsafe fn ob_type_declare_vtab(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        parent_class: OB_CLASS_ID,
        error: *mut INT
    ) -> () {
        (self.ob_type_declare_vtab)(obthis, group, class_id, parent_class, error)
    }
    pub unsafe fn ob_type_reference(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        type_name: LPTSTR
    ) -> OB_CLASS_ID {
        (self.ob_type_reference)(obthis, group, type_name)
    }
    pub unsafe fn ob_get_first_type(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: POB_CLASS_ID,
        style: POB_CLASS_STYLE
    ) -> LPTSTR {
        (self.ob_get_first_type)(obthis, group, class_id, style)
    }
    pub unsafe fn ob_get_next_type(
        &self,
        obthis: POB_THIS,
        class_id: POB_CLASS_ID,
        style: POB_CLASS_STYLE
    ) -> LPTSTR {
        (self.ob_get_next_type)(obthis, class_id, style)
    }
    pub unsafe fn ob_type_init_process(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        class_style: OB_CLASS_STYLE
    ) -> () {
        (self.ob_type_init_process)(obthis, group, class_id, class_style)
    }
    pub unsafe fn ob_type_decl_process(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> () {
        (self.ob_type_decl_process)(obthis, group, class_id)
    }
    pub unsafe fn ob_get_nested_class(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> OB_CLASS_ID {
        (self.ob_get_nested_class)(obthis, group, class_id)
    }
    pub unsafe fn ob_get_class_entry(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> POB_CLASS_ENTRY {
        (self.ob_get_class_entry)(obthis, group, class_id)
    }
    pub unsafe fn ob_is_class_indirect(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> BOOL {
        (self.ob_is_class_indirect)(obthis, group, class_id)
    }
    pub unsafe fn ob_fetch_routine(
        &self,
        class_entry: POB_CLASS_ENTRY,
        rout_id: OB_ROUT_ID,
        type_: *mut OB_ROUT_TYPE
    ) -> POB_ROUTNODE {
        (self.ob_fetch_routine)(class_entry, rout_id, type_)
    }
    pub unsafe fn ob_type_proto_decl(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        type_: OB_CLASS_ID,
        mod_id: OB_MODULE_ID,
        args: POB_PROTO_ARG,
        no_args: UINT,
        func_type: OB_FUNC_TYPE,
        dllname: LPTSTR,
        aliasname: LPTSTR,
        sys_func_id: OB_VTABLE_ID,
        proto_style: OB_FUNCPROTO_STYLE,
        member_access: OB_MEMBER_ACCESS,
        is_obsolete: BOOL,
        is_local_decl: BOOL,
        token_id: OB_EVT_TOKEN_ID,
        is_event_external: BOOL,
        throws_list: POB_CLASS_ID,
        no_throws: UINT,
        error: *mut INT
    ) -> OB_PROTO_ID {
        (self.ob_type_proto_decl)(
            obthis,
            group,
            class_id,
            name,
            rout_type,
            type_,
            mod_id,
            args,
            no_args,
            func_type,
            dllname,
            aliasname,
            sys_func_id,
            proto_style,
            member_access,
            is_obsolete,
            is_local_decl,
            token_id,
            is_event_external,
            throws_list,
            no_throws,
            error
        )
    }
    pub unsafe fn ob_type_proto_ref(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        funcname: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        access_type: OB_MEMBER_ACCESS_TYPE,
        funcargs: *mut POB_ACT_ARG,
        no_args: UINT,
        ret_type: POB_CLASS_ID,
        func_type: POB_FUNC_TYPE,
        dllname: *mut LPTSTR,
        proto_id: POB_PROTO_ID,
        vtable_id: POB_VTABLE_ID,
        error: POB_PROTOREF_ERROR,
        bound_exact_match: BOOL
    ) -> OB_MODULE_ID {
        (self.ob_type_proto_ref)(
            obthis,
            group,
            class_id,
            funcname,
            rout_type,
            access_type,
            funcargs,
            no_args,
            ret_type,
            func_type,
            dllname,
            proto_id,
            vtable_id,
            error,
            bound_exact_match
        )
    }
    pub unsafe fn ob_proto_error_upgrade(
        &self,
        obthis: POB_THIS,
        currerror: OB_PROTOREF_ERROR,
        newerror: OB_PROTOREF_ERROR
    ) -> OB_PROTOREF_ERROR {
        (self.ob_proto_error_upgrade)(obthis, currerror, newerror)
    }
    pub unsafe fn ob_get_proto_access_type(
        &self,
        obthis: POB_THIS,
        curr_group: POB_GROUP,
        curr_class_id: OB_CLASS_ID,
        formal_arg_group: POB_GROUP,
        formal_arg_class_id: OB_CLASS_ID
    ) -> OB_MEMBER_ACCESS_TYPE {
        (self.ob_get_proto_access_type)(
            obthis,
            curr_group,
            curr_class_id,
            formal_arg_group,
            formal_arg_class_id
        )
    }
    pub unsafe fn ob_type_process_protos(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> () {
        (self.ob_type_process_protos)(obthis, group, class_id)
    }
    pub unsafe fn ob_type_reprocess_protos(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        delete_proto_name: LPTSTR,
        delete_proto_rout_type: OB_ROUT_TYPE,
        delete_proto_args: POB_PROTO_ARG,
        delete_proto_no_args: UINT,
        filter_userprotos: BOOL
    ) -> INT {
        (self.ob_type_reprocess_protos)(
            obthis,
            group,
            class_id,
            delete_proto_name,
            delete_proto_rout_type,
            delete_proto_args,
            delete_proto_no_args,
            filter_userprotos
        )
    }
    pub unsafe fn ob_get_type_proto_names(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        rout_type: OB_ROUT_TYPE,
        include_ancestors: BOOL,
        local_protos_only: BOOL,
        nprotos: *mut UINT,
        error: *mut BOOL
    ) -> POB_PROTONAME {
        (self.ob_get_type_proto_names)(
            obthis,
            group,
            class_id,
            rout_type,
            include_ancestors,
            local_protos_only,
            nprotos,
            error
        )
    }
    pub unsafe fn ob_declare_external_event_type(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        parent_class: OB_CLASS_ID,
        error: *mut INT
    ) -> () {
        (self.ob_declare_external_event_type)(obthis, group, class_id, parent_class, error)
    }
    pub unsafe fn ob_get_type_proto_names_for_ide(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        rout_type: OB_ROUT_TYPE,
        include_ancestors: BOOL,
        local_protos_only: BOOL,
        nprotos: *mut UINT,
        error: *mut BOOL
    ) -> POB_PROTONAME {
        (self.ob_get_type_proto_names_for_ide)(
            obthis,
            group,
            class_id,
            rout_type,
            include_ancestors,
            local_protos_only,
            nprotos,
            error
        )
    }
    pub unsafe fn ob_type_vtable_module_srch(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        vtable_id: OB_VTABLE_ID
    ) -> OB_PROTO_ID {
        (self.ob_type_vtable_module_srch)(obthis, group, class_id, vtable_id)
    }
    pub unsafe fn ob_get_prototype(
        &self,
        obthis: POB_THIS,
        curr_group: *mut POB_GROUP,
        curr_class_id: POB_CLASS_ID,
        vtable_id: OB_VTABLE_ID
    ) -> POB_PROTOTYPE {
        (self.ob_get_prototype)(obthis, curr_group, curr_class_id, vtable_id)
    }
    pub unsafe fn ob_update_proto_mod_id(
        &self,
        obthis: POB_THIS,
        proto_id: OB_PROTO_ID,
        mod_id: OB_MODULE_ID
    ) -> () {
        (self.ob_update_proto_mod_id)(obthis, proto_id, mod_id)
    }
    pub unsafe fn ob_update_proto_rout_id(
        &self,
        obthis: POB_THIS,
        proto_id: OB_PROTO_ID,
        rout_id: OB_ROUT_ID
    ) -> () {
        (self.ob_update_proto_rout_id)(obthis, proto_id, rout_id)
    }
    pub unsafe fn ob_protolist_read(
        &self,
        obthis: POB_THIS,
        class_entry: POB_CLASS_ENTRY,
        subpool: OB_SUBPOOL
    ) -> INT {
        (self.ob_protolist_read)(obthis, class_entry, subpool)
    }
    pub unsafe fn ob_protolist_write(&self, obthis: POB_THIS, class_entry: POB_CLASS_ENTRY) -> OB_ERROR {
        (self.ob_protolist_write)(obthis, class_entry)
    }
    pub unsafe fn ob_prototype_match_for_event(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        proto: POB_PROTOTYPE,
        proto_group: POB_GROUP,
        result_type: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT
    ) -> BOOL {
        (self.ob_prototype_match_for_event)(obthis, group, proto, proto_group, result_type, args, no_args)
    }
    pub unsafe fn ob_prototype_search(
        &self,
        obthis: POB_THIS,
        proto_list: POB_PROTOTYPE,
        no_proto_list: UINT,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        proto_group: POB_GROUP,
        result_type: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT,
        error: POB_PROTO_OVERLOAD_ERROR
    ) -> OB_PROTO_ID {
        (self.ob_prototype_search)(
            obthis,
            proto_list,
            no_proto_list,
            group,
            class_id,
            name,
            rout_type,
            proto_group,
            result_type,
            args,
            no_args,
            error
        )
    }
    pub unsafe fn ob_proto_overload_search(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        type_: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT
    ) -> OB_PROTO_OVERLOAD_ERROR {
        (self.ob_proto_overload_search)(obthis, group, class_id, name, rout_type, type_, args, no_args)
    }
    pub unsafe fn ob_event_module_name(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        mod_id: OB_MODULE_ID
    ) -> LPTSTR {
        (self.ob_event_module_name)(obthis, group, class_entry, mod_id)
    }
    pub unsafe fn ob_find_first_event(
        &self,
        obthis: POB_THIS,
        class_hndl: POB_CLASS_HNDL,
        event_name: LPTSTR
    ) -> OB_VTABLE_ID {
        (self.ob_find_first_event)(obthis, class_hndl, event_name)
    }
    pub unsafe fn ob_type_event_script_srch(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        name: LPTSTR,
        error: *mut INT
    ) -> OB_MODULE_ID {
        (self.ob_type_event_script_srch)(obthis, group, class_id, name, error)
    }
    pub unsafe fn ob_build_proto_vtable(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY
    ) -> INT {
        (self.ob_build_proto_vtable)(obthis, group, class_entry)
    }
    pub unsafe fn ob_type_field_decl(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        target_class_id: OB_CLASS_ID,
        name: LPTSTR,
        info: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        class_id: OB_CLASS_ID,
        arrdef: POB_ARRAYDEF,
        dup_field_type: POB_FIELD_TYPE,
        error: *mut INT
    ) -> OB_SYM_ID {
        (self.ob_type_field_decl)(
            obthis,
            group,
            target_class_id,
            name,
            info,
            lookup_info,
            class_id,
            arrdef,
            dup_field_type,
            error
        )
    }
    pub unsafe fn ob_type_field_search(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        fieldtype: POB_CLASS_ID,
        actual_field_id: POB_SYM_ID
    ) -> OB_SYM_ID {
        (self.ob_type_field_search)(obthis, group, class_id, name, fieldtype, actual_field_id)
    }
    pub unsafe fn ob_type_field_ref(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        name: LPTSTR,
        curr_group: POB_GROUP,
        curr_class_id: OB_CLASS_ID,
        field_type: POB_CLASS_ID,
        grouping: POB_GROUPTYPE,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        rel_field_id: POB_SYM_ID,
        access_check_type: POB_MEMBER_ACCESS_TYPE,
        level: *mut UINT,
        error: *mut INT
    ) -> OB_SYM_ID {
        (self.ob_type_field_ref)(
            obthis,
            group,
            class_id,
            name,
            curr_group,
            curr_class_id,
            field_type,
            grouping,
            lookup_info,
            init_value,
            rel_field_id,
            access_check_type,
            level,
            error
        )
    }
    pub unsafe fn ob_get_type_field_info(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        nfields: *mut UINT,
        error: *mut INT,
        filter_fields: BOOL
    ) -> POB_TYPEINFO {
        (self.ob_get_type_field_info)(obthis, group, class_id, nfields, error, filter_fields)
    }
    pub unsafe fn ob_set_field_init_value(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        value: OB_CONST_REF
    ) -> () {
        (self.ob_set_field_init_value)(obthis, group, class_id, field_id, value)
    }
    pub unsafe fn ob_get_field_init_value(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID
    ) -> POB_DATA {
        (self.ob_get_field_init_value)(obthis, group, class_id, field_id)
    }
    pub unsafe fn ob_type_field_clear_instvars(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> () {
        (self.ob_type_field_clear_instvars)(obthis, group, class_id)
    }
    pub unsafe fn ob_convert_fields_to_const(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> INT {
        (self.ob_convert_fields_to_const)(obthis, group, class_id)
    }
    pub unsafe fn ob_build_instance_image(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> INT {
        (self.ob_build_instance_image)(obthis, group, class_id)
    }
    pub unsafe fn ob_field_decl_indattr_funcs(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        func_templates: POB_INDATTR_FUNCTMPLT,
        no_template_items: UINT
    ) -> () {
        (self.ob_field_decl_indattr_funcs)(
            obthis,
            group,
            class_id,
            field_id,
            func_templates,
            no_template_items
        )
    }
    pub unsafe fn ob_field_get_indattr_funcs(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        no_tmplts: *mut UINT
    ) -> POB_INDATTR_FUNCTMPLT {
        (self.ob_field_get_indattr_funcs)(obthis, group, class_id, field_id, no_tmplts)
    }
    pub unsafe fn ob_field_requires_update_notification(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID
    ) -> BOOL {
        (self.ob_field_requires_update_notification)(obthis, group, class_id, field_id)
    }
    pub unsafe fn ob_get_field_symtab(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> POB_LOOK_SYMTAB {
        (self.ob_get_field_symtab)(obthis, group, class_id)
    }
    pub unsafe fn ob_enum_entry_decl(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        has_val: BOOL,
        value: INT
    ) -> INT {
        (self.ob_enum_entry_decl)(obthis, group, class_id, name, has_val, value)
    }
    pub unsafe fn ob_enum_decl_process(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> () {
        (self.ob_enum_decl_process)(obthis, group, class_id)
    }
    pub unsafe fn ob_enum_reference(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        enumname: LPTSTR,
        enum_val: *mut INT,
        class_id: POB_CLASS_ID,
        group_id: POB_GROUP_ID
    ) -> INT {
        (self.ob_enum_reference)(obthis, group, enumname, enum_val, class_id, group_id)
    }
    pub unsafe fn ob_get_type_enum_info(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        nenums: *mut UINT
    ) -> POB_ENUM_INFO {
        (self.ob_get_type_enum_info)(obthis, group, class_id, nenums)
    }
    pub unsafe fn ob_is_type_enum(&self, obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> BOOL {
        (self.ob_is_type_enum)(obthis, group, class_id)
    }
    pub unsafe fn ob_type_indattr_search(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        no_tmplts: *mut UINT
    ) -> POB_INDATTR_FUNCTMPLT {
        (self.ob_type_indattr_search)(obthis, group, class_id, no_tmplts)
    }
    pub unsafe fn ob_type_decl_indattr_funcs(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        func_templates: POB_INDATTR_FUNCTMPLT,
        no_func_templates: UINT
    ) -> () {
        (self.ob_type_decl_indattr_funcs)(obthis, group, class_id, func_templates, no_func_templates)
    }
    pub unsafe fn ob_is_an_ancestor(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        of_group: POB_GROUP,
        of_class_id: OB_CLASS_ID,
        ret: *mut INT
    ) -> BOOL {
        (self.ob_is_an_ancestor)(obthis, group, class_id, of_group, of_class_id, ret)
    }
    pub unsafe fn ob_is_an_ancestor_excl(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        of_group: POB_GROUP,
        of_class_id: OB_CLASS_ID,
        ret: *mut INT
    ) -> BOOL {
        (self.ob_is_an_ancestor_excl)(obthis, group, class_id, of_group, of_class_id, ret)
    }
    pub unsafe fn ob_find_type_ancestor(
        &self,
        obthis: POB_THIS,
        group1: POB_GROUP,
        class_id1: OB_CLASS_ID,
        group2: POB_GROUP,
        class_id2: OB_CLASS_ID
    ) -> INT {
        (self.ob_find_type_ancestor)(obthis, group1, class_id1, group2, class_id2)
    }
    pub unsafe fn ob_find_common_ancestor(
        &self,
        obthis: POB_THIS,
        group1: POB_GROUP,
        class_id1: OB_CLASS_ID,
        group2: *mut POB_GROUP,
        class_id2: OB_CLASS_ID
    ) -> OB_CLASS_ID {
        (self.ob_find_common_ancestor)(obthis, group1, class_id1, group2, class_id2)
    }
    pub unsafe fn ob_get_ancestor_system_class(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> OB_CLASS_ID {
        (self.ob_get_ancestor_system_class)(obthis, group, class_id)
    }
    pub unsafe fn ob_get_runtime_class(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> POB_RUNTIME_CLASS {
        (self.ob_get_runtime_class)(obthis, group, class_id)
    }
    pub unsafe fn ob_get_func_vtable_entry(&self, obinst: OB_INST_ID, offset: ULONG) -> OB_FUNC_FUNC {
        (self.ob_get_func_vtable_entry)(obinst, offset)
    }
    pub unsafe fn ob_rout_declare(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        routname: LPTSTR,
        qual_routname: LPTSTR,
        rout_type: OB_ROUT_TYPE,
        func_type: OB_FUNC_TYPE,
        proto_id: OB_PROTO_ID,
        glob_id: OB_SYM_ID,
        rout_id: POB_ROUT_ID,
        subpool: OB_SUBPOOL,
        clear_routine: BOOL,
        error: *mut INT
    ) -> OB_MODULE_ID {
        (self.ob_rout_declare)(
            obthis,
            group,
            class_entry,
            routname,
            qual_routname,
            rout_type,
            func_type,
            proto_id,
            glob_id,
            rout_id,
            subpool,
            clear_routine,
            error
        )
    }
    pub unsafe fn ob_open_routine(
        &self,
        obthis: POB_THIS,
        class_entry: POB_CLASS_ENTRY,
        module_id: OB_MODULE_ID
    ) -> POB_ROUTNODE {
        (self.ob_open_routine)(obthis, class_entry, module_id)
    }
    pub unsafe fn ob_close_routine(&self, obthis: POB_THIS) -> () { (self.ob_close_routine)(obthis) }
    pub unsafe fn ob_func_indirect(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_entry: *mut POB_CLASS_ENTRY,
        mod_id: POB_MODULE_ID
    ) -> INT {
        (self.ob_func_indirect)(obthis, group, class_entry, mod_id)
    }
    pub unsafe fn ob_local_var_declare(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        error: *mut INT
    ) -> OB_SYM_ID {
        (self.ob_local_var_declare)(obthis, group, varname, varinfo, lookup_info, type_, error)
    }
    pub unsafe fn ob_local_array_declare(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        arrdef: POB_ARRAYDEF,
        error: *mut INT
    ) -> OB_SYM_ID {
        (self.ob_local_array_declare)(obthis, group, varname, varinfo, lookup_info, type_, arrdef, error)
    }
    pub unsafe fn ob_local_var_reference(
        &self,
        obthis: POB_THIS,
        varname: LPTSTR,
        type_: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> OB_SYM_ID {
        (self.ob_local_var_reference)(obthis, varname, type_, varinfo, lookup_info, init_value, array_def)
    }
    pub unsafe fn ob_local_set_var(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        var_id: OB_SYM_ID,
        value: OB_CONST_REF
    ) -> () {
        (self.ob_local_set_var)(obthis, group, var_id, value)
    }
    pub unsafe fn ob_local_set_id_var(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        var_id: OB_SYM_ID,
        value: UINT
    ) -> () {
        (self.ob_local_set_id_var)(obthis, group, var_id, value)
    }
    pub unsafe fn ob_set_const(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        value: *mut ::std::os::raw::c_void,
        item_type: OB_CONPOOL_ITEM_TYPE,
        nitems: UINT,
        len: ULONG
    ) -> OB_CONST_REF {
        (self.ob_set_const)(obthis, group, value, item_type, nitems, len)
    }
    pub unsafe fn ob_get_const(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        const_ref: OB_CONST_REF
    ) -> *mut ::std::os::raw::c_void {
        (self.ob_get_const)(obthis, group, const_ref)
    }
    pub unsafe fn ob_convert_vars_to_const(&self, obthis: POB_THIS, group: POB_GROUP) -> INT {
        (self.ob_convert_vars_to_const)(obthis, group)
    }
    pub unsafe fn ob_clear_group_objects(&self, obthis: POB_THIS, pGroup: POB_GROUP) -> BOOL {
        (self.ob_clear_group_objects)(obthis, pGroup)
    }
    pub unsafe fn ob_init_group_objects(&self, obthis: POB_THIS, pGroup: POB_GROUP) -> () {
        (self.ob_init_group_objects)(obthis, pGroup)
    }
    pub unsafe fn shformatDateTimeWeb(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: PSH_TIME,
        flags: ::std::os::raw::c_int,
        cultureInfo: LPMONTHANDDAYNAMESSTRUCT
    ) -> ::std::os::raw::c_long {
        (self.shformatDateTimeWeb)(pResult, maxLen, prMask, value, flags, cultureInfo)
    }
    pub unsafe fn shformatDateTime(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: PSH_TIME,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long {
        (self.shformatDateTime)(pResult, maxLen, prMask, value, flags)
    }
    pub unsafe fn shformatDecimal(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prFmt: LPTSTR,
        value: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long {
        (self.shformatDecimal)(pResult, maxLen, prFmt, value, flags)
    }
    pub unsafe fn shformatDecimalWeb(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prFmt: LPTSTR,
        value: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long {
        (self.shformatDecimalWeb)(pResult, maxLen, prFmt, value, flags, dwCultureFormat)
    }
    pub unsafe fn shformatDouble(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: f64,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long {
        (self.shformatDouble)(pResult, maxLen, prMask, value, flags)
    }
    pub unsafe fn shformatDoubleWeb(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: f64,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long {
        (self.shformatDoubleWeb)(pResult, maxLen, prMask, value, flags, dwCultureFormat)
    }
    pub unsafe fn shformatLonglong(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: ::std::os::raw::c_longlong,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long {
        (self.shformatLonglong)(pResult, maxLen, prMask, value, flags)
    }
    pub unsafe fn shformatLonglongWeb(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: ::std::os::raw::c_longlong,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long {
        (self.shformatLonglongWeb)(pResult, maxLen, prMask, value, flags, dwCultureFormat)
    }
    pub unsafe fn shformatReal(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        pValue: *mut f32,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long {
        (self.shformatReal)(pResult, maxLen, prMask, pValue, flags)
    }
    pub unsafe fn shformatRealWeb(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        pValue: *mut f32,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long {
        (self.shformatRealWeb)(pResult, maxLen, prMask, pValue, flags, dwCultureFormat)
    }
    pub unsafe fn shformatString(
        &self,
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: LPTSTR,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long {
        (self.shformatString)(pResult, maxLen, prMask, value, flags)
    }
    pub unsafe fn shformatCmplDateTimeMask(
        &self,
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int {
        (self.shformatCmplDateTimeMask)(prMask, psMask, maxLen)
    }
    pub unsafe fn shformatCmplDateTimeMaskWeb(
        &self,
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int {
        (self.shformatCmplDateTimeMaskWeb)(prMask, psMask, maxLen, dwCultureFormat)
    }
    pub unsafe fn shformatCmplNumericMask(
        &self,
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int {
        (self.shformatCmplNumericMask)(prMask, psMask, maxLen)
    }
    pub unsafe fn shformatCmplNumericMaskWeb(
        &self,
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int {
        (self.shformatCmplNumericMaskWeb)(prMask, psMask, maxLen, dwCultureFormat)
    }
    pub unsafe fn shformatCmplNumericMaskWebCommasPos(
        &self,
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int {
        (self.shformatCmplNumericMaskWebCommasPos)(prMask, psMask, maxLen, dwCultureFormat)
    }
    pub unsafe fn shformatCmplStringMask(
        &self,
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int {
        (self.shformatCmplStringMask)(prMask, psMask, maxLen)
    }
    pub unsafe fn shformatErrorString(&self, errMsg: LPTSTR, err: ::std::os::raw::c_int) -> () {
        (self.shformatErrorString)(errMsg, err)
    }
    pub unsafe fn ob_add_glbsym_var(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        class_id: OB_CLASS_ID,
        id: OB_SYM_ID
    ) -> OB_SYM_ID {
        (self.ob_add_glbsym_var)(obthis, group, name, reftype, class_id, id)
    }
    pub unsafe fn ob_add_glbsym_class(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        refstyle: OB_GLOB_REFSTYLE,
        group_id: OB_GROUP_ID,
        class_id: OB_CLASS_ID,
        sys_class_id: OB_CLASS_ID
    ) -> OB_SYM_ID {
        (self.ob_add_glbsym_class)(obthis, group, name, reftype, refstyle, group_id, class_id, sys_class_id)
    }
    pub unsafe fn ob_add_glbsym_func(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        class_id: OB_CLASS_ID,
        mod_id: OB_MODULE_ID
    ) -> OB_SYM_ID {
        (self.ob_add_glbsym_func)(obthis, group, name, reftype, class_id, mod_id)
    }
    pub unsafe fn rt_set_class_handle(
        &self,
        rtthis: POB_THIS,
        appclasshndl: OB_CLASS_HNDL,
        appinst: OB_INST_ID
    ) -> () {
        (self.rt_set_class_handle)(rtthis, appclasshndl, appinst)
    }
    pub unsafe fn rt_init(&self, obthis: POB_THIS, stgthis: ppbstg_anchor) -> POB_THIS {
        (self.rt_init)(obthis, stgthis)
    }
    pub unsafe fn rt_start_debug(
        &self,
        rtthis: POB_THIS,
        rtBreakCallback: *mut RT_BREAK_PROC,
        pUserData: *mut ::std::os::raw::c_void
    ) -> INT {
        (self.rt_start_debug)(rtthis, rtBreakCallback, pUserData)
    }
    pub unsafe fn rt_stop_debug(&self, rtthis: POB_THIS) -> INT { (self.rt_stop_debug)(rtthis) }
    pub unsafe fn rt_set_pcode_to_line(&self, obthis: POB_THIS, line_no: UINT) -> INT {
        (self.rt_set_pcode_to_line)(obthis, line_no)
    }
    pub unsafe fn rt_breakpoint(
        &self,
        rtthis: POB_THIS,
        bSet: BOOL,
        obClassHndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        iLineNumber: UINT,
        n_times: UINT,
        condition: LPTSTR,
        id: ::std::os::raw::c_long
    ) -> PRT_BREAKPOINT {
        (self.rt_breakpoint)(rtthis, bSet, obClassHndl, vtable_id, iLineNumber, n_times, condition, id)
    }
    pub unsafe fn rt_create_watchpoint(
        &self,
        rtthis: POB_THIS,
        pdata_info: POB_DATA_INFO,
        watch_type: WATCHPOINT_TYPE,
        item_scope: ::std::os::raw::c_uchar,
        id: ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void {
        (self.rt_create_watchpoint)(rtthis, pdata_info, watch_type, item_scope, id)
    }
    pub unsafe fn rt_find_watchpoint_for_watchid(
        &self,
        rtthis: POB_THIS,
        watchId: ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void {
        (self.rt_find_watchpoint_for_watchid)(rtthis, watchId)
    }
    pub unsafe fn rt_delete_watchpoint(&self, rtthis: POB_THIS, watchpt: *mut ::std::os::raw::c_void) -> () {
        (self.rt_delete_watchpoint)(rtthis, watchpt)
    }
    pub unsafe fn rt_is_line_executable(
        &self,
        rtthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        line_no: UINT
    ) -> BOOL {
        (self.rt_is_line_executable)(rtthis, class_hndl, vtable_id, line_no)
    }
    pub unsafe fn rt_closest_executable_line(
        &self,
        rtthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        line_no: UINT
    ) -> UINT {
        (self.rt_closest_executable_line)(rtthis, class_hndl, vtable_id, line_no)
    }
    pub unsafe fn rt_start_run(&self, rtthis: POB_THIS) -> INT { (self.rt_start_run)(rtthis) }
    pub unsafe fn rt_stop_run(&self, rtthis: POB_THIS) -> INT { (self.rt_stop_run)(rtthis) }
    pub unsafe fn rt_create_obinst(&self, rtthis: POB_THIS, name: LPTSTR, obinst: POB_INST_ID) -> INT {
        (self.rt_create_obinst)(rtthis, name, obinst)
    }
    pub unsafe fn rtReturnValGet(&self, rtThis: POB_THIS) -> POB_DATA { (self.rtReturnValGet)(rtThis) }
    pub unsafe fn rtReturnValFree(&self, rtThis: POB_THIS) -> () { (self.rtReturnValFree)(rtThis) }
    pub unsafe fn rt_error(&self, rtthis: POB_THIS, iMessageID: INT) -> INT {
        (self.rt_error)(rtthis, iMessageID)
    }
    pub unsafe fn rt_free_error_struct(&self, rtthis: POB_THIS, error_struct: PRT_ERROR_STRUCT) -> () {
        (self.rt_free_error_struct)(rtthis, error_struct)
    }
    pub unsafe fn rt_error_using_struct(
        &self,
        rtthis: POB_THIS,
        error_struct: PRT_ERROR_STRUCT,
        exceptionClassName: LPTSTR
    ) -> INT {
        (self.rt_error_using_struct)(rtthis, error_struct, exceptionClassName)
    }
    pub unsafe fn rt_normalize_error_id(&self, obthis: POB_THIS, iMessageID: INT) -> INT {
        (self.rt_normalize_error_id)(obthis, iMessageID)
    }
    pub unsafe fn ot_handle_exception(
        &self,
        rtthis: POB_THIS,
        pException_Stack: *mut ::std::os::raw::c_void,
        currDepth: USHORT
    ) -> INT {
        (self.ot_handle_exception)(rtthis, pException_Stack, currDepth)
    }
    pub unsafe fn ob_dbg_pop_call_stack_ntimes(&self, obthis: POB_THIS, n: UINT) -> INT {
        (self.ob_dbg_pop_call_stack_ntimes)(obthis, n)
    }
    pub unsafe fn ob_dbg_push_call_stack_ntimes(&self, obthis: POB_THIS, n: UINT) -> INT {
        (self.ob_dbg_push_call_stack_ntimes)(obthis, n)
    }
    pub unsafe fn ob_get_current_stack_location(&self, obthis: POB_THIS) -> PRT_BREAKPOINT {
        (self.ob_get_current_stack_location)(obthis)
    }
    pub unsafe fn rtRoutineSearch(
        &self,
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pchRoutineName: LPTSTR,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineType: OB_ROUT_TYPE,
        pobRoutineId: POB_VTABLE_ID
    ) -> INT {
        (self.rtRoutineSearch)(
            obThis,
            rtCallInfo,
            pchRoutineName,
            pobdArgArray,
            uiNoArgs,
            obRoutineType,
            pobRoutineId
        )
    }
    pub unsafe fn rtRoutineExec(
        &self,
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineId: OB_VTABLE_ID,
        obRoutineType: OB_ROUT_TYPE,
        bConvert: BOOL
    ) -> RT_EXEC_STATUS {
        (self.rtRoutineExec)(obThis, rtCallInfo, pobdArgArray, uiNoArgs, obRoutineId, obRoutineType, bConvert)
    }
    pub unsafe fn rtRoutineExecByName(
        &self,
        obThis: POB_THIS,
        pchRoutineName: LPTSTR,
        rtCallInfo: RT_CALL_INFO,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineType: OB_ROUT_TYPE,
        bConvert: BOOL
    ) -> RT_EXEC_STATUS {
        (self.rtRoutineExecByName)(
            obThis,
            pchRoutineName,
            rtCallInfo,
            pobdArgArray,
            uiNoArgs,
            obRoutineType,
            bConvert
        )
    }
    pub unsafe fn rtRoutineExecPosted(&self, pData: *mut ::std::os::raw::c_void) -> RT_EXEC_STATUS {
        (self.rtRoutineExecPosted)(pData)
    }
    pub unsafe fn rtRoutineInfo(
        &self,
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        obRoutineId: OB_VTABLE_ID,
        pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO
    ) -> INT {
        (self.rtRoutineInfo)(obThis, rtCallInfo, obRoutineId, pRoutineProtoInfo)
    }
    pub unsafe fn rtInitializeInfoForCall(
        &self,
        obThis: POB_THIS,
        pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO
    ) -> INT {
        (self.rtInitializeInfoForCall)(obThis, pRoutineProtoInfo)
    }
    pub unsafe fn rtCleanupInfoAfterCall(
        &self,
        obThis: POB_THIS,
        pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO
    ) -> INT {
        (self.rtCleanupInfoAfterCall)(obThis, pRoutineProtoInfo)
    }
    pub unsafe fn rtRoutineCount(
        &self,
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pusRoutineTotal: *mut USHORT,
        pusFuncTotal: *mut USHORT,
        pusEventTotal: *mut USHORT
    ) -> INT {
        (self.rtRoutineCount)(obThis, rtCallInfo, pusRoutineTotal, pusFuncTotal, pusEventTotal)
    }
    pub unsafe fn rtReferenceArgCreate(
        &self,
        obThis: POB_THIS,
        pobdRefArg: POB_DATA,
        prtRefArgInfo: PRT_REFARG_INFO
    ) -> INT {
        (self.rtReferenceArgCreate)(obThis, pobdRefArg, prtRefArgInfo)
    }
    pub unsafe fn rtReferenceArgFree(&self, obThis: POB_THIS, pobdRefArg: POB_DATA) -> INT {
        (self.rtReferenceArgFree)(obThis, pobdRefArg)
    }
    pub unsafe fn rtGetClassDescrip(
        &self,
        obThis: POB_THIS,
        obClassHndl: OB_CLASS_HNDL,
        prtClassDescrip: PRT_CLASS_DESCRIP,
        pobClassIdSystem: POB_CLASS_ID
    ) -> INT {
        (self.rtGetClassDescrip)(obThis, obClassHndl, prtClassDescrip, pobClassIdSystem)
    }
    pub unsafe fn rtDataFree(&self, pobThis: POB_THIS, pobdVal: POB_DATA) -> () {
        (self.rtDataFree)(pobThis, pobdVal)
    }
    pub unsafe fn rtDataCopy(
        &self,
        pobThis: POB_THIS,
        pobdDest: POB_DATA,
        pobdSrc: POB_DATA,
        AddReference: BOOL
    ) -> () {
        (self.rtDataCopy)(pobThis, pobdDest, pobdSrc, AddReference)
    }
    pub unsafe fn rt_hit_level_0(&self, obthis: POB_THIS) -> () { (self.rt_hit_level_0)(obthis) }
    pub unsafe fn ob_create_object(
        &self,
        obthis: POB_THIS,
        destination: POB_DATA,
        p_group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> INT {
        (self.ob_create_object)(obthis, destination, p_group, class_id)
    }
    pub unsafe fn ob_create_object_using(
        &self,
        obthis: POB_THIS,
        destination: POB_DATA,
        context: POB_RUNTIME_INST,
        class_name: LPTSTR
    ) -> HRESULT {
        (self.ob_create_object_using)(obthis, destination, context, class_name)
    }
    pub unsafe fn ob_copy_rtinst(&self, obthis: POB_THIS, from_rtinst: POB_RUNTIME_INST) -> POB_RUNTIME_INST {
        (self.ob_copy_rtinst)(obthis, from_rtinst)
    }
    pub unsafe fn ob_destroy_rtinst(&self, obthis: POB_THIS, rtinst: POB_RUNTIME_INST) -> INT {
        (self.ob_destroy_rtinst)(obthis, rtinst)
    }
    pub unsafe fn ob_get_primary_rtinst(
        &self,
        obthis: POB_THIS,
        rtinst: POB_RUNTIME_INST
    ) -> POB_RUNTIME_INST {
        (self.ob_get_primary_rtinst)(obthis, rtinst)
    }
    pub unsafe fn ob_is_rtinst_autoinstantiate(&self, obthis: POB_THIS, rtinst: POB_RUNTIME_INST) -> BOOL {
        (self.ob_is_rtinst_autoinstantiate)(obthis, rtinst)
    }
    pub unsafe fn ob_object_compare(
        &self,
        obthis: POB_THIS,
        rtinst1: POB_RUNTIME_INST,
        rtinst2: POB_RUNTIME_INST
    ) -> BOOL {
        (self.ob_object_compare)(obthis, rtinst1, rtinst2)
    }
    pub unsafe fn ob_invoke_static(
        &self,
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        numArgs: USHORT,
        args: POB_DATA,
        result: POB_DATA
    ) -> HRESULT {
        (self.ob_invoke_static)(rtinst, context, vtableId, numArgs, args, result)
    }
    pub unsafe fn ob_invoke_dynamic(
        &self,
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        routType: OB_ROUT_TYPE,
        name: LPTSTR,
        numArgs: USHORT,
        args: POB_DATA,
        result: POB_DATA
    ) -> HRESULT {
        (self.ob_invoke_dynamic)(rtinst, context, routType, name, numArgs, args, result)
    }
    pub unsafe fn ob_invoke_staticAsync(
        &self,
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        numArgs: USHORT,
        args: POB_DATA
    ) -> HRESULT {
        (self.ob_invoke_staticAsync)(rtinst, context, vtableId, numArgs, args)
    }
    pub unsafe fn ob_invoke_dynamicAsync(
        &self,
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        routType: OB_ROUT_TYPE,
        name: LPTSTR,
        numArgs: USHORT,
        args: POB_DATA
    ) -> HRESULT {
        (self.ob_invoke_dynamicAsync)(rtinst, context, routType, name, numArgs, args)
    }
    pub unsafe fn ob_instance_lv(
        &self,
        obthis: POB_THIS,
        current_inst: POB_RUNTIME_INST,
        var_id: OB_SYM_ID
    ) -> POB_DATA {
        (self.ob_instance_lv)(obthis, current_inst, var_id)
    }
    pub unsafe fn ob_instance_fldupdate_refpkt(
        &self,
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        var_id: OB_SYM_ID
    ) -> () {
        (self.ob_instance_fldupdate_refpkt)(obthis, destination, current_inst, var_id)
    }
    pub unsafe fn ob_instance_flditemupdate_refpkt(
        &self,
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        group_id: OB_GROUP_ID,
        var_id: OB_SYM_ID,
        lvalue: POB_DATA,
        item_index: ULONG
    ) -> () {
        (self.ob_instance_flditemupdate_refpkt)(
            obthis,
            destination,
            current_inst,
            group_id,
            var_id,
            lvalue,
            item_index
        )
    }
    pub unsafe fn ob_instance_simple_refpkt(
        &self,
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        var_id: OB_SYM_ID
    ) -> () {
        (self.ob_instance_simple_refpkt)(obthis, destination, current_inst, var_id)
    }
    pub unsafe fn ob_get_group_load_state(
        &self,
        pGroupReference: *mut ::std::os::raw::c_void
    ) -> OB_GROUP_LOAD_STATE {
        (self.ob_get_group_load_state)(pGroupReference)
    }
    pub unsafe fn ob_get_groupref_group(&self, pGroupReference: *mut ::std::os::raw::c_void) -> POB_GROUP {
        (self.ob_get_groupref_group)(pGroupReference)
    }
    pub unsafe fn ob_group_get_next_index(&self, obthis: POB_THIS) -> ULONG {
        (self.ob_group_get_next_index)(obthis)
    }
    pub unsafe fn ob_close_typedef_group(&self, obThis: POB_THIS) -> () {
        (self.ob_close_typedef_group)(obThis)
    }
    pub unsafe fn ob_create_group_structure(&self, obThis: POB_THIS, lpszGroupName: LPTSTR) -> POB_GROUP {
        (self.ob_create_group_structure)(obThis, lpszGroupName)
    }
    pub unsafe fn ob_new_group(
        &self,
        obthis: POB_THIS,
        lib_name: LPTSTR,
        qual_group_name: LPTSTR,
        group_lock_state: OB_GROUP_LOCK_STATE,
        group_load_state: OB_GROUP_LOAD_STATE
    ) -> POB_GROUP {
        (self.ob_new_group)(obthis, lib_name, qual_group_name, group_lock_state, group_load_state)
    }
    pub unsafe fn ob_del_group_structure(&self, obThis: POB_THIS, pGroup: POB_GROUP) -> () {
        (self.ob_del_group_structure)(obThis, pGroup)
    }
    pub unsafe fn ob_group_data_srch(&self, obThis: POB_THIS, obGroupHandle: OB_GROUP_HNDL) -> POB_GROUP {
        (self.ob_group_data_srch)(obThis, obGroupHandle)
    }
    pub unsafe fn ob_replace_group(
        &self,
        obThis: POB_THIS,
        obGroupID: OB_GROUP_ID,
        pNewGroup: POB_GROUP
    ) -> () {
        (self.ob_replace_group)(obThis, obGroupID, pNewGroup)
    }
    pub unsafe fn ob_copy_group_shrsym_data(&self, obthis: POB_THIS, group: POB_GROUP) -> () {
        (self.ob_copy_group_shrsym_data)(obthis, group)
    }
    pub unsafe fn ob_get_qualified_name_with_namespace(
        &self,
        obThis: POB_THIS,
        pGroup: POB_GROUP,
        lpszNamespace: LPTSTR
    ) -> LPTSTR {
        (self.ob_get_qualified_name_with_namespace)(obThis, pGroup, lpszNamespace)
    }
    pub unsafe fn ob_get_source_from_group(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        src_type: *mut POB_SOURCE_BLK_TYPE,
        no_blocks: *mut UINT,
        subpool: OB_SUBPOOL,
        ppSrcLastEdit: *mut POB_SRC_LAST_EDIT,
        pNoSrcLastEdit: *mut UINT
    ) -> *mut LPTSTR {
        (self.ob_get_source_from_group)(
            obthis,
            group,
            src_type,
            no_blocks,
            subpool,
            ppSrcLastEdit,
            pNoSrcLastEdit
        )
    }
    pub unsafe fn ob_get_var(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        look_symtab: POB_LOOK_SYMTAB,
        var_id: OB_SYM_ID
    ) -> POB_DATA {
        (self.ob_get_var)(obthis, group, look_symtab, var_id)
    }
    pub unsafe fn ob_init_var_data(&self, obthis: POB_THIS, var_data: POB_DATA, group: POB_GROUP) -> () {
        (self.ob_init_var_data)(obthis, var_data, group)
    }
    pub unsafe fn ob_global_indirect(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        glob_id: POB_SYM_ID
    ) -> POB_DATA {
        (self.ob_global_indirect)(obthis, group, glob_id)
    }
    pub unsafe fn ob_global_var_declare(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        error: *mut INT
    ) -> OB_SYM_ID {
        (self.ob_global_var_declare)(obthis, group, varname, varinfo, lookup_info, type_, error)
    }
    pub unsafe fn ob_global_array_declare(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        var: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        arrdef: POB_ARRAYDEF,
        error: *mut INT
    ) -> OB_SYM_ID {
        (self.ob_global_array_declare)(obthis, group, var, varinfo, lookup_info, type_, arrdef, error)
    }
    pub unsafe fn ob_shared_var_reference(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        type_: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> OB_SYM_ID {
        (self.ob_shared_var_reference)(
            obthis,
            group,
            varname,
            type_,
            varinfo,
            lookup_info,
            init_value,
            array_def
        )
    }
    pub unsafe fn ob_global_set_var(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        var_id: OB_SYM_ID,
        value: OB_CONST_REF
    ) -> () {
        (self.ob_global_set_var)(obthis, group, var_id, value)
    }
    pub unsafe fn ob_global_set_id_var(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        var_id: OB_SYM_ID,
        value: UINT
    ) -> () {
        (self.ob_global_set_id_var)(obthis, group, var_id, value)
    }
    pub unsafe fn ob_get_local_symtab(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        var_id: POB_SYM_ID
    ) -> POB_LOOK_SYMTAB {
        (self.ob_get_local_symtab)(obthis, group, var_id)
    }
    pub unsafe fn ob_get_unconverted_var(
        &self,
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        var: OB_SYM_ID,
        level: UINT
    ) -> POB_DATA {
        (self.ob_get_unconverted_var)(obthis, group, var, level)
    }
    pub unsafe fn ob_lookup_shared_var_info(
        &self,
        obThis: POB_THIS,
        iGroupID: OB_GROUP_ID,
        iSymbolID: OB_SYM_ID,
        pType: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> INT {
        (self.ob_lookup_shared_var_info)(
            obThis,
            iGroupID,
            iSymbolID,
            pType,
            varinfo,
            lookup_info,
            init_value,
            array_def
        )
    }
    pub unsafe fn ob_clear_shared_vars(&self, obthis: POB_THIS, group: POB_GROUP, level: INT) -> () {
        (self.ob_clear_shared_vars)(obthis, group, level)
    }
    pub unsafe fn ot_eval_expr(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        pcode_blk: POB_PCODE_BLK,
        expr_result_buf: POB_DATA
    ) -> POB_DATA {
        (self.ot_eval_expr)(obthis, group, class_entry, pcode_blk, expr_result_buf)
    }
    pub unsafe fn ot_dbg_funccall(
        &self,
        obthis: POB_THIS,
        call_label: LPTSTR,
        group: POB_GROUP,
        class_entry: OB_CLASS_ID,
        name: LPTSTR
    ) -> () {
        (self.ot_dbg_funccall)(obthis, call_label, group, class_entry, name)
    }
    pub unsafe fn ot_run_dllfunccall(
        &self,
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        funcname: LPTSTR,
        evaled_arglist: POB_DATA,
        no_args: UINT,
        funcproto: POB_PROTOTYPE
    ) -> INT {
        (self.ot_run_dllfunccall)(obthis, group, class_id, funcname, evaled_arglist, no_args, funcproto)
    }
    pub unsafe fn ot_run_rpcfunccall(
        &self,
        obthis: POB_THIS,
        rtinst: POB_RUNTIME_INST,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        funcname: LPTSTR,
        evaled_arglist: POB_DATA,
        no_args: UINT,
        funcproto: POB_PROTOTYPE,
        rpc_funcname: LPTSTR
    ) -> INT {
        (self.ot_run_rpcfunccall)(
            obthis,
            rtinst,
            group,
            class_id,
            funcname,
            evaled_arglist,
            no_args,
            funcproto,
            rpc_funcname
        )
    }
    pub unsafe fn ot_get_dll_funcptr_by_name(
        &self,
        obthis: POB_THIS,
        dllname: LPTSTR,
        funcname: LPTSTR
    ) -> OS_CALLC_FUNC {
        (self.ot_get_dll_funcptr_by_name)(obthis, dllname, funcname)
    }
    pub unsafe fn ot_post_call(
        &self,
        obthis: POB_THIS,
        pRuntimeClass: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        uiNoArgs: UINT,
        args: POB_DATA
    ) -> INT {
        (self.ot_post_call)(obthis, pRuntimeClass, vtableId, uiNoArgs, args)
    }
    pub unsafe fn ot_check_types(
        &self,
        obthis: POB_THIS,
        group1: POB_GROUP,
        type1: OB_CLASS_ID,
        grouping1: OB_GROUPTYPE,
        group2: POB_GROUP,
        type2: OB_CLASS_ID,
        grouping2: OB_GROUPTYPE,
        ancestor_flag: *mut UINT
    ) -> OT_TYPE_CHECK_ERROR {
        (self.ot_check_types)(obthis, group1, type1, grouping1, group2, type2, grouping2, ancestor_flag)
    }
    pub unsafe fn ot_type_loc(&self, obthis: POB_THIS, data_node: POB_DATA) -> OT_TYPE_LOC {
        (self.ot_type_loc)(obthis, data_node)
    }
    pub unsafe fn ot_init_data_node(
        &self,
        obthis: POB_THIS,
        data_node: POB_DATA,
        type_: OB_CLASS_ID,
        varinfo: OB_INFO_FLAGS
    ) -> () {
        (self.ot_init_data_node)(obthis, data_node, type_, varinfo)
    }
    pub unsafe fn ot_set_lvalue(
        &self,
        obthis: POB_THIS,
        group_id: OB_GROUP_ID,
        lvalue_data: POB_DATA,
        rvalue_data: POB_DATA,
        do_error_check: BOOL
    ) -> INT {
        (self.ot_set_lvalue)(obthis, group_id, lvalue_data, rvalue_data, do_error_check)
    }
    pub unsafe fn ot_free_out_node(&self, obthis: POB_THIS, data_node: POB_DATA) -> () {
        (self.ot_free_out_node)(obthis, data_node)
    }
    pub unsafe fn ot_free_inv_meth_args(
        &self,
        obthis: POB_THIS,
        pArrayDataNode: POB_DATA,
        pFreeFlags: LPTSTR
    ) -> INT {
        (self.ot_free_inv_meth_args)(obthis, pArrayDataNode, pFreeFlags)
    }
    pub unsafe fn ot_copy_array(&self, obthis: POB_THIS, old_array_inst: POB_ARRAY_INST) -> POB_ARRAY_INST {
        (self.ot_copy_array)(obthis, old_array_inst)
    }
    pub unsafe fn ot_get_string_from_chararray(&self, obthis: POB_THIS, arrayinst: POB_ARRAY_INST) -> LPTSTR {
        (self.ot_get_string_from_chararray)(obthis, arrayinst)
    }
    pub unsafe fn ot_create_chararray_from_string(
        &self,
        obthis: POB_THIS,
        string_val: LPTSTR,
        target_data_node: POB_DATA
    ) -> POB_DATA {
        (self.ot_create_chararray_from_string)(obthis, string_val, target_data_node)
    }
    pub unsafe fn ot_create_bounded_chararray_from_string(
        &self,
        obthis: POB_THIS,
        string_val: LPTSTR,
        bounds: *mut ::std::os::raw::c_long,
        target_data_node: POB_DATA
    ) -> POB_DATA {
        (self.ot_create_bounded_chararray_from_string)(obthis, string_val, bounds, target_data_node)
    }
    pub unsafe fn ot_get_char_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> TCHAR {
        (self.ot_get_char_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_string_value(&self, obthis: POB_THIS, data_node: POB_DATA) -> LPTSTR {
        (self.ot_get_string_value)(obthis, data_node)
    }
    pub unsafe fn ot_get_string_from_char(&self, obthis: POB_THIS, char_val: TCHAR) -> LPTSTR {
        (self.ot_get_string_from_char)(obthis, char_val)
    }
    pub unsafe fn ot_string_cat(&self, obthis: POB_THIS, string1: LPTSTR, string2: LPTSTR) -> LPTSTR {
        (self.ot_string_cat)(obthis, string1, string2)
    }
    pub unsafe fn ot_binary_cat(&self, obthis: POB_THIS, bin1: PSH_BINARY, bin2: PSH_BINARY) -> PSH_BINARY {
        (self.ot_binary_cat)(obthis, bin1, bin2)
    }
    pub unsafe fn ot_halt(&self, obthis: POB_THIS, send_close_event: BOOL) -> INT {
        (self.ot_halt)(obthis, send_close_event)
    }
    pub unsafe fn ot_convert_bounded_to_bounded(
        &self,
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long,
        free_old_array: BOOL
    ) -> POB_ARRAY_INST {
        (self.ot_convert_bounded_to_bounded)(
            obthis,
            old_array_inst,
            new_class_id,
            new_nitems,
            new_ndims,
            bounds,
            free_old_array
        )
    }
    pub unsafe fn ot_convert_bounded_to_unbounded(
        &self,
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST {
        (self.ot_convert_bounded_to_unbounded)(obthis, old_array_inst, new_class_id)
    }
    pub unsafe fn ot_convert_unbounded_to_bounded(
        &self,
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long
    ) -> POB_ARRAY_INST {
        (self.ot_convert_unbounded_to_bounded)(
            obthis,
            old_array_inst,
            new_class_id,
            new_nitems,
            new_ndims,
            bounds
        )
    }
    pub unsafe fn ot_convert_unbounded_to_unbounded(
        &self,
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST {
        (self.ot_convert_unbounded_to_unbounded)(obthis, old_array_inst, new_class_id)
    }
    pub unsafe fn ot_convert_any_to_bounded(
        &self,
        obthis: POB_THIS,
        any_node: POB_DATA,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long
    ) -> POB_ARRAY_INST {
        (self.ot_convert_any_to_bounded)(obthis, any_node, new_class_id, new_nitems, new_ndims, bounds)
    }
    pub unsafe fn ot_convert_any_to_unbounded(
        &self,
        obthis: POB_THIS,
        any_node: POB_DATA,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST {
        (self.ot_convert_any_to_unbounded)(obthis, any_node, new_class_id)
    }
    pub unsafe fn ot_convert_array_to_object(
        &self,
        obthis: POB_THIS,
        any_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_RUNTIME_INST {
        (self.ot_convert_array_to_object)(obthis, any_array_inst, new_class_id)
    }
    pub unsafe fn ot_build_simple_refpak(
        &self,
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        group_id: OB_GROUP_ID
    ) -> POT_REF_PAK {
        (self.ot_build_simple_refpak)(obthis, lvalue_data, group_id)
    }
    pub unsafe fn ot_build_field_refpak(
        &self,
        obthis: POB_THIS,
        group_id: OB_GROUP_ID,
        rtinst: POB_RUNTIME_INST,
        field_id: UINT,
        item_index: ULONG,
        bTriggerFieldUpdate: BOOL
    ) -> POT_REF_PAK {
        (self.ot_build_field_refpak)(obthis, group_id, rtinst, field_id, item_index, bTriggerFieldUpdate)
    }
    pub unsafe fn ot_add_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_add_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_sub_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_sub_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_mul_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_mul_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_div_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_div_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_pow_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_pow_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_neg_any(&self, prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT {
        (self.ot_neg_any)(prtThis, pResult, pAny1)
    }
    pub unsafe fn ot_eq_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_eq_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_ne_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_ne_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_gt_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_gt_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_lt_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_lt_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_ge_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_ge_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_le_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_le_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_and_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_and_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_or_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_or_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_not_any(&self, prtThis: POB_THIS, pResult: POB_DATA, pAny: POB_DATA) -> INT {
        (self.ot_not_any)(prtThis, pResult, pAny)
    }
    pub unsafe fn ot_incr_any(&self, prtThis: POB_THIS, pAny: POB_DATA) -> INT {
        (self.ot_incr_any)(prtThis, pAny)
    }
    pub unsafe fn ot_decr_any(&self, prtThis: POB_THIS, pAny: POB_DATA) -> INT {
        (self.ot_decr_any)(prtThis, pAny)
    }
    pub unsafe fn ot_mod_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_mod_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_min_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_min_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_max_any(
        &self,
        prtThis: POB_THIS,
        pResult: POB_DATA,
        pAny1: POB_DATA,
        pAny2: POB_DATA
    ) -> INT {
        (self.ot_max_any)(prtThis, pResult, pAny1, pAny2)
    }
    pub unsafe fn ot_check_any_exact_type(
        &self,
        obthis: POB_THIS,
        any_var: POB_DATA,
        expected_type: OB_CLASS_ID
    ) -> INT {
        (self.ot_check_any_exact_type)(obthis, any_var, expected_type)
    }
    pub unsafe fn ot_check_any_string_type(
        &self,
        obthis: POB_THIS,
        any_var: POB_DATA,
        expected_type: OB_CLASS_ID
    ) -> INT {
        (self.ot_check_any_string_type)(obthis, any_var, expected_type)
    }
    pub unsafe fn ot_check_any_binary_type(
        &self,
        obthis: POB_THIS,
        any_var: POB_DATA,
        expected_type: OB_CLASS_ID
    ) -> INT {
        (self.ot_check_any_binary_type)(obthis, any_var, expected_type)
    }
    pub unsafe fn ot_check_any_math_type(
        &self,
        obthis: POB_THIS,
        any_var: POB_DATA,
        expected_type: OB_CLASS_ID
    ) -> INT {
        (self.ot_check_any_math_type)(obthis, any_var, expected_type)
    }
    pub unsafe fn ot_check_any_enum_type(
        &self,
        obthis: POB_THIS,
        any_var: POB_DATA,
        current_group: POB_GROUP,
        expected_type: OB_CLASS_ID
    ) -> INT {
        (self.ot_check_any_enum_type)(obthis, any_var, current_group, expected_type)
    }
    pub unsafe fn ot_check_any_object_type(
        &self,
        obthis: POB_THIS,
        any_var: POB_DATA,
        current_group: POB_GROUP,
        expected_type: OB_CLASS_ID
    ) -> INT {
        (self.ot_check_any_object_type)(obthis, any_var, current_group, expected_type)
    }
    pub unsafe fn ot_duplicate_any(&self, pobThis: POB_THIS, pAny: POB_DATA) -> INT {
        (self.ot_duplicate_any)(pobThis, pAny)
    }
    pub unsafe fn ot_abs_any(&self, prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT {
        (self.ot_abs_any)(prtThis, pResult, pAny1)
    }
    pub unsafe fn ot_ceiling_any(&self, prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT {
        (self.ot_ceiling_any)(prtThis, pResult, pAny1)
    }
    pub unsafe fn ot_string_to_binary(
        &self,
        rtThis: POB_THIS,
        lpStr: LPTSTR,
        EncodingType: ::std::os::raw::c_int,
        bNullTerminated: BOOL
    ) -> PSH_BINARY {
        (self.ot_string_to_binary)(rtThis, lpStr, EncodingType, bNullTerminated)
    }
    pub unsafe fn ot_bytearray_to_binary(&self, rtThis: POB_THIS, array_inst: POB_ARRAY_INST) -> PSH_BINARY {
        (self.ot_bytearray_to_binary)(rtThis, array_inst)
    }
    pub unsafe fn ot_any_to_binary(&self, rtThis: POB_THIS, obData: POB_DATA) -> PSH_BINARY {
        (self.ot_any_to_binary)(rtThis, obData)
    }
    pub unsafe fn ob_set_curr_rtinst_and_return(&self, obthis: POB_THIS, new_rtinst: POB_RUNTIME_INST) -> () {
        (self.ob_set_curr_rtinst_and_return)(obthis, new_rtinst)
    }
    pub unsafe fn ob_unset_curr_rtinst_and_return(&self, obthis: POB_THIS) -> () {
        (self.ob_unset_curr_rtinst_and_return)(obthis)
    }
    pub unsafe fn ob_open_trace(
        &self,
        obthis: POB_THIS,
        filename: LPTSTR,
        kind: OB_TIMERKIND
    ) -> OB_ERROR_RETURN {
        (self.ob_open_trace)(obthis, filename, kind)
    }
    pub unsafe fn ob_close_trace(&self, obthis: POB_THIS) -> OB_ERROR_RETURN { (self.ob_close_trace)(obthis) }
    pub unsafe fn ob_begin_trace(&self, obthis: POB_THIS, message: LPTSTR) -> OB_ERROR_RETURN {
        (self.ob_begin_trace)(obthis, message)
    }
    pub unsafe fn ob_end_trace(&self, obthis: POB_THIS) -> OB_ERROR_RETURN { (self.ob_end_trace)(obthis) }
    pub unsafe fn ob_enable_event_trace(&self, obthis: POB_THIS, event: OB_TRACEID) -> OB_ERROR_RETURN {
        (self.ob_enable_event_trace)(obthis, event)
    }
    pub unsafe fn ob_disable_event_trace(&self, obthis: POB_THIS, event: OB_TRACEID) -> OB_ERROR_RETURN {
        (self.ob_disable_event_trace)(obthis, event)
    }
}
