#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::*;
pub use winapi::{
    shared::{
        minwindef::{
            BOOL, DWORD, GLOBALHANDLE, HFILE, HINSTANCE, INT, LPARAM, LPBYTE, LPVOID, UINT, ULONG, WORD, WPARAM
        }, ntdef::{HANDLE, HRESULT, LPCSTR, LPCWSTR, LPSTR, LPWSTR, SHORT, USHORT}, windef::{HDC, HMENU, HWND, LPRECT, POINT, RECT}, wtypesbase::LPOLESTR
    }, ucrt::corecrt::time_t, um::wingdi::LOGFONTW
};

pub type LOGFONT = LOGFONTW;

#[repr(C)]
pub struct IUnknown__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IUnknown {
    pub vtable_: *const IUnknown__bindgen_vtable
}
pub type PASCALFAR_INT_PROC = ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pbstg_statistics {
    pub lStgCount: ::std::os::raw::c_long,
    pub lStgHigh: ::std::os::raw::c_long,
    pub lStgCurrent: ::std::os::raw::c_long,
    pub lStgAlloc: ::std::os::raw::c_long,
    pub lStgMemAlloc: ::std::os::raw::c_long,
    pub lStgMemFree: ::std::os::raw::c_long
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct stg_subpool_entryS {
    pub pNext: *mut stg_subpool_entryS,
    pub pPrev: *mut stg_subpool_entryS,
    pub lpstrPoolName: LPTSTR
}
pub type pbstg_subpool = *mut stg_subpool_entryS;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct stg_anchorS {
    pub uiBlockSize: UINT,
    pub iAllocFlags: UINT,
    pub lpszOwner: LPTSTR,
    pub pSubpoolList: pbstg_subpool,
    pub pNext: *mut stg_anchorS
}
pub type ppbstg_anchor = *mut stg_anchorS;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct tag_os_stackgrowblock {
    pub stackframe: *mut ::std::os::raw::c_void,
    pub stackframe_subpool: pbstg_subpool
}
pub type pos_stackgrowblock = *mut tag_os_stackgrowblock;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sh_dbg_node {
    pub unused: INT,
    pub code: INT
}
pub type SH_DBG_NODE = sh_dbg_node;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
pub type POB_GROUP = *mut ob_group;
pub mod ob_group_types {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_SIMPLE: Type = 0;
    pub const OB_ARRAY: Type = 1;
}
pub use self::ob_group_types::Type as OB_GROUPTYPE;
pub type POB_GROUPTYPE = *mut ob_group_types::Type;
pub mod OB_FIELD_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_TYPEDEF_FIELD: Type = 0;
    pub const OB_INSTVAR_FIELD: Type = 1;
    pub const OB_GLOBAL_VAR: Type = 0;
    pub const OB_SHARED_VAR: Type = 1;
}
pub type POB_FIELD_TYPE = *mut OB_FIELD_TYPE::Type;
pub mod OB_MEMBER_ACCESS {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_PUBLIC_MEMBER: Type = 0;
    pub const OB_PRIVATE_MEMBER: Type = 1;
    pub const OB_PROTECTED_MEMBER: Type = 2;
    pub const OB_SYSTEM_MEMBER: Type = 3;
}
pub type POB_MEMBER_ACCESS = *mut OB_MEMBER_ACCESS::Type;
pub type OB_BASE_ID = USHORT;
pub type OB_GROUP_ID = OB_BASE_ID;
pub type POB_GROUP_ID = *mut OB_BASE_ID;
pub type OB_GROUP_HNDL = OB_GROUP_ID;
pub type POB_GROUP_HNDL = *mut OB_GROUP_ID;
pub type OB_CLASS_ID = OB_BASE_ID;
pub type POB_CLASS_ID = *mut OB_BASE_ID;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
pub mod OB_TYPE_KIND {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_SIMPLE_TYPE: Type = 0;
    pub const OB_SYSTEM_TYPE: Type = 1;
    pub const OB_USER_TYPE: Type = 2;
    pub const OB_UNDEFINED_TYPE: Type = 3;
}
pub mod ob_func_type {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_LOCAL_FUNC_DEF: Type = 0;
    pub const OB_GLOBAL_FUNC_REF: Type = 1;
    pub const OB_DLL_FUNC_DEF: Type = 2;
    pub const OB_SYSTEM_FUNC_DEF: Type = 3;
    pub const OB_RPC_FUNC_DEF: Type = 4;
    pub const OB_SYSDLL_FUNC_DEF: Type = 5;
    pub const OB_PSPP_FUNC_DEF: Type = 6;
}
pub use self::ob_func_type::Type as OB_FUNC_TYPE;
pub type POB_FUNC_TYPE = *mut ob_func_type::Type;
pub mod ob_protoarg_type {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_ARG_VAL: Type = 0;
    pub const OB_ARG_REF: Type = 1;
    pub const OB_ARG_VARLIST: Type = 2;
    pub const OB_ARG_READONLY: Type = 3;
}
pub use self::ob_protoarg_type::Type as OB_PROTOARG_TYPE;
pub type POB_PROTOARG_TYPE = *mut ob_protoarg_type::Type;
pub mod OB_ROUT_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_FUNCTION: Type = 0;
    pub const OB_EVENT: Type = 1;
    pub const OB_ANY_ROUT_TYPE: Type = 2;
}
pub type OB_SUBPOOL = pbstg_subpool;
pub mod OB_SOURCE_BLK_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_FORWARD_BLOCK: Type = 0;
    pub const OB_VAR_BLOCK: Type = 1;
    pub const OB_VAR_DECL_BLOCK: Type = 2;
    pub const OB_TYPEDEF_BLOCK: Type = 3;
    pub const OB_ON_EVT_BLOCK: Type = 4;
    pub const OB_FUNC_BLOCK: Type = 5;
    pub const OB_SUBROUTINE_BLOCK: Type = 6;
    pub const OB_PROTOTYPE_BLOCK: Type = 7;
    pub const OB_INSTVAR_BLOCK: Type = 8;
    pub const OB_FWDPROTO_BLOCK: Type = 9;
    pub const OB_NAMESPACE_BLOCK: Type = 10;
}
pub type POB_SOURCE_BLK_TYPE = *mut OB_SOURCE_BLK_TYPE::Type;
pub mod ob_glob_refstyle {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_GLOB_PARENT_REF: Type = 0;
    pub const OB_GLOB_ATTR_REF: Type = 1;
    pub const OB_GLOB_OTHER_REF: Type = 2;
    pub const OB_GLOB_NOT_REF: Type = 3;
}
pub use self::ob_glob_refstyle::Type as OB_GLOB_REFSTYLE;
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
pub type SH_LIST_HANDLE = *mut ::std::os::raw::c_void;
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
#[derive(Copy, Clone)]
pub struct sh_growblock {
    pub block: *mut ::std::os::raw::c_void,
    pub incr: UINT,
    pub size: UINT,
    pub pos: UINT,
    pub struct_size: UINT
}
pub type PSH_GROWBLOCK = *mut sh_growblock;
pub type RT_BREAK_PROC = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void) -> INT
>;
pub mod rt_mode {
    pub type Type = ::std::os::raw::c_int;
    pub const RT_DEVELOPMENT_MODE: Type = 0;
    pub const RT_RUNTIME_MODE: Type = 1;
}
pub use self::rt_mode::Type as RT_MODE;
pub mod rt_opt_mode {
    pub type Type = ::std::os::raw::c_int;
    pub const RT_OPTIMIZED: Type = 0;
    pub const RT_NOT_OPTIMIZED: Type = 1;
}
pub use self::rt_opt_mode::Type as RT_OPT_MODE;
pub mod ob_mode {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_LINK_AS_YOU_GO_MODE: Type = 0;
    pub const OB_BUILD_EXE_MODE: Type = 1;
    pub const OB_RUN_EXE_MODE: Type = 2;
    pub const OB_COMPILE_MODE: Type = 3;
    pub const OB_DEBUG_MODE: Type = 4;
    pub const OB_DEFAULT_MODE: Type = 5;
    pub const OB_BUILD_APPL_REPORT: Type = 6;
    pub const OB_BUILD_COMPILE_LIST_MODE: Type = 7;
    pub const OB_BUILD_OBJECT_REPORT: Type = 8;
    pub const OB_OBJECT_LOAD: Type = 9;
}
pub use self::ob_mode::Type as OB_MODE;
pub mod ob_exe_code_type {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_PCODE_EXE: Type = 0;
    pub const OB_CCODE_EXE: Type = 1;
}
pub use self::ob_exe_code_type::Type as OB_EXE_CODE_TYPE;
#[doc = ""]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_response_window_stack_node {
    pub routine_level: UINT,
    pub expr_stack_ptr: INT
}
#[doc = ""]
pub type ResponseWindowStackNode = ob_response_window_stack_node;
pub type POB_THIS = *mut ob_this;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct ob_this_ResponseWindowStack {
    pub stack: *mut ResponseWindowStackNode,
    pub capacity: UINT,
    pub count: UINT
}
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
#[derive(Copy, Clone)]
pub struct ob_data {
    pub val: OB_VALUE,
    pub info: OB_INFO_FLAGS,
    pub type_: OB_CLASS_ID
}
pub type OB_DATA = ob_data;
pub type POB_DATA = *mut ob_data;
pub mod OB_ARRAY_SYMBOL_STYLE {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_UNBOUNDED_ARRAY: Type = 0;
    pub const OB_SIMPLE_ARRAY: Type = 1;
    pub const OB_COMPLEX_ARRAY: Type = 2;
}
pub type KEY_FUNC = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void
>;
pub type KEY_FUNC_ARG = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void
>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct HASHSTAT {
    pub optSpread: f64,
    pub actSpread: f64,
    pub maxInList: INT
}
pub type PHASHSTAT = *mut HASHSTAT;
pub mod OB_ERROR {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_SUCCESS: Type = 0;
    pub const OB_OPEN_ERROR: Type = 1;
    pub const OB_READ_ERROR: Type = 2;
    pub const OB_WRITE_ERROR: Type = 3;
    pub const OB_SCAN_ERROR: Type = 4;
    pub const OB_VERSION_ERROR: Type = 5;
    pub const OB_NOTFOUND: Type = 6;
    pub const OB_SEMI_COMPILED_OBJ_ERROR: Type = 7;
    pub const OB_MISSING_ANCESTOR_ERROR: Type = 8;
    pub const OB_DUPLICATE_ANCESTOR_ERROR: Type = 9;
    pub const OB_INTERNAL_OVERFLOW: Type = 10;
    pub const OB_GOT_RUNTIME_ERROR: Type = 11;
    pub const OB_EXECUTION_ERROR: Type = 12;
    pub const OB_GENERAL_ERROR: Type = 13;
    pub const OB_GROUP_WRONG_FORMAT_ERROR: Type = 14;
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct shBinary {
    pub len: ULONG,
    pub data: [::std::os::raw::c_uchar; 1usize]
}
pub type PSH_BINARY = *mut shBinary;
pub mod OB_CONPOOL_ITEM_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_CONPOOL_STRING: Type = 0;
    pub const OB_CONPOOL_SHORT: Type = 1;
    pub const OB_CONPOOL_LONG: Type = 2;
    pub const OB_CONPOOL_FLOAT: Type = 3;
    pub const OB_CONPOOL_DOUBLE: Type = 4;
    pub const OB_CONPOOL_DEC: Type = 5;
    pub const OB_CONPOOL_TIME: Type = 6;
    pub const OB_CONPOOL_FUNCARG: Type = 7;
    pub const OB_CONPOOL_ARRAYDEF: Type = 8;
    pub const OB_CONPOOL_DBSTMT: Type = 9;
    pub const OB_CONPOOL_DBOUTVAR: Type = 10;
    pub const OB_CONPOOL_PCODE: Type = 11;
    pub const OB_CONPOOL_FLDNAMEID: Type = 12;
    pub const OB_CONPOOL_ROUTNAMEID: Type = 13;
    pub const OB_CONPOOL_OBINFO: Type = 14;
    pub const OB_CONPOOL_OBDATA: Type = 15;
    pub const OB_CONPOOL_FUNCTMPLTARG: Type = 16;
    pub const OB_CONPOOL_FUNCTMPLT: Type = 17;
    pub const OB_CONPOOL_CLSNAMEID: Type = 18;
    pub const OB_CONPOOL_ARRAYDATA: Type = 19;
    pub const OB_CONPOOL_DBVARS: Type = 20;
    pub const OB_CONPOOL_DBSTMT_INDIRECT: Type = 21;
    pub const OB_CONPOOL_CLASSID: Type = 22;
    pub const OB_CONPOOL_LONGLONG: Type = 23;
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct conpool_map {
    pub offset: OB_CONST_REF,
    pub item_type: SHORT,
    pub no_items: USHORT
}
pub type POB_CONPOOL_MAP = *mut conpool_map;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct perm_conpool {
    pub pool_size: ULONG,
    pub map_size: ULONG
}
pub type OB_PERM_CONPOOL = perm_conpool;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct ob_conpool {
    pub ps: OB_PERM_CONPOOL,
    pub ts: OB_TEMP_CONPOOL,
    pub strings_in_pool: *mut shhash
}
pub type POB_CONPOOL = *mut ob_conpool;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_symtab {
    pub table: *mut shhash,
    pub no_slots: INT,
    pub curr_id: INT,
    pub conpool: POB_CONPOOL,
    pub subpool: OB_SUBPOOL
}
pub type OB_SYMTAB = ob_symtab;
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
pub type POB_LOOKUP_ENTRY = *mut ob_lookup_entry;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct perm_lookup {
    pub alloc_size: USHORT
}
pub type OB_PERM_LOOKUP = perm_lookup;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct ob_lookup_table {
    pub ps: OB_PERM_LOOKUP,
    pub ts: OB_TEMP_LOOKUP
}
pub type POB_LOOKUP = *mut ob_lookup_table;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct perm_looksym {
    pub lookup_slots: USHORT,
    pub conpool_size: USHORT,
    pub symtab_slots: USHORT
}
pub type OB_PERM_LOOKSYM = perm_looksym;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct temp_looksym {
    pub lookup: POB_LOOKUP,
    pub conpool: POB_CONPOOL,
    pub symtab: *mut OB_SYMTAB
}
pub type OB_TEMP_LOOKSYM = temp_looksym;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
pub type PTNULL = *mut LPBYTE;
pub type PNARRAY_INIT_FN = ::std::option::Option<
    unsafe extern "C" fn(arg1: POB_THIS, arg2: *mut tag_OB_NARRAY, arg3: *mut ::std::os::raw::c_void) -> BOOL
>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
pub type POB_ARRAY_INST = *mut OB_NARRAY;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_arraydef {
    pub flags: USHORT,
    pub varinfo: OB_INFO_FLAGS,
    pub bounds: [::std::os::raw::c_long; 1usize]
}
pub type POB_ARRAYDEF = *mut ob_arraydef;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_pcode_line_node {
    pub line_no: USHORT,
    pub pcode_loc: USHORT
}
pub type POB_PCODE_LINE_NODE = *mut ob_pcode_line_node;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_perm_pcode_blk {
    pub len: USHORT,
    pub no_line_block: USHORT,
    pub max_stack_depth: USHORT
}
pub type OB_PERM_PCODE_BLK = ob_perm_pcode_blk;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct ob_pcode_blk {
    pub ps: OB_PERM_PCODE_BLK,
    pub ts: OB_TEMP_PCODE_BLK
}
pub type POB_PCODE_BLK = *mut ob_pcode_blk;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_perm_routnode {
    pub status: USHORT,
    pub proto_id: OB_PROTO_ID
}
pub type POB_PERM_ROUTNODE = *mut ob_perm_routnode;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_routnode {
    pub perm_entry: POB_PERM_ROUTNODE,
    pub source: LPTSTR,
    pub len: UINT,
    pub pcode: POB_PCODE_BLK,
    pub rout_symtab: OB_LOOK_SYMTAB,
    pub local_conpool: POB_CONPOOL
}
pub type POB_ROUTNODE = *mut ob_routnode;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_perm_routlist {
    pub no_slots: USHORT
}
pub type OB_PERM_ROUTLIST = ob_perm_routlist;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct ob_routlist {
    pub ps: OB_PERM_ROUTLIST,
    pub ts: OB_TEMP_ROUTLIST
}
pub type POB_ROUTLIST = *mut ob_routlist;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct _tagSH_DEC {
    pub v: [USHORT; 7usize],
    pub flags: USHORT
}
pub type SH_DEC = _tagSH_DEC;
pub type PSH_DEC = *mut _tagSH_DEC;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct OLD_IDEC {
    pub attr: ::std::os::raw::c_short,
    pub id: ::std::os::raw::c_short,
    pub sl: [::std::os::raw::c_ushort; 4usize],
    pub msd: ::std::os::raw::c_short,
    pub dummy: ::std::os::raw::c_short
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct OLD_LDEC {
    pub lattr: ::std::os::raw::c_short,
    pub lid: ::std::os::raw::c_short,
    pub lsl: [::std::os::raw::c_ulong; 2usize],
    pub lmsd: ::std::os::raw::c_short,
    pub ldummy: ::std::os::raw::c_short
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union OLD_DEC {
    pub dc: OLD_IDEC,
    pub ls: OLD_LDEC
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
    pub scope: OB_MEMBER_ACCESS::Type,
    pub read_access: OB_MEMBER_ACCESS::Type,
    pub write_access: OB_MEMBER_ACCESS::Type,
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
pub type POB_DATA_INFO = *mut ob_data_info;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_arg_info {
    pub argname: LPTSTR,
    pub datatype: LPTSTR,
    pub argtype: OB_PROTOARG_TYPE,
    pub grouping: OB_GROUPTYPE,
    pub array_bounds: LPTSTR
}
pub type POB_ARG_INFO = *mut ob_arg_info;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_class_info {
    pub classname: LPTSTR,
    pub class_hndl: OB_CLASS_HNDL
}
pub type POB_CLASS_INFO = *mut ob_class_info;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_event_info {
    pub event_name: LPTSTR,
    pub token_name: LPTSTR,
    pub class_hndl: OB_CLASS_HNDL,
    pub vtable_id: OB_VTABLE_ID
}
pub type POB_EVENT_INFO = *mut ob_event_info;
pub mod OB_CALL_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_SYSFUNC_CALL: Type = 0;
    pub const OB_DLLFUNC_CALL: Type = 1;
    pub const OB_GLOBFUNC_CALL: Type = 2;
    pub const OB_OBJFUNC_CALL: Type = 3;
    pub const OB_LOCALFUNC_CALL: Type = 4;
    pub const OB_PARENTFUNC_CALL: Type = 5;
    pub const OB_PRIMARYFUNC_CALL: Type = 6;
}
pub mod OB_PROTOREF_ERROR {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_PROTOREF_OK: Type = 0;
    pub const OB_PROTOREF_NOTFOUND: Type = 1;
    pub const OB_PROTOREF_BADNOARGS: Type = 2;
    pub const OB_PROTOREF_BADARGS: Type = 3;
    pub const OB_PROTOREF_INACCESSABLE: Type = 4;
    pub const OB_PROTOREF_BADREFARG: Type = 5;
    pub const OB_PROTOREF_BAD: Type = 6;
    pub const OB_PROTOREF_BADREFTYPE: Type = 7;
    pub const OB_PROTOREF_BADOVERLOAD: Type = 8;
    pub const OB_PROTOREF_ANCREFTYPE: Type = 9;
    pub const OB_PROTOREF_AMBIGUOUS: Type = 10;
}
pub type POB_PROTOREF_ERROR = *mut OB_PROTOREF_ERROR::Type;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_funccall_info {
    pub funcname: LPTSTR,
    pub argtypes: POB_CLASS_ID,
    pub no_args: UINT,
    pub functype: OB_CLASS_ID,
    pub id: UINT,
    pub calltype: OB_CALL_TYPE::Type,
    pub dllname: LPTSTR,
    pub group_id: OB_GROUP_HNDL,
    pub class_id: OB_CLASS_ID
}
pub type POB_FUNCCALL_INFO = *mut ob_funccall_info;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_array_dim {
    pub upbound: INT,
    pub lowbound: INT
}
pub type OB_ARRAY_DIM = ob_array_dim;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_array_info_tag {
    pub array_data: *mut ::std::os::raw::c_void,
    pub no_dims: UINT,
    pub dimensions: [OB_ARRAY_DIM; 1usize]
}
pub type POB_ARRAY_INFO = *mut ob_array_info_tag;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_enum_info {
    pub name: LPTSTR,
    pub value: INT
}
pub type POB_ENUM_INFO = *mut ob_enum_info;
pub mod ob_mac_target {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_MAC_POWERPC_TARGET: Type = 0;
    pub const OB_MAC_68K_TARGET: Type = 1;
    pub const OB_MAC_FAT_TARGET: Type = 2;
}
pub use self::ob_mac_target::Type as OB_MAC_TARGET;
pub mod ob_exec_optimize {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_OPTIMIZE_SPEED: Type = 0;
    pub const OB_OPTIMIZE_SPACE: Type = 1;
    pub const OB_OPTIMIZE_NONE: Type = 2;
    pub const OB_OPTIMIZE_DEBUG: Type = 3;
}
pub use self::ob_exec_optimize::Type as OB_EXEC_OPTIMIZE;
pub mod ob_exec_category {
    pub type Type = ::std::os::raw::c_int;
    pub const EXEC_CHECKING_REFERENCES: Type = 0;
    pub const EXEC_WRITING_OBJECT: Type = 1;
    pub const EXEC_GENERATING_CODE_FOR_OBJECT: Type = 2;
    pub const EXEC_COMPILING_FILE: Type = 3;
    pub const EXEC_LINKING: Type = 4;
}
pub use self::ob_exec_category::Type as OB_EXEC_STAGE;
pub type POB_EXEC = *mut ob_exec;
pub type OB_EXEC_CALLBACK = ::std::option::Option<unsafe extern "C" fn(arg1: POB_EXEC) -> BOOL>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_conflict_list {
    pub error_type: OB_ERROR::Type,
    pub original_group_name: LPTSTR,
    pub conflict_group_name: LPTSTR,
    pub class_name: LPTSTR
}
pub type POB_CONFLICT_LIST = *mut ob_conflict_list;
pub type POB_SOURCE_BLOCK = *mut TCHAR;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_compile_list {
    pub lib_name: LPTSTR,
    pub entry_name: LPTSTR
}
pub type POB_COMPILE_LIST = *mut ob_compile_list;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_hierarchy_list {
    pub class_name: LPTSTR,
    pub class_hndl: OB_CLASS_HNDL,
    pub parent_loc: UINT
}
pub type POB_HIERARCHY_LIST = *mut ob_hierarchy_list;
pub mod ob_field_filter {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_ANY_FIELDS: Type = 0;
    pub const OB_INSTANCE_FIELDS_ONLY: Type = 1;
    pub const OB_TYPEDEF_FIELDS_ONLY: Type = 2;
}
pub use self::ob_field_filter::Type as OB_FIELD_FILTER;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_appl_report {
    pub lpszLibraryName: LPTSTR,
    pub lpszName: LPTSTR,
    pub pList: *mut ::std::os::raw::c_void,
    pub iType: UINT,
    pub bIsInstanced: BOOL
}
pub type POB_APPL_REPORT = *mut ob_appl_report;
pub mod ob_proto_overload_error {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_NO_OVERLOAD_ERROR: Type = 0;
    pub const OB_ARG_TYPE_ERROR: Type = 1;
    pub const OB_RETURN_TYPE_ERROR: Type = 2;
}
pub use self::ob_proto_overload_error::Type as OB_PROTO_OVERLOAD_ERROR;
pub type POB_PROTO_OVERLOAD_ERROR = *mut ob_proto_overload_error::Type;
pub mod ob_lib_include_type {
    pub type Type = ::std::os::raw::c_int;
    pub const EXCLUDE_ALL: Type = 0;
    pub const INCLUDE_REFERENCED: Type = 1;
    pub const INCLUDE_REFERENCED_AND_DWS: Type = 2;
    pub const INCLUDE_INDEPENDENT_OBJECTS: Type = 3;
    pub const INCLUDE_ALL: Type = 4;
}
pub type POB_LIB_INCLUDE_TYPE = *mut ob_lib_include_type::Type;
pub type PBD_ARRAY = *mut LPTSTR;
pub type PBCHAR = u16;
pub mod PBObjectType {
    pub type Type = ::std::os::raw::c_int;
    pub const PBObjectTypeNVO: Type = 0;
    pub const PBObjectTypeCustomVisual: Type = 1;
    pub const PBObjectTypeWindow: Type = 2;
    pub const PBObjectTypeApplication: Type = 3;
    pub const PBObjectTypeCount: Type = 4;
}
pub mod PBValueType {
    pub type Type = ::std::os::raw::c_int;
    pub const PBValueTypeNull: Type = 0;
    pub const PBValueTypeAny: Type = 1;
    pub const PBValueTypePointer: Type = 2;
    pub const PBValueTypeChar: Type = 3;
    pub const PBValueTypeByte: Type = 4;
    pub const PBValueTypeInt: Type = 5;
    pub const PBValueTypeUInt: Type = 6;
    pub const PBValueTypeShort: Type = 7;
    pub const PBValueTypeUShort: Type = 8;
    pub const PBValueTypeLong: Type = 9;
    pub const PBValueTypeULong: Type = 10;
    pub const PBValueTypeLonglong: Type = 11;
    pub const PBValueTypeDecimal: Type = 12;
    pub const PBValueTypeFloat: Type = 13;
    pub const PBValueTypeDouble: Type = 14;
    pub const PBValueTypeBoolean: Type = 15;
    pub const PBValueTypeDate: Type = 16;
    pub const PBValueTypeTime: Type = 17;
    pub const PBValueTypeDateTime: Type = 18;
    pub const PBValueTypeString: Type = 19;
    pub const PBValueTypeBlob: Type = 20;
    pub const PBValueTypeInstance: Type = 21;
    pub const PBValueTypeArray: Type = 22;
    pub const PBValueTypeUserDefined: Type = 23;
    pub const PBValueTypeCount: Type = 24;
}
pub mod PBVariableKind {
    pub type Type = ::std::os::raw::c_int;
    pub const PBVariableKindGlobal: Type = 0;
    pub const PBVariableKindShared: Type = 1;
    pub const PBVariableKindInstance: Type = 2;
    pub const PBVariableKindArgument: Type = 3;
    pub const PBVariableKindLocal: Type = 4;
}
pub mod PBVariableAccess {
    pub type Type = ::std::os::raw::c_int;
    pub const PBVariableAccessPrivate: Type = 0;
    pub const PBVariableAccessPublic: Type = 1;
    pub const PBVariablerAccessProtected: Type = 2;
    pub const PBVariableAccessSystem: Type = 3;
}
pub mod PBArgCallConvention {
    pub type Type = ::std::os::raw::c_int;
    pub const PBArgCallConventionByReference: Type = 0;
    pub const PBArgCallConventionByValue: Type = 1;
    pub const PBArgCallConventionReadOnly: Type = 2;
    pub const PBArgCallConventionVariableList: Type = 3;
}
pub mod PBVariableCardinalityType {
    pub type Type = ::std::os::raw::c_int;
    pub const PBVariableCardinalityTypeScalarType: Type = 0;
    pub const PBVariableCardinalityTypeUnboundedArray: Type = 1;
    pub const PBVariableCardinalityTypeBoundedArray: Type = 2;
}
pub mod PBScriptKind {
    pub type Type = ::std::os::raw::c_int;
    pub const PBScriptKindEvent: Type = 0;
    pub const PBScriptKindFunction: Type = 1;
}
pub mod PBTypeCategory {
    pub type Type = ::std::os::raw::c_int;
    pub const PBTypeCategorySimpleType: Type = 0;
    pub const PBTypeCategoryEnumeratedType: Type = 1;
    pub const PBTypeCategoryClassOrStructureType: Type = 2;
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBID {
    pub ulData1: ::std::os::raw::c_ulong,
    pub usData2: ::std::os::raw::c_ushort,
    pub usData3: ::std::os::raw::c_ushort,
    pub ucData4: [::std::os::raw::c_uchar; 8usize]
}
pub type PBIDREF = *mut PBID;
pub type PBIIDREF = PBIDREF;
#[repr(C)]
pub struct PBIUnknown__bindgen_vtable(::std::os::raw::c_void);
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIUnknown {
    pub vtable_: *const PBIUnknown__bindgen_vtable
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIString {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIDateTime {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIDate {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBITime {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIDecimal {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIBlob {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBISession {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIContext {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIInstance {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIArgument {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIValue {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIArray {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIArrayBounds {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIArrayBoundsList {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIException {
    pub _base: PBIUnknown
}
pub type PPBIClassDef = *mut PBIClassDef;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBITypeDef {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIClassDef {
    pub _base: PBITypeDef
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIClassDefList {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIVariableDef {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIVariableDefList {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIScriptDef {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIScriptDefList {
    pub _base: PBIUnknown
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PBIVariableCardinalityDef {
    pub _base: PBIUnknown
}
pub mod ob_compile_list_type {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_INCREMENTAL_LIST: Type = 0;
    pub const OB_FULL_LIST: Type = 1;
    pub const OB_MIGRATION_LIST: Type = 2;
}
pub use self::ob_compile_list_type::Type as OB_COMPILE_LIST_TYPE;
pub mod ob_inconsistency_type {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_NO_INCONSISTENCY: Type = 0;
    pub const OB_INCONSISTENT_VERSION: Type = 1;
    pub const OB_INCONSISTENT_COMPILE: Type = 2;
}
pub use self::ob_inconsistency_type::Type as OB_INCONSISTENCY_TYPE;
pub type POB_INCONSISTENCY_TYPE = *mut ob_inconsistency_type::Type;
pub type OS_CALLC_FUNC = ::std::option::Option<unsafe extern "C" fn()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdb_main {
    _unused: [u8; 0]
}
pub type PCMDB_MAIN = *mut cmdb_main;
pub mod CM_COMPILE_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const CM_COMPILE_TYPEDEFS_ONLY: Type = 0;
    pub const CM_COMPILE_SCRIPTS_ONLY: Type = 1;
    pub const CM_COMPILE_ALL: Type = 2;
}
pub type CM_DBSIGNON_PROC = ::std::option::Option<
    unsafe extern "C" fn(pbThis: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void
>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
    pub compile_type: CM_COMPILE_TYPE::Type,
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
    pub source_type: OB_SOURCE_BLK_TYPE::Type,
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
    pub obRoutType: OB_ROUT_TYPE::Type,
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
pub type PCM_THIS = *mut cm_this;
pub mod cm_rebuild_type {
    pub type Type = ::std::os::raw::c_int;
    pub const CM_REBUILD_INCREMENTAL: Type = 0;
    pub const CM_REBUILD_FULL: Type = 1;
    pub const CM_REBUILD_MIGRATE: Type = 2;
}
pub use self::cm_rebuild_type::Type as CM_REBUILD_TYPE;
pub mod cm_rebuild_category {
    pub type Type = ::std::os::raw::c_int;
    pub const REBUILD_COMPILE_LIST: Type = 0;
    pub const REBUILD_CREATE_TYPES: Type = 1;
    pub const REBUILD_OBJECT_STAGE1: Type = 2;
    pub const REBUILD_OBJECT_STAGE2: Type = 3;
    pub const REBUILD_OBJECT_REGENERATE: Type = 4;
    pub const REBUILD_COMPLETE: Type = 5;
    pub const REBUILD_LIB_READONLY: Type = 6;
}
pub use self::cm_rebuild_category::Type as CM_REBUILD_STAGE;
pub mod cm_rebuild_status {
    pub type Type = ::std::os::raw::c_int;
    pub const REBUILD_NO_MESSAGES: Type = 0;
    pub const REBUILD_MESSAGES: Type = 1;
    pub const REBUILD_ERRORS: Type = 2;
}
pub type PCM_REBUILD_STATUS = *mut cm_rebuild_status::Type;
pub type PCM_REBUILD = *mut cm_rebuild;
pub type CM_REBUILD_CALLBACK = ::std::option::Option<unsafe extern "C" fn(arg1: PCM_REBUILD) -> BOOL>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct cm_rebuild {
    pub pGlobals: *mut ::std::os::raw::c_void,
    pub hParent: UINT,
    pub fnCallback: CM_REBUILD_CALLBACK,
    pub rebuildStage: CM_REBUILD_STAGE,
    pub lpszStageArgument: LPTSTR,
    pub lpszLibraryName: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ot_lvalue_info {
    pub group_hndl: OB_GROUP_HNDL
}
pub type POT_LVALUE_INFO = *mut ot_lvalue_info;
pub mod OT_REFPAK_STYLE {
    pub type Type = ::std::os::raw::c_int;
    pub const OT_SIMPLE_REF: Type = 0;
    pub const OT_FIELD_REF: Type = 1;
    pub const OT_FIELD_ITEM_REF: Type = 2;
}
pub type OT_FIELDUPDATE_FUNC = ::std::option::Option<
    unsafe extern "C" fn(rtthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, index: ULONG) -> INT
>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ot_ref_pak_simple_ref_tag {
    pub lvalue: POB_DATA
}
pub type ot_ref_pak_simple_ref = ot_ref_pak_simple_ref_tag;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
    pub style: OT_REFPAK_STYLE::Type,
    pub group_hndl: OB_GROUP_HNDL,
    pub type_: OB_CLASS_ID,
    pub flags: USHORT,
    pub ref_: OT_REF_TAG_UNION
}
pub type POT_REF_PAK = *mut ot_ref_pak;
pub type TIME_T = time_t;
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
pub mod OB_SESSION_STATE {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_SESSION_ACTIVE: Type = 0;
    pub const OB_SESSION_SHUTTING_DOWN: Type = 1;
    pub const OB_SESSION_SHUTDOWN: Type = 2;
}
pub mod OB_REMREF_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const PB_TYPE: Type = 0;
    pub const JAG_TYPE: Type = 1;
}
#[repr(C, packed)]
pub struct OB_ILOCAL_SESSION {
    pub _base: OB_ISESSION
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VTAB_INFO {
    pub vtab_index: USHORT,
    pub function: *mut ::std::os::raw::c_void
}
pub type PVTAB_INFO = *mut VTAB_INFO;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VTAB_CLASS_INFO {
    pub classId: OB_CLASS_ID,
    pub numFuncs: USHORT,
    pub numEvents: USHORT,
    pub funcTableOffset: ::std::os::raw::c_long,
    pub eventTableOffset: ::std::os::raw::c_long,
    pub classVtableThunked: ::std::os::raw::c_long
}
pub type OB_FUNC_FUNC = ::std::option::Option<unsafe extern "C" fn(arg1: POB_THIS, arg2: UINT) -> INT>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_runtime_vtable {
    pub func_ptr: OS_CALLC_FUNC
}
pub type POB_RUNTIME_VTABLE = *mut ob_runtime_vtable;
pub type PPOB_RUNTIME_CLASS = *mut POB_RUNTIME_CLASS;
pub type POB_PROTOTYPE = *mut ob_prototype;
pub type POB_CLASS_ENTRY = *mut ob_class_entry;
#[repr(C)]
#[derive(Copy, Clone)]
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
pub mod OB_MEMBER_ACCESS_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const IGNORE_ACCESS_CHECK: Type = 0;
    pub const LOCAL_CLASS_ACCESS_CHECK: Type = 1;
    pub const ANC_CLASS_ACCESS_CHECK: Type = 2;
    pub const FOREIGN_CLASS_ACCESS_CHECK: Type = 3;
}
pub type POB_MEMBER_ACCESS_TYPE = *mut OB_MEMBER_ACCESS_TYPE::Type;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct tag_OB_EVENT_LOOKUP_ITEM {
    pub token: OB_EVT_TOKEN_ID,
    pub vtable_id: OB_VTABLE_ID
}
pub type POB_EVENT_LOOKUP_ITEM = *mut tag_OB_EVENT_LOOKUP_ITEM;
pub mod OB_FUNCPROTO_STYLE {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_SYS_PROTOTYPE: Type = 0;
    pub const OB_USER_PROTOTYPE: Type = 1;
    pub const OB_FWD_PROTOTYPE: Type = 2;
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
pub type POB_PERM_PROTOTYPE = *mut ob_perm_prototype;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_prototype {
    pub perm_entry: POB_PERM_PROTOTYPE,
    pub func_ptr: OS_CALLC_FUNC
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_indattr_functmplt {
    pub name: OB_CONST_REF,
    pub args: OB_CONST_REF,
    pub no_args: USHORT,
    pub func_type: USHORT,
    pub isDynamic: USHORT,
    pub padding: USHORT
}
pub type POB_INDATTR_FUNCTMPLT = *mut ob_indattr_functmplt;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_enumfield {
    pub name: OB_CONST_REF,
    pub val: SHORT,
    pub padding: SHORT
}
pub type POB_ENUMFIELD = *mut ob_enumfield;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_perm_virtual_node {
    pub vtable_id: OB_VTABLE_ID,
    pub proto_id: OB_PROTO_ID,
    pub class_id: OB_CLASS_ID
}
pub type POB_PERM_VIRTUAL_NODE = *mut ob_perm_virtual_node;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_virtual_node {
    pub rout_id: OB_ROUT_ID,
    pub proto_id: OB_PROTO_ID,
    pub group_id: OB_GROUP_ID,
    pub class_id: OB_CLASS_ID
}
pub type POB_VIRTUAL_NODE = *mut ob_virtual_node;
pub mod OB_CLASS_STYLE {
    pub type Type = ::std::os::raw::c_int;
    pub const TYPE_CLASS: Type = 0;
    pub const TYPE_ENUM: Type = 1;
    pub const TYPE_INIT_SOURCE: Type = 2;
    pub const TYPE_INDIRECT: Type = 3;
    pub const TYPE_VAR_BLOCK: Type = 4;
    pub const TYPE_INHERITED: Type = 5;
}
pub type POB_CLASS_STYLE = *mut OB_CLASS_STYLE::Type;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_proto_arg {
    pub name: OB_CONST_REF,
    pub arrdef: OB_CONST_REF,
    pub datatype: OB_CLASS_ID,
    pub info: USHORT
}
pub type POB_PROTO_ARG = *mut ob_proto_arg;
pub type OB_MOD_SYMTAB = OB_LOOK_SYMTAB;
pub type OB_ENUM_SYMTAB = OB_LOOK_SYMTAB;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
pub type POB_PERM_CLASS_ENTRY = *mut ob_perm_class_entry;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct perm_type_descript {
    pub no_slots: USHORT,
    pub no_def_slots: USHORT
}
pub type OB_PERM_TYPE_DESCRIPT = perm_type_descript;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct ob_type_descript {
    pub ps: OB_PERM_TYPE_DESCRIPT,
    pub ts: OB_TEMP_TYPE_DESCRIPT
}
pub type POB_TYPE_DESCRIPT = *mut ob_type_descript;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_typedef {
    pub type_symtab: OB_LOOK_SYMTAB,
    pub enum_symtab: OB_ENUM_SYMTAB,
    pub conpool: POB_CONPOOL,
    pub descript: POB_TYPE_DESCRIPT,
    pub arg_conpool: POB_CONPOOL
}
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
    pub scope: OB_MEMBER_ACCESS::Type,
    pub read_access: OB_MEMBER_ACCESS::Type,
    pub write_access: OB_MEMBER_ACCESS::Type,
    pub flags: UINT,
    pub set_func: LPTSTR,
    pub get_func: LPTSTR,
    pub array_set_func: LPTSTR,
    pub array_get_func: LPTSTR,
    pub array_upper_func: LPTSTR
}
pub type POB_TYPEINFO = *mut ob_typeinfo;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_act_arg {
    pub datatype: OB_CLASS_ID,
    pub group_id: OB_GROUP_ID,
    pub info: USHORT,
    pub num_dims: ULONG,
    pub dimensions: *mut ::std::os::raw::c_long
}
pub type POB_ACT_ARG = *mut ob_act_arg;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_protoname {
    pub group_id: OB_GROUP_ID,
    pub class_id: OB_CLASS_ID,
    pub proto_id: OB_PROTO_ID,
    pub vtable_id: OB_VTABLE_ID,
    pub rout_type: OB_ROUT_TYPE::Type,
    pub protoname: LPTSTR,
    pub classname: LPTSTR,
    pub is_a_dllfunc: BOOL,
    pub is_a_dbrpc: BOOL,
    pub token: OB_EVT_TOKEN_ID,
    pub type_: OB_CLASS_ID,
    pub args: POB_PROTO_ARG,
    pub no_args: UINT
}
pub type POB_PROTONAME = *mut ob_protoname;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MONTHANDDAYNAMESSTRUCT_TAG {
    pub monAbbrev: *mut LPTSTR,
    pub monName: *mut LPTSTR,
    pub dayAbbrev: *mut LPTSTR,
    pub dayName: *mut LPTSTR
}
pub type LPMONTHANDDAYNAMESSTRUCT = *mut MONTHANDDAYNAMESSTRUCT_TAG;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct REGPROFILESTRUCT_TAG {
    pub hKey: HANDLE,
    pub lpszSubKey: LPTSTR,
    pub lpszValueName: LPTSTR,
    pub lRegError: ::std::os::raw::c_long,
    pub lpszIniFileName: LPTSTR,
    pub lpszSectionName: LPTSTR,
    pub lpszKeyName: LPTSTR,
    pub lpszValueReceiver: LPTSTR,
    pub dwValueSize: DWORD,
    pub lpszDefaultValue: LPTSTR
}
pub type LPREGPROFILESTRUCT = *mut REGPROFILESTRUCT_TAG;
pub mod ob_glob_reftype {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_GLOB_REF: Type = 0;
    pub const OB_GLOB_DECL: Type = 1;
}
pub use self::ob_glob_reftype::Type as OB_GLOB_REFTYPE;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_globsym_entry {
    pub name: OB_CONST_REF,
    pub group_id: OB_GROUP_ID,
    pub class_id: OB_CLASS_ID,
    pub id: OB_SYM_ID,
    pub info: USHORT
}
pub type POB_GLOBSYM_ENTRY = *mut ob_globsym_entry;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_perm_globsym {
    pub no_slots: USHORT
}
pub type OB_PERM_GLOBSYM = ob_perm_globsym;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_temp_globsym {
    pub table: POB_GLOBSYM_ENTRY,
    pub alloc_incr: UINT,
    pub alloc_size: UINT,
    pub slot_incr: UINT,
    pub next_free: UINT
}
pub type OB_TEMP_GLOBSYM = ob_temp_globsym;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ob_globsym {
    pub ps: OB_PERM_GLOBSYM,
    pub ts: OB_TEMP_GLOBSYM
}
pub type POB_GLOBSYM = *mut ob_globsym;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
pub mod RT_EXEC_STATUS {
    pub type Type = ::std::os::raw::c_int;
    pub const RT_EXEC_SUCCESS: Type = 0;
    pub const RT_EXEC_NO_SCRIPT: Type = 1;
    pub const RT_EXEC_FAILURE: Type = 2;
    pub const RT_EXEC_BADTOKEN: Type = 3;
    pub const RT_EXEC_NO_MATCH: Type = 4;
}
pub mod WATCHPOINT_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const LOCAL_WATCH: Type = 0;
    pub const GLOBAL_WATCH: Type = 1;
    pub const SHARED_WATCH: Type = 2;
    pub const INSTANCE_WATCH: Type = 3;
}
#[doc = "In order not to include an extreme number of PB header files in ocx"]
#[doc = "the rt_error_struct structure which is defined here"]
#[doc = "is redefined in pbrxctl.h"]
#[doc = ""]
#[doc = "IT MUST BE KEPT IN SYNC WITH THE VERSION WITHIN PBRXCTL.H"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rt_error_struct {
    pub rtthis: POB_THIS,
    pub message: LPTSTR,
    pub error_no: SHORT,
    pub line_no: USHORT,
    pub group_name: LPTSTR,
    pub class_name: LPTSTR,
    pub rout_name: LPTSTR
}
#[doc = "In order not to include an extreme number of PB header files in ocx"]
#[doc = "the rt_error_struct structure which is defined here"]
#[doc = "is redefined in pbrxctl.h"]
#[doc = ""]
#[doc = "IT MUST BE KEPT IN SYNC WITH THE VERSION WITHIN PBRXCTL.H"]
#[doc = ""]
pub type PRT_ERROR_STRUCT = *mut rt_error_struct;
pub mod RT_CALL_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const RT_INST_CALL: Type = 0;
    pub const RT_CLASS_CALL: Type = 1;
    pub const RT_CLASS_QUALIFIED_CALL: Type = 2;
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rtClassInfo_tag {
    pub obClassHndl: OB_CLASS_HNDL,
    pub obInst: OB_INST_ID
}
pub type RT_CALL_TYPE_INFO = rtClassInfo_tag;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rtCallInfo {
    pub rtClassInfo: RT_CALL_TYPE_INFO,
    pub enCallType: RT_CALL_TYPE::Type,
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
    pub obRoutineType: OB_ROUT_TYPE::Type,
    pub obMemberAccess: OB_MEMBER_ACCESS::Type,
    pub pobdArgArray: POB_DATA,
    pub ppchArgNameArray: *mut LPTSTR,
    pub bVarArgs: BOOL,
    pub ppArrayDefs: *mut ::std::os::raw::c_void,
    pub pchSystemEventName: LPTSTR
}
pub type PRT_ROUTINE_PROTO_INFO = *mut rtRoutineProtoInfo;
pub mod RT_REFARG_TYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const RT_SIMPLE: Type = 0;
    pub const RT_NOTIFY: Type = 1;
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rtRefArgInfo {
    pub rtRefType: RT_REFARG_TYPE::Type,
    pub pobdRefData: POB_DATA
}
pub type PRT_REFARG_INFO = *mut rtRefArgInfo;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rtClassDescrip {
    pub pchClassName: LPTSTR,
    pub bIsStruct: BOOL,
    pub bIsGlobalStruct: BOOL
}
pub type PRT_CLASS_DESCRIP = *mut rtClassDescrip;
pub type POB_RUNTIME_INST = *mut OB_OBJECT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_perm_group {
    pub name: OB_CONST_REF,
    pub modify_time: TIME_T,
    pub compile_time: TIME_T,
    pub info: USHORT,
    pub padding: SHORT
}
pub type OB_PERM_GROUP = ob_perm_group;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOB_PsppDll {
    _unused: [u8; 0]
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct ob_group {
    pub ps: OB_PERM_GROUP,
    pub ts: OB_TEMP_GROUP,
    pub lpszGroupName: LPTSTR
}
pub mod ob_group_lock_state {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_READLOCKED_GROUP: Type = 0;
    pub const OB_SYSTEM_GROUP: Type = 1;
    pub const OB_UNLOCKED_GROUP: Type = 2;
    pub const OB_WRITELOCKED_GROUP: Type = 3;
}
pub use self::ob_group_lock_state::Type as OB_GROUP_LOCK_STATE;
pub mod ob_group_load_state {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_GROUP_NOT_LOADED: Type = 0;
    pub const OB_GROUP_GLOBSYM_LOADED: Type = 1;
    pub const OB_GROUP_TYPEDEFS_LOADED: Type = 2;
    pub const OB_GROUP_ALL_LOADED: Type = 3;
}
pub use self::ob_group_load_state::Type as OB_GROUP_LOAD_STATE;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ob_src_last_edit {
    pub pEntry: LPTSTR,
    pub usLastScript: USHORT
}
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
pub mod OT_TYPE_LOC {
    pub type Type = ::std::os::raw::c_int;
    pub const OT_IN_DATA_NODE: Type = 0;
    pub const OT_OUT_DATA_NODE: Type = 1;
}
pub mod OT_TYPE_CHECK_ERROR {
    pub type Type = ::std::os::raw::c_int;
    pub const OT_TYPCHECK_SUCCESS: Type = 0;
    pub const OT_TYPCHECK_BAD_ARRAY_TYPES: Type = 1;
    pub const OT_TYPCHECK_BAD_TYPES: Type = 2;
    pub const OT_TYPCHECK_MIXED_GROUPING: Type = 3;
    pub const OT_TYPCHECK_UNDECLARED: Type = 4;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IPB_String {
    pub _address: u8
}
pub type IPB_String = _IPB_String;
pub mod DWPortDataFormat {
    pub type Type = ::std::os::raw::c_int;
    pub const DW_XML: Type = 0;
    pub const DW_PDF: Type = 1;
    pub const DW_XHTML: Type = 2;
}
#[repr(C)]
pub struct IDW_DataTemplateList__bindgen_vtable(::std::os::raw::c_void);
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IDW_DataTemplateList {
    pub vtable_: *const IDW_DataTemplateList__bindgen_vtable
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fillBrushInfo {
    pub brushmode: SHORT,
    pub color: ::std::os::raw::c_long,
    pub transparency: SHORT,
    pub gradientcolor: ::std::os::raw::c_long,
    pub gradienttransparency: SHORT,
    pub gradientangle: SHORT,
    pub gradientrepetitionmode: SHORT,
    pub gradientrepetitioncount: ::std::os::raw::c_long,
    pub gradientrepetitionlength: ::std::os::raw::c_long,
    pub gradientfocus: SHORT,
    pub gradientscale: SHORT,
    pub gradientspread: SHORT,
    pub picturefile: LPTSTR,
    pub picturemode: SHORT,
    pub picturescalex: SHORT,
    pub picturescaley: SHORT,
    pub pictureclipleft: SHORT,
    pub pictureclipright: SHORT,
    pub picturecliptop: SHORT,
    pub pictureclipbottom: SHORT,
    pub pictureblur: SHORT,
    pub picturetransparency: SHORT,
    pub pIA: *mut ::std::os::raw::c_void
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IPB_XMLDOMDocument {
    pub _address: u8
}
pub type IPB_XMLDOMDocument = _IPB_XMLDOMDocument;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINKATT {
    pub lInkColor: DWORD,
    pub lInkHeight: ::std::os::raw::c_long,
    pub lInkWidth: ::std::os::raw::c_long,
    pub iPenTip: ::std::os::raw::c_ushort,
    pub iTransparency: ::std::os::raw::c_ushort,
    pub iRasterOperation: ::std::os::raw::c_ushort,
    pub inkFlags: ::std::os::raw::c_ushort
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWEDITINKEDIT {
    pub strFactoid: LPTSTR,
    pub lRecogTimeout: ::std::os::raw::c_long,
    pub iInkMode: ::std::os::raw::c_short,
    pub iStatus: ::std::os::raw::c_short,
    pub iZoomFactor: ::std::os::raw::c_short,
    pub inkedFlags: ::std::os::raw::c_ushort
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DDCALENDARPROPS {
    pub lTextColor: ::std::os::raw::c_long,
    pub lTitleTextColor: ::std::os::raw::c_long,
    pub lTrailingTextColor: ::std::os::raw::c_long,
    pub lBackColor: ::std::os::raw::c_long,
    pub lTitleBackColor: ::std::os::raw::c_long,
    pub uFlags: ::std::os::raw::c_ushort,
    pub uFiller: ::std::os::raw::c_ushort
}
pub mod DWTYPES {
    pub type Type = ::std::os::raw::c_int;
    pub const DWBITMAP: Type = 0;
    pub const DWCOL: Type = 1;
    pub const DWCOMPUTE: Type = 2;
    pub const DWOVAL: Type = 3;
    pub const DWLINE: Type = 4;
    pub const DWRECT: Type = 5;
    pub const DWRRECT: Type = 6;
    pub const DWTEXT: Type = 7;
    pub const DWTBLOB: Type = 8;
    pub const DWGRAPH: Type = 9;
    pub const DWCROSSTAB: Type = 10;
    pub const DWREPORT: Type = 11;
    pub const DWOLE: Type = 12;
    pub const DWBUTTON: Type = 13;
    pub const DWGROUPBOX: Type = 14;
    pub const DWINKPIC: Type = 15;
    pub const DWWPF: Type = 16;
    pub const DWNULL: Type = 17;
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWOLER {
    pub prog: *mut ::std::os::raw::c_void,
    pub hWnd: HWND,
    pub hWndClient: HWND,
    pub activeOle: *mut ::std::os::raw::c_void,
    pub pControl: *mut ::std::os::raw::c_void,
    pub sharedMenuCount: ::std::os::raw::c_long,
    pub pSharedMenu: *mut ::std::os::raw::c_void,
    pub bMDI: ::std::os::raw::c_short,
    pub rect: RECT
}
pub type PDWOLER = *mut DWOLER;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWOPTIONS {
    pub timer_interval: ::std::os::raw::c_long,
    pub bkinfo: fillBrushInfo,
    pub rows_per_detail: ::std::os::raw::c_long,
    pub pointer: LPTSTR,
    pub mapmode: ::std::os::raw::c_short,
    pub processing: ::std::os::raw::c_short,
    pub print: DWOPTIONS__bindgen_ty_1,
    pub lpOleClientClass: LPTSTR,
    pub lpOleClientName: LPTSTR,
    pub label: DWOPTIONS__bindgen_ty_2,
    pub iGridLines: ::std::os::raw::c_short,
    pub iRowResize: ::std::os::raw::c_short,
    pub bGridColMove: BOOL,
    pub bSelectedMouse: BOOL,
    pub bHTMLDW: BOOL,
    pub bExprGenerateCss: LPTSTR,
    pub bExprNoWrap: LPTSTR,
    pub lExprBorder: LPTSTR,
    pub lExprCellSpacing: LPTSTR,
    pub lExprCellPadding: LPTSTR,
    pub lpszStyleSheet: LPTSTR,
    pub lExprWidth: LPTSTR,
    pub bExprGenerateJavaScript: LPTSTR,
    pub bExprClientEvents: LPTSTR,
    pub bExprClientValidation: LPTSTR,
    pub bExprClientComputedFields: LPTSTR,
    pub bExprClientFormatting: LPTSTR,
    pub bExprClientScriptable: LPTSTR,
    pub bExprFirstOnPage: LPTSTR,
    pub lExprPageSize: LPTSTR,
    pub sExprSelfLink: LPTSTR,
    pub lpszSelfLinkArgs: LPTSTR,
    pub bExprEncodeSelfLinkArgs: LPTSTR,
    pub sExprObjectName: LPTSTR,
    pub sExprBrowser: LPTSTR,
    pub sExprHTMLVersion: LPTSTR,
    pub sExprResourceBase: LPTSTR,
    pub sExprCommonJSFile: LPTSTR,
    pub sExprDateJSFile: LPTSTR,
    pub sExprNumberJSFile: LPTSTR,
    pub sExprStringJSFile: LPTSTR,
    pub sExprUserJSFile: LPTSTR,
    pub bExprNetscapeLayers: LPTSTR,
    pub lExprTabIndexBase: LPTSTR,
    pub iPagingMethod: SHORT,
    pub bExprGenerateDDDWFrames: LPTSTR,
    pub sExprXHTMLBrowser: LPTSTR,
    pub sExprCSSResourceBase: LPTSTR,
    pub sExprCSSPublishPath: LPTSTR,
    pub bExprCSSSessionSpecific: LPTSTR,
    pub sExprXMLResourceBase: LPTSTR,
    pub sExprXMLPublishPath: LPTSTR,
    pub bExprXMLInline: LPTSTR,
    pub sExprXSLTResourceBase: LPTSTR,
    pub sExprXSLTPublishPath: LPTSTR,
    pub sExprJSResourceBase: LPTSTR,
    pub sExprJSPublishPath: LPTSTR,
    pub pIDWExportDataTemplateList: *mut IDW_DataTemplateList,
    pub pIDWImportDataTemplateList: *mut IDW_DataTemplateList,
    pub bImportXMLTrace: BOOL,
    pub sImportXMLTraceFile: LPTSTR,
    pub pRTEDetailText: LPTSTR,
    pub pRTEHeaderText: LPTSTR,
    pub pRTEFooterText: LPTSTR,
    pub pRTEComputes: *mut ::std::os::raw::c_void,
    pub pRTEColumns: *mut ::std::os::raw::c_void,
    pub bRTEHeaderFooter: BOOL,
    pub bRTELineBreak: BOOL,
    pub bRTEReturnsVisible: BOOL,
    pub bRTESpacesVisible: BOOL,
    pub bRTETabsVisible: BOOL,
    pub bRTEInputFieldsVisible: BOOL,
    pub bRTEPictureFrame: BOOL,
    pub bRTEInputFieldNamesVisible: BOOL,
    pub bRTEPopMenu: BOOL,
    pub bRTEToolBar: BOOL,
    pub bRTETabBar: BOOL,
    pub bRTERulerBar: BOOL,
    pub bRTEReadOnly: BOOL,
    pub iRTEUnit: ::std::os::raw::c_short,
    pub lRTEInputFieldBackColor: ::std::os::raw::c_long,
    pub lRTEBackColor: ::std::os::raw::c_long,
    pub pRTEInitialFile: LPTSTR,
    pub bHideGrayLine: BOOL,
    pub bShowBackcolorOnXp: BOOL,
    pub viewport3d: DWOPTIONS__bindgen_ty_3
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWOPTIONS__bindgen_ty_1 {
    pub szDocument: [TCHAR; 32usize],
    pub szPrinterName: [TCHAR; 64usize],
    pub orientation: ::std::os::raw::c_long,
    pub left: ::std::os::raw::c_long,
    pub right: ::std::os::raw::c_long,
    pub top: ::std::os::raw::c_long,
    pub bottom: ::std::os::raw::c_long,
    pub scolumns_width: ::std::os::raw::c_long,
    pub scolumns: ::std::os::raw::c_short,
    pub psize: ::std::os::raw::c_short,
    pub psource: ::std::os::raw::c_short,
    pub bCanUseDefaultPrinter: BOOL,
    pub bPrompt: BOOL,
    pub bPrintButtons: BOOL,
    pub bPrintPreviewButtons: BOOL,
    pub bOverridePrintJob: BOOL,
    pub bClipTextOnPrint: BOOL,
    pub bPrintCollate: BOOL,
    pub bPrintPreviewShowsOutline: BOOL,
    pub bPrintShowsBackground: BOOL,
    pub bPrintPreviewShowsBackground: BOOL
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWOPTIONS__bindgen_ty_2 {
    pub name: LPTSTR,
    pub width: ::std::os::raw::c_short,
    pub height: ::std::os::raw::c_short,
    pub rows: ::std::os::raw::c_short,
    pub rows_spacing: ::std::os::raw::c_short,
    pub columns: ::std::os::raw::c_short,
    pub columns_spacing: ::std::os::raw::c_short,
    pub bTopDown: BOOL,
    pub bSheet: BOOL,
    pub gType: DWTYPES::Type,
    pub ellipse_height: ::std::os::raw::c_short,
    pub ellipse_width: ::std::os::raw::c_short
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWOPTIONS__bindgen_ty_3 {
    pub camera_distance: ::std::os::raw::c_long,
    pub camera_elevation: ::std::os::raw::c_long,
    pub camera_rotation: ::std::os::raw::c_long,
    pub animation_duration: ::std::os::raw::c_long,
    pub animation_transitions: ::std::os::raw::c_long,
    pub current_page_on_click: ::std::os::raw::c_long,
    pub pages_width: ::std::os::raw::c_long,
    pub pages_height: ::std::os::raw::c_long,
    pub perspective: ::std::os::raw::c_long,
    pub quality: ::std::os::raw::c_long,
    pub orthogonal: ::std::os::raw::c_long,
    pub refresh_on_mouseover: ::std::os::raw::c_long,
    pub expr_pages_total_visible: LPTSTR,
    pub expr_pages_focal: LPTSTR,
    pub expr_reflection: LPTSTR,
    pub expr_rotation_x: LPTSTR,
    pub expr_rotation_y: LPTSTR,
    pub expr_rotation_z: LPTSTR,
    pub expr_scaling_width: LPTSTR,
    pub expr_scaling_height: LPTSTR,
    pub expr_translation_x: LPTSTR,
    pub expr_translation_y: LPTSTR,
    pub expr_translation_z: LPTSTR,
    pub expr_transparency: LPTSTR
}
pub type PDWOPTIONS = *mut DWOPTIONS;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union editUnion {
    pub dddw_displaycol: LPTSTR,
    pub spin_range: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct EXTCOLEDIT {
    pub name: LPTSTR,
    pub style: ::std::os::raw::c_long,
    pub values: LPTSTR,
    pub dddw_name: LPTSTR,
    pub u: editUnion,
    pub dddw_datacol: LPTSTR,
    pub edit_format: LPTSTR,
    pub style_name: [TCHAR; 31usize],
    pub edit_bFocusRect: BOOL,
    pub dddw_pwidth: ::std::os::raw::c_short,
    pub limit: ::std::os::raw::c_short,
    pub accelerator: TCHAR,
    pub type_: ::std::os::raw::c_short,
    pub dddw_lines: ::std::os::raw::c_short,
    pub inkatt: DWINKATT,
    pub inkedit: DWEDITINKEDIT,
    pub ddcal_props: DDCALENDARPROPS
}
pub type PEXTCOLEDIT = *mut EXTCOLEDIT;
pub type PPDWXEDT = *mut tagEditStyle;
pub type PB_EDITSTYLECALLBACK = ::std::option::Option<unsafe extern "C" fn(arg1: PPDWXEDT) -> BOOL>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct tagEditStyle {
    pub pbThis: LPTSTR,
    pub sa: ppbstg_anchor,
    pub subpool: pbstg_subpool,
    pub caller_id: ::std::os::raw::c_long,
    pub hWndDW: HWND,
    pub szAppName: LPTSTR,
    pub table_name: LPTSTR,
    pub column_name: LPTSTR,
    pub edit_style_info: PEXTCOLEDIT,
    pub new_edit_style_info: PEXTCOLEDIT,
    pub pInitCallBackProc: PB_EDITSTYLECALLBACK,
    pub pCallBackProc: PB_EDITSTYLECALLBACK,
    pub pExtension: *mut ::std::os::raw::c_void,
    pub pUserData: *mut ::std::os::raw::c_void,
    pub pvFormatBlock: *mut ::std::os::raw::c_void,
    pub lpObjectName: LPTSTR,
    pub workname: [TCHAR; 31usize],
    pub edtFlags: ::std::os::raw::c_uchar
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union runArgs {
    pub text: LPTSTR,
    pub number: *mut f64,
    pub dec: PSH_DEC,
    pub dt: PSH_TIME,
    pub textlist: *mut LPTSTR,
    pub numberlist: *mut *mut f64,
    pub decimallist: *mut PSH_DEC,
    pub dtlist: *mut PSH_TIME
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWRETRIEVEARG {
    pub type_: ::std::os::raw::c_uchar,
    pub listcount: ::std::os::raw::c_short,
    pub u: runArgs
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWRETRIEVEARGS {
    pub argcount: ::std::os::raw::c_short,
    pub args: [DWRETRIEVEARG; 1usize]
}
pub type PDWRETRIEVEARGS = *mut DWRETRIEVEARGS;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWBARGENT {
    pub name: [TCHAR; 129usize],
    pub type_: ::std::os::raw::c_uchar,
    pub value: [TCHAR; 100usize]
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWBTBLARG {
    pub maxCount: ::std::os::raw::c_short,
    pub argCount: ::std::os::raw::c_short,
    pub args: [DWBARGENT; 1usize]
}
pub type PDWBTBLARG = *mut DWBTBLARG;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWBNESTENT {
    pub nestExpr: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWBNESTARG {
    pub argCount: ::std::os::raw::c_short,
    pub args: [DWBNESTENT; 1usize]
}
pub type PDWBNESTARG = *mut DWBNESTARG;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOEFFECTS {
    pub transparency: LPTSTR,
    pub blur: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOTOOLTIP {
    pub enabled: LPTSTR,
    pub tip: LPTSTR,
    pub title: LPTSTR,
    pub isBubble: LPTSTR,
    pub textcolor: LPTSTR,
    pub backcolor: LPTSTR,
    pub icon: LPTSTR,
    pub maxwidth: LPTSTR,
    pub transparency: LPTSTR,
    pub delayvisible: LPTSTR,
    pub delayinitial: LPTSTR,
    pub hasclosebutton: LPTSTR,
    pub position: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWFILLBRUSH {
    pub color: LPTSTR,
    pub transparency: LPTSTR,
    pub gradientcolor: LPTSTR,
    pub gradienttransparency: LPTSTR,
    pub gradientangle: LPTSTR,
    pub brushmode: LPTSTR,
    pub gradientrepetitionmode: LPTSTR,
    pub gradientrepetitioncount: LPTSTR,
    pub gradientrepetitionlength: LPTSTR,
    pub gradientfocus: LPTSTR,
    pub gradientscale: LPTSTR,
    pub gradientspread: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOBK {
    pub mode: LPTSTR,
    pub fillBrush: DWFILLBRUSH
}
pub type PDWINFOBK = *mut DWINFOBK;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOBR {
    pub color: LPTSTR,
    pub hatch: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOFONT {
    pub height: LPTSTR,
    pub width: LPTSTR,
    pub weight: LPTSTR,
    pub italic: LPTSTR,
    pub underline: LPTSTR,
    pub strikeout: LPTSTR,
    pub charset: LPTSTR,
    pub pitch: LPTSTR,
    pub family: LPTSTR,
    pub face: LPTSTR,
    pub orientation: LPTSTR,
    pub escapement: LPTSTR
}
pub type PDWINFOFONT = *mut DWINFOFONT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOPEN {
    pub style: LPTSTR,
    pub width: LPTSTR,
    pub color: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOSORTKEY {
    pub keyExpr: LPTSTR,
    pub keySeq: ::std::os::raw::c_uchar
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOTBLSORT {
    pub keyCount: ::std::os::raw::c_short,
    pub keys: [DWINFOSORTKEY; 1usize]
}
pub type PDWINFOTBLSORT = *mut DWINFOTBLSORT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOREGION {
    pub gobs: ::std::os::raw::c_long,
    pub height: ::std::os::raw::c_long,
    pub type_: ::std::os::raw::c_short,
    pub level: ::std::os::raw::c_short,
    pub item_count: ::std::os::raw::c_short,
    pub items: *mut LPTSTR,
    pub pointer: LPTSTR,
    pub fillBrush: DWFILLBRUSH,
    pub pSort: PDWINFOTBLSORT,
    pub newpage: BOOL,
    pub resetpagecount: BOOL,
    pub bHeightAuto: BOOL,
    pub bSuppressHeader: BOOL
}
pub type PDWINFOREGION = *mut DWINFOREGION;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOBITMAP {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub name: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub effects: DWINFOEFFECTS,
    pub tooltip: DWINFOTOOLTIP,
    pub transparentcolor: LPTSTR,
    pub invert: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub border: LPTSTR,
    pub htmlLinkExpr: LPTSTR,
    pub htmlLinkTargetExpr: LPTSTR,
    pub htmlLinkArgs: LPTSTR,
    pub htmlAppendedExpr: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub bOriginalSize: BOOL,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR,
    pub bFocusRect: BOOL
}
pub type PDWINFOBITMAP = *mut DWINFOBITMAP;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOBLOB {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub tooltip: DWINFOTOOLTIP,
    pub table: LPTSTR,
    pub keyClause: LPTSTR,
    pub column: LPTSTR,
    pub template_file: LPTSTR,
    pub oleclass: LPTSTR,
    pub clientname: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub border: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub fileType: ::std::os::raw::c_long,
    pub bHeightAuto: BOOL,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR,
    pub bFocusRect: BOOL,
    pub bRichAutoSelect: BOOL,
    pub bRichDisplayOnly: BOOL,
    pub bRichVScrollBar: BOOL
}
pub type PDWINFOBLOB = *mut DWINFOBLOB;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOBUTTON {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub font: DWINFOFONT,
    pub effects: DWINFOEFFECTS,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub color: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub t: LPTSTR,
    pub accent: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub filename: LPTSTR,
    pub action: LPTSTR,
    pub vTextAlign: LPTSTR,
    pub hTextAlign: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bHeightAuto: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub bDefaultPicture: BOOL,
    pub bSuppressEventProcessing: BOOL,
    pub bEnabled: BOOL,
    pub bOriginalSize: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR
}
pub type PDWINFOBUTTON = *mut DWINFOBUTTON;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOCOL {
    pub visible: LPTSTR,
    pub username: LPTSTR,
    pub format: LPTSTR,
    pub font: DWINFOFONT,
    pub effects: DWINFOEFFECTS,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub color: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub alignment: LPTSTR,
    pub accent: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub htmlLinkExpr: LPTSTR,
    pub htmlLinkTargetExpr: LPTSTR,
    pub htmlLinkArgs: LPTSTR,
    pub htmlValueIs: LPTSTR,
    pub htmlAppendedExpr: LPTSTR,
    pub editstyle: ::std::os::raw::c_long,
    pub editname: LPTSTR,
    pub dddw_name: LPTSTR,
    pub u: editUnion,
    pub dddw_datacol: LPTSTR,
    pub edit_format: LPTSTR,
    pub protect: LPTSTR,
    pub row_in_detail: ::std::os::raw::c_long,
    pub dddw_pwidth: ::std::os::raw::c_short,
    pub editlimit: ::std::os::raw::c_short,
    pub ci: ::std::os::raw::c_short,
    pub tabseq: ::std::os::raw::c_short,
    pub dddw_lines: ::std::os::raw::c_short,
    pub bitmapname: BOOL,
    pub edit_bfocusrect: BOOL,
    pub accelerator: TCHAR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bHeightAuto: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub colFlags: ::std::os::raw::c_uchar,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub inkatt: DWINKATT,
    pub inkedit: DWEDITINKEDIT,
    pub ddcal_props: DDCALENDARPROPS,
    pub sWidthAuto: ::std::os::raw::c_short
}
pub type PDWINFOCOL = *mut DWINFOCOL;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOTCOL {
    pub username: LPTSTR,
    pub dbname: LPTSTR,
    pub pattern: LPTSTR,
    pub validation_msg: LPTSTR,
    pub values: LPTSTR,
    pub compute: LPTSTR,
    pub initial: LPTSTR,
    pub length: ::std::os::raw::c_long,
    pub ci: ::std::os::raw::c_short,
    pub type_: ::std::os::raw::c_short,
    pub key: BOOL,
    pub update: BOOL,
    pub bCriteriaDialog: BOOL,
    pub bIdentity: BOOL,
    pub updatewhereclause: BOOL,
    pub decimals: ::std::os::raw::c_uchar,
    pub bBinaryHex: BOOL
}
pub type PDWINFOTCOL = *mut DWINFOTCOL;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOCOMPUTE {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub font: DWINFOFONT,
    pub effects: DWINFOEFFECTS,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub color: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub dispsyn: LPTSTR,
    pub exprsyn: LPTSTR,
    pub alignment: LPTSTR,
    pub accent: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub htmlLinkExpr: LPTSTR,
    pub htmlLinkTargetExpr: LPTSTR,
    pub htmlLinkArgs: LPTSTR,
    pub htmlValueIs: LPTSTR,
    pub htmlAppendedExpr: LPTSTR,
    pub iDataType: ::std::os::raw::c_short,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bHeightAuto: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub bCrosstabRepeat: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR,
    pub bFocusRect: BOOL,
    pub sWidthAuto: ::std::os::raw::c_short
}
pub type PDWINFOCOMPUTE = *mut DWINFOCOMPUTE;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOCROSSTAB {
    pub columns: LPTSTR,
    pub rows: LPTSTR,
    pub values: LPTSTR,
    pub source_names: *mut ::std::os::raw::c_void,
    pub RowCount: ::std::os::raw::c_long,
    pub ctFlags: ::std::os::raw::c_uchar,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long
}
pub type PDWINFOCROSSTAB = *mut DWINFOCROSSTAB;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct COLEXPR {
    pub bMatch: BOOL,
    pub oldLevel: ::std::os::raw::c_short,
    pub newLevel: ::std::os::raw::c_short
}
pub type PCOLEXPR = *mut COLEXPR;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CROSSMODEL {
    pub prog: *mut ::std::os::raw::c_void,
    pub numOldRows: ::std::os::raw::c_short,
    pub numOldCols: ::std::os::raw::c_short,
    pub numOldVals: ::std::os::raw::c_short,
    pub bColChanged: BOOL,
    pub bValChanged: BOOL,
    pub bRowChanged: BOOL,
    pub pColExpr: PCOLEXPR,
    pub newColumns: *mut ::std::os::raw::c_void,
    pub newRows: *mut ::std::os::raw::c_void,
    pub newValues: *mut ::std::os::raw::c_void
}
pub type PCROSSMODEL = *mut CROSSMODEL;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOGRAPHDISP {
    pub font: DWINFOFONT,
    pub bkcolor: LPTSTR,
    pub textcolor: LPTSTR,
    pub align: LPTSTR,
    pub format: LPTSTR,
    pub autosize: LPTSTR,
    pub dsplexpr: LPTSTR
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOGRAPHAXIS {
    pub autoscale: LPTSTR,
    pub dsplnlabels: LPTSTR,
    pub droplines: LPTSTR,
    pub frame: LPTSTR,
    pub label: LPTSTR,
    pub majordiv: LPTSTR,
    pub majorgrid: LPTSTR,
    pub majortic: LPTSTR,
    pub maxval: LPTSTR,
    pub minval: LPTSTR,
    pub maxvaldatetime: LPTSTR,
    pub minvaldatetime: LPTSTR,
    pub minordiv: LPTSTR,
    pub minorgrid: LPTSTR,
    pub minortic: LPTSTR,
    pub originline: LPTSTR,
    pub primaryline: LPTSTR,
    pub roundto: LPTSTR,
    pub roundtounit: LPTSTR,
    pub scaletype: LPTSTR,
    pub scalevalue: LPTSTR,
    pub secondaryline: LPTSTR,
    pub shadebackedge: LPTSTR,
    pub dispattr: DWINFOGRAPHDISP,
    pub labeldispattr: DWINFOGRAPHDISP
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOGRAPH {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub grtype: LPTSTR,
    pub new3drendering: LPTSTR,
    pub color: LPTSTR,
    pub backcolor: LPTSTR,
    pub shadecolor: LPTSTR,
    pub perspective: LPTSTR,
    pub rotation: LPTSTR,
    pub title: LPTSTR,
    pub tooltip: DWINFOTOOLTIP,
    pub titledisp: DWINFOGRAPHDISP,
    pub piedisp: DWINFOGRAPHDISP,
    pub ax_series: DWINFOGRAPHAXIS,
    pub ax_category: DWINFOGRAPHAXIS,
    pub ax_values: DWINFOGRAPHAXIS,
    pub series: LPTSTR,
    pub seriessort: LPTSTR,
    pub category: LPTSTR,
    pub categorysort: LPTSTR,
    pub values: LPTSTR,
    pub spacing: LPTSTR,
    pub depth: LPTSTR,
    pub elevation: LPTSTR,
    pub overlappercent: LPTSTR,
    pub border: LPTSTR,
    pub legendType: LPTSTR,
    pub plotnulldata: LPTSTR,
    pub legenddisp: DWINFOGRAPHDISP,
    pub range: ::std::os::raw::c_short,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSizeToDisplay: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub iSeriesType: ::std::os::raw::c_short,
    pub iCategoryType: ::std::os::raw::c_short,
    pub iValuesType: ::std::os::raw::c_short,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR,
    pub bFocusRect: BOOL
}
pub type PDWINFOGRAPH = *mut DWINFOGRAPH;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOGROUPBOX {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub font: DWINFOFONT,
    pub effects: DWINFOEFFECTS,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub color: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub t: LPTSTR,
    pub accent: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bHeightAuto: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub bRTOL: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long
}
pub type PDWINFOGROUPBOX = *mut DWINFOGROUPBOX;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOLINE {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub pen: DWINFOPEN,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub x1: LPTSTR,
    pub y1: LPTSTR,
    pub x2: LPTSTR,
    pub y2: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL
}
pub type PDWINFOLINE = *mut DWINFOLINE;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOOLE {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub tooltip: DWINFOTOOLTIP,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub border: LPTSTR,
    pub target: LPTSTR,
    pub summary: LPTSTR,
    pub pBinary: *mut ::std::os::raw::c_void,
    pub clientname: LPTSTR,
    pub pOleExtra: *mut ::std::os::raw::c_void,
    pub type_: ::std::os::raw::c_short,
    pub activation: INT,
    pub displaytype: INT,
    pub contentsallowed: INT,
    pub linkupdateoptions: INT,
    pub row: INT,
    pub isdragtarget: BOOL,
    pub bOleInsert: BOOL,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub bSizeToDisplay: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR,
    pub bFocusRect: BOOL
}
pub type PDWINFOOLE = *mut DWINFOOLE;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOINKPIC {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub tooltip: DWINFOTOOLTIP,
    pub table: LPTSTR,
    pub keyClause: LPTSTR,
    pub column: LPTSTR,
    pub bkcolumnName: LPTSTR,
    pub inkpic_autoerase: BOOL,
    pub inkpic_bkcolor: ::std::os::raw::c_long,
    pub inkpic_collectionmode: ::std::os::raw::c_short,
    pub inkpic_dynrender: BOOL,
    pub inkpic_editmode: ::std::os::raw::c_short,
    pub inkpic_enabled: BOOL,
    pub inkpic_erasermode: ::std::os::raw::c_short,
    pub inkpic_eraserwidth: ::std::os::raw::c_long,
    pub inkpic_highcontrastink: BOOL,
    pub inkpic_inkenabled: BOOL,
    pub inkpic_marginx: ::std::os::raw::c_long,
    pub inkpic_marginy: ::std::os::raw::c_long,
    pub inkpic_pointer: LPTSTR,
    pub inkpic_picsizemode: ::std::os::raw::c_short,
    pub ink_antialiased: BOOL,
    pub ink_ignorepressure: BOOL,
    pub ink_inkcolor: ::std::os::raw::c_long,
    pub ink_inkheight: ::std::os::raw::c_long,
    pub ink_inkwidth: ::std::os::raw::c_long,
    pub ink_pentip: ::std::os::raw::c_short,
    pub ink_transparency: ::std::os::raw::c_short,
    pub ink_fittocurve: LPTSTR,
    pub ink_rasteroperation: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub border: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub row_in_detail: ::std::os::raw::c_short
}
pub type PDWINFOINKPIC = *mut DWINFOINKPIC;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOOVAL {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub pen: DWINFOPEN,
    pub brush: DWINFOBR,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL
}
pub type PDWINFOOVAL = *mut DWINFOOVAL;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFORECT {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub pen: DWINFOPEN,
    pub brush: DWINFOBR,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL
}
pub type PDWINFORECT = *mut DWINFORECT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFORRECT {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub pen: DWINFOPEN,
    pub brush: DWINFOBR,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub ellipse_width: LPTSTR,
    pub ellipse_height: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHeightAuto: BOOL,
    pub bHideSnake: BOOL
}
pub type PDWINFORRECT = *mut DWINFORRECT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOREPORT {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub tooltip: DWINFOTOOLTIP,
    pub criteria: LPTSTR,
    pub dataobject: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub border: LPTSTR,
    pub nestArgs: PDWBNESTARG,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHeightAuto: BOOL,
    pub bHideSnake: BOOL,
    pub bNewPage: BOOL,
    pub bShowBackground: BOOL,
    pub bTrailFooter: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR,
    pub bFocusRect: BOOL
}
pub type PDWINFOREPORT = *mut DWINFOREPORT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct infoCrossData {
    pub numLevels: ::std::os::raw::c_short,
    pub filler: ::std::os::raw::c_short,
    pub pData: [LPTSTR; 1usize]
}
pub type pinfoCrossData = *mut infoCrossData;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct webServiceMetadataStrings {
    pub m_assembly: LPTSTR,
    pub m_namespace: LPTSTR,
    pub m_classname: LPTSTR,
    pub m_method: LPTSTR,
    pub m_methoddefinition: LPTSTR,
    pub m_resultset: LPTSTR,
    pub m_structurename: LPTSTR,
    pub m_inputparms: LPTSTR,
    pub m_WSDL: LPTSTR,
    pub m_resultsetparm: ::std::os::raw::c_long
}
pub type PDWWSMETA = *mut webServiceMetadataStrings;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOUPDATEARG {
    pub spArgName: LPTSTR,
    pub spColName: LPTSTR,
    pub spExpression: LPTSTR,
    pub ordinal: ::std::os::raw::c_short,
    pub spColId: ::std::os::raw::c_short,
    pub spMatch: ::std::os::raw::c_short,
    pub spInOut: ::std::os::raw::c_short,
    pub bOriginalValue: BOOL
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOUPDATE {
    pub type_: ::std::os::raw::c_short,
    pub spName: LPTSTR,
    pub ws_MetaData: PDWWSMETA,
    pub argCount: ::std::os::raw::c_short,
    pub filler: ::std::os::raw::c_short,
    pub spArg: [DWINFOUPDATEARG; 1usize]
}
pub type PDWINFOUPDATE = *mut DWINFOUPDATE;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOTABLE {
    pub sort: PDWINFOTBLSORT,
    pub rargs: PDWBTBLARG,
    pub updttbl: LPTSTR,
    pub exprSyn: LPTSTR,
    pub select: LPTSTR,
    pub nl: LPTSTR,
    pub savedata: LPTSTR,
    pub numCols: ::std::os::raw::c_short,
    pub type_: ::std::os::raw::c_short,
    pub update_where: ::std::os::raw::c_short,
    pub update_keyinplace: BOOL,
    pub updateable: BOOL,
    pub bRetrieveAsreq: BOOL,
    pub bRetrieveMem: BOOL,
    pub pCrossData: pinfoCrossData,
    pub pUpdateInsert: PDWINFOUPDATE,
    pub pUpdateDelete: PDWINFOUPDATE,
    pub pUpdateUpdate: PDWINFOUPDATE,
    pub m_assembly: LPTSTR,
    pub m_namespace: LPTSTR,
    pub m_classname: LPTSTR,
    pub m_directory: LPTSTR,
    pub m_method: LPTSTR,
    pub m_methoddefinition: LPTSTR,
    pub m_structurename: LPTSTR,
    pub m_inputparms: LPTSTR,
    pub m_WSDL: LPTSTR,
    pub m_resultsetparm: INT
}
pub type PDWINFOTABLE = *mut DWINFOTABLE;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOTEXT {
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub font: DWINFOFONT,
    pub effects: DWINFOEFFECTS,
    pub bk: DWINFOBK,
    pub tooltip: DWINFOTOOLTIP,
    pub color: LPTSTR,
    pub align: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub t: LPTSTR,
    pub accent: LPTSTR,
    pub tag: LPTSTR,
    pub pointer: LPTSTR,
    pub htmlLinkExpr: LPTSTR,
    pub htmlLinkTargetExpr: LPTSTR,
    pub htmlLinkArgs: LPTSTR,
    pub htmlValueIs: LPTSTR,
    pub htmlAppendedExpr: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bHeightAuto: BOOL,
    pub bSlideLeft: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR,
    pub bFocusRect: BOOL
}
pub type PDWINFOTEXT = *mut DWINFOTEXT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFOWPF {
    pub tooltip: DWINFOTOOLTIP,
    pub visible: LPTSTR,
    pub obj_name: LPTSTR,
    pub x: LPTSTR,
    pub y: LPTSTR,
    pub width: LPTSTR,
    pub height: LPTSTR,
    pub xaml: LPTSTR,
    pub contentProperty: LPTSTR,
    pub itemsProperty: LPTSTR,
    pub table: LPTSTR,
    pub keyClause: LPTSTR,
    pub column: LPTSTR,
    pub AccessibleDescription: LPTSTR,
    pub AccessibleName: LPTSTR,
    pub AccessibleRole: ::std::os::raw::c_long,
    pub pointer: LPTSTR,
    pub bResizeable: BOOL,
    pub bMoveable: BOOL,
    pub bSlideUpAllAbove: BOOL,
    pub bSlideUpDirectlyAbove: BOOL,
    pub bHideSnake: BOOL,
    pub ci: ::std::os::raw::c_short,
    pub tabseq: ::std::os::raw::c_short,
    pub enabled: LPTSTR,
    pub bFocusRect: BOOL
}
pub type PDWINFOWPF = *mut DWINFOWPF;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct gentreeLevelStruct {
    pub ExpandTreeNodeIconName: LPTSTR,
    pub CollapseTreeNodeIconName: LPTSTR,
    pub ExpandTreeNodeIconTransColor: ::std::os::raw::c_long,
    pub CollapseTreeNodeIconTransColor: ::std::os::raw::c_long,
    pub pNext: *mut gentreeLevelStruct,
    pub pPrev: *mut gentreeLevelStruct,
    pub ExpandTreeNodeIconName_expr: LPTSTR,
    pub CollapseTreeNodeIconName_expr: LPTSTR,
    pub ExpandTreeNodeIconTransColor_expr: LPTSTR,
    pub CollapseTreeNodeIconTransColor_expr: LPTSTR
}
pub type pgen_treeLevel = *mut gentreeLevelStruct;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct gen_treeInfo {
    pub MaxLevel: ::std::os::raw::c_long,
    pub bShowLines: BOOL,
    pub bConnectLeafNodes: BOOL,
    pub Indent: ::std::os::raw::c_long,
    pub bShowTreeNodeIcon: BOOL,
    pub TreeLevels: pgen_treeLevel,
    pub pLastLevel: pgen_treeLevel,
    pub lDefaultExpandToLevel: ::std::os::raw::c_long,
    pub StateIconAlignMode: SHORT,
    pub bSelectNodeByMouse: BOOL,
    pub bRtoLLayout: BOOL
}
pub type pgen_treeInfo = *mut gen_treeInfo;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWINFO {
    pub prog: *mut ::std::os::raw::c_void,
    pub sa: ppbstg_anchor,
    pub subpool: pbstg_subpool,
    pub stglist: *mut ::std::os::raw::c_void,
    pub rgnlist: *mut ::std::os::raw::c_void,
    pub currentGob: ::std::os::raw::c_long,
    pub curCol: ::std::os::raw::c_short,
    pub pTree: pgen_treeInfo
}
pub type PDWINFO = *mut DWINFO;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWEXPRCOL {
    pub coltype: ::std::os::raw::c_uchar,
    pub bColUsed: ::std::os::raw::c_uchar,
    pub colname: LPTSTR,
    pub colid: ::std::os::raw::c_long,
    pub collen: ::std::os::raw::c_long
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWEXPRTEST {
    pub sa: ppbstg_anchor,
    pub subpool: pbstg_subpool,
    pub obthis: *mut ::std::os::raw::c_void,
    pub syntax: LPTSTR,
    pub collen: ::std::os::raw::c_long,
    pub numCols: ::std::os::raw::c_short,
    pub numLevels: ::std::os::raw::c_short,
    pub coltype: ::std::os::raw::c_uchar,
    pub names: *mut ::std::os::raw::c_void,
    pub pGobRunHash: *mut ::std::os::raw::c_void,
    pub pGobPaintHash: *mut ::std::os::raw::c_void,
    pub pArgRun: *mut ::std::os::raw::c_void,
    pub pArgPaint: *mut ::std::os::raw::c_void,
    pub errmsg: [TCHAR; 256usize],
    pub pWebUserFunctionCallBack: *mut ::std::os::raw::c_void,
    pub cols: [DWEXPRCOL; 1usize]
}
pub type PDWEXPRTEST = *mut DWEXPRTEST;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWCOLEXT {
    pub pExtInfo: *mut ::std::os::raw::c_void,
    pub bBold: BOOL
}
pub type PDWCOLEXT = *mut DWCOLEXT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWDATAEXT {
    pub prog: *mut ::std::os::raw::c_void,
    pub pModel: PCROSSMODEL,
    pub hdrFont: PDWINFOFONT,
    pub hdrBk: PDWINFOBK,
    pub hdrColor: LPTSTR,
    pub hdrBorder: LPTSTR,
    pub hdrHeight: ::std::os::raw::c_long,
    pub dataFont: PDWINFOFONT,
    pub dataBk: PDWINFOBK,
    pub dataColor: LPTSTR,
    pub dataHeight: ::std::os::raw::c_long,
    pub dataBorder: LPTSTR
}
pub type PDWDATAEXT = *mut DWDATAEXT;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWDATACOL {
    pub name: [TCHAR; 512usize],
    pub alias: [TCHAR; 512usize],
    pub dwname: [TCHAR; 512usize],
    pub coltype: ::std::os::raw::c_uchar,
    pub collen: ::std::os::raw::c_long,
    pub lpHeader: LPTSTR,
    pub decimals: ::std::os::raw::c_short,
    pub width: ::std::os::raw::c_long,
    pub expression: LPTSTR,
    pub pColExt: PDWCOLEXT,
    pub key: BOOL
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DWDATATABLE {
    pub pDataExt: PDWDATAEXT,
    pub numCols: ::std::os::raw::c_short,
    pub m_assembly: LPTSTR,
    pub m_namespace: LPTSTR,
    pub m_classname: LPTSTR,
    pub m_method: LPTSTR,
    pub m_methoddefinition: LPTSTR,
    pub m_structurename: LPTSTR,
    pub m_inputparms: LPTSTR,
    pub m_WSDL: LPTSTR,
    pub m_resultsetparm: INT,
    pub cols: [DWDATACOL; 1usize]
}
pub type PDWDATATABLE = *mut DWDATATABLE;
pub type PDBI_COMMAND = *mut DBI_Command;
pub type DBI_FUNC =
    ::std::option::Option<unsafe extern "C" fn(arg1: PDBI_COMMAND) -> *mut ::std::os::raw::c_void>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPB_DBI_Connection {
    _unused: [u8; 0]
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPB_DBI_TableSchema {
    _unused: [u8; 0]
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DBI_PrimKy {
    pub cName: [TCHAR; 129usize],
    pub iNumberOfColumns: INT,
    pub iColumns: [INT; 32usize],
    pub lpParentTable: *mut DBI_Table,
    pub szCreator: [TCHAR; 129usize],
    pub bClustered: ::std::os::raw::c_uchar,
    pub filler: [TCHAR; 31usize]
}
pub type PDBI_PRIMARY_KEY = *mut DBI_PrimKy;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DBI_ForeignKyOptions {
    pub wOptionCount: WORD,
    pub lpTitle: LPTSTR,
    pub lpOptions: LPTSTR,
    pub DefOptionNumber: WORD
}
pub type PDBI_FOREIGNKYOPTIONS = *mut DBI_ForeignKyOptions;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DBI_Table {
    pub cTableName: [TCHAR; 129usize],
    pub ObjId: ::std::os::raw::c_long,
    pub lpszOwner: LPTSTR,
    pub iOwnerId: INT,
    pub lpszComments: LPTSTR,
    pub iNumberOfColumns: INT,
    pub iNumberOfKeys: INT,
    pub fTableFont: [LOGFONT; 3usize],
    pub pTableColumnList: *mut shlist,
    pub pTableIndexList: *mut shlist,
    pub lpszOldTableName: LPTSTR,
    pub pTableInfo: *mut ::std::os::raw::c_void,
    pub pPrimaryKy: PDBI_PRIMARY_KEY,
    pub pForeignKyList: *mut shlist,
    pub filler2: [TCHAR; 130usize],
    pub bView: BOOL,
    pub bSystem: BOOL,
    pub bUniqueKeyConstraint: BOOL,
    pub bHaveDict: BOOL,
    pub bAlias: BOOL,
    pub bTempTable: BOOL,
    pub bProxy: BOOL,
    pub bCISProxy: BOOL,
    pub lpUserData1: *mut ::std::os::raw::c_void,
    pub lpszQualifier: LPTSTR,
    pub lpszWebMethod: LPTSTR,
    pub lpszBaseObject: LPTSTR,
    pub lTimeout: ::std::os::raw::c_long,
    pub pTriggerList: *mut shlist,
    pub filler: [TCHAR; 32usize]
}
pub type DBI_TBLPTR = *mut DBI_Table;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Col_ExtractSeq {
    pub lpszColumnName: LPTSTR,
    pub szTableName: [TCHAR; 129usize],
    pub szOwnerName: [TCHAR; 129usize],
    pub bAsc: BOOL,
    pub szAliasName: [TCHAR; 129usize],
    pub bComputed: BOOL,
    pub szDBName: [TCHAR; 129usize],
    pub filler: [TCHAR; 31usize]
}
pub type PCOL_EXTRACT_SEQ = *mut Col_ExtractSeq;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DBI_Ident {
    pub szColumnName: [TCHAR; 129usize],
    pub szTableName: [TCHAR; 129usize],
    pub szOwnerName: [TCHAR; 129usize],
    pub lpIdentPtr: LPTSTR,
    pub bTable: BOOL,
    pub bCompute: BOOL,
    pub szDBName: [TCHAR; 129usize]
}
pub type PDBI_IDENT = *mut DBI_Ident;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DBI_Proc {
    pub cProcName: [TCHAR; 256usize],
    pub ObjId: ::std::os::raw::c_long,
    pub lpszOwner: LPTSTR,
    pub iOwnerId: INT,
    pub iNumber: INT,
    pub bSystem: BOOL,
    pub iReturnValuePBType: UINT,
    pub bHasReturnValue: BOOL,
    pub lpszalias: LPTSTR,
    pub iDbId: INT,
    pub bTrigger: BOOL,
    pub lpParentTable: *mut DBI_Table,
    pub pText: LPTSTR,
    pub bArray: BOOL,
    pub filler: [TCHAR; 36usize]
}
pub type DBI_PROCPTR = *mut DBI_Proc;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DBI_Event {
    pub cEventName: [TCHAR; 256usize],
    pub ObjId: ::std::os::raw::c_long,
    pub lpszOwner: LPTSTR,
    pub cEnabled: TCHAR,
    pub cLocation: TCHAR,
    pub cEventType: [TCHAR; 30usize],
    pub pAction: LPTSTR,
    pub pExtAction: LPTSTR,
    pub pCondition: LPTSTR,
    pub pRemarks: LPTSTR,
    pub pGenSyntax: LPTSTR,
    pub pScheduleList: *mut shlist,
    pub filler: [TCHAR; 8usize]
}
pub type DBI_EVENTPTR = *mut DBI_Event;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DBI_Outer {
    pub lpFirstJoin: LPTSTR,
    pub lpSecondJoin: LPTSTR,
    pub lpOperator: LPTSTR,
    pub lpOuterJoin: LPTSTR,
    pub lpBuffer: LPTSTR,
    pub lBufferSize: ::std::os::raw::c_long,
    pub lpFirstTable: LPTSTR,
    pub lpSecondTable: LPTSTR,
    pub lpFirstAlias: LPTSTR,
    pub lpSecondAlias: LPTSTR,
    pub iVersionNumber: ::std::os::raw::c_int,
    pub filler: [TCHAR; 54usize]
}
pub type PDBI_OUTER = *mut DBI_Outer;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct CacheEntry {
    pub lpszSqlStatement: LPTSTR,
    pub pDescribeList: *mut shlist,
    pub pInputBindList: *mut shlist,
    pub lpCursor: LPVOID,
    pub cHits: ULONG,
    pub lParam1: ULONG,
    pub wParam1: WPARAM,
    pub filler: [TCHAR; 2usize]
}
pub type pCacheEntry = *mut CacheEntry;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CacheList {
    pub pEntries: *mut shlist,
    pub cEntryLimit: ULONG,
    pub cHits: ULONG,
    pub cMisses: ULONG,
    pub filler: [TCHAR; 4usize]
}
pub type pCacheList = *mut CacheList;
pub mod DBI_TRANSEVTTYPE {
    pub type Type = ::std::os::raw::c_int;
    pub const DBI_TRANSEVT_DBNOTIF: Type = 0;
    pub const DBI_TRANSEVT_DBERROR: Type = 1;
    pub const DBI_TRANSEVT_SQLPREV: Type = 2;
}
pub type DBIPSCallback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut DBI_Command,
        arg2: *mut ::std::os::raw::c_void,
        arg3: DBI_TRANSEVTTYPE::Type
    ) -> ::std::os::raw::c_long
>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
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
pub struct DBI_ParmBlk {
    pub lpKey: LPTSTR,
    pub lpValue: LPTSTR,
    pub lValue: ::std::os::raw::c_long,
    pub filler: [TCHAR; 32usize]
}
pub type PDBI_PARMBLK = *mut DBI_ParmBlk;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DBI_Signon {
    pub pOldCommandBlk: PDBI_COMMAND,
    pub lpszDbms: LPTSTR,
    pub lpszServerName: LPTSTR,
    pub lpszLogId: LPTSTR,
    pub lpszLogPassword: LPTSTR,
    pub lpszDatabaseName: LPTSTR,
    pub lpszUserId: LPTSTR,
    pub lpszUserPassword: LPTSTR,
    pub lpszAppName: LPTSTR,
    pub nConnectStep: ::std::os::raw::c_short,
    pub hParentWnd: HWND,
    pub lpszLock: LPTSTR,
    pub lpszDbParm: LPTSTR,
    pub stgThis: ppbstg_anchor,
    pub bReadOnly: ::std::os::raw::c_uchar,
    pub bNoPBCatalog: ::std::os::raw::c_uchar,
    pub filler: [TCHAR; 63usize]
}
pub type PDBI_SIGNON = *mut DBI_Signon;
pub type DBI_DLGPROC =
    ::std::option::Option<unsafe extern "C" fn(arg1: HWND, arg2: UINT, arg3: WPARAM, arg4: LPARAM) -> BOOL>;
pub mod DBI_PROCFILTER {
    pub type Type = ::std::os::raw::c_int;
    pub const DBI_PROCINFO_ALL: Type = 0;
    pub const DBI_PROCINFO_RESULTSET: Type = 1;
    pub const DBI_PROCINFO_NO_RESULTSET: Type = 2;
}
pub type EXPLODESTAR_FUNC = ::std::option::Option<
    unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: PCOL_EXTRACT_SEQ, arg3: *mut shlist, arg4: *mut shlist)
>;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rtdb_statement {
    pub bBindSelectBuffer: SHORT,
    pub risc_padding: SHORT,
    pub obcrOutputInfoBuffer: OB_CONST_REF,
    pub obcrDeclaration: OB_CONST_REF,
    pub obcrName: OB_CONST_REF,
    pub obcrOffsetArray: OB_CONST_REF,
    pub obcrStagingArea: OB_CONST_REF,
    pub obcrSyntax: OB_CONST_REF,
    pub obcrTransaction: OB_CONST_REF,
    pub obcrCursprocVarPcode: OB_CONST_REF,
    pub iFetchDirection: USHORT,
    pub risc_padding2: SHORT,
    pub iNumberOfInputItems: USHORT,
    pub risc_padding3: SHORT,
    pub iNumberOfOutputItems: USHORT,
    pub risc_padding4: SHORT,
    pub timeStamp: TIME_T,
    pub pGroup: POB_GROUP
}
pub type PRTDB_STATEMENT = *mut rtdb_statement;
pub mod ob_timer_kind {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_TIMER_NONE: Type = 1;
    pub const OB_TIMER_CLOCK: Type = 2;
    pub const OB_TIMER_PROCESS: Type = 3;
    pub const OB_TIMER_THREAD: Type = 4;
}
pub use self::ob_timer_kind::Type as OB_TIMERKIND;
pub mod ob_trace_id {
    pub type Type = ::std::os::raw::c_int;
    pub const OB_TRACEID_ROUTINE: Type = 1;
    pub const OB_TRACEID_LINE: Type = 2;
    pub const OB_TRACEID_PCODE: Type = 3;
    pub const OB_TRACEID_ESQL: Type = 4;
    pub const OB_TRACEID_OBJECT_CREATE: Type = 5;
    pub const OB_TRACEID_OBJECT_DESTROY: Type = 6;
    pub const OB_TRACEID_USER: Type = 7;
    pub const OB_TRACEID_ERROR: Type = 8;
    pub const OB_TRACEID_BEGIN: Type = 9;
    pub const OB_TRACEID_GC: Type = 10;
    pub const OB_TRACEID_LAST: Type = 11;
}
pub use self::ob_trace_id::Type as OB_TRACEID;
pub mod ob_error_return {
    pub type Type = ::std::os::raw::c_int;
    pub const ERR_RET_Success: Type = 1;
    pub const ERR_RET_TraceStartedError: Type = 2;
    pub const ERR_RET_TraceNotStartedError: Type = 3;
    pub const ERR_RET_FileCloseError: Type = 4;
    pub const ERR_RET_FileOpenError: Type = 5;
    pub const ERR_RET_FileReadError: Type = 6;
    pub const ERR_RET_FileWriteError: Type = 7;
    pub const ERR_RET_FileNotOpenError: Type = 8;
    pub const ERR_RET_FileAlreadyOpenError: Type = 9;
    pub const ERR_RET_NoMoreNodes: Type = 10;
    pub const ERR_RET_FileInvalidFormatError: Type = 11;
    pub const ERR_RET_ModelNotExistsError: Type = 12;
    pub const ERR_RET_ModelExistsError: Type = 13;
    pub const ERR_RET_GeneralError: Type = 14;
    pub const ERR_RET_FileNotSetError: Type = 15;
    pub const ERR_RET_EventNotExistError: Type = 16;
    pub const ERR_RET_EventWrongPrototypeError: Type = 17;
    pub const ERR_RET_FeatureNotSupportedError: Type = 18;
    pub const ERR_RET_SharedObjectNotExistsError: Type = 19;
    pub const ERR_RET_SharedObjectExistsError: Type = 20;
    pub const ERR_RET_MutexCreateError: Type = 21;
    pub const ERR_RET_SharedObjectCreateInstanceError: Type = 22;
    pub const ERR_RET_SharedObjectCreatePBSessionError: Type = 23;
    pub const ERR_RET_EnterpriseOnlyFeature: Type = 24;
    pub const ERR_RET_SourcePBLNotFound: Type = 25;
}
pub use self::ob_error_return::Type as OB_ERROR_RETURN;
extern crate libloading;
pub struct Api {
    __library: ::libloading::Library,
    pub pbstg_begin: unsafe extern "C" fn(buffer: USHORT) -> ppbstg_anchor,
    pub pbstg_begin_allocflags: unsafe extern "C" fn(buffer: USHORT, lAllocFlags: UINT) -> ppbstg_anchor,
    pub pbstg_begin_nofast: unsafe extern "C" fn(buffer: USHORT) -> ppbstg_anchor,
    pub pbstg_end: unsafe extern "C" fn(pthis: ppbstg_anchor),
    pub pbstg_free_pool: unsafe extern "C" fn(pthis: ppbstg_anchor, subPool: pbstg_subpool),
    pub pbstg_new_pool: unsafe extern "C" fn(pthis: ppbstg_anchor) -> pbstg_subpool,
    pub pbstg_new_pool_nofast: unsafe extern "C" fn(pthis: ppbstg_anchor) -> pbstg_subpool,
    pub pbstg_new_pool_with_size:
        unsafe extern "C" fn(pthis: ppbstg_anchor, page_size: USHORT) -> pbstg_subpool,
    pub pbstg_new_pool_with_size_nofast:
        unsafe extern "C" fn(pthis: ppbstg_anchor, page_size: USHORT) -> pbstg_subpool,
    pub pbstg_set_pool_name:
        unsafe extern "C" fn(pthis: ppbstg_anchor, subPool: pbstg_subpool, lpstrName: LPTSTR),
    pub pbstg_set_poolpagesize: unsafe extern "C" fn(pthis: ppbstg_anchor, pagesize: ULONG) -> BOOL,
    pub pbstg_write_debug: unsafe extern "C" fn(
        pthis: ppbstg_anchor,
        subpool: pbstg_subpool,
        lpFile: LPTSTR
    ) -> ::std::os::raw::c_short,
    pub pbstg_stat: unsafe extern "C" fn(pthis: ppbstg_anchor, stat: *mut pbstg_statistics),
    pub pbstg_shrink: unsafe extern "C" fn(pThis: ppbstg_anchor),
    pub pbstg_nextGeneration: unsafe extern "C" fn() -> ::std::os::raw::c_long,
    pub pbstg_dumpLeaks: unsafe extern "C" fn(generation: ::std::os::raw::c_long),
    pub pbstg_dumpHeap: unsafe extern "C" fn(),
    pub pbstg_alloc: unsafe extern "C" fn(
        pthis: ppbstg_anchor,
        iNumberOfBytes: ULONG,
        subPool: pbstg_subpool
    ) -> *mut ::std::os::raw::c_void,
    pub pbstg_free: unsafe extern "C" fn(pthis: ppbstg_anchor, stg: *mut ::std::os::raw::c_void),
    pub pbstg_realloc: unsafe extern "C" fn(
        pthis: ppbstg_anchor,
        pOldStorage: *mut ::std::os::raw::c_void,
        iLength: ULONG,
        subPool: pbstg_subpool
    ) -> *mut ::std::os::raw::c_void,
    pub pbstg_size: unsafe extern "C" fn(pthis: ppbstg_anchor, pStg: *mut ::std::os::raw::c_void) -> ULONG,
    pub pbstg_fast_strlen: unsafe extern "C" fn(s: LPTSTR) -> ULONG,
    pub pbstg_ansitoupper: unsafe extern "C" fn(c: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    pub pbstg_ansitolower: unsafe extern "C" fn(c: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    pub pbstg_strdup:
        unsafe extern "C" fn(pthis: ppbstg_anchor, string: LPCTSTR, subpool: pbstg_subpool) -> LPTSTR,
    pub pbstg_strdup_malloc: unsafe extern "C" fn(lpstrString: LPTSTR) -> LPTSTR,
    pub pbstg_str_build: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: LPTSTR
    ),
    pub pbstg_str_build_char: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        c: TCHAR
    ),
    pub pbstg_str_build_huge: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPTSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: *mut TCHAR
    ),
    pub pbstg_str_remove_char: unsafe extern "C" fn(string: LPTSTR, c: TCHAR) -> LPTSTR,
    pub pbstg_str_trim_left: unsafe extern "C" fn(string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR,
    pub pbstg_str_trim_right: unsafe extern "C" fn(string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR,
    pub pbstg_str_trim: unsafe extern "C" fn(string: LPTSTR, IncludeAllSpaceTypes: BOOL) -> LPTSTR,
    pub pbstg_str_wordcap: unsafe extern "C" fn(s: LPTSTR) -> LPTSTR,
    pub pbstg_atoi_imp: unsafe extern "C" fn(arg1: LPTSTR) -> INT,
    pub pbstg_atof_imp: unsafe extern "C" fn(arg1: LPTSTR) -> f64,
    pub pbstg_strtod_imp: unsafe extern "C" fn(pText: LPTSTR, endptr: *mut LPTSTR) -> f64,
    pub pbstg_atol_imp: unsafe extern "C" fn(arg1: LPTSTR) -> ::std::os::raw::c_long,
    pub pbstg_strtol_imp: unsafe extern "C" fn(
        arg1: LPTSTR,
        arg2: *mut LPTSTR,
        arg3: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub pbstg_atou_imp: unsafe extern "C" fn(arg1: LPTSTR) -> UINT,
    pub pbstg_atoul_imp: unsafe extern "C" fn(arg1: LPTSTR) -> ULONG,
    pub pbstg_strtoul_imp:
        unsafe extern "C" fn(arg1: LPTSTR, arg2: *mut LPTSTR, arg3: ::std::os::raw::c_int) -> ULONG,
    pub pbstg_remove_imp: unsafe extern "C" fn(arg1: LPTSTR) -> INT,
    pub pbstg_dde_alloc:
        unsafe extern "C" fn(iNumberOfBytes: ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_void,
    pub pbstg_dde_free: unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
    pub pbstg_dde_get_handle: unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> GLOBALHANDLE,
    pub pbstg_dde_lock: unsafe extern "C" fn(arg1: GLOBALHANDLE) -> *mut ::std::os::raw::c_void,
    pub pbstg_dde_unlock: unsafe extern "C" fn(arg1: GLOBALHANDLE),
    pub pbstg_dde_strdup: unsafe extern "C" fn(string: LPTSTR) -> LPTSTR,
    pub pbstg_huge_memcmp: unsafe extern "C" fn(
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> ::std::os::raw::c_short,
    pub pbstg_huge_memcpy: unsafe extern "C" fn(
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void,
    pub pbstg_huge_memmove: unsafe extern "C" fn(
        v1: *mut ::std::os::raw::c_void,
        v2: *mut ::std::os::raw::c_void,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void,
    pub pbstg_huge_memset: unsafe extern "C" fn(
        v1: *mut ::std::os::raw::c_void,
        c: ::std::os::raw::c_short,
        count: ULONG
    ) -> *mut ::std::os::raw::c_void,
    pub pbstg_huge_strchr: unsafe extern "C" fn(s: *mut TCHAR, c: TCHAR) -> *mut TCHAR,
    pub pbstg_huge_strcpy: unsafe extern "C" fn(s: *mut TCHAR, s2: *mut TCHAR) -> *mut TCHAR,
    pub pbstg_huge_strlen: unsafe extern "C" fn(s: *mut TCHAR) -> ULONG,
    pub pbstg_huge_strncpy: unsafe extern "C" fn(s: *mut TCHAR, s2: *mut TCHAR, count: ULONG) -> *mut TCHAR,
    pub pbstg_huge_strstr: unsafe extern "C" fn(s: *mut TCHAR, s2: *mut TCHAR) -> *mut TCHAR,
    pub OS_UtilGetProfInt: unsafe extern "C" fn(lpSection: LPCTSTR, lpKey: LPCTSTR, nDefault: INT) -> INT,
    pub OS_UtilGetProfStr: unsafe extern "C" fn(
        lpSection: LPCTSTR,
        lpKey: LPCTSTR,
        lpDefault: LPCTSTR,
        lpReturnedStr: LPTSTR,
        nSize: INT
    ) -> INT,
    pub OS_UtilPutProfStr: unsafe extern "C" fn(pSection: LPCTSTR, pKey: LPCTSTR, pValue: LPCTSTR) -> INT,
    pub OS_NetworkGetProfInt: unsafe extern "C" fn(lpSection: LPTSTR, lpKey: LPTSTR, nDefault: INT) -> INT,
    pub OS_NetworkGetProfStr: unsafe extern "C" fn(
        lpSection: LPTSTR,
        lpKey: LPTSTR,
        lpDefault: LPTSTR,
        lpReturnedStr: LPTSTR,
        nSize: INT
    ) -> INT,
    pub OS_NetworkPutProfStr: unsafe extern "C" fn(pSection: LPTSTR, pKey: LPTSTR, pValue: LPTSTR) -> INT,
    pub OS_NetworkSharedProfileAccessible: unsafe extern "C" fn() -> INT,
    pub OS_UtilGetInitPath: unsafe extern "C" fn(pBuff: LPTSTR, nBuffLen: INT, pName: LPTSTR) -> LPTSTR,
    pub OS_UtilPutInitPath: unsafe extern "C" fn(pBuff: LPTSTR) -> BOOL,
    pub UtilIniFile:
        unsafe extern "C" fn(lpszFileNameBuf: LPTSTR, cbFileNameBufSize: WORD, bCheck: BOOL) -> LPTSTR,
    pub FDCC_Get_Registry_Entry:
        unsafe extern "C" fn(pEntryname: LPTSTR, pValue: LPTSTR, nBuffLen: INT, pToolName: LPTSTR) -> DWORD,
    pub FDCC_Get_CSIDL_Path: unsafe extern "C" fn(
        CSIDL_Value: INT,
        pPath: LPTSTR,
        nBuffLen: INT,
        bWithSuffix: BOOL,
        bCreate: BOOL
    ) -> DWORD,
    pub FDCC_Create_Registry_Entry: unsafe extern "C" fn(pEntryName: LPTSTR, pValue: LPTSTR) -> BOOL,
    pub FDCC_Get_Install_Location: unsafe extern "C" fn(pValue: LPTSTR, nBuffLen: INT, iSuffix: INT) -> DWORD,
    pub FDCC_New_INI_Installed: unsafe extern "C" fn(pNewIniFileName: LPTSTR) -> BOOL,
    pub pbstg_unicodestrdup:
        unsafe extern "C" fn(sa: ppbstg_anchor, pwsz: LPCWSTR, subpool: pbstg_subpool) -> LPWSTR,
    pub pbstg_unicodestr_build: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        syn: *mut LPWSTR,
        synLen: *mut ::std::os::raw::c_long,
        synOff: *mut ::std::os::raw::c_long,
        string: LPCWSTR
    ),
    pub pbstg_strtounicodedup:
        unsafe extern "C" fn(pthis: ppbstg_anchor, psz: LPCTSTR, subpool: pbstg_subpool) -> LPWSTR,
    pub pbstg_unicodetostrdup:
        unsafe extern "C" fn(pthis: ppbstg_anchor, pwsz: LPCWSTR, subpool: pbstg_subpool) -> LPTSTR,
    pub pbstg_strtoansidup:
        unsafe extern "C" fn(pthis: ppbstg_anchor, psz: LPCTSTR, subpool: pbstg_subpool) -> LPSTR,
    pub pbstg_ansitostrdup:
        unsafe extern "C" fn(pthis: ppbstg_anchor, pasz: LPCSTR, subpool: pbstg_subpool) -> LPTSTR,
    pub pbstg_strtoprintable: unsafe extern "C" fn(dest: LPSTR, source: LPCTSTR) -> LPSTR,
    pub pbstg_strtoprintabledup:
        unsafe extern "C" fn(pthis: ppbstg_anchor, psz: LPCTSTR, subpool: pbstg_subpool) -> LPSTR,
    pub pbstg_printabletostr: unsafe extern "C" fn(dest: LPTSTR, source: LPCSTR) -> LPTSTR,
    pub pbstg_printabletostrdup:
        unsafe extern "C" fn(pthis: ppbstg_anchor, pasz: LPCSTR, subpool: pbstg_subpool) -> LPTSTR,
    pub PBUNI_memchr: unsafe extern "C" fn(lpszString: LPTSTR, cToFind: TCHAR, ulLength: ULONG) -> LPTSTR,
    pub pbstg_lchrcmp: unsafe extern "C" fn(c1: TCHAR, c2: TCHAR) -> INT,
    pub pbstg_lchrcmpi: unsafe extern "C" fn(c1: TCHAR, c2: TCHAR) -> INT,
    pub os_vabuild_start:
        unsafe extern "C" fn(stgthis: ppbstg_anchor, pStackFrame: pos_stackgrowblock, subpool: pbstg_subpool),
    pub os_vabuild_ptrarg: unsafe extern "C" fn(
        stgthis: ppbstg_anchor,
        pStackFrame: pos_stackgrowblock,
        argument: *mut ::std::os::raw::c_void
    ),
    pub os_vabuild_end: unsafe extern "C" fn(
        stgthis: ppbstg_anchor,
        pStackFrame: pos_stackgrowblock,
        frame_len: *mut UINT
    ) -> *mut ::std::os::raw::c_void,
    pub os_vabuild_add: unsafe extern "C" fn(
        stgthis: ppbstg_anchor,
        pStackFrame: pos_stackgrowblock,
        size: UINT,
        arg: *mut ::std::os::raw::c_void
    ),
    pub sh_dbg_console_init: unsafe extern "C" fn(),
    pub sh_dbg_console_out: unsafe extern "C" fn(string: LPTSTR),
    pub sh_dbg_console_lock: unsafe extern "C" fn(),
    pub sh_dbg_console_unlock: unsafe extern "C" fn(),
    pub sh_dbg_init: unsafe extern "C" fn(arg1: ppbstg_anchor) -> *mut SH_DBG_THIS,
    pub sh_dbg_this: unsafe extern "C" fn() -> *mut SH_DBG_THIS,
    pub sh_dbg_term: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS),
    pub sh_dbg_read_input: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, dbg_infile: LPTSTR) -> INT,
    pub sh_dbg_outfile: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, filename: LPTSTR),
    pub sh_dbg_open: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS) -> INT,
    pub sh_dbg_close: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS) -> INT,
    pub sh_dbg_set: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, dbg_code: INT) -> INT,
    pub sh_dbg_del: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, dbg_code: INT) -> INT,
    pub sh_dbg_header: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, header_state: INT),
    pub sh_dbg_indent: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, indent_state: INT),
    pub sh_dbg_set_this: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS),
    pub sh_dbg_out: unsafe extern "C" fn(code: INT, format_str: LPTSTR, ...) -> INT,
    pub sh_dbg_start_indent: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS),
    pub sh_dbg_end_indent: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS),
    pub sh_dbg_enter: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, dbg_code: INT, string: LPTSTR) -> INT,
    pub sh_dbg_leave: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, dbg_code: INT, string: LPTSTR) -> INT,
    pub sh_dbg_on: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS),
    pub sh_dbg_off: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS),
    pub sh_dbg_query: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, dbg_code: INT) -> INT,
    pub sh_dbg_is_hdr_on: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS) -> INT,
    pub sh_dbg_is_indent_on: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS) -> INT,
    pub osAssert: unsafe extern "C" fn(arg1: LPTSTR, arg2: LPTSTR, arg3: ::std::os::raw::c_int),
    pub shlist_delete: unsafe extern "C" fn(self_: *mut shlist),
    pub shlist_deleteFree: unsafe extern "C" fn(self_: *mut shlist) -> *mut ::std::os::raw::c_void,
    pub shlist_get_next: unsafe extern "C" fn(self_: *mut shlist) -> *mut ::std::os::raw::c_void,
    pub shlist_get_prev: unsafe extern "C" fn(self_: *mut shlist) -> *mut ::std::os::raw::c_void,
    pub shlist_putafter: unsafe extern "C" fn(self_: *mut shlist, node: *mut shlnode) -> INT,
    pub shlist_addafter: unsafe extern "C" fn(self_: *mut shlist, data: *mut ::std::os::raw::c_void) -> INT,
    pub shlist_addbefore: unsafe extern "C" fn(self_: *mut shlist, data: *mut ::std::os::raw::c_void) -> INT,
    pub shlist_remove: unsafe extern "C" fn(self_: *mut shlist) -> *mut ::std::os::raw::c_void,
    pub shlist_insert_at_curr:
        unsafe extern "C" fn(self_: *mut shlist, userdata: *mut ::std::os::raw::c_void) -> INT,
    pub shlist_insert: unsafe extern "C" fn(self_: *mut shlist, userdata: *mut ::std::os::raw::c_void) -> INT,
    pub shlist_new: unsafe extern "C" fn(sa: ppbstg_anchor, subpool: pbstg_subpool) -> *mut shlist,
    pub shlist_curr_node: unsafe extern "C" fn(self_: *mut shlist) -> *mut shlnode,
    pub shlist_get_count: unsafe extern "C" fn(self_: *mut shlist) -> UINT,
    pub shlist_get_first: unsafe extern "C" fn(self_: *mut shlist) -> *mut ::std::os::raw::c_void,
    pub shlist_get_last: unsafe extern "C" fn(self_: *mut shlist) -> *mut ::std::os::raw::c_void,
    pub shlist_get_curr: unsafe extern "C" fn(self_: *mut shlist) -> *mut ::std::os::raw::c_void,
    pub shlist_update: unsafe extern "C" fn(
        self_: *mut shlist,
        newdata: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub shlist_get_handle: unsafe extern "C" fn(self_: *mut shlist) -> SH_LIST_HANDLE,
    pub shlist_set_current: unsafe extern "C" fn(self_: *mut shlist, handle: SH_LIST_HANDLE),
    pub shlist_traversal: unsafe extern "C" fn(
        self_: *mut shlist,
        data: *mut ::std::os::raw::c_void,
        trav_func: PASCALFAR_INT_PROC
    ) -> INT,
    pub shlist_sort: unsafe extern "C" fn(
        self_: *mut shlist,
        compare_func: PASCALFAR_INT_PROC,
        options: ::std::os::raw::c_int
    ) -> *mut LPTSTR,
    pub shlist_sort_param: unsafe extern "C" fn(
        self_: *mut shlist,
        lpData: *mut ::std::os::raw::c_void,
        compare_func: PASCALFAR_INT_PROC,
        options: ::std::os::raw::c_int
    ) -> *mut LPTSTR,
    pub sh_grwblk_init: unsafe extern "C" fn(grwblk: PSH_GROWBLOCK, incr: UINT, str_size: UINT),
    pub sh_new_grwblk: unsafe extern "C" fn(
        stgthis: ppbstg_anchor,
        increment: UINT,
        struct_size: UINT,
        subpool: pbstg_subpool
    ) -> PSH_GROWBLOCK,
    pub sh_set_grwblk_item: unsafe extern "C" fn(
        stgthis: ppbstg_anchor,
        grwblk: PSH_GROWBLOCK,
        pos: UINT,
        item: *mut ::std::os::raw::c_void,
        subpool: pbstg_subpool
    ),
    pub sh_add_to_grwblk: unsafe extern "C" fn(
        stgthis: ppbstg_anchor,
        grwblk: PSH_GROWBLOCK,
        item: *mut ::std::os::raw::c_void,
        subpool: pbstg_subpool
    ) -> UINT,
    pub sh_append_to_grwblk: unsafe extern "C" fn(
        stgthis: ppbstg_anchor,
        grwblk: PSH_GROWBLOCK,
        item_array: *mut ::std::os::raw::c_void,
        array_len: UINT,
        subpool: pbstg_subpool
    ),
    pub sh_grwblk_delete: unsafe extern "C" fn(stgthis: ppbstg_anchor, grwblk: PSH_GROWBLOCK),
    pub sh_grwblk_close: unsafe extern "C" fn(
        stgthis: ppbstg_anchor,
        grwblk: PSH_GROWBLOCK,
        no_items: *mut UINT
    ) -> *mut ::std::os::raw::c_void,
    pub ob_set_session_icontext:
        unsafe extern "C" fn(obthis: POB_THIS, pNewContext: *mut ::std::os::raw::c_void),
    pub ob_set_main_obthis: unsafe extern "C" fn(obthis: POB_THIS) -> BOOL,
    pub rt_move_thread: unsafe extern "C" fn(rtthis: POB_THIS) -> BOOL,
    pub rt_clear_thread: unsafe extern "C" fn(),
    pub rt_get_current_this: unsafe extern "C" fn() -> POB_THIS,
    pub rt_set_current_this: unsafe extern "C" fn(arg1: POB_THIS) -> BOOL,
    pub rt_add_task: unsafe extern "C" fn(rtthis: POB_THIS) -> BOOL,
    pub rt_free_task: unsafe extern "C" fn() -> BOOL,
    pub rt_get_current_task_info: unsafe extern "C" fn(info_pos: INT) -> *mut ::std::os::raw::c_void,
    pub rt_set_current_task_info:
        unsafe extern "C" fn(info_pos: INT, user_info: *mut ::std::os::raw::c_void) -> BOOL,
    pub rt_get_free_task_slot: unsafe extern "C" fn() -> INT,
    pub rt_is_running_exe: unsafe extern "C" fn() -> BOOL,
    pub shhash_new: unsafe extern "C" fn(
        arg1: INT,
        arg2: KEY_FUNC,
        arg3: BOOL,
        arg4: BOOL,
        arg5: ppbstg_anchor,
        arg6: pbstg_subpool
    ) -> *mut shhash,
    pub shhash_new_arg: unsafe extern "C" fn(
        arg1: INT,
        arg2: KEY_FUNC_ARG,
        arg3: BOOL,
        arg4: BOOL,
        arg5: ppbstg_anchor,
        arg6: pbstg_subpool,
        arg7: *mut ::std::os::raw::c_void
    ) -> *mut shhash,
    pub shhash_delete: unsafe extern "C" fn(arg1: *mut shhash),
    pub shhash_clear: unsafe extern "C" fn(arg1: *mut shhash),
    pub shhash_get_first: unsafe extern "C" fn(arg1: *mut shhash) -> *mut ::std::os::raw::c_void,
    pub shhash_get_next: unsafe extern "C" fn(arg1: *mut shhash) -> *mut ::std::os::raw::c_void,
    pub shhash_insert:
        unsafe extern "C" fn(arg1: *mut shhash, arg2: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    pub shhash_search: unsafe extern "C" fn(
        arg1: *mut shhash,
        arg2: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub shhash_search_arg: unsafe extern "C" fn(
        arg1: *mut shhash,
        arg2: *mut ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub shhash_search_unique: unsafe extern "C" fn(
        pThis: *mut shhash,
        key: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub shhash_search_unique_arg: unsafe extern "C" fn(
        pThis: *mut shhash,
        key: *mut ::std::os::raw::c_void,
        arg: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub shhash_searchNext: unsafe extern "C" fn(
        arg1: *mut shhash,
        arg2: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub shhash_searchSlot:
        unsafe extern "C" fn(arg1: *mut shhash, arg2: *mut ::std::os::raw::c_void) -> *mut shlist,
    pub shhash_remove: unsafe extern "C" fn(arg1: *mut shhash) -> *mut ::std::os::raw::c_void,
    pub shhash_statistics: unsafe extern "C" fn(arg1: *mut shhash, arg2: PHASHSTAT) -> PHASHSTAT,
    pub shhash_traversal: unsafe extern "C" fn(
        pthis: *mut shhash,
        data: *mut ::std::os::raw::c_void,
        trav_func: PASCALFAR_INT_PROC
    ) -> BOOL,
    pub obAllocDebug: unsafe extern "C" fn(
        obthis: POB_THIS,
        addr: *mut ::std::os::raw::c_void,
        len: ULONG,
        subpool: OB_SUBPOOL
    ) -> *mut ::std::os::raw::c_void,
    pub obFreeDebug: unsafe extern "C" fn(obthis: POB_THIS, addr: *mut ::std::os::raw::c_void),
    pub ob_add_const_data: unsafe extern "C" fn(
        obthis: POB_THIS,
        conpool: POB_CONPOOL,
        data: *mut ::std::os::raw::c_void,
        item_type: OB_CONPOOL_ITEM_TYPE::Type,
        nitems: UINT,
        len: ULONG
    ) -> OB_CONST_REF,
    pub parse_stringvalue_hash: unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> LPTSTR,
    pub obconpool_stringvalue_del:
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void) -> BOOL,
    pub ob_looksym_keyfunc: unsafe extern "C" fn(
        pDataNode: *mut ::std::os::raw::c_void,
        tobthis: *mut ::std::os::raw::c_void
    ) -> LPTSTR,
    pub ob_looksym_reference:
        unsafe extern "C" fn(obthis: POB_THIS, look_symtab: POB_LOOK_SYMTAB, name: LPTSTR) -> OB_SYM_ID,
    pub ob_looksym_delete: unsafe extern "C" fn(obthis: POB_THIS, look_symtab: POB_LOOK_SYMTAB, slot: UINT),
    pub ob_dynarray_index: unsafe extern "C" fn(
        obthis: POB_THIS,
        theArray: *mut tag_OB_DYNARRAY,
        index: ULONG,
        extend: BOOL
    ) -> *mut ::std::os::raw::c_void,
    pub ob_dynarray_grow: unsafe extern "C" fn(
        obthis: POB_THIS,
        theArray: *mut tag_OB_DYNARRAY,
        limit: ULONG,
        initialize: BOOL
    ),
    pub ob_narray_create_static: unsafe extern "C" fn(
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
    pub ob_narray_create_dynamic: unsafe extern "C" fn(
        obthis: POB_THIS,
        subpool: OB_SUBPOOL,
        elmtType: OB_CLASS_HNDL,
        elmtSize: USHORT,
        userData: USHORT,
        useNulls: BOOL,
        freeData: BOOL,
        initFn: PNARRAY_INIT_FN
    ) -> *mut tag_OB_NARRAY,
    pub ob_narray_dynamic_item_init_callback: unsafe extern "C" fn(
        obthis: POB_THIS,
        dynArray: *mut tag_OB_DYNARRAY,
        item: *mut ::std::os::raw::c_void
    ) -> BOOL,
    pub ob_set_arraydef: unsafe extern "C" fn(
        obthis: POB_THIS,
        arraydef: POB_ARRAYDEF,
        no_dims: UINT,
        arr_style: OB_ARRAY_SYMBOL_STYLE::Type,
        bounds: *mut ::std::os::raw::c_long
    ),
    pub ob_get_array_len: unsafe extern "C" fn(obthis: POB_THIS, arraydef: POB_ARRAYDEF) -> ULONG,
    pub ob_array_item_init_callback: unsafe extern "C" fn(
        obthis: POB_THIS,
        theArray: *mut tag_OB_NARRAY,
        theItem: *mut ::std::os::raw::c_void
    ) -> BOOL,
    pub ob_init_array: unsafe extern "C" fn(
        obthis: POB_THIS,
        arrdef: POB_ARRAYDEF,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        init_data: BOOL
    ) -> POB_ARRAY_INST,
    pub ob_array_varinfo_nullval: unsafe extern "C" fn(obthis: POB_THIS, array_inst: POB_ARRAY_INST) -> BOOL,
    pub ob_array_set_varinfo_nullval:
        unsafe extern "C" fn(obthis: POB_THIS, array_inst: POB_ARRAY_INST, bNull: BOOL),
    pub ob_remove_array_data:
        unsafe extern "C" fn(obthis: POB_THIS, array_inst: POB_ARRAY_INST, IsNullVarInfor: BOOL),
    pub ob_init_pcode_blk: unsafe extern "C" fn(
        obthis: POB_THIS,
        no_items: UINT,
        no_line_incr: UINT,
        subpool: OB_SUBPOOL
    ) -> POB_PCODE_BLK,
    pub ob_del_pcode_blk: unsafe extern "C" fn(obthis: POB_THIS, pcode_blk: POB_PCODE_BLK),
    pub ob_reuse_routine: unsafe extern "C" fn(
        obthis: POB_THIS,
        routlist: POB_ROUTLIST,
        rout_id: OB_ROUT_ID,
        proto_id: OB_PROTO_ID,
        subpool: OB_SUBPOOL,
        clear_pcode: BOOL
    ),
    pub sh_MAX_DEC: unsafe extern "C" fn() -> *const SH_DEC,
    pub shMaxDec: unsafe extern "C" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    pub shMinDec: unsafe extern "C" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    pub shCompareDec: unsafe extern "C" fn(src1: PSH_DEC, src2: PSH_DEC) -> SHORT,
    pub shAbsDec: unsafe extern "C" fn(dst: PSH_DEC, src: PSH_DEC) -> PSH_DEC,
    pub shNegateDec: unsafe extern "C" fn(dst: PSH_DEC, src: PSH_DEC) -> PSH_DEC,
    pub shRoundDec: unsafe extern "C" fn(dst: PSH_DEC, src: PSH_DEC, n: SHORT) -> PSH_DEC,
    pub shTruncDec: unsafe extern "C" fn(dst: PSH_DEC, src: PSH_DEC, n: SHORT) -> PSH_DEC,
    pub shAddDec: unsafe extern "C" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    pub shSubDec: unsafe extern "C" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    pub shMultDec: unsafe extern "C" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    pub shDivDec: unsafe extern "C" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    pub shModDec: unsafe extern "C" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    pub shExpDec: unsafe extern "C" fn(dst: PSH_DEC, src1: PSH_DEC, src2: PSH_DEC) -> PSH_DEC,
    pub shIntToDec: unsafe extern "C" fn(dst: PSH_DEC, src: SHORT) -> PSH_DEC,
    pub shDecToInt: unsafe extern "C" fn(dst: *mut SHORT, src: PSH_DEC) -> *mut SHORT,
    pub shUintToDec: unsafe extern "C" fn(dst: PSH_DEC, src: USHORT) -> PSH_DEC,
    pub shDecToUint: unsafe extern "C" fn(dst: *mut USHORT, src: PSH_DEC) -> *mut USHORT,
    pub shByteToDec: unsafe extern "C" fn(dst: PSH_DEC, src: ::std::os::raw::c_uchar) -> PSH_DEC,
    pub shDecToByte:
        unsafe extern "C" fn(dst: *mut ::std::os::raw::c_uchar, src: PSH_DEC) -> *mut ::std::os::raw::c_uchar,
    pub shLongToDec: unsafe extern "C" fn(dst: PSH_DEC, src: ::std::os::raw::c_long) -> PSH_DEC,
    pub shDecToLong:
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_long, src: PSH_DEC) -> *mut ::std::os::raw::c_long,
    pub shUlongToDec: unsafe extern "C" fn(dst: PSH_DEC, src: ULONG) -> PSH_DEC,
    pub shDecToUlong: unsafe extern "C" fn(dst: *mut ULONG, src: PSH_DEC) -> *mut ULONG,
    pub shLonglongToDec: unsafe extern "C" fn(dst: PSH_DEC, src: *mut ::std::os::raw::c_longlong) -> PSH_DEC,
    pub shDecToLonglong: unsafe extern "C" fn(
        dst: *mut ::std::os::raw::c_longlong,
        src: PSH_DEC
    ) -> *mut ::std::os::raw::c_longlong,
    pub shDecToFloat: unsafe extern "C" fn(dst: *mut f32, src: PSH_DEC) -> *mut f32,
    pub shFloatToDec: unsafe extern "C" fn(dst: PSH_DEC, src: *mut f32) -> PSH_DEC,
    pub shDoubleToDec: unsafe extern "C" fn(dst: PSH_DEC, src: *mut f64) -> PSH_DEC,
    pub shDecToDouble: unsafe extern "C" fn(dst: *mut f64, src: PSH_DEC) -> *mut f64,
    pub shDecToAscii: unsafe extern "C" fn(dst: LPTSTR, src: PSH_DEC) -> LPTSTR,
    pub shAsciiToDec: unsafe extern "C" fn(dst: PSH_DEC, src: LPTSTR) -> PSH_DEC,
    pub shAsciiToDecRnd: unsafe extern "C" fn(dst: PSH_DEC, src: LPTSTR, n: SHORT) -> PSH_DEC,
    pub shSetDecFractions: unsafe extern "C" fn(d: PSH_DEC, n: SHORT),
    pub shSetDecNegative: unsafe extern "C" fn(d: PSH_DEC, n: BOOL),
    pub shDecSetOverflow: unsafe extern "C" fn(dec: PSH_DEC, neg: BOOL) -> BOOL,
    pub ConvertOldDecToString: unsafe extern "C" fn(s: LPTSTR, pAddress: *mut OLD_DEC) -> LPTSTR,
    pub shdtDayName: unsafe extern "C" fn(w_day: ::std::os::raw::c_short, lpName: LPTSTR) -> BOOL,
    pub shdtDayOfWeek: unsafe extern "C" fn(t: PSH_TIME) -> ::std::os::raw::c_int,
    pub shdtBuildTime: unsafe extern "C" fn(
        fTime1: PSH_TIME,
        year: INT,
        mon: ::std::os::raw::c_uchar,
        day: ::std::os::raw::c_uchar,
        hour: ::std::os::raw::c_uchar,
        min: ::std::os::raw::c_uchar,
        sec: ::std::os::raw::c_uchar,
        msec: ::std::os::raw::c_long
    ),
    pub shdtDiffDate: unsafe extern "C" fn(fTime1: PSH_TIME, fTime2: PSH_TIME) -> ::std::os::raw::c_long,
    pub shdtDiffTime: unsafe extern "C" fn(fTime1: PSH_TIME, fTime2: PSH_TIME) -> ::std::os::raw::c_long,
    pub shdtDiffMSec: unsafe extern "C" fn(fTime1: PSH_TIME, fTime2: PSH_TIME) -> ::std::os::raw::c_long,
    pub shdtNow: unsafe extern "C" fn(fTime: PSH_TIME),
    pub shdtParse: unsafe extern "C" fn(
        fTime: PSH_TIME,
        sTime: LPTSTR,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    pub shdtParseEx: unsafe extern "C" fn(
        fTime: PSH_TIME,
        sTime: LPTSTR,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int,
    pub shdtParseStringEx: unsafe extern "C" fn(
        fTime: PSH_TIME,
        sTime: LPTSTR,
        flags: ::std::os::raw::c_int,
        bStrictly: BOOL
    ) -> ::std::os::raw::c_int,
    pub shdtParseStringExWithLcid: unsafe extern "C" fn(
        fTime: PSH_TIME,
        sTime: LPTSTR,
        flags: ::std::os::raw::c_int,
        bStrictly: BOOL,
        uInLcid: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    pub shdtParseToString: unsafe extern "C" fn(fTime: PSH_TIME, sTime: LPTSTR, flags: ::std::os::raw::c_int),
    pub shdtRelativeDate:
        unsafe extern "C" fn(oDate: PSH_TIME, iDate: PSH_TIME, dayCount: ::std::os::raw::c_long),
    pub shdtToMJDDate: unsafe extern "C" fn(pDate: PSH_TIME, pMJDDate: *mut f64),
    pub shdtToMJDTime: unsafe extern "C" fn(pTime: PSH_TIME, pMJDTime: *mut f64),
    pub shdtToMJDTimestamp: unsafe extern "C" fn(pDateTime: PSH_TIME, pMJDDate: *mut f64, pMJDTime: *mut f64),
    pub shMJDDateTodt: unsafe extern "C" fn(pDate: PSH_TIME, MJDDate: f64),
    pub shMJDTimeTodt: unsafe extern "C" fn(pTime: PSH_TIME, MJDTime: f64),
    pub shMJDTimestampTodt: unsafe extern "C" fn(pDateTime: PSH_TIME, MJDDate: f64, MJDTime: f64),
    pub shdtString: unsafe extern "C" fn(sdate: LPTSTR, stime: LPTSTR),
    pub ob_mgr_init: unsafe extern "C" fn(dbgthis: *mut SH_DBG_THIS, stgthis: ppbstg_anchor) -> POB_THIS,
    pub ob_mgr_init_ex: unsafe extern "C" fn(
        dbgthis: *mut SH_DBG_THIS,
        stgthis: ppbstg_anchor,
        lpstrTypdefPblName: LPTSTR
    ) -> POB_THIS,
    pub ob_mgr_restart: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_mgr_terminate: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_free_memory: unsafe extern "C" fn(obthis: POB_THIS, data: *mut ::std::os::raw::c_void),
    pub ob_free_link_error_list: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_get_link_error_list: unsafe extern "C" fn(obthis: POB_THIS) -> *mut ::std::os::raw::c_void,
    pub ob_enter_critical_section: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_leave_critical_section: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_alloc_string: unsafe extern "C" fn(obthis: POB_THIS, len: ULONG) -> LPTSTR,
    pub ob_alloc_blob: unsafe extern "C" fn(obthis: POB_THIS, len: ULONG) -> PSH_BINARY,
    pub ob_alloc_dec: unsafe extern "C" fn(obthis: POB_THIS) -> PSH_DEC,
    pub ob_alloc_double: unsafe extern "C" fn(obthis: POB_THIS) -> *mut f64,
    pub ob_alloc_longlong: unsafe extern "C" fn(obthis: POB_THIS) -> *mut ::std::os::raw::c_longlong,
    pub ob_alloc_time: unsafe extern "C" fn(obthis: POB_THIS) -> PSH_TIME,
    pub ob_realloc_string: unsafe extern "C" fn(obthis: POB_THIS, string: LPTSTR, len: ULONG) -> LPTSTR,
    pub ob_realloc_blob: unsafe extern "C" fn(obthis: POB_THIS, blob: PSH_BINARY, len: ULONG) -> PSH_BINARY,
    pub ob_dup_string: unsafe extern "C" fn(obthis: POB_THIS, string: LPTSTR) -> LPTSTR,
    pub ob_dup_blob: unsafe extern "C" fn(obthis: POB_THIS, blob: PSH_BINARY) -> PSH_BINARY,
    pub ob_dup_dec: unsafe extern "C" fn(obthis: POB_THIS, dec_val: PSH_DEC) -> PSH_DEC,
    pub ob_dup_double: unsafe extern "C" fn(obthis: POB_THIS, double_val: *mut f64) -> *mut f64,
    pub ob_dup_longlong: unsafe extern "C" fn(
        obthis: POB_THIS,
        longlong_val: *mut ::std::os::raw::c_longlong
    ) -> *mut ::std::os::raw::c_longlong,
    pub ob_dup_time: unsafe extern "C" fn(obthis: POB_THIS, time_val: PSH_TIME) -> PSH_TIME,
    pub ob_free_value: unsafe extern "C" fn(obthis: POB_THIS, data: *mut ::std::os::raw::c_void),
    pub ob_create_appl_report: unsafe extern "C" fn(obthis: POB_THIS) -> POB_APPL_REPORT,
    pub ob_create_object_report: unsafe extern "C" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        object_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> POB_APPL_REPORT,
    pub ob_free_appl_report: unsafe extern "C" fn(obthis: POB_THIS, appl_report: POB_APPL_REPORT),
    pub ob_get_mode: unsafe extern "C" fn(obthis: POB_THIS) -> OB_MODE,
    pub ob_set_mode: unsafe extern "C" fn(obthis: POB_THIS, mode: OB_MODE) -> OB_MODE,
    pub ob_get_field: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        pData: POB_DATA
    ) -> POB_DATA,
    pub ob_set_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, pData: POB_DATA),
    pub ob_get_field_data:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> POB_DATA,
    pub ob_get_no_fields: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG,
    pub ob_get_parent_obinst: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_INST_ID,
    pub ob_get_first_user_field: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG,
    pub ob_group_is_normalized_window: unsafe extern "C" fn(obthis: POB_THIS, obInst: OB_INST_ID) -> BOOL,
    pub ob_group_set_normalized_window:
        unsafe extern "C" fn(obthis: POB_THIS, obInst: OB_INST_ID, bValue: BOOL),
    pub ob_get_field_type:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> OB_CLASS_ID,
    pub ob_get_int_field: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> INT,
    pub ob_get_uint_field: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> UINT,
    pub ob_get_byte_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> ::std::os::raw::c_uchar,
    pub ob_get_long_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> ::std::os::raw::c_long,
    pub ob_get_ulong_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> ULONG,
    pub ob_get_float_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, fl: *mut f32) -> *mut f32,
    pub ob_get_ptr_field: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT
    ) -> *mut ::std::os::raw::c_void,
    pub ob_get_inst_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT) -> OB_INST_ID,
    pub ob_get_array_field: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        no_items: *mut UINT
    ) -> *mut ::std::os::raw::c_void,
    pub ob_array_index: unsafe extern "C" fn(
        obthis: POB_THIS,
        array_vals: *mut ::std::os::raw::c_void,
        index: ULONG,
        type_: POB_CLASS_ID
    ) -> *mut ::std::os::raw::c_void,
    pub ob_get_indirect_obdata:
        unsafe extern "C" fn(obthis: POB_THIS, obInst: OB_INST_ID, obInfo: POB_DATA_INFO) -> POB_DATA,
    pub ob_array_item: unsafe extern "C" fn(
        obthis: POB_THIS,
        array_vals: *mut ::std::os::raw::c_void,
        index: ULONG
    ) -> POB_DATA,
    pub ob_array_get_index_from_subs: unsafe extern "C" fn(
        obthis: POB_THIS,
        theArray: OB_ARRAY_ID,
        subs: *mut ::std::os::raw::c_long
    ) -> ULONG,
    pub ob_array_calc_index: unsafe extern "C" fn(
        obthis: POB_THIS,
        numDims: UINT,
        bounds: *mut ::std::os::raw::c_long,
        subs: *mut ::std::os::raw::c_long
    ) -> ULONG,
    pub ob_set_int_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, int_val: INT),
    pub ob_set_uint_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, uint_val: UINT),
    pub ob_set_long_field: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        long_val: ::std::os::raw::c_long
    ),
    pub ob_set_ulong_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, ulong_val: ULONG),
    pub ob_set_float_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, flval: f32),
    pub ob_set_ptr_field: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        ptrval: *mut ::std::os::raw::c_void
    ),
    pub ob_set_array_field: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        pArray: *mut ::std::os::raw::c_void
    ),
    pub ob_set_obinst_field:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, field_id: UINT, obinstval: OB_INST_ID),
    pub ob_set_underlying_object:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, obj: *mut ::std::os::raw::c_void),
    pub ob_get_underlying_object:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> *mut ::std::os::raw::c_void,
    pub ob_is_any_group_locked: unsafe extern "C" fn(obthis: POB_THIS) -> BOOL,
    pub ob_get_group_lock_count: unsafe extern "C" fn(obthis: POB_THIS) -> UINT,
    pub ob_is_group_locked: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> BOOL,
    pub ob_is_group_unlocked: unsafe extern "C" fn(obthis: POB_THIS, obGroupHandle: OB_GROUP_HNDL) -> BOOL,
    pub ob_is_group_write_locked: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> BOOL,
    pub ob_lock_group:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, write_only: BOOL) -> INT,
    pub ob_unlock_group: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT,
    pub ob_clear_unlocked_groups: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_clear_all_other_unlocked_groups: unsafe extern "C" fn(obthis: POB_THIS, obGroupId: OB_GROUP_ID),
    pub ob_is_ancestor_locked:
        unsafe extern "C" fn(obthis: POB_THIS, groupid: OB_GROUP_ID, cReadWrite: TCHAR) -> BOOL,
    pub ob_is_descendent_locked:
        unsafe extern "C" fn(obthis: POB_THIS, groupid: OB_GROUP_ID, cReadWrite: TCHAR) -> BOOL,
    pub ob_validate_liblist:
        unsafe extern "C" fn(obThis: POB_THIS, pLibList: *mut LPTSTR, iNumberOfItems: UINT) -> INT,
    pub ob_set_liblist: unsafe extern "C" fn(
        obthis: POB_THIS,
        lib_list: *mut LPTSTR,
        no_items: UINT,
        bCreateNewLoader: BOOL
    ) -> INT,
    pub ob_add_liblist: unsafe extern "C" fn(obthis: POB_THIS, lib_list: *mut LPTSTR, no_items: UINT) -> INT,
    pub ob_get_liblist: unsafe extern "C" fn(obthis: POB_THIS, no_items: *mut UINT) -> *mut LPTSTR,
    pub ob_set_default_appl:
        unsafe extern "C" fn(obthis: POB_THIS, lib_name: LPTSTR, appl_group_name: LPTSTR),
    pub ob_load_appl_group: unsafe extern "C" fn(obthis: POB_THIS) -> BOOL,
    pub ob_is_group_in_memory:
        unsafe extern "C" fn(obthis: POB_THIS, lib_name: LPTSTR, qualified_name: LPTSTR) -> OB_GROUP_HNDL,
    pub ob_group_declare: unsafe extern "C" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> OB_GROUP_HNDL,
    pub ob_group_reference: unsafe extern "C" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> OB_GROUP_HNDL,
    pub ob_get_group_name: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> LPTSTR,
    pub ob_get_group_full_name: unsafe extern "C" fn(obthis: POB_THIS, grouphndl: OB_GROUP_HNDL) -> LPTSTR,
    pub ob_group_save: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        lib_name: LPTSTR,
        comment: LPTSTR
    ) -> INT,
    pub ob_group_save_win: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        lib_name: LPTSTR,
        comment: LPTSTR,
        bSaveNormalize: BOOL
    ) -> INT,
    pub ob_load_group_source: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT,
    pub ob_reload_group_source: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT,
    pub ob_rename_group:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, new_name: LPTSTR) -> INT,
    pub ob_move_group:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, lib_name: LPTSTR) -> INT,
    pub ob_move_group_with_name:
        unsafe extern "C" fn(obthis: POB_THIS, qual_name: LPTSTR, oldlib: LPTSTR, newlib: LPTSTR) -> INT,
    pub ob_copy_group_with_name:
        unsafe extern "C" fn(obthis: POB_THIS, qual_name: LPTSTR, oldlib: LPTSTR, newlib: LPTSTR) -> INT,
    pub ob_copy_group:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, new_name: LPTSTR, lib_name: LPTSTR),
    pub ob_delete_group: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL),
    pub ob_delete_group_with_name: unsafe extern "C" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        group_name: LPTSTR,
        class_id: OB_CLASS_ID
    ) -> INT,
    pub ob_restore_group: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL),
    pub ob_save_working_group: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL),
    pub ob_delete_working_group: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_restore_working_group: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_open_group_id: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL),
    pub ob_close_group: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_get_group_lib: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> LPTSTR,
    pub ob_run_garbage_collection: unsafe extern "C" fn(obthis: POB_THIS, force: BOOL) -> INT,
    pub ob_delete_instlist_shlist: unsafe extern "C" fn(obthis: POB_THIS, instlist: *mut shlist),
    pub ob_get_group_instlist_as_shlist:
        unsafe extern "C" fn(obthis: POB_THIS, groupId: OB_GROUP_ID) -> *mut shlist,
    pub ob_delete_groups_shlist: unsafe extern "C" fn(obthis: POB_THIS, groups: *mut shlist),
    pub ob_get_groups_shlist: unsafe extern "C" fn(obthis: POB_THIS) -> *mut shlist,
    pub ob_store_source:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, source: LPTSTR, len: UINT),
    pub ob_init_source:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, source: LPTSTR, len: UINT) -> INT,
    pub ob_store_global_src:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, source: LPTSTR, len: UINT) -> INT,
    pub ob_store_namespace_decl_src:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, source: LPTSTR, len: UINT) -> INT,
    pub ob_store_shared_src:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, source: LPTSTR, len: UINT) -> INT,
    pub ob_store_prototype_source:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, source: LPTSTR, len: UINT),
    pub ob_store_instvar_source:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, source: LPTSTR, len: UINT),
    pub ob_get_global_src:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, len: *mut UINT) -> LPTSTR,
    pub ob_get_namespace_decl_src:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, len: *mut UINT) -> LPTSTR,
    pub ob_get_shared_src:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, len: *mut UINT) -> LPTSTR,
    pub ob_get_prototype_source:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, len: *mut UINT) -> LPTSTR,
    pub ob_get_instvar_source:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, len: *mut UINT) -> LPTSTR,
    pub ob_get_routine_src:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> LPTSTR,
    pub ob_decl_and_store_routine_src: unsafe extern "C" fn(
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
    pub ob_store_routine_src: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID,
        source: LPTSTR,
        len: UINT
    ),
    pub ob_store_create_src:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, source: LPTSTR, len: UINT) -> INT,
    pub ob_store_destroy_src:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, source: LPTSTR, len: UINT) -> INT,
    pub ob_get_function_src:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> LPTSTR,
    pub ob_store_function_src: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID,
        source: LPTSTR,
        len: UINT
    ),
    pub ob_symbol_search_extended: unsafe extern "C" fn(
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
    pub ob_symbol_search: unsafe extern "C" fn(
        obthis: POB_THIS,
        obClassHandle: OB_CLASS_HNDL,
        iCurrScope: INT,
        pszVarName: LPTSTR,
        bSkipVars: BOOL,
        bSkipTHIS: BOOL,
        puiLevel: *mut UINT,
        pbIsConstantField: *mut BOOL
    ) -> BOOL,
    pub ob_class_declare: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR,
        parent_class: OB_CLASS_ID,
        within_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL,
    pub ob_get_full_qualified_typename:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, class_id: OB_CLASS_ID) -> LPTSTR,
    pub ob_class_declare_inh: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR,
        parent_class: OB_CLASS_HNDL,
        within_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL,
    pub ob_class_reference: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        class_name: LPTSTR
    ) -> OB_CLASS_HNDL,
    pub ob_class_name: unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> LPTSTR,
    pub ob_class_name_not_indirect:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> LPTSTR,
    pub ob_get_type_name: unsafe extern "C" fn(obthis: POB_THIS, datanode: POB_DATA) -> LPTSTR,
    pub ob_classhndl_indirect:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL,
    pub ob_get_parent_class:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL,
    pub ob_get_within_class:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_HNDL,
    pub ob_class_delete: unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL),
    pub ob_class_rename:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, new_name: LPTSTR) -> INT,
    pub ob_is_a_system_class: unsafe extern "C" fn(obthis: POB_THIS, class_name: LPTSTR) -> BOOL,
    pub ob_is_class_inherited: unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    pub ob_is_class_descendant:
        unsafe extern "C" fn(obthis: POB_THIS, ancestor: OB_CLASS_HNDL, descendant: OB_CLASS_HNDL) -> BOOL,
    pub ob_is_inh_from_user_class: unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    pub ob_get_sec_class_ancestor:
        unsafe extern "C" fn(obthis: POB_THIS, sec_class: OB_CLASS_HNDL) -> OB_CLASS_HNDL,
    pub obIsClassAutoinstantiate: unsafe extern "C" fn(obThis: POB_THIS, obClassHndl: OB_CLASS_HNDL) -> BOOL,
    pub ob_is_class_enum: unsafe extern "C" fn(obThis: POB_THIS, obClassHndl: OB_CLASS_HNDL) -> BOOL,
    pub ob_new_event: unsafe extern "C" fn(
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
    pub ob_update_event: unsafe extern "C" fn(
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
    pub ob_delete_event:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, event_name: LPTSTR) -> INT,
    pub ob_has_events: unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    pub ob_get_event_token_id: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_name: LPTSTR,
        vtable_id: POB_VTABLE_ID
    ) -> OB_EVT_TOKEN_ID,
    pub ob_get_event_id_from_token: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        event_token: OB_EVT_TOKEN_ID
    ) -> OB_VTABLE_ID,
    pub ob_does_event_script_exist:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, event_id: OB_VTABLE_ID) -> BOOL,
    pub ob_get_routine_name:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> LPTSTR,
    pub ob_delete_routine:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID),
    pub ob_get_curr_routine:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_VTABLE_ID,
    pub ob_get_curr_function:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_VTABLE_ID,
    pub ob_get_routid_from_vtable_id: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID
    ) -> OB_ROUT_ID,
    pub ob_is_valid_event_index:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> BOOL,
    pub ob_has_scripts: unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    pub ob_get_routine_type: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        index: OB_VTABLE_ID
    ) -> OB_ROUT_TYPE::Type,
    pub ob_get_function_vtable_ids: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        include_ancestors: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID,
    pub ob_get_function_vtable_ids_for_ide: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        include_ancestors: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID,
    pub ob_get_event_vtable_ids: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        include_dlls: BOOL,
        no_items: *mut UINT
    ) -> POB_VTABLE_ID,
    pub ob_get_function_name:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> LPTSTR,
    pub ob_delete_function:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> INT,
    pub ob_find_routine: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        routine_type: OB_ROUT_TYPE::Type,
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
    pub ob_get_vtable_id_from_proto_id: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        proto_id: OB_PROTO_ID
    ) -> OB_VTABLE_ID,
    pub ob_get_dll_func_names: unsafe extern "C" fn(obthis: POB_THIS, no_funcs: *mut UINT) -> *mut LPTSTR,
    pub ob_get_global_func_names_in_lib:
        unsafe extern "C" fn(obthis: POB_THIS, no_funcs: *mut UINT, lib_name: LPTSTR) -> *mut LPTSTR,
    pub ob_get_global_func_index:
        unsafe extern "C" fn(obthis: POB_THIS, name: LPTSTR, class_hndl: POB_CLASS_HNDL) -> OB_VTABLE_ID,
    pub ob_get_func_index_in_lib: unsafe extern "C" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        lib_name: LPTSTR,
        class_hndl: POB_CLASS_HNDL
    ) -> OB_VTABLE_ID,
    pub ob_get_proto_is_external_event:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, index: OB_VTABLE_ID) -> BOOL,
    pub ob_get_protoarg_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        no_items: *mut UINT,
        type_name: *mut LPTSTR,
        member_access: POB_MEMBER_ACCESS
    ) -> POB_ARG_INFO,
    pub ob_get_proto_info: unsafe extern "C" fn(
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
        rout_type: *mut OB_ROUT_TYPE::Type,
        is_inherit: *mut BOOL
    ) -> POB_ARG_INFO,
    pub ob_get_method_signature:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, vtable_id: OB_VTABLE_ID) -> LPTSTR,
    pub ob_was_event_prototype_changed:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, event_id: OB_VTABLE_ID) -> BOOL,
    pub ob_get_proto_name_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        func_id: OB_VTABLE_ID,
        name: *mut LPTSTR,
        is_obsolete: *mut BOOL
    ),
    pub ob_get_proto_throws_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        func_id: OB_VTABLE_ID,
        throws_list: *mut POB_CLASS_ID,
        no_throws: *mut UINT,
        group_id: POB_GROUP_ID
    ),
    pub ob_lookup_routine_by_name: unsafe extern "C" fn(
        obthis: POB_THIS,
        obInst: OB_INST_ID,
        lpstrRoutineName: LPTSTR,
        pVtableId: POB_VTABLE_ID,
        pNumRoutines: *mut UINT,
        pobRoutineType: *mut OB_ROUT_TYPE::Type,
        pNoArgs: *mut UINT,
        ppobArgClassId: *mut POB_CLASS_ID,
        pbVarArgs: *mut BOOL
    ) -> HRESULT,
    pub ob_get_objnames_of_class:
        unsafe extern "C" fn(obthis: POB_THIS, class_id: OB_CLASS_ID, no_items: *mut UINT) -> *mut LPTSTR,
    pub ob_has_object_of_class: unsafe extern "C" fn(obthis: POB_THIS, class_id: OB_CLASS_ID) -> BOOL,
    pub ob_get_obj_classhndls_of_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        obClassID: OB_CLASS_ID,
        pNumberOfItems: *mut UINT
    ) -> POB_CLASS_HNDL,
    pub ob_get_objnames_of_class_in_lib: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_id: OB_CLASS_ID,
        no_items: *mut UINT,
        lib_name: LPTSTR
    ) -> *mut LPTSTR,
    pub ob_global_reference: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_name: LPTSTR,
        group_hndl: POB_GROUP_HNDL
    ) -> OB_CLASS_HNDL,
    pub ob_global_reference_in_lib: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_name: LPTSTR,
        lib_name: LPTSTR,
        group_hndl: POB_GROUP_HNDL,
        class_id: OB_CLASS_ID
    ) -> OB_CLASS_HNDL,
    pub ob_global_reference_of_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        grouphndl: POB_GROUP_HNDL,
        of_class: OB_CLASS_ID
    ) -> OB_CLASS_HNDL,
    pub ob_get_obinst_class_hndl: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_CLASS_HNDL,
    pub ob_is_a_typedef: unsafe extern "C" fn(obthis: POB_THIS, data: POB_DATA) -> BOOL,
    pub ob_is_an_enum:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, data_node: POB_DATA) -> BOOL,
    pub ob_get_system_class: unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_CLASS_ID,
    pub ob_get_obinst_system_class: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_CLASS_ID,
    pub ob_get_obinst_group_hndl: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_GROUP_HNDL,
    pub ob_get_obinst_class_name: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> LPTSTR,
    pub ob_fetch_fields_of_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        in_class: OB_CLASS_HNDL,
        of_class: OB_CLASS_ID,
        field_filter: OB_FIELD_FILTER,
        class_list: *mut POB_CLASS_ID,
        no_items: *mut UINT
    ) -> *mut LPTSTR,
    pub ob_get_fields_of_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        in_class: OB_CLASS_HNDL,
        of_class: OB_CLASS_ID,
        class_list: *mut POB_CLASS_ID,
        no_items: *mut UINT
    ) -> *mut LPTSTR,
    pub ob_get_local_var_info: unsafe extern "C" fn(obthis: POB_THIS, no_items: *mut UINT) -> POB_DATA_INFO,
    pub ob_get_shared_vars_of_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        return_shared: BOOL,
        return_global: BOOL,
        of_class: OB_CLASS_ID,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    pub ob_get_shared_var_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        return_shared: BOOL,
        return_global: BOOL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    pub ob_get_global_vars_of_class:
        unsafe extern "C" fn(obthis: POB_THIS, of_class: OB_CLASS_ID, no_items: *mut UINT) -> POB_DATA_INFO,
    pub ob_get_class_field_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    pub ob_get_enum_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_enums: *mut UINT
    ) -> POB_ENUM_INFO,
    pub ob_get_class_event_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_events: *mut UINT
    ) -> POB_EVENT_INFO,
    pub ob_get_instance_field_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_DATA_INFO,
    pub ob_get_obinst_field_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        no_items: *mut UINT,
        field_filter: BOOL
    ) -> POB_DATA_INFO,
    pub ob_get_obinst_all_field_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        no_items: *mut UINT,
        field_filter: BOOL
    ) -> POB_DATA_INFO,
    pub ob_get_classes_within_group: unsafe extern "C" fn(
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        of_class_id: OB_CLASS_ID,
        total_items: *mut UINT
    ) -> POB_CLASS_INFO,
    pub ob_get_enums_within_group: unsafe extern "C" fn(
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        total_items: *mut UINT
    ) -> POB_CLASS_INFO,
    pub ob_get_global_var_data: unsafe extern "C" fn(obthis: POB_THIS, var_name: LPTSTR) -> POB_DATA,
    pub ob_object_reference_count: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> ULONG,
    pub ob_named_global_var_info: unsafe extern "C" fn(obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO,
    pub ob_named_shared_var_info:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, varname: LPTSTR) -> POB_DATA_INFO,
    pub ob_named_special_var_info: unsafe extern "C" fn(obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO,
    pub ob_named_local_var_info: unsafe extern "C" fn(obthis: POB_THIS, varname: LPTSTR) -> POB_DATA_INFO,
    pub ob_named_field_info:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID, fieldname: LPTSTR) -> POB_DATA_INFO,
    pub ob_get_array_info: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> POB_ARRAY_INFO,
    pub ob_get_array_bounds_string:
        unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL, data_node: POB_DATA) -> LPTSTR,
    pub ob_get_array_bounds_string_from_field_info:
        unsafe extern "C" fn(obthis: POB_THIS, fieldinfo: POB_DATA_INFO) -> LPTSTR,
    pub ob_get_info_watchpoint:
        unsafe extern "C" fn(obthis: POB_THIS, info: POB_DATA_INFO) -> *mut ::std::os::raw::c_void,
    pub ob_compile_lib_entry: unsafe extern "C" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR,
        write_source: BOOL
    ) -> INT,
    pub ob_compile_lib_typedefs: unsafe extern "C" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        entry_name: LPTSTR,
        bUpdateModifyTime: BOOL
    ) -> INT,
    pub ob_compile_lib_entry_3_pass:
        unsafe extern "C" fn(obThis: POB_THIS, lpszLibraryName: LPTSTR, lpszEntryName: LPTSTR) -> BOOL,
    pub ob_compile_lib_scripts:
        unsafe extern "C" fn(obthis: POB_THIS, lib_name: LPTSTR, entry_name: LPTSTR) -> INT,
    pub ob_func_search: unsafe extern "C" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        argtypes: POB_CLASS_ID,
        no_args: UINT,
        type_: POB_CLASS_ID,
        error: POB_PROTOREF_ERROR
    ) -> POB_FUNCCALL_INFO,
    pub ob_del_funccall_info: unsafe extern "C" fn(obthis: POB_THIS, funccall_info: POB_FUNCCALL_INFO),
    pub ob_link_project: unsafe extern "C" fn(obthis: POB_THIS, group_hndl: OB_GROUP_HNDL) -> INT,
    pub ob_get_runtime_group_hndl: unsafe extern "C" fn(obthis: POB_THIS) -> OB_GROUP_HNDL,
    pub ob_check_for_locked_menu: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_create_obinst: unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> OB_INST_ID,
    pub ob_instantiate_child_object: unsafe extern "C" fn(
        obthis: POB_THIS,
        parent_obinst: OB_INST_ID,
        child_type: OB_CLASS_ID,
        field_id: UINT,
        child_obinst: POB_INST_ID
    ) -> INT,
    pub ob_instantiate_system_object:
        unsafe extern "C" fn(obthis: POB_THIS, object_type: OB_CLASS_ID, pObint: POB_INST_ID) -> INT,
    pub ob_destroy_obinst: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> INT,
    pub ob_set_runtime: unsafe extern "C" fn(obthis: POB_THIS, bInRuntime: BOOL),
    pub ob_create_executable: unsafe extern "C" fn(
        obthis: POB_THIS,
        pExecBlock: POB_EXEC,
        bFreeGroups: BOOL,
        pManifestInfo: LPTSTR
    ) -> INT,
    pub ob_create_library: unsafe extern "C" fn(obthis: POB_THIS, pExecBlock: POB_EXEC) -> INT,
    pub ob_create_consolidated_library: unsafe extern "C" fn(
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
    pub CreatePBITypeDef: unsafe extern "C" fn(
        pName: *mut PBIString,
        pLibraryList: *mut PBIString,
        ppTypeDef: *mut *mut PBITypeDef
    ) -> HRESULT,
    pub CreatePBIClassDef: unsafe extern "C" fn(
        pName: *mut PBIString,
        pLibraryList: *mut PBIString,
        ppClassDef: *mut *mut PBIClassDef
    ) -> HRESULT,
    pub CreatePBIScriptDef: unsafe extern "C" fn(
        pName: *mut PBIString,
        pLibraryList: *mut PBIString,
        ppScriptDef: *mut *mut PBIScriptDef
    ) -> HRESULT,
    pub ob_create_interface_in_library: unsafe extern "C" fn(
        pClassArray: *mut PPBIClassDef,
        ulNumClasses: ULONG,
        lpstrDestLibrary: LPTSTR,
        lpstrLibraryComments: LPTSTR,
        bCreatePbl: BOOL
    ) -> HRESULT,
    pub ob_create_interface_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        hSourceClass: OB_CLASS_HNDL,
        lpstrDestClassName: LPTSTR,
        lpstrDestLibrary: LPTSTR,
        lpstrComments: LPTSTR,
        lpstrSourceClassName: LPTSTR
    ) -> HRESULT,
    pub ob_init_executable: unsafe extern "C" fn(obthis: POB_THIS, executable_name: LPTSTR) -> OB_CLASS_HNDL,
    pub ob_scan_source_blocks: unsafe extern "C" fn(
        obthis: POB_THIS,
        source: POB_SOURCE_BLOCK,
        src_len: ULONG,
        srcloc: *mut *mut ::std::os::raw::c_void,
        no_blocks: *mut UINT,
        subpool: OB_SUBPOOL
    ) -> *mut LPTSTR,
    pub ob_create_launcher: unsafe extern "C" fn(
        obThis: POB_THIS,
        pExecBlock: POB_EXEC,
        pObjectList: *mut ::std::os::raw::c_void
    ) -> INT,
    pub ob_sanitize_pb_name: unsafe extern "C" fn(
        obThis: POB_THIS,
        lpszDestName: LPTSTR,
        destLength: ::std::os::raw::c_long,
        lpszNameToSanitize: LPTSTR
    ),
    pub ob_validate_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_items: *mut UINT
    ) -> POB_CONFLICT_LIST,
    pub ob_get_orphaned_classes: unsafe extern "C" fn(
        obthis: POB_THIS,
        in_group_hndl: OB_GROUP_HNDL,
        total_items: *mut UINT
    ) -> POB_CLASS_HNDL,
    pub ob_validate_type_name:
        unsafe extern "C" fn(obThis: POB_THIS, obGroupHndl: OB_GROUP_HNDL, TypeName: LPTSTR) -> BOOL,
    pub ob_convert_to_ver2_source:
        unsafe extern "C" fn(obthis: POB_THIS, lib_name: LPTSTR, entry_name: LPTSTR) -> INT,
    pub ob_is_vers2_obj:
        unsafe extern "C" fn(obthis: POB_THIS, lib_name: LPTSTR, entry_name: LPTSTR, error: *mut INT) -> BOOL,
    pub ob_build_ordered_compile_list: unsafe extern "C" fn(
        obthis: POB_THIS,
        list_type: OB_COMPILE_LIST_TYPE,
        no_items: *mut UINT,
        inconsistency: POB_INCONSISTENCY_TYPE
    ) -> POB_COMPILE_LIST,
    pub ob_free_ordered_compile_list:
        unsafe extern "C" fn(obthis: POB_THIS, compile_list: POB_COMPILE_LIST, no_items: UINT),
    pub ob_build_hierarchy_list:
        unsafe extern "C" fn(obthis: POB_THIS, no_items: *mut UINT, type_: OB_CLASS_ID) -> POB_HIERARCHY_LIST,
    pub ob_free_hierarchy_list:
        unsafe extern "C" fn(obthis: POB_THIS, hierarchy_list: POB_HIERARCHY_LIST, no_items: UINT),
    pub ob_clear_instance_ref: unsafe extern "C" fn(obthis: POB_THIS, back_ptr: *mut ::std::os::raw::c_void),
    pub ob_insert_inst_ref_dbg: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        ref_addr: *mut ::std::os::raw::c_void,
        fileName: LPTSTR,
        lineNo: UINT
    ),
    pub ob_insert_local_inst_ref_dbg: unsafe extern "C" fn(
        obthis: POB_THIS,
        obinst: OB_INST_ID,
        ref_addr: *mut ::std::os::raw::c_void,
        fileName: LPTSTR,
        lineNo: UINT
    ),
    pub ob_open_typedef_group: unsafe extern "C" fn(
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszGroupName: LPTSTR,
        bCreateIfNotFound: BOOL
    ) -> INT,
    pub ob_load_pspp_dlls: unsafe extern "C" fn(obThis: POB_THIS) -> INT,
    pub ob_save_dll_to_pbd: unsafe extern "C" fn(argc: ::std::os::raw::c_int, argv: *mut LPTSTR) -> INT,
    pub ob_convert_pbx_to_native_groups:
        unsafe extern "C" fn(obthis: POB_THIS, pbl_name: LPCTSTR, dll_name: LPCTSTR) -> INT,
    pub ObPsppCreateControl: unsafe extern "C" fn(
        visualObject: *mut ::std::os::raw::c_void,
        dwExStyle: DWORD,
        lpWindowName: LPCTSTR,
        dwStyle: DWORD,
        x: INT,
        y: INT,
        nWidth: INT,
        nHeight: INT,
        hWndParent: HWND,
        hInstance: HINSTANCE
    ) -> HWND,
    pub ObPsppGetEventID: unsafe extern "C" fn(
        visualObj: *mut ::std::os::raw::c_void,
        hWnd: HWND,
        iMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> INT,
    pub DrawPsppObject: unsafe extern "C" fn(
        obthis: POB_THIS,
        hDC: HDC,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        group_id: OB_GROUP_ID,
        class_id: OB_CLASS_ID,
        objectName: LPCTSTR,
        tag: LPCTSTR,
        backColor: DWORD,
        enabled: BOOL,
        visible: BOOL,
        borderStyle: DWORD
    ),
    pub ob_share_typedef_group: unsafe extern "C" fn(destObThis: POB_THIS, srcObThis: POB_THIS) -> INT,
    pub ob_unshare_typedef_group: unsafe extern "C" fn(obThis: POB_THIS) -> INT,
    pub ob_cm_evaluate_expression:
        unsafe extern "C" fn(obthis: POB_THIS, text: LPTSTR, result_data_node: POB_DATA) -> INT,
    pub ob_entryInheritsFromClass: unsafe extern "C" fn(
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszTypeName: LPTSTR,
        lpszEntryName: LPTSTR
    ) -> BOOL,
    pub ob_get_class_from_name:
        unsafe extern "C" fn(obThis: POB_THIS, lpszClassName: LPTSTR, pbIsEnum: *mut BOOL) -> OB_CLASS_HNDL,
    pub ob_local_global_lv:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID) -> POB_DATA,
    pub ob_local_global_refpkt:
        unsafe extern "C" fn(obthis: POB_THIS, destination: POB_DATA, group: POB_GROUP, var_id: OB_SYM_ID),
    pub ob_shared_global_lv:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID) -> POB_DATA,
    pub ob_shared_global_refpkt:
        unsafe extern "C" fn(obthis: POB_THIS, destination: POB_DATA, group: POB_GROUP, var_id: OB_SYM_ID),
    pub ob_shared_lv: unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID) -> POB_DATA,
    pub ob_shared_refpkt:
        unsafe extern "C" fn(obthis: POB_THIS, destination: POB_DATA, group: POB_GROUP, var_id: OB_SYM_ID),
    pub ob_convert_chararray_to_string: unsafe extern "C" fn(obthis: POB_THIS, data: POB_DATA) -> BOOL,
    pub obJagResultsetNotify: unsafe extern "C" fn(obthis: *mut ::std::os::raw::c_void) -> HRESULT,
    pub ob_class_delete_and_withinclass:
        unsafe extern "C" fn(obthis: POB_THIS, class_hndl: OB_CLASS_HNDL, class_id: OB_CLASS_ID),
    pub ob_find_orphan_class: unsafe extern "C" fn(
        obThis: POB_THIS,
        lpszLibraryName: LPTSTR,
        lpszEntryName: LPTSTR,
        bFoundAncestor: BOOL
    ) -> INT,
    pub ob_nuke_orphan_class:
        unsafe extern "C" fn(obThis: POB_THIS, lpszLibraryName: LPTSTR, lpszEntryName: LPTSTR) -> BOOL,
    pub ob_is_ancestor_class_modified:
        unsafe extern "C" fn(obThis: POB_THIS, class_hndl: OB_CLASS_HNDL) -> BOOL,
    pub ob_rebuild_instance_image: unsafe extern "C" fn(obThis: POB_THIS, class_hndl: OB_CLASS_HNDL),
    pub ob_build_compile_list:
        unsafe extern "C" fn(obthis: POB_THIS, no_items: *mut UINT) -> POB_COMPILE_LIST,
    pub os_openlib: unsafe extern "C" fn(libname: LPTSTR, error: *mut INT, newlib: *mut BOOL) -> HINSTANCE,
    pub os_get_funcptr: unsafe extern "C" fn(funcname: LPTSTR, libhndl: HINSTANCE) -> OS_CALLC_FUNC,
    pub os_callc: unsafe extern "C" fn(
        func_ptr: OS_CALLC_FUNC,
        stack_frame: *mut ::std::os::raw::c_void,
        frame_len: ::std::os::raw::c_ushort,
        ret_buffer: *mut ::std::os::raw::c_void,
        ret_length: ::std::os::raw::c_ushort,
        bIsFloatingPoint: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub os_closelib: unsafe extern "C" fn(libhndl: HINSTANCE),
    pub cm_init_script_compiler: unsafe extern "C" fn(
        obthis: POB_THIS,
        stgthis: ppbstg_anchor,
        pbThis: *mut ::std::os::raw::c_void,
        pSignonProc: *mut ::std::os::raw::c_void,
        pSignoffProc: *mut ::std::os::raw::c_void,
        cmFlags: UINT
    ) -> PCM_THIS,
    pub cm_enable_debug_symbol: unsafe extern "C" fn(cmthis: PCM_THIS),
    pub cm_disable_debug_symbol: unsafe extern "C" fn(cmthis: PCM_THIS),
    pub cm_set_compiler_context: unsafe extern "C" fn(cmthis: PCM_THIS, obthis: POB_THIS),
    pub cm_terminate: unsafe extern "C" fn(cmthis: PCM_THIS),
    pub cm_compile_script:
        unsafe extern "C" fn(cmthis: PCM_THIS, classhndl: OB_CLASS_HNDL, source: LPTSTR) -> INT,
    pub cm_free_error_list: unsafe extern "C" fn(cmthis: PCM_THIS),
    pub cm_get_error_list: unsafe extern "C" fn(cmthis: PCM_THIS) -> *mut shlist,
    pub cm_keep_error_list: unsafe extern "C" fn(cmthis: PCM_THIS) -> *mut shlist,
    pub cm_combine_error_list: unsafe extern "C" fn(cmthis: PCM_THIS, theList: *mut shlist, bInsert: BOOL),
    pub cm_compile_namespace_block: unsafe extern "C" fn(cmthis: PCM_THIS, class_hndl: OB_CLASS_HNDL),
    pub cm_group_compile: unsafe extern "C" fn(cmthis: PCM_THIS, grouphndl: OB_GROUP_HNDL) -> INT,
    pub cm_is_word_reserved: unsafe extern "C" fn(cmthis: PCM_THIS, name: LPTSTR) -> BOOL,
    pub cm_stream_compile: unsafe extern "C" fn(grouphndl: OB_GROUP_HNDL, source: LPTSTR) -> INT,
    pub cm_src_block_compile: unsafe extern "C" fn(
        cmthis: PCM_THIS,
        grouphndl: OB_GROUP_HNDL,
        source_blocks: *mut LPTSTR,
        srcloc: *mut ::std::os::raw::c_void,
        no_blocks: UINT,
        compile_type: CM_COMPILE_TYPE::Type
    ) -> INT,
    pub cm_describe_statement: unsafe extern "C" fn(
        cmthis: PCM_THIS,
        grouphandle: ::std::os::raw::c_long,
        varname: LPTSTR,
        no_items: *mut UINT,
        class_id: OB_CLASS_ID
    ) -> LPTSTR,
    pub cm_compiler_error: unsafe extern "C" fn(cm_this: PCM_THIS, iMessageID: INT, ...) -> INT,
    pub cm_compiler_error_ln:
        unsafe extern "C" fn(cmthis: PCM_THIS, iLineNumber: UINT, iMessageID: INT, ...) -> INT,
    pub cm_get_curr_this: unsafe extern "C" fn() -> PCM_THIS,
    pub cm_rebuild_application: unsafe extern "C" fn(
        obthis: POB_THIS,
        rebuildType: CM_REBUILD_TYPE,
        cmRebuild: PCM_REBUILD,
        rebuildStatus: PCM_REBUILD_STATUS
    ) -> BOOL,
    pub cm_rebuild_from_compile_list: unsafe extern "C" fn(
        obthis: POB_THIS,
        rebuildType: CM_REBUILD_TYPE,
        cmRebuild: PCM_REBUILD,
        rebuildStatus: PCM_REBUILD_STATUS,
        pCompileList: POB_COMPILE_LIST,
        iNumberOfItems: UINT,
        inconsistency: OB_INCONSISTENCY_TYPE
    ) -> BOOL,
    pub cm_regen_datawindow:
        unsafe extern "C" fn(obthis: POB_THIS, lpszLibraryName: LPTSTR, lpszDWName: LPTSTR) -> BOOL,
    pub cm_read_alias_table: unsafe extern "C" fn(cmthis: PCM_THIS) -> INT,
    pub cm_find_alias: unsafe extern "C" fn(cmthis: PCM_THIS, pKey: LPTSTR, pCurrentName: LPTSTR) -> LPTSTR,
    pub cm_add_alias_entry: unsafe extern "C" fn(cmthis: PCM_THIS, pKey: LPTSTR, pName: LPTSTR) -> INT,
    pub cm_remove_alias_entry: unsafe extern "C" fn(cmthis: PCM_THIS, pKey: LPTSTR) -> INT,
    pub cm_write_alias_table: unsafe extern "C" fn(cmthis: PCM_THIS) -> INT,
    pub cm_add_global_var: unsafe extern "C" fn(obthis: POB_THIS, var_name: LPTSTR, type_name: LPTSTR) -> INT,
    pub cm_import_pb_extension:
        unsafe extern "C" fn(obthis: POB_THIS, pbl_name: LPCTSTR, dll_name: LPCTSTR) -> INT,
    pub cm_convert_pbl_to_unicode: unsafe extern "C" fn(obthis: POB_THIS, lpszLibraryName: LPTSTR) -> BOOL,
    pub ot_get_next_evaled_arg: unsafe extern "C" fn(obthis: POB_THIS) -> POB_DATA,
    pub ot_get_next_evaled_arg_no_convert: unsafe extern "C" fn(obthis: POB_THIS) -> POB_DATA,
    pub ot_get_next_lvalue_arg:
        unsafe extern "C" fn(obthis: POB_THIS, str_: *mut POT_LVALUE_INFO) -> POB_DATA,
    pub ot_get_simple_intarg: unsafe extern "C" fn(obthis: POB_THIS, null: *mut BOOL) -> INT,
    pub ot_get_simple_longarg:
        unsafe extern "C" fn(obthis: POB_THIS, null: *mut BOOL) -> ::std::os::raw::c_long,
    pub ot_get_intarg: unsafe extern "C" fn(obthis: POB_THIS, null: *mut BOOL) -> INT,
    pub ot_get_uintarg: unsafe extern "C" fn(obthis: POB_THIS, null: *mut BOOL) -> UINT,
    pub ot_get_longarg: unsafe extern "C" fn(obthis: POB_THIS, null: *mut BOOL) -> ::std::os::raw::c_long,
    pub ot_get_ulongarg: unsafe extern "C" fn(obthis: POB_THIS, null: *mut BOOL) -> ULONG,
    pub ot_get_decarg: unsafe extern "C" fn(obthis: POB_THIS, null: *mut BOOL) -> PSH_DEC,
    pub ot_get_floatarg: unsafe extern "C" fn(obthis: POB_THIS, fl: *mut f32, null: *mut BOOL) -> *mut f32,
    pub ot_get_doublearg: unsafe extern "C" fn(obthis: POB_THIS, doub: *mut f64, null: *mut BOOL) -> *mut f64,
    pub ot_get_longlongarg: unsafe extern "C" fn(
        obthis: POB_THIS,
        longlong_val: *mut ::std::os::raw::c_longlong,
        null: *mut BOOL
    ) -> *mut ::std::os::raw::c_longlong,
    pub ot_get_obinstarg:
        unsafe extern "C" fn(obthis: POB_THIS, obinst: POB_INST_ID, null: *mut BOOL) -> POB_INST_ID,
    pub ot_get_valptr_arg:
        unsafe extern "C" fn(obthis: POB_THIS, null: *mut BOOL) -> *mut ::std::os::raw::c_void,
    pub ot_init_arglist: unsafe extern "C" fn(obthis: POB_THIS, nargs: UINT) -> UINT,
    pub ot_get_valptr: unsafe extern "C" fn(obthis: POB_THIS, data: POB_DATA) -> *mut ::std::os::raw::c_void,
    pub ot_type_srch: unsafe extern "C" fn(name: LPTSTR) -> INT,
    pub ot_type_attr: unsafe extern "C" fn(type_: OB_CLASS_ID) -> INT,
    pub ot_get_class_name:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> LPTSTR,
    pub ot_is_array_eq: unsafe extern "C" fn(
        obthis: POB_THIS,
        array_id1: OB_ARRAY_ID,
        array_id2: OB_ARRAY_ID,
        nullval: *mut BOOL
    ) -> BOOL,
    pub ot_is_struct_eq: unsafe extern "C" fn(
        obthis: POB_THIS,
        data_node1: POB_DATA,
        data_node2: POB_DATA,
        nullval: *mut BOOL
    ) -> BOOL,
    pub ot_create_obinst_with_name: unsafe extern "C" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        lvalue_info: POT_LVALUE_INFO,
        class_name: LPTSTR,
        nested_obinst: OB_INST_ID
    ) -> OB_INST_ID,
    pub ot_create_obinst_at_lval: unsafe extern "C" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        lvalue_info: POT_LVALUE_INFO,
        nested_obinst: OB_INST_ID
    ) -> OB_INST_ID,
    pub ot_get_curr_obinst_expr:
        unsafe extern "C" fn(obthis: POB_THIS, obinst_buf: POB_INST_ID, nullval: *mut BOOL) -> POB_INST_ID,
    pub ot_func_call: unsafe extern "C" fn(
        obthis: POB_THIS,
        funccall_info: POB_FUNCCALL_INFO,
        actual_args: *mut *mut ::std::os::raw::c_void
    ) -> POB_DATA,
    pub ot_set_return_val: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA),
    pub ot_set_return_double: unsafe extern "C" fn(obthis: POB_THIS, doub_val: *mut f64, null_val: BOOL),
    pub ot_set_return_longlong:
        unsafe extern "C" fn(obthis: POB_THIS, longl_val: *mut ::std::os::raw::c_longlong, null_val: BOOL),
    pub ot_set_return_dec: unsafe extern "C" fn(obthis: POB_THIS, dec_val: PSH_DEC, null_val: BOOL),
    pub ot_no_return_val: unsafe extern "C" fn(obthis: POB_THIS),
    pub ot_assign_lvalue_dec:
        unsafe extern "C" fn(obthis: POB_THIS, lvalue_data: POB_DATA, val: PSH_DEC, nullval: BOOL),
    pub ot_assign_lvalue_double:
        unsafe extern "C" fn(obthis: POB_THIS, lvalue_data: POB_DATA, val: f64, nullval: BOOL),
    pub ot_assign_lvalue_longlong: unsafe extern "C" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        val: ::std::os::raw::c_longlong,
        nullval: BOOL
    ),
    pub ot_assign_lvalue_blob:
        unsafe extern "C" fn(obthis: POB_THIS, lvalue_data: POB_DATA, val: PSH_BINARY, nullval: BOOL),
    pub ot_assign_lvalue_obinst:
        unsafe extern "C" fn(obthis: POB_THIS, lvalue_data: POB_DATA, val: OB_INST_ID, nullval: BOOL),
    pub ot_assign_lvalue_array: unsafe extern "C" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        rvalue_array: OB_ARRAY_ID,
        nullval: BOOL
    ),
    pub ot_assign_lvalue_any: unsafe extern "C" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        rvalue_data: POB_DATA,
        rhs_class_id: OB_CLASS_ID
    ),
    pub ot_set_local_var: unsafe extern "C" fn(ths: POB_THIS, sym_id: OB_SYM_ID, data_node: POB_DATA) -> INT,
    pub ot_set_shared_var: unsafe extern "C" fn(
        ths: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        sym_id: OB_SYM_ID,
        data_node: POB_DATA
    ) -> INT,
    pub ot_set_obinst_var: unsafe extern "C" fn(
        ths: POB_THIS,
        ob_inst_id: OB_INST_ID,
        field_id: UINT,
        data_node: POB_DATA
    ) -> INT,
    pub ot_set_local_array_item:
        unsafe extern "C" fn(ths: POB_THIS, sym_id: OB_SYM_ID, index: UINT, data_node: POB_DATA) -> INT,
    pub ot_set_shared_array_item: unsafe extern "C" fn(
        ths: POB_THIS,
        group_hndl: OB_GROUP_HNDL,
        sym_id: OB_SYM_ID,
        index: UINT,
        data_node: POB_DATA
    ) -> INT,
    pub ot_set_obinst_array_item: unsafe extern "C" fn(
        ths: POB_THIS,
        obinst: OB_INST_ID,
        field_id: UINT,
        index: ULONG,
        new_data: POB_DATA
    ) -> INT,
    pub ot_get_array_values: unsafe extern "C" fn(
        obthis: POB_THIS,
        arraynode: POB_DATA,
        nitems: *mut UINT
    ) -> *mut ::std::os::raw::c_void,
    pub ot_reset_array: unsafe extern "C" fn(obthis: POB_THIS, array_node: POB_DATA, nitems: ULONG) -> INT,
    pub ot_get_local_var:
        unsafe extern "C" fn(obthis: POB_THIS, grphndl: OB_GROUP_HNDL, sym_id: OB_SYM_ID) -> POB_DATA,
    pub ot_get_shared_var:
        unsafe extern "C" fn(obthis: POB_THIS, grphndl: OB_GROUP_HNDL, sym_id: OB_SYM_ID) -> POB_DATA,
    pub ot_math_type_convert:
        unsafe extern "C" fn(class_id1: OB_CLASS_ID, class_id2: OB_CLASS_ID) -> OB_CLASS_ID,
    pub ot_get_int_value: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> INT,
    pub ot_get_uint_value: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> UINT,
    pub ot_get_byte_value:
        unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_uchar,
    pub ot_get_long_value:
        unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_long,
    pub ot_get_ulong_value: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> ULONG,
    pub ot_get_dec_value: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> PSH_DEC,
    pub ot_get_float_value: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> f32,
    pub ot_get_double_value: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> f64,
    pub ot_get_longlong_value:
        unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_longlong,
    pub ot_free_val_ptr: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA),
    pub ot_free_array: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA),
    pub ot_convert_to_int: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> INT,
    pub ot_convert_to_uint: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> UINT,
    pub ot_convert_to_byte:
        unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_uchar,
    pub ot_convert_to_long:
        unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_long,
    pub ot_convert_to_ulong: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> ULONG,
    pub ot_convert_to_dec: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> PSH_DEC,
    pub ot_convert_to_float: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> f32,
    pub ot_convert_to_double: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> f64,
    pub ot_convert_to_longlong:
        unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> ::std::os::raw::c_longlong,
    pub ot_ansi_lower: unsafe extern "C" fn(obthis: POB_THIS, string: LPTSTR) -> LPTSTR,
    pub ot_ansi_upper: unsafe extern "C" fn(obthis: POB_THIS, string: LPTSTR) -> LPTSTR,
    pub ot_ansi_strcmp: unsafe extern "C" fn(obthis: POB_THIS, stringOne: LPTSTR, stringTwo: LPTSTR) -> INT,
    pub ot_get_field_lv:
        unsafe extern "C" fn(obthis: POB_THIS, obInst: OB_INST_ID, fieldId: UINT) -> POB_DATA,
    pub ot_get_field_item_lv: unsafe extern "C" fn(
        obthis: POB_THIS,
        obInst: OB_INST_ID,
        fieldId: UINT,
        item_index: ULONG
    ) -> POB_DATA,
    pub ot_assign_ref_int:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: INT, nullval: BOOL),
    pub ot_assign_ref_uint:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: UINT, nullval: BOOL),
    pub ot_assign_ref_byte: unsafe extern "C" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_uchar,
        nullval: BOOL
    ),
    pub ot_assign_ref_long: unsafe extern "C" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_long,
        nullval: BOOL
    ),
    pub ot_assign_ref_ulong:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: ULONG, nullval: BOOL),
    pub ot_assign_ref_dec:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_DEC, nullval: BOOL),
    pub ot_assign_ref_float:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: f32, nullval: BOOL),
    pub ot_assign_ref_double:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: f64, nullval: BOOL),
    pub ot_assign_ref_longlong: unsafe extern "C" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: ::std::os::raw::c_longlong,
        nullval: BOOL
    ),
    pub ot_assign_ref_string:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: LPTSTR, nullval: BOOL),
    pub ot_assign_ref_bool:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: BOOL, nullval: BOOL),
    pub ot_assign_ref_char:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: TCHAR, nullval: BOOL),
    pub ot_assign_ref_blob:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_BINARY, nullval: BOOL),
    pub ot_assign_ref_time:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_TIME, nullval: BOOL),
    pub ot_assign_ref_date:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_TIME, nullval: BOOL),
    pub ot_assign_ref_datetime:
        unsafe extern "C" fn(obthis: POB_THIS, refpak: POT_REF_PAK, value: PSH_TIME, nullval: BOOL),
    pub ot_assign_ref_obinst: unsafe extern "C" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: OB_INST_ID,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ) -> INT,
    pub ot_assign_ref_enum: unsafe extern "C" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: INT,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ),
    pub ot_assign_ref_array: unsafe extern "C" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        value: OB_ARRAY_ID,
        nullval: BOOL,
        type_: OB_CLASS_ID
    ),
    pub ot_assign_ref_any: unsafe extern "C" fn(
        obthis: POB_THIS,
        refpak: POT_REF_PAK,
        rvalue_data: POB_DATA,
        rhs_class_id: OB_CLASS_ID
    ),
    pub ot_get_nested_obinst: unsafe extern "C" fn(obthis: POB_THIS, obinst: OB_INST_ID) -> OB_INST_ID,
    pub ot_array_create_bounded: unsafe extern "C" fn(
        obthis: POB_THIS,
        num_items: ULONG,
        elmtType: OB_CLASS_HNDL,
        varInfo: USHORT,
        numDim: USHORT,
        boundsArray: *mut ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void,
    pub ot_array_create_unbounded: unsafe extern "C" fn(
        obthis: POB_THIS,
        elmtType: OB_CLASS_HNDL,
        varInfo: USHORT
    ) -> *mut ::std::os::raw::c_void,
    pub ot_array_index:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void, index: ULONG) -> POB_DATA,
    pub ot_array_set_free_data:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void, newValue: BOOL),
    pub ot_array_free_data:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> BOOL,
    pub ot_array_class_id:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> OB_CLASS_ID,
    pub ot_array_class_hndl:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> OB_CLASS_HNDL,
    pub ot_array_num_dimensions:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> USHORT,
    pub ot_array_num_items:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> ULONG,
    pub ot_is_array_unbounded:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void) -> BOOL,
    pub ot_get_arraydef_no_dims:
        unsafe extern "C" fn(obthis: POB_THIS, arrdef: *mut ::std::os::raw::c_void) -> USHORT,
    pub ot_get_arraydef_style: unsafe extern "C" fn(
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void
    ) -> OB_ARRAY_SYMBOL_STYLE::Type,
    pub ot_get_arraydef_bounds: unsafe extern "C" fn(
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_long,
    pub ot_get_arraydef_varinfo:
        unsafe extern "C" fn(obthis: POB_THIS, arrdef: *mut ::std::os::raw::c_void) -> OB_INFO_FLAGS,
    pub ot_get_arraydef_upper_bound: unsafe extern "C" fn(
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void,
        dimension: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long,
    pub ot_get_arraydef_lower_bound: unsafe extern "C" fn(
        obthis: POB_THIS,
        arrdef: *mut ::std::os::raw::c_void,
        dimension: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long,
    pub ot_randomize: unsafe extern "C" fn(obthis: POB_THIS, iSeed: UINT),
    pub ot_rand:
        unsafe extern "C" fn(obthis: POB_THIS, lLimit: ::std::os::raw::c_long) -> ::std::os::raw::c_long,
    pub ot_class_compare:
        unsafe extern "C" fn(obthis: POB_THIS, classHndl1: OB_CLASS_HNDL, classHndl2: OB_CLASS_HNDL) -> BOOL,
    pub ot_assign_global_var_obinst:
        unsafe extern "C" fn(obthis: POB_THIS, szName: LPTSTR, obInst: OB_INST_ID) -> INT,
    pub ot_clear_array_data:
        unsafe extern "C" fn(obthis: POB_THIS, array: *mut ::std::os::raw::c_void, bArrayShrink: BOOL),
    pub ob_get_local_session: unsafe extern "C" fn(obthis: POB_THIS) -> POB_ILOCAL_SESSION,
    pub ob_class_indirect:
        unsafe extern "C" fn(obthis: POB_THIS, group: *mut POB_GROUP, class_id: POB_CLASS_ID) -> INT,
    pub ob_add_external_class_ref: unsafe extern "C" fn(
        obthis: POB_THIS,
        name: LPTSTR,
        local_group: POB_GROUP,
        ext_group_id: OB_GROUP_ID,
        ext_class_id: OB_CLASS_ID,
        refstyle: OB_GLOB_REFSTYLE,
        error: *mut INT
    ) -> OB_CLASS_ID,
    pub ob_get_local_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        refstyle: OB_GLOB_REFSTYLE,
        error: *mut INT
    ) -> OB_CLASS_ID,
    pub ob_get_primary_class:
        unsafe extern "C" fn(obthis: POB_THIS, group: *mut POB_GROUP, class_id: OB_CLASS_ID) -> OB_CLASS_ID,
    pub ob_build_qual_sec_class_name:
        unsafe extern "C" fn(obthis: POB_THIS, primary_class_name: LPTSTR, sec_class_name: LPTSTR) -> LPTSTR,
    pub ob_decl_indirect_sec_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        target_group: POB_GROUP,
        prim_class_name: LPTSTR,
        sec_class_name: LPTSTR,
        error: *mut INT
    ) -> OB_CLASS_ID,
    pub ob_update_class_ref: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        refstyle: OB_GLOB_REFSTYLE,
        is_prim_parent: BOOL
    ),
    pub ob_update_glob_class_instflag:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID, is_instance: BOOL),
    pub ob_is_class_member_accessable: unsafe extern "C" fn(
        obthis: POB_THIS,
        member_access: OB_MEMBER_ACCESS::Type,
        access_check_type: OB_MEMBER_ACCESS_TYPE::Type,
        inheritance_level: UINT,
        in_system_routine: BOOL
    ) -> BOOL,
    pub ob_get_system_func_class: unsafe extern "C" fn(obthis: POB_THIS) -> POB_RUNTIME_CLASS,
    pub ob_get_global_func_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        pGroup: POB_GROUP,
        classId: OB_CLASS_ID,
        module_id: OB_MODULE_ID
    ) -> POB_RUNTIME_CLASS,
    pub ob_type_declare: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        type_name: LPTSTR,
        type_kind: OB_TYPE_KIND::Type,
        style: OB_CLASS_STYLE::Type,
        parent_type: OB_CLASS_ID,
        nested_type: OB_CLASS_ID,
        autoinstantiate: BOOL,
        error: *mut INT
    ) -> OB_CLASS_ID,
    pub ob_type_declare_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        type_name: LPTSTR,
        type_kind: OB_TYPE_KIND::Type,
        class_style: OB_CLASS_STYLE::Type,
        parent_class: OB_CLASS_ID,
        nested_class: OB_CLASS_ID,
        autoinstantiate: BOOL,
        error: *mut INT
    ) -> OB_CLASS_ID,
    pub ob_type_declare_vtab: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        parent_class: OB_CLASS_ID,
        error: *mut INT
    ),
    pub ob_type_reference:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, type_name: LPTSTR) -> OB_CLASS_ID,
    pub ob_get_first_type: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: POB_CLASS_ID,
        style: POB_CLASS_STYLE
    ) -> LPTSTR,
    pub ob_get_next_type:
        unsafe extern "C" fn(obthis: POB_THIS, class_id: POB_CLASS_ID, style: POB_CLASS_STYLE) -> LPTSTR,
    pub ob_type_init_process: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        class_style: OB_CLASS_STYLE::Type
    ),
    pub ob_type_decl_process: unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID),
    pub ob_get_nested_class:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> OB_CLASS_ID,
    pub ob_get_class_entry: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> POB_CLASS_ENTRY,
    pub ob_is_class_indirect:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> BOOL,
    pub ob_fetch_routine: unsafe extern "C" fn(
        class_entry: POB_CLASS_ENTRY,
        rout_id: OB_ROUT_ID,
        type_: *mut OB_ROUT_TYPE::Type
    ) -> POB_ROUTNODE,
    pub ob_type_proto_decl: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE::Type,
        type_: OB_CLASS_ID,
        mod_id: OB_MODULE_ID,
        args: POB_PROTO_ARG,
        no_args: UINT,
        func_type: OB_FUNC_TYPE,
        dllname: LPTSTR,
        aliasname: LPTSTR,
        sys_func_id: OB_VTABLE_ID,
        proto_style: OB_FUNCPROTO_STYLE::Type,
        member_access: OB_MEMBER_ACCESS::Type,
        is_obsolete: BOOL,
        is_local_decl: BOOL,
        token_id: OB_EVT_TOKEN_ID,
        is_event_external: BOOL,
        throws_list: POB_CLASS_ID,
        no_throws: UINT,
        error: *mut INT
    ) -> OB_PROTO_ID,
    pub ob_type_proto_ref: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        funcname: LPTSTR,
        rout_type: OB_ROUT_TYPE::Type,
        access_type: OB_MEMBER_ACCESS_TYPE::Type,
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
    pub ob_lookup_routine_by_signature: unsafe extern "C" fn(
        obthis: POB_THIS,
        pGroup: *mut POB_GROUP,
        classId: POB_CLASS_ID,
        lpstrRoutineName: LPTSTR,
        lpstrSignature: LPTSTR,
        pVtableId: POB_VTABLE_ID,
        ppProtoGroup: *mut POB_GROUP,
        ppPrototype: *mut POB_PROTOTYPE,
        pError: POB_PROTOREF_ERROR,
        bLookupPublicOnly: BOOL
    ) -> HRESULT,
    pub ob_proto_error_upgrade: unsafe extern "C" fn(
        obthis: POB_THIS,
        currerror: OB_PROTOREF_ERROR::Type,
        newerror: OB_PROTOREF_ERROR::Type
    ) -> OB_PROTOREF_ERROR::Type,
    pub ob_get_proto_access_type: unsafe extern "C" fn(
        obthis: POB_THIS,
        curr_group: POB_GROUP,
        curr_class_id: OB_CLASS_ID,
        formal_arg_group: POB_GROUP,
        formal_arg_class_id: OB_CLASS_ID
    ) -> OB_MEMBER_ACCESS_TYPE::Type,
    pub ob_type_process_protos:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID),
    pub ob_type_reprocess_protos: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        delete_proto_name: LPTSTR,
        delete_proto_rout_type: OB_ROUT_TYPE::Type,
        delete_proto_args: POB_PROTO_ARG,
        delete_proto_no_args: UINT,
        filter_userprotos: BOOL
    ) -> INT,
    pub ob_type_proto_add: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE::Type,
        type_: OB_CLASS_ID,
        mod_id: OB_MODULE_ID,
        no_args: UINT,
        args: POB_PROTO_ARG,
        func_type: OB_FUNC_TYPE,
        dllname: LPTSTR,
        aliasname: LPTSTR,
        sys_func_id: OB_VTABLE_ID,
        proto_style: OB_FUNCPROTO_STYLE::Type,
        member_access: OB_MEMBER_ACCESS::Type,
        is_obsolete: BOOL,
        is_local_decl: BOOL,
        token_id: OB_EVT_TOKEN_ID,
        is_external_event: BOOL,
        throws_list: POB_CLASS_ID,
        no_throws: UINT,
        error: *mut INT
    ) -> OB_PROTO_ID,
    pub ob_get_type_proto_names: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        rout_type: OB_ROUT_TYPE::Type,
        include_ancestors: BOOL,
        local_protos_only: BOOL,
        nprotos: *mut UINT,
        error: *mut BOOL
    ) -> POB_PROTONAME,
    pub ob_declare_external_event_type: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        parent_class: OB_CLASS_ID,
        error: *mut INT
    ),
    pub ob_get_type_proto_names_for_ide: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        rout_type: OB_ROUT_TYPE::Type,
        include_ancestors: BOOL,
        local_protos_only: BOOL,
        nprotos: *mut UINT,
        error: *mut BOOL
    ) -> POB_PROTONAME,
    pub ob_type_vtable_module_srch: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        vtable_id: OB_VTABLE_ID
    ) -> OB_PROTO_ID,
    pub ob_get_prototype: unsafe extern "C" fn(
        obthis: POB_THIS,
        curr_group: *mut POB_GROUP,
        curr_class_id: POB_CLASS_ID,
        vtable_id: OB_VTABLE_ID
    ) -> POB_PROTOTYPE,
    pub ob_update_proto_mod_id:
        unsafe extern "C" fn(obthis: POB_THIS, proto_id: OB_PROTO_ID, mod_id: OB_MODULE_ID),
    pub ob_update_proto_rout_id:
        unsafe extern "C" fn(obthis: POB_THIS, proto_id: OB_PROTO_ID, rout_id: OB_ROUT_ID),
    pub ob_protolist_read:
        unsafe extern "C" fn(obthis: POB_THIS, class_entry: POB_CLASS_ENTRY, subpool: OB_SUBPOOL) -> INT,
    pub ob_protolist_write:
        unsafe extern "C" fn(obthis: POB_THIS, class_entry: POB_CLASS_ENTRY) -> OB_ERROR::Type,
    pub ob_prototype_match_for_event: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        proto: POB_PROTOTYPE,
        proto_group: POB_GROUP,
        result_type: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT
    ) -> BOOL,
    pub ob_prototype_search: unsafe extern "C" fn(
        obthis: POB_THIS,
        proto_list: POB_PROTOTYPE,
        no_proto_list: UINT,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE::Type,
        proto_group: POB_GROUP,
        result_type: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT,
        error: POB_PROTO_OVERLOAD_ERROR
    ) -> OB_PROTO_ID,
    pub ob_proto_overload_search: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE::Type,
        type_: OB_CLASS_ID,
        args: POB_PROTO_ARG,
        no_args: UINT
    ) -> OB_PROTO_OVERLOAD_ERROR,
    pub ob_create_proto_throws_list: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        no_throws: UINT,
        throws_type_names: *mut LPTSTR,
        throws_list: *mut POB_CLASS_ID
    ) -> INT,
    pub ob_create_proto_args: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        result_name: LPTSTR,
        no_args: UINT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        result_type: POB_CLASS_ID,
        args: *mut POB_PROTO_ARG
    ) -> INT,
    pub ob_proto_overload_search_src: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        name: LPTSTR,
        rout_type: OB_ROUT_TYPE::Type,
        result_type: LPTSTR,
        no_args: UINT,
        arg_pass_style: POB_PROTOARG_TYPE,
        arg_type_names: *mut LPTSTR,
        arg_grouping: POB_GROUPTYPE,
        error: POB_PROTO_OVERLOAD_ERROR
    ) -> INT,
    pub ob_event_module_name: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        mod_id: OB_MODULE_ID
    ) -> LPTSTR,
    pub ob_find_first_event: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_hndl: POB_CLASS_HNDL,
        event_name: LPTSTR
    ) -> OB_VTABLE_ID,
    pub ob_type_event_script_srch: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        name: LPTSTR,
        error: *mut INT
    ) -> OB_MODULE_ID,
    pub ob_build_proto_vtable:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_entry: POB_CLASS_ENTRY) -> INT,
    pub ob_type_field_decl: unsafe extern "C" fn(
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
    pub ob_type_field_search: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        fieldtype: POB_CLASS_ID,
        actual_field_id: POB_SYM_ID
    ) -> OB_SYM_ID,
    pub ob_type_field_ref: unsafe extern "C" fn(
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
    pub ob_get_type_field_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        nfields: *mut UINT,
        error: *mut INT,
        filter_fields: BOOL
    ) -> POB_TYPEINFO,
    pub ob_set_field_init_value: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        value: OB_CONST_REF
    ),
    pub ob_get_field_init_value: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID
    ) -> POB_DATA,
    pub ob_type_field_clear_instvars:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID),
    pub ob_convert_fields_to_const:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> INT,
    pub ob_build_instance_image:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> INT,
    pub ob_field_decl_indattr_funcs: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        func_templates: POB_INDATTR_FUNCTMPLT,
        no_template_items: UINT
    ),
    pub ob_field_get_indattr_funcs: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID,
        no_tmplts: *mut UINT
    ) -> POB_INDATTR_FUNCTMPLT,
    pub ob_field_requires_update_notification: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        field_id: OB_SYM_ID
    ) -> BOOL,
    pub ob_get_field_symtab:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> POB_LOOK_SYMTAB,
    pub ob_enum_entry_decl: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        name: LPTSTR,
        has_val: BOOL,
        value: INT
    ) -> INT,
    pub ob_enum_decl_process: unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID),
    pub ob_enum_reference: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        enumname: LPTSTR,
        enum_val: *mut INT,
        class_id: POB_CLASS_ID,
        group_id: POB_GROUP_ID
    ) -> INT,
    pub ob_get_type_enum_info: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        nenums: *mut UINT
    ) -> POB_ENUM_INFO,
    pub ob_is_type_enum:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> BOOL,
    pub ob_type_indattr_search: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_id: POB_CLASS_ID,
        no_tmplts: *mut UINT
    ) -> POB_INDATTR_FUNCTMPLT,
    pub ob_type_decl_indattr_funcs: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        func_templates: POB_INDATTR_FUNCTMPLT,
        no_func_templates: UINT
    ),
    pub ob_is_an_ancestor: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        of_group: POB_GROUP,
        of_class_id: OB_CLASS_ID,
        ret: *mut INT
    ) -> BOOL,
    pub ob_is_an_ancestor_excl: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        of_group: POB_GROUP,
        of_class_id: OB_CLASS_ID,
        ret: *mut INT
    ) -> BOOL,
    pub ob_find_type_ancestor: unsafe extern "C" fn(
        obthis: POB_THIS,
        group1: POB_GROUP,
        class_id1: OB_CLASS_ID,
        group2: POB_GROUP,
        class_id2: OB_CLASS_ID
    ) -> INT,
    pub ob_find_type_ancestor_assign: unsafe extern "C" fn(
        obthis: POB_THIS,
        group1: POB_GROUP,
        class_id1: OB_CLASS_ID,
        group2: POB_GROUP,
        class_id2: OB_CLASS_ID
    ) -> INT,
    pub ob_find_common_ancestor: unsafe extern "C" fn(
        obthis: POB_THIS,
        group1: POB_GROUP,
        class_id1: OB_CLASS_ID,
        group2: *mut POB_GROUP,
        class_id2: OB_CLASS_ID
    ) -> OB_CLASS_ID,
    pub ob_get_ancestor_system_class:
        unsafe extern "C" fn(obthis: POB_THIS, group: *mut POB_GROUP, class_id: OB_CLASS_ID) -> OB_CLASS_ID,
    pub ob_get_runtime_class:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, class_id: OB_CLASS_ID) -> POB_RUNTIME_CLASS,
    pub ob_get_pspp_class_name:
        unsafe extern "C" fn(obthis: POB_THIS, group_id: OB_GROUP_ID, class_id: OB_CLASS_ID) -> LPCTSTR,
    pub ob_get_func_vtable_entry: unsafe extern "C" fn(obinst: OB_INST_ID, offset: ULONG) -> OB_FUNC_FUNC,
    pub ob_find_method: unsafe extern "C" fn(
        pLocalSession: POB_ILOCAL_SESSION,
        pObject: POB_OBJECT,
        lpstrMethodName: LPTSTR,
        ppProtoGroup: *mut POB_GROUP,
        ppPrototype: *mut POB_PROTOTYPE,
        bLookupPublicOnly: BOOL
    ) -> HRESULT,
    pub ob_rout_declare: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        routname: LPTSTR,
        qual_routname: LPTSTR,
        rout_type: OB_ROUT_TYPE::Type,
        func_type: OB_FUNC_TYPE,
        proto_id: OB_PROTO_ID,
        glob_id: OB_SYM_ID,
        rout_id: POB_ROUT_ID,
        subpool: OB_SUBPOOL,
        clear_routine: BOOL,
        error: *mut INT
    ) -> OB_MODULE_ID,
    pub ob_open_routine: unsafe extern "C" fn(
        obthis: POB_THIS,
        class_entry: POB_CLASS_ENTRY,
        module_id: OB_MODULE_ID
    ) -> POB_ROUTNODE,
    pub ob_close_routine: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_func_indirect: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        class_entry: *mut POB_CLASS_ENTRY,
        mod_id: POB_MODULE_ID
    ) -> INT,
    pub ob_local_var_declare: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        error: *mut INT
    ) -> OB_SYM_ID,
    pub ob_local_array_declare: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        arrdef: POB_ARRAYDEF,
        error: *mut INT
    ) -> OB_SYM_ID,
    pub ob_local_var_reference: unsafe extern "C" fn(
        obthis: POB_THIS,
        varname: LPTSTR,
        type_: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> OB_SYM_ID,
    pub ob_local_set_var:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID, value: OB_CONST_REF),
    pub ob_local_set_id_var:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID, value: UINT),
    pub ob_set_const: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        value: *mut ::std::os::raw::c_void,
        item_type: OB_CONPOOL_ITEM_TYPE::Type,
        nitems: UINT,
        len: ULONG
    ) -> OB_CONST_REF,
    pub ob_get_const: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        const_ref: OB_CONST_REF
    ) -> *mut ::std::os::raw::c_void,
    pub ob_convert_vars_to_const: unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP) -> INT,
    pub ob_clear_group_objects: unsafe extern "C" fn(obthis: POB_THIS, pGroup: POB_GROUP) -> BOOL,
    pub ob_init_group_objects: unsafe extern "C" fn(obthis: POB_THIS, pGroup: POB_GROUP),
    pub getCultureValueStr: unsafe extern "C" fn(
        formatStr: LPTSTR,
        name: LPTSTR,
        defaultValue: LPTSTR,
        outValue: LPTSTR,
        outSize: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub getCultureValueInt: unsafe extern "C" fn(
        formatStr: LPTSTR,
        name: LPTSTR,
        defaultValue: ::std::os::raw::c_int,
        outValue: *mut ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub shformatDateTimeWeb: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: PSH_TIME,
        flags: ::std::os::raw::c_int,
        cultureInfo: LPMONTHANDDAYNAMESSTRUCT
    ) -> ::std::os::raw::c_long,
    pub shformatDateTime: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: PSH_TIME,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub shformatDecimal: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prFmt: LPTSTR,
        value: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub shformatDecimalWeb: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prFmt: LPTSTR,
        value: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long,
    pub shformatDouble: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: f64,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub shformatDoubleWeb: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: f64,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long,
    pub shformatLonglong: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: ::std::os::raw::c_longlong,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub shformatLonglongWeb: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: ::std::os::raw::c_longlong,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long,
    pub shformatReal: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        pValue: *mut f32,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub shformatRealWeb: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        pValue: *mut f32,
        flags: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_long,
    pub shformatString: unsafe extern "C" fn(
        pResult: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        prMask: LPTSTR,
        value: LPTSTR,
        flags: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_long,
    pub shformatCmplDateTimeMask: unsafe extern "C" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    pub shformatCmplDateTimeMaskWeb: unsafe extern "C" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int,
    pub shformatCmplNumericMask: unsafe extern "C" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    pub shformatCmplNumericMaskWeb: unsafe extern "C" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int,
    pub shformatCmplNumericMaskWebCommasPos: unsafe extern "C" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int,
        dwCultureFormat: LPTSTR
    ) -> ::std::os::raw::c_int,
    pub shformatCmplStringMask: unsafe extern "C" fn(
        prMask: LPTSTR,
        psMask: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    pub shformatErrorString: unsafe extern "C" fn(errMsg: LPTSTR, err: ::std::os::raw::c_int),
    pub shregExprCmpl: unsafe extern "C" fn(
        pattern: LPTSTR,
        srcPattern: LPTSTR,
        maxLen: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int,
    pub shregExprMatch: unsafe extern "C" fn(string: LPTSTR, pattern: LPTSTR) -> ::std::os::raw::c_int,
    pub shIsValidReal: unsafe extern "C" fn(n: LPTSTR) -> BOOL,
    pub shNormalizeReal: unsafe extern "C" fn(out: LPTSTR, in_: LPTSTR),
    pub shNormalizeRealbyLocale: unsafe extern "C" fn(out: LPTSTR, in_: LPTSTR),
    pub shIsValidRealWeb:
        unsafe extern "C" fn(n: LPTSTR, paramPeriodChar: TCHAR, paramCommaChar: TCHAR) -> BOOL,
    pub shNormalizeRealWeb:
        unsafe extern "C" fn(out: LPTSTR, in_: LPTSTR, paramPeriodChar: TCHAR, paramCommaChar: TCHAR),
    pub shNormalizeRealbyLocaleWeb:
        unsafe extern "C" fn(out: LPTSTR, in_: LPTSTR, paramPeriodChar: TCHAR, paramCommaChar: TCHAR),
    pub pbshr_intl: unsafe extern "C" fn(),
    pub shIsValidRealNoLocale: unsafe extern "C" fn(n: LPTSTR) -> BOOL,
    pub shGetRegProfileStringValue:
        unsafe extern "C" fn(lpRegProfileStruct: LPREGPROFILESTRUCT) -> ::std::os::raw::c_long,
    pub ob_add_glbsym_var: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        class_id: OB_CLASS_ID,
        id: OB_SYM_ID
    ) -> OB_SYM_ID,
    pub ob_add_glbsym_class: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        refstyle: OB_GLOB_REFSTYLE,
        group_id: OB_GROUP_ID,
        class_id: OB_CLASS_ID,
        sys_class_id: OB_CLASS_ID
    ) -> OB_SYM_ID,
    pub ob_add_glbsym_func: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        name: LPTSTR,
        reftype: OB_GLOB_REFTYPE,
        class_id: OB_CLASS_ID,
        mod_id: OB_MODULE_ID
    ) -> OB_SYM_ID,
    pub rt_set_class_handle:
        unsafe extern "C" fn(rtthis: POB_THIS, appclasshndl: OB_CLASS_HNDL, appinst: OB_INST_ID),
    pub rt_init: unsafe extern "C" fn(obthis: POB_THIS, stgthis: ppbstg_anchor) -> POB_THIS,
    pub rt_start_debug: unsafe extern "C" fn(
        rtthis: POB_THIS,
        rtBreakCallback: *mut RT_BREAK_PROC,
        pUserData: *mut ::std::os::raw::c_void
    ) -> INT,
    pub rt_stop_debug: unsafe extern "C" fn(rtthis: POB_THIS) -> INT,
    pub rt_set_pcode_to_line: unsafe extern "C" fn(obthis: POB_THIS, line_no: UINT) -> INT,
    pub rt_breakpoint: unsafe extern "C" fn(
        rtthis: POB_THIS,
        bSet: BOOL,
        obClassHndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        iLineNumber: UINT,
        n_times: UINT,
        condition: LPTSTR,
        id: ::std::os::raw::c_long
    ) -> PRT_BREAKPOINT,
    pub rt_create_watchpoint: unsafe extern "C" fn(
        rtthis: POB_THIS,
        pdata_info: POB_DATA_INFO,
        watch_type: WATCHPOINT_TYPE::Type,
        item_scope: ::std::os::raw::c_uchar,
        id: ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void,
    pub rt_find_watchpoint_for_watchid: unsafe extern "C" fn(
        rtthis: POB_THIS,
        watchId: ::std::os::raw::c_long
    ) -> *mut ::std::os::raw::c_void,
    pub rt_delete_watchpoint: unsafe extern "C" fn(rtthis: POB_THIS, watchpt: *mut ::std::os::raw::c_void),
    pub rt_is_line_executable: unsafe extern "C" fn(
        rtthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        line_no: UINT
    ) -> BOOL,
    pub rt_closest_executable_line: unsafe extern "C" fn(
        rtthis: POB_THIS,
        class_hndl: OB_CLASS_HNDL,
        vtable_id: OB_VTABLE_ID,
        line_no: UINT
    ) -> UINT,
    pub rt_start_run: unsafe extern "C" fn(rtthis: POB_THIS) -> INT,
    pub rt_stop_run: unsafe extern "C" fn(rtthis: POB_THIS) -> INT,
    pub rt_create_obinst: unsafe extern "C" fn(rtthis: POB_THIS, name: LPTSTR, obinst: POB_INST_ID) -> INT,
    pub rtReturnValGet: unsafe extern "C" fn(rtThis: POB_THIS) -> POB_DATA,
    pub rtReturnValFree: unsafe extern "C" fn(rtThis: POB_THIS),
    pub rt_error: unsafe extern "C" fn(rtthis: POB_THIS, iMessageID: INT) -> INT,
    pub rt_free_error_struct: unsafe extern "C" fn(rtthis: POB_THIS, error_struct: PRT_ERROR_STRUCT),
    pub rt_error_using_struct: unsafe extern "C" fn(
        rtthis: POB_THIS,
        error_struct: PRT_ERROR_STRUCT,
        exceptionClassName: LPTSTR
    ) -> INT,
    pub rt_build_exception_using_error:
        unsafe extern "C" fn(obthis: POB_THIS, pError: PRT_ERROR_STRUCT, className: LPTSTR) -> OB_INST_ID,
    pub rt_handle_uncaught_exception: unsafe extern "C" fn(obthis: POB_THIS),
    pub rt_populate_error_struct: unsafe extern "C" fn(
        obthis: POB_THIS,
        error_struct: PRT_ERROR_STRUCT,
        message: LPTSTR,
        iMessageID: INT
    ),
    pub rt_populate_error_from_stack: unsafe extern "C" fn(
        obthis: POB_THIS,
        error_struct: PRT_ERROR_STRUCT,
        message: LPTSTR,
        iMessageID: INT
    ),
    pub rt_call_error_callback:
        unsafe extern "C" fn(rtthis: POB_THIS, error_struct: PRT_ERROR_STRUCT, bAllowHalt: BOOL) -> BOOL,
    pub rt_normalize_error_id: unsafe extern "C" fn(obthis: POB_THIS, iMessageID: INT) -> INT,
    pub ot_handle_exception: unsafe extern "C" fn(
        rtthis: POB_THIS,
        pException_Stack: *mut ::std::os::raw::c_void,
        currDepth: USHORT
    ) -> INT,
    pub ob_dbg_pop_call_stack_ntimes: unsafe extern "C" fn(obthis: POB_THIS, n: UINT) -> INT,
    pub ob_dbg_push_call_stack_ntimes: unsafe extern "C" fn(obthis: POB_THIS, n: UINT) -> INT,
    pub ob_get_current_stack_location: unsafe extern "C" fn(obthis: POB_THIS) -> PRT_BREAKPOINT,
    pub rtRoutineSearch: unsafe extern "C" fn(
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pchRoutineName: LPTSTR,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineType: OB_ROUT_TYPE::Type,
        pobRoutineId: POB_VTABLE_ID
    ) -> INT,
    pub rtRoutineExec: unsafe extern "C" fn(
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineId: OB_VTABLE_ID,
        obRoutineType: OB_ROUT_TYPE::Type,
        bConvert: BOOL
    ) -> RT_EXEC_STATUS::Type,
    pub rtRoutineExecByName: unsafe extern "C" fn(
        obThis: POB_THIS,
        pchRoutineName: LPTSTR,
        rtCallInfo: RT_CALL_INFO,
        pobdArgArray: POB_DATA,
        uiNoArgs: UINT,
        obRoutineType: OB_ROUT_TYPE::Type,
        bConvert: BOOL
    ) -> RT_EXEC_STATUS::Type,
    pub rtRoutineExecPosted: unsafe extern "C" fn(pData: *mut ::std::os::raw::c_void) -> RT_EXEC_STATUS::Type,
    pub rtRoutineInfo: unsafe extern "C" fn(
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        obRoutineId: OB_VTABLE_ID,
        pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO
    ) -> INT,
    pub rtRoutineProtoInfoFree:
        unsafe extern "C" fn(obThis: POB_THIS, pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO),
    pub rtInitializeInfoForCall:
        unsafe extern "C" fn(obThis: POB_THIS, pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO) -> INT,
    pub rtCleanupInfoAfterCall:
        unsafe extern "C" fn(obThis: POB_THIS, pRoutineProtoInfo: PRT_ROUTINE_PROTO_INFO) -> INT,
    pub rtRoutineCount: unsafe extern "C" fn(
        obThis: POB_THIS,
        rtCallInfo: RT_CALL_INFO,
        pusRoutineTotal: *mut USHORT,
        pusFuncTotal: *mut USHORT,
        pusEventTotal: *mut USHORT
    ) -> INT,
    pub rtReferenceArgCreate:
        unsafe extern "C" fn(obThis: POB_THIS, pobdRefArg: POB_DATA, prtRefArgInfo: PRT_REFARG_INFO) -> INT,
    pub rtReferenceArgFree: unsafe extern "C" fn(obThis: POB_THIS, pobdRefArg: POB_DATA) -> INT,
    pub rtGetClassDescrip: unsafe extern "C" fn(
        obThis: POB_THIS,
        obClassHndl: OB_CLASS_HNDL,
        prtClassDescrip: PRT_CLASS_DESCRIP,
        pobClassIdSystem: POB_CLASS_ID
    ) -> INT,
    pub rtDataFree: unsafe extern "C" fn(pobThis: POB_THIS, pobdVal: POB_DATA),
    pub rtDataCopy:
        unsafe extern "C" fn(pobThis: POB_THIS, pobdDest: POB_DATA, pobdSrc: POB_DATA, AddReference: BOOL),
    pub rt_hit_level_0: unsafe extern "C" fn(obthis: POB_THIS),
    pub rt_StartJaguarDebug: unsafe extern "C" fn(obthis: POB_THIS, hwnd: HWND) -> HRESULT,
    pub rt_StopJaguarDebug: unsafe extern "C" fn(obthis: POB_THIS) -> HRESULT,
    pub rt_JagBreakpointHit:
        unsafe extern "C" fn(obthis: POB_THIS, instance_id: ULONG, breakpointID: ULONG) -> HRESULT,
    pub rt_JaguarGetCurrentContext: unsafe extern "C" fn(
        obthis: POB_THIS,
        instance_id: *mut ULONG,
        breakpointID: *mut PRT_BREAKPOINT
    ) -> HRESULT,
    pub obPsppNormalBody: unsafe extern "C" fn(
        obThis: POB_THIS,
        numArgs: UINT,
        pGroup: POB_GROUP,
        class_id: OB_CLASS_ID,
        parent_class_id: OB_CLASS_ID,
        protId: UINT
    ) -> INT,
    pub ob_create_object: unsafe extern "C" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        p_group: POB_GROUP,
        class_id: OB_CLASS_ID
    ) -> INT,
    pub ob_create_object_using: unsafe extern "C" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        context: POB_RUNTIME_INST,
        class_name: LPTSTR
    ) -> HRESULT,
    pub ob_copy_rtinst:
        unsafe extern "C" fn(obthis: POB_THIS, from_rtinst: POB_RUNTIME_INST) -> POB_RUNTIME_INST,
    pub ob_destroy_rtinst: unsafe extern "C" fn(obthis: POB_THIS, rtinst: POB_RUNTIME_INST) -> INT,
    pub ob_get_primary_rtinst:
        unsafe extern "C" fn(obthis: POB_THIS, rtinst: POB_RUNTIME_INST) -> POB_RUNTIME_INST,
    pub ob_is_rtinst_autoinstantiate:
        unsafe extern "C" fn(obthis: POB_THIS, rtinst: POB_RUNTIME_INST) -> BOOL,
    pub ob_object_compare:
        unsafe extern "C" fn(obthis: POB_THIS, rtinst1: POB_RUNTIME_INST, rtinst2: POB_RUNTIME_INST) -> BOOL,
    pub ob_invoke_static: unsafe extern "C" fn(
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        numArgs: USHORT,
        args: POB_DATA,
        result: POB_DATA
    ) -> HRESULT,
    pub ob_invoke_dynamic: unsafe extern "C" fn(
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        routType: OB_ROUT_TYPE::Type,
        name: LPTSTR,
        numArgs: USHORT,
        args: POB_DATA,
        result: POB_DATA
    ) -> HRESULT,
    pub ob_invoke_staticAsync: unsafe extern "C" fn(
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        numArgs: USHORT,
        args: POB_DATA
    ) -> HRESULT,
    pub ob_invoke_dynamicAsync: unsafe extern "C" fn(
        rtinst: POB_RUNTIME_INST,
        context: POB_RUNTIME_CLASS,
        routType: OB_ROUT_TYPE::Type,
        name: LPTSTR,
        numArgs: USHORT,
        args: POB_DATA
    ) -> HRESULT,
    pub ob_instance_lv:
        unsafe extern "C" fn(obthis: POB_THIS, current_inst: POB_RUNTIME_INST, var_id: OB_SYM_ID) -> POB_DATA,
    pub ob_instance_fldupdate_refpkt: unsafe extern "C" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        var_id: OB_SYM_ID
    ),
    pub ob_instance_flditemupdate_refpkt: unsafe extern "C" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        group_id: OB_GROUP_ID,
        var_id: OB_SYM_ID,
        lvalue: POB_DATA,
        item_index: ULONG
    ),
    pub ob_instance_simple_refpkt: unsafe extern "C" fn(
        obthis: POB_THIS,
        destination: POB_DATA,
        current_inst: POB_RUNTIME_INST,
        var_id: OB_SYM_ID
    ),
    pub ob_get_group_load_state:
        unsafe extern "C" fn(pGroupReference: *mut ::std::os::raw::c_void) -> OB_GROUP_LOAD_STATE,
    pub ob_get_groupref_group:
        unsafe extern "C" fn(pGroupReference: *mut ::std::os::raw::c_void) -> POB_GROUP,
    pub ob_group_get_next_index: unsafe extern "C" fn(obthis: POB_THIS) -> ULONG,
    pub ob_close_typedef_group: unsafe extern "C" fn(obThis: POB_THIS),
    pub ob_create_group_structure: unsafe extern "C" fn(obThis: POB_THIS, lpszGroupName: LPTSTR) -> POB_GROUP,
    pub ob_new_group: unsafe extern "C" fn(
        obthis: POB_THIS,
        lib_name: LPTSTR,
        qual_group_name: LPTSTR,
        group_lock_state: OB_GROUP_LOCK_STATE,
        group_load_state: OB_GROUP_LOAD_STATE
    ) -> POB_GROUP,
    pub ob_del_group_structure: unsafe extern "C" fn(obThis: POB_THIS, pGroup: POB_GROUP),
    pub ob_group_data_srch: unsafe extern "C" fn(obThis: POB_THIS, obGroupHandle: OB_GROUP_HNDL) -> POB_GROUP,
    pub ob_replace_group:
        unsafe extern "C" fn(obThis: POB_THIS, obGroupID: OB_GROUP_ID, pNewGroup: POB_GROUP),
    pub ob_copy_group_shrsym_data: unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP),
    pub ob_get_qualified_name_with_namespace:
        unsafe extern "C" fn(obThis: POB_THIS, pGroup: POB_GROUP, lpszNamespace: LPTSTR) -> LPTSTR,
    pub ob_get_source_from_group: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        src_type: *mut POB_SOURCE_BLK_TYPE,
        no_blocks: *mut UINT,
        subpool: OB_SUBPOOL,
        ppSrcLastEdit: *mut POB_SRC_LAST_EDIT,
        pNoSrcLastEdit: *mut UINT
    ) -> *mut LPTSTR,
    pub obUpdateSrcLastEdit: unsafe extern "C" fn(
        obThis: POB_THIS,
        pGroup: POB_GROUP,
        pSrcLastEdit: POB_SRC_LAST_EDIT,
        NoSrcLastEdit: UINT
    ),
    pub ob_get_var: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        look_symtab: POB_LOOK_SYMTAB,
        var_id: OB_SYM_ID
    ) -> POB_DATA,
    pub ob_init_var_data: unsafe extern "C" fn(obthis: POB_THIS, var_data: POB_DATA, group: POB_GROUP),
    pub ob_global_indirect:
        unsafe extern "C" fn(obthis: POB_THIS, group: *mut POB_GROUP, glob_id: POB_SYM_ID) -> POB_DATA,
    pub ob_global_var_declare: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        error: *mut INT
    ) -> OB_SYM_ID,
    pub ob_global_array_declare: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        var: LPTSTR,
        varinfo: OB_INFO_FLAGS,
        lookup_info: OB_LOOKUP_INFO,
        type_: OB_CLASS_ID,
        arrdef: POB_ARRAYDEF,
        error: *mut INT
    ) -> OB_SYM_ID,
    pub ob_shared_var_reference: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        varname: LPTSTR,
        type_: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> OB_SYM_ID,
    pub ob_global_set_var:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID, value: OB_CONST_REF),
    pub ob_global_set_id_var:
        unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, var_id: OB_SYM_ID, value: UINT),
    pub ob_get_local_symtab:
        unsafe extern "C" fn(obthis: POB_THIS, group: *mut POB_GROUP, var_id: POB_SYM_ID) -> POB_LOOK_SYMTAB,
    pub ob_get_unconverted_var: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: *mut POB_GROUP,
        var: OB_SYM_ID,
        level: UINT
    ) -> POB_DATA,
    pub ob_lookup_shared_var_info: unsafe extern "C" fn(
        obThis: POB_THIS,
        iGroupID: OB_GROUP_ID,
        iSymbolID: OB_SYM_ID,
        pType: POB_CLASS_ID,
        varinfo: POB_INFO_FLAGS,
        lookup_info: POB_LOOKUP_INFO,
        init_value: *mut POB_DATA,
        array_def: *mut POB_ARRAYDEF
    ) -> INT,
    pub ob_clear_shared_vars: unsafe extern "C" fn(obthis: POB_THIS, group: POB_GROUP, level: INT),
    pub ot_eval_expr: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_entry: POB_CLASS_ENTRY,
        pcode_blk: POB_PCODE_BLK,
        expr_result_buf: POB_DATA
    ) -> POB_DATA,
    pub ot_dbg_funccall: unsafe extern "C" fn(
        obthis: POB_THIS,
        call_label: LPTSTR,
        group: POB_GROUP,
        class_entry: OB_CLASS_ID,
        name: LPTSTR
    ),
    pub ot_run_dllfunccall: unsafe extern "C" fn(
        obthis: POB_THIS,
        group: POB_GROUP,
        class_id: OB_CLASS_ID,
        funcname: LPTSTR,
        evaled_arglist: POB_DATA,
        no_args: UINT,
        funcproto: POB_PROTOTYPE
    ) -> INT,
    pub ot_run_rpcfunccall: unsafe extern "C" fn(
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
    pub ot_get_dll_funcptr_by_name:
        unsafe extern "C" fn(obthis: POB_THIS, dllname: LPTSTR, funcname: LPTSTR) -> OS_CALLC_FUNC,
    pub ot_post_call: unsafe extern "C" fn(
        obthis: POB_THIS,
        pRuntimeClass: POB_RUNTIME_CLASS,
        vtableId: OB_VTABLE_ID,
        uiNoArgs: UINT,
        args: POB_DATA
    ) -> INT,
    pub ot_check_types: unsafe extern "C" fn(
        obthis: POB_THIS,
        group1: POB_GROUP,
        type1: OB_CLASS_ID,
        grouping1: OB_GROUPTYPE,
        group2: POB_GROUP,
        type2: OB_CLASS_ID,
        grouping2: OB_GROUPTYPE,
        ancestor_flag: *mut UINT
    ) -> OT_TYPE_CHECK_ERROR::Type,
    pub ot_strict_type_check: unsafe extern "C" fn(
        obthis: POB_THIS,
        group1: POB_GROUP,
        type1: OB_CLASS_ID,
        group2: POB_GROUP,
        type2: OB_CLASS_ID
    ) -> INT,
    pub ot_generateVarInfo: unsafe extern "C" fn(obthis: POB_THIS, obClassHndl: OB_CLASS_HNDL) -> USHORT,
    pub ot_type_loc: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> OT_TYPE_LOC::Type,
    pub ot_init_data_node: unsafe extern "C" fn(
        obthis: POB_THIS,
        data_node: POB_DATA,
        type_: OB_CLASS_ID,
        varinfo: OB_INFO_FLAGS
    ),
    pub ot_set_lvalue: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_id: OB_GROUP_ID,
        lvalue_data: POB_DATA,
        rvalue_data: POB_DATA,
        do_error_check: BOOL
    ) -> INT,
    pub ot_free_out_node: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA),
    pub ot_free_inv_meth_args:
        unsafe extern "C" fn(obthis: POB_THIS, pArrayDataNode: POB_DATA, pFreeFlags: LPTSTR) -> INT,
    pub ot_copy_array:
        unsafe extern "C" fn(obthis: POB_THIS, old_array_inst: POB_ARRAY_INST) -> POB_ARRAY_INST,
    pub ot_get_string_from_chararray:
        unsafe extern "C" fn(obthis: POB_THIS, arrayinst: POB_ARRAY_INST) -> LPTSTR,
    pub ot_create_chararray_from_string:
        unsafe extern "C" fn(obthis: POB_THIS, string_val: LPTSTR, target_data_node: POB_DATA) -> POB_DATA,
    pub ot_create_bounded_chararray_from_string: unsafe extern "C" fn(
        obthis: POB_THIS,
        string_val: LPTSTR,
        bounds: *mut ::std::os::raw::c_long,
        target_data_node: POB_DATA
    ) -> POB_DATA,
    pub ot_get_char_value: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> TCHAR,
    pub ot_get_string_value: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA) -> LPTSTR,
    pub ot_get_string_from_char: unsafe extern "C" fn(obthis: POB_THIS, char_val: TCHAR) -> LPTSTR,
    pub ot_string_cat: unsafe extern "C" fn(obthis: POB_THIS, string1: LPTSTR, string2: LPTSTR) -> LPTSTR,
    pub ot_binary_cat:
        unsafe extern "C" fn(obthis: POB_THIS, bin1: PSH_BINARY, bin2: PSH_BINARY) -> PSH_BINARY,
    pub ot_halt: unsafe extern "C" fn(obthis: POB_THIS, send_close_event: BOOL) -> INT,
    pub ot_convert_bounded_to_bounded: unsafe extern "C" fn(
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long,
        free_old_array: BOOL
    ) -> POB_ARRAY_INST,
    pub ot_convert_bounded_to_unbounded: unsafe extern "C" fn(
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST,
    pub ot_convert_unbounded_to_bounded: unsafe extern "C" fn(
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long
    ) -> POB_ARRAY_INST,
    pub ot_convert_unbounded_to_unbounded: unsafe extern "C" fn(
        obthis: POB_THIS,
        old_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST,
    pub ot_convert_any_to_bounded: unsafe extern "C" fn(
        obthis: POB_THIS,
        any_node: POB_DATA,
        new_class_id: OB_CLASS_ID,
        new_nitems: ULONG,
        new_ndims: INT,
        bounds: *mut ::std::os::raw::c_long
    ) -> POB_ARRAY_INST,
    pub ot_convert_any_to_unbounded: unsafe extern "C" fn(
        obthis: POB_THIS,
        any_node: POB_DATA,
        new_class_id: OB_CLASS_ID
    ) -> POB_ARRAY_INST,
    pub ot_convert_array_to_object: unsafe extern "C" fn(
        obthis: POB_THIS,
        any_array_inst: POB_ARRAY_INST,
        new_class_id: OB_CLASS_ID
    ) -> POB_RUNTIME_INST,
    pub ot_build_simple_refpak:
        unsafe extern "C" fn(obthis: POB_THIS, lvalue_data: POB_DATA, group_id: OB_GROUP_ID) -> POT_REF_PAK,
    pub ot_build_field_refpak: unsafe extern "C" fn(
        obthis: POB_THIS,
        group_id: OB_GROUP_ID,
        rtinst: POB_RUNTIME_INST,
        field_id: UINT,
        item_index: ULONG,
        bTriggerFieldUpdate: BOOL
    ) -> POT_REF_PAK,
    pub ot_build_flditemupdate_refpak: unsafe extern "C" fn(
        obthis: POB_THIS,
        lvalue_data: POB_DATA,
        group_id: OB_GROUP_ID,
        rtinst: POB_RUNTIME_INST,
        field_id: UINT,
        item_index: ULONG
    ) -> POT_REF_PAK,
    pub ot_add_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_sub_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_mul_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_div_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_pow_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_neg_any: unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT,
    pub ot_eq_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_ne_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_gt_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_lt_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_ge_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_le_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_and_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_or_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_not_any: unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny: POB_DATA) -> INT,
    pub ot_incr_any: unsafe extern "C" fn(prtThis: POB_THIS, pAny: POB_DATA) -> INT,
    pub ot_decr_any: unsafe extern "C" fn(prtThis: POB_THIS, pAny: POB_DATA) -> INT,
    pub ot_mod_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_min_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_max_any:
        unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA, pAny2: POB_DATA) -> INT,
    pub ot_check_any_exact_type:
        unsafe extern "C" fn(obthis: POB_THIS, any_var: POB_DATA, expected_type: OB_CLASS_ID) -> INT,
    pub ot_check_any_string_type:
        unsafe extern "C" fn(obthis: POB_THIS, any_var: POB_DATA, expected_type: OB_CLASS_ID) -> INT,
    pub ot_check_any_binary_type:
        unsafe extern "C" fn(obthis: POB_THIS, any_var: POB_DATA, expected_type: OB_CLASS_ID) -> INT,
    pub ot_check_any_math_type:
        unsafe extern "C" fn(obthis: POB_THIS, any_var: POB_DATA, expected_type: OB_CLASS_ID) -> INT,
    pub ot_check_any_enum_type: unsafe extern "C" fn(
        obthis: POB_THIS,
        any_var: POB_DATA,
        current_group: POB_GROUP,
        expected_type: OB_CLASS_ID
    ) -> INT,
    pub ot_check_any_object_type: unsafe extern "C" fn(
        obthis: POB_THIS,
        any_var: POB_DATA,
        current_group: POB_GROUP,
        expected_type: OB_CLASS_ID
    ) -> INT,
    pub ot_duplicate_any: unsafe extern "C" fn(pobThis: POB_THIS, pAny: POB_DATA) -> INT,
    pub ot_abs_any: unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT,
    pub ot_ceiling_any: unsafe extern "C" fn(prtThis: POB_THIS, pResult: POB_DATA, pAny1: POB_DATA) -> INT,
    pub ot_string_to_binary: unsafe extern "C" fn(
        rtThis: POB_THIS,
        lpStr: LPTSTR,
        EncodingType: ::std::os::raw::c_int,
        bNullTerminated: BOOL
    ) -> PSH_BINARY,
    pub ot_bytearray_to_binary:
        unsafe extern "C" fn(rtThis: POB_THIS, array_inst: POB_ARRAY_INST) -> PSH_BINARY,
    pub ot_any_to_binary: unsafe extern "C" fn(rtThis: POB_THIS, obData: POB_DATA) -> PSH_BINARY,
    pub dwCompile: unsafe extern "C" fn(
        syntax: LPTSTR,
        errmsg: LPTSTR,
        obthis: *mut ::std::os::raw::c_void,
        dbtran: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub dwCreateWindow: unsafe extern "C" fn(
        arg1: LPTSTR,
        arg2: LPTSTR,
        arg3: DWORD,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
        arg7: ::std::os::raw::c_int,
        arg8: HWND,
        arg9: HMENU,
        arg10: HANDLE,
        arg11: LPTSTR
    ) -> HWND,
    pub dwCreateWindowEx: unsafe extern "C" fn(
        arg1: DWORD,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: DWORD,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
        arg7: ::std::os::raw::c_int,
        arg8: ::std::os::raw::c_int,
        arg9: HWND,
        arg10: HMENU,
        arg11: HANDLE,
        arg12: LPTSTR
    ) -> HWND,
    pub dwCrosstabDLG: unsafe extern "C" fn(hWnd: HWND) -> BOOL,
    pub dwDescribe: unsafe extern "C" fn(hWnd: HWND, syntax: LPTSTR) -> LPTSTR,
    pub dwFree: unsafe extern "C" fn(prog: *mut ::std::os::raw::c_void) -> BOOL,
    pub dwSetDataStoreHWndBehavior: unsafe extern "C" fn(bUseHWnd: BOOL) -> BOOL,
    pub dwLoadData: unsafe extern "C" fn(pFileName: LPOLESTR, hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwLoadDataProg: unsafe extern "C" fn(
        pFileName: LPOLESTR,
        pProg: *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_short,
    pub dwLoadDataCrosstab: unsafe extern "C" fn(pFileName: LPTSTR, hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwLoadDataStorage: unsafe extern "C" fn(
        pStg: *mut ::std::os::raw::c_void,
        given_pDW: *mut ::std::os::raw::c_void,
        bInDWSync: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwLoadDataOle: unsafe extern "C" fn(pFileName: LPOLESTR, pDW: *mut ::std::os::raw::c_void),
    pub dwLoadDW:
        unsafe extern "C" fn(pDW: *mut ::std::os::raw::c_void, name: LPTSTR) -> *mut ::std::os::raw::c_void,
    pub dwLoadFile: unsafe extern "C" fn(
        pFileName: LPOLESTR,
        obthis: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub dwLoadLibrary: unsafe extern "C" fn(
        files: *mut LPTSTR,
        filecount: UINT,
        entry: LPTSTR,
        obthis: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub dwLoadMemory: unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
    pub dwLoadStorage: unsafe extern "C" fn(
        pStg: *mut ::std::os::raw::c_void,
        obthis: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub dwModify: unsafe extern "C" fn(hWnd: HWND, syntax: LPTSTR) -> LPTSTR,
    pub dwSaveData: unsafe extern "C" fn(pFileName: LPOLESTR, hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwSaveDataStorage:
        unsafe extern "C" fn(pStg: *mut ::std::os::raw::c_void, hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwSaveDataCopy: unsafe extern "C" fn(
        pOutFileName: LPOLESTR,
        pInFileName: LPOLESTR,
        sa: *mut ::std::os::raw::c_void,
        subpool: pbstg_subpool
    ) -> ::std::os::raw::c_short,
    pub dwSaveDataCopyStorage: unsafe extern "C" fn(
        pOutStg: *mut ::std::os::raw::c_void,
        pInStg: *mut ::std::os::raw::c_void,
        sa: *mut ::std::os::raw::c_void,
        subpool: pbstg_subpool
    ) -> ::std::os::raw::c_short,
    pub dwSaveFile: unsafe extern "C" fn(
        pFileName: LPOLESTR,
        pSyn: LPTSTR,
        pProg: *mut ::std::os::raw::c_void,
        pData: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_uint
    ) -> ::std::os::raw::c_short,
    pub dwSaveLibrary: unsafe extern "C" fn(
        prog: *mut ::std::os::raw::c_void,
        file: LPTSTR,
        entry: LPTSTR
    ) -> ::std::os::raw::c_short,
    pub dwSaveOle: unsafe extern "C" fn(pFileName: LPOLESTR, hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwSaveObjects: unsafe extern "C" fn(pFileName: LPOLESTR, hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwSaveStorage: unsafe extern "C" fn(
        pStg: *mut ::std::os::raw::c_void,
        pSyn: LPTSTR,
        pProg: *mut ::std::os::raw::c_void,
        pData: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_uint
    ) -> ::std::os::raw::c_short,
    pub dwFileSaveStorage: unsafe extern "C" fn(
        hWnd: HWND,
        pStorage: *mut ::std::os::raw::c_void,
        bInDWSync: BOOL
    ) -> ::std::os::raw::c_long,
    pub dwSaveStorageStg: unsafe extern "C" fn(
        pStg: *mut ::std::os::raw::c_void,
        pSyn: LPTSTR,
        pProg: *mut ::std::os::raw::c_void,
        pData: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_uint
    ) -> ::std::os::raw::c_short,
    pub dwSaveOleStg: unsafe extern "C" fn(
        pStorage: *mut ::std::os::raw::c_void,
        pDW: *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_short,
    pub dwSaveObjectsStg: unsafe extern "C" fn(
        pStorage: *mut ::std::os::raw::c_void,
        pDW: *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_short,
    pub dwSaveDataStg: unsafe extern "C" fn(
        pStorage: *mut ::std::os::raw::c_void,
        hWnd: HWND,
        bInDWSync: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwLoadFileBlob: unsafe extern "C" fn(
        pBlob: *mut ::std::os::raw::c_void,
        obthis: *mut ::std::os::raw::c_void,
        ulLen: ULONG,
        pStg: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub dwStgFree: unsafe extern "C" fn(hWnd: HWND, s: *mut ::std::os::raw::c_void),
    pub dwGetMessageText: unsafe extern "C" fn(hWnd: HWND) -> LPTSTR,
    pub dwGetExceedPageMessage:
        unsafe extern "C" fn(hWnd: HWND, sa: ppbstg_anchor, subpool: pbstg_subpool) -> LPTSTR,
    pub DW_PluginStart: unsafe extern "C" fn(
        hInst: HINSTANCE,
        pAppName: LPTSTR,
        pLibListEntries: *mut LPTSTR,
        iLibListEntries: INT,
        bWindow: BOOL,
        iWidth: INT,
        iHeight: INT,
        iExeType: INT,
        obShare: *mut ::std::os::raw::c_void,
        hParent: HWND,
        pObjectName: LPTSTR,
        pCommandLine: LPTSTR,
        stgThis: *mut ppbstg_anchor,
        obThis: *mut ::std::os::raw::c_void,
        obInst: *mut ::std::os::raw::c_void,
        hWnd: *mut HWND,
        context: *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_long,
    pub DW_PluginStop: unsafe extern "C" fn(
        pobThis: *mut ::std::os::raw::c_void,
        pstgThis: *mut ppbstg_anchor,
        pobInst: *mut ::std::os::raw::c_void,
        phWnd: *mut HWND
    ) -> ::std::os::raw::c_long,
    pub dwSyntaxGen: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        db: *mut ::std::os::raw::c_void,
        selectSyn: LPTSTR,
        tableName: LPTSTR,
        keys: LPTSTR,
        args: PDWBTBLARG
    ) -> LPTSTR,
    pub dwSyntaxGenForm: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        db: *mut ::std::os::raw::c_void,
        selectSyn: LPTSTR,
        tableName: LPTSTR,
        keys: LPTSTR,
        args: PDWBTBLARG
    ) -> LPTSTR,
    pub dwSyntaxFree: unsafe extern "C" fn(sa: ppbstg_anchor, progSyn: LPTSTR),
    pub dwGenDataListSyntax:
        unsafe extern "C" fn(sa: ppbstg_anchor, dtable: PDWDATATABLE, sp: LPTSTR, arg: PDWBTBLARG) -> LPTSTR,
    pub dwGenDataFormSyntax:
        unsafe extern "C" fn(sa: ppbstg_anchor, dtable: PDWDATATABLE, sp: LPTSTR, arg: PDWBTBLARG) -> LPTSTR,
    pub dwSyntaxFromSQL: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        db: *mut ::std::os::raw::c_void,
        selectSyn: LPTSTR,
        args: PDWBTBLARG,
        style: LPTSTR,
        errmsg: LPTSTR
    ) -> LPTSTR,
    pub dwSyntaxFromDesc: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        dtable: PDWDATATABLE,
        sp: LPTSTR,
        args: PDWBTBLARG,
        style: LPTSTR,
        errmsg: LPTSTR
    ) -> LPTSTR,
    pub dwClearOriginalValues: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_long,
    pub dwRowsDiscard: unsafe extern "C" fn(
        hWnd: HWND,
        lStartRow: ::std::os::raw::c_long,
        lEndRow: ::std::os::raw::c_long,
        queue: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwRowsMove: unsafe extern "C" fn(
        hWndFrom: HWND,
        lStartRow: ::std::os::raw::c_long,
        lEndRow: ::std::os::raw::c_long,
        queueFrom: ::std::os::raw::c_short,
        hWndTo: HWND,
        lInsertRow: ::std::os::raw::c_long,
        queueTo: ::std::os::raw::c_short,
        iFlags: UINT
    ) -> ::std::os::raw::c_short,
    pub dwCreateObjectStorage: unsafe extern "C" fn(
        hWnd: HWND,
        obInst: *mut ::std::os::raw::c_void,
        obThis: *mut ::std::os::raw::c_void
    ) -> *mut ::std::os::raw::c_void,
    pub dwDefaultArgs: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        selectSyn: LPTSTR,
        db: *mut ::std::os::raw::c_void,
        arg: PDWBTBLARG,
        rc: *mut ::std::os::raw::c_short
    ) -> LPTSTR,
    pub dwDefaultArgsFree: unsafe extern "C" fn(sa: ppbstg_anchor, syn: LPTSTR),
    pub dwCrosstabDef: unsafe extern "C" fn(
        prog: *mut ::std::os::raw::c_void,
        pData: *mut *mut ::std::os::raw::c_void
    ) -> HWND,
    pub dwCrosstabDefDynamic: unsafe extern "C" fn(
        pMainStg: *mut ::std::os::raw::c_void,
        pvData: *mut *mut ::std::os::raw::c_void
    ) -> HWND,
    pub dwCrosstabModifyDynamic: unsafe extern "C" fn(
        pMainStg: *mut ::std::os::raw::c_void,
        pvData: *mut *mut ::std::os::raw::c_void,
        pModel: PCROSSMODEL,
        baseTbl: *mut ::std::os::raw::c_void
    ) -> HWND,
    pub dwCrosstabModifyStatic: unsafe extern "C" fn(
        pMainStg: *mut ::std::os::raw::c_void,
        pvData: *mut *mut ::std::os::raw::c_void,
        pModel: PCROSSMODEL,
        baseTbl: *mut ::std::os::raw::c_void
    ) -> HWND,
    pub dwCrosstabBuildModel: unsafe extern "C" fn(
        prog: *mut ::std::os::raw::c_void,
        columns: LPTSTR,
        rows: LPTSTR,
        values: LPTSTR,
        baseTbl: *mut ::std::os::raw::c_void,
        runProg: *mut ::std::os::raw::c_void
    ) -> PCROSSMODEL,
    pub dwInfoBegin: unsafe extern "C" fn(prog: *mut ::std::os::raw::c_void, ia: PDWINFO),
    pub dwInfoBitmap: unsafe extern "C" fn(ia: PDWINFO, lbk: ::std::os::raw::c_long, bk: PDWINFOBITMAP),
    pub dwInfoBlob: unsafe extern "C" fn(ia: PDWINFO, lblob: ::std::os::raw::c_long, bm: PDWINFOBLOB),
    pub dwInfoInkpic: unsafe extern "C" fn(ia: PDWINFO, lblob: ::std::os::raw::c_long, bm: PDWINFOINKPIC),
    pub dwInfoButton: unsafe extern "C" fn(ia: PDWINFO, lblob: ::std::os::raw::c_long, bt: PDWINFOBUTTON),
    pub dwInfoColumn: unsafe extern "C" fn(ia: PDWINFO, lc: ::std::os::raw::c_long, c: PDWINFOCOL),
    pub dwInfoCompute: unsafe extern "C" fn(ia: PDWINFO, lc: ::std::os::raw::c_long, c: PDWINFOCOMPUTE),
    pub dwInfoCrosstab: unsafe extern "C" fn(ia: PDWINFO, lc: ::std::os::raw::c_long, c: PDWINFOCROSSTAB),
    pub dwInfoWpf: unsafe extern "C" fn(ia: PDWINFO, lblob: ::std::os::raw::c_long, w: PDWINFOWPF),
    pub dwInfoDataWindow: unsafe extern "C" fn(ia: PDWINFO, dwo: PDWOPTIONS),
    pub dwInfoEnd: unsafe extern "C" fn(ia: PDWINFO),
    pub dwInfoFunctions: unsafe extern "C" fn(
        arg1: ppbstg_anchor,
        hWnd: HWND,
        level: ::std::os::raw::c_short,
        iNumFuncs: *mut INT,
        pFuncs: *mut shlist
    ),
    pub dwInfoGetFirstColumn: unsafe extern "C" fn(ia: PDWINFO, c: PDWINFOTCOL) -> BOOL,
    pub dwInfoGetNextColumn: unsafe extern "C" fn(ia: PDWINFO, c: PDWINFOTCOL) -> BOOL,
    pub dwInfoGetDWWidth: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_long,
    pub dwInfoGetFirstObject: unsafe extern "C" fn(ia: PDWINFO) -> ::std::os::raw::c_long,
    pub dwInfoGetNextObject: unsafe extern "C" fn(ia: PDWINFO) -> ::std::os::raw::c_long,
    pub dwInfoGetObjectType:
        unsafe extern "C" fn(ia: PDWINFO, obj: ::std::os::raw::c_long) -> ::std::os::raw::c_short,
    pub dwInfoGraph: unsafe extern "C" fn(ia: PDWINFO, lgraph: ::std::os::raw::c_long, g: PDWINFOGRAPH),
    pub dwInfoGroupBox: unsafe extern "C" fn(ia: PDWINFO, lgraph: ::std::os::raw::c_long, g: PDWINFOGROUPBOX),
    pub dwInfoLine: unsafe extern "C" fn(ia: PDWINFO, lline: ::std::os::raw::c_long, l: PDWINFOLINE),
    pub dwInfoOle: unsafe extern "C" fn(ia: PDWINFO, lole: ::std::os::raw::c_long, o: PDWINFOOLE),
    pub dwInfoOval: unsafe extern "C" fn(ia: PDWINFO, loval: ::std::os::raw::c_long, o: PDWINFOOVAL),
    pub dwInfoProcessType: unsafe extern "C" fn(prog: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_short,
    pub dwInfoRect: unsafe extern "C" fn(ia: PDWINFO, lrect: ::std::os::raw::c_long, r: PDWINFORECT),
    pub dwInfoRegionFirst: unsafe extern "C" fn(ia: PDWINFO, r: PDWINFOREGION) -> BOOL,
    pub dwInfoRegionNext: unsafe extern "C" fn(ia: PDWINFO, r: PDWINFOREGION) -> BOOL,
    pub dwInfoReport: unsafe extern "C" fn(ia: PDWINFO, lrect: ::std::os::raw::c_long, r: PDWINFOREPORT),
    pub dwInfoResourceNames: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        lpLibrary: LPTSTR,
        lpEntry: LPTSTR
    ) -> *mut ::std::os::raw::c_void,
    pub dwInfoRoundRect: unsafe extern "C" fn(ia: PDWINFO, lrect: ::std::os::raw::c_long, r: PDWINFORRECT),
    pub dwInfoSparse: unsafe extern "C" fn(ia: PDWINFO) -> LPTSTR,
    pub dwInfoTable: unsafe extern "C" fn(ia: PDWINFO, t: PDWINFOTABLE),
    pub dwInfoTableExtract:
        unsafe extern "C" fn(prog: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
    pub dwInfoText: unsafe extern "C" fn(ia: PDWINFO, ltext: ::std::os::raw::c_long, t: PDWINFOTEXT),
    pub dwInfoVerifyCompute: unsafe extern "C" fn(gx: PDWEXPRTEST) -> ::std::os::raw::c_short,
    pub dwInfoVerifyFilter: unsafe extern "C" fn(gx: PDWEXPRTEST) -> ::std::os::raw::c_short,
    pub dwInfoVerifyStmt: unsafe extern "C" fn(gx: PDWEXPRTEST) -> *mut ::std::os::raw::c_void,
    pub dwInfoVerifyString: unsafe extern "C" fn(gx: PDWEXPRTEST) -> ::std::os::raw::c_short,
    pub dwInfoParseStmt: unsafe extern "C" fn(gx: PDWEXPRTEST) -> *mut ::std::os::raw::c_void,
    pub dwInfoSetPSRFileName: unsafe extern "C" fn(
        prog: *mut ::std::os::raw::c_void,
        szFileName: LPOLESTR,
        iLen: ::std::os::raw::c_short
    ),
    pub dwClientToObject: unsafe extern "C" fn(
        hWnd: HWND,
        lpName: LPTSTR,
        p: *mut POINT,
        ri: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_short,
    pub dwDBCancel: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwDeleteCrosstabSourceList:
        unsafe extern "C" fn(prog: *mut ::std::os::raw::c_void, source_names: *mut ::std::os::raw::c_void),
    pub dwDragObjectCreate: unsafe extern "C" fn(
        hWnd: HWND,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        pdwo: *mut *mut ::std::os::raw::c_void,
        row: *mut ::std::os::raw::c_long
    ) -> ::std::os::raw::c_short,
    pub dwDragObjectDestroy:
        unsafe extern "C" fn(hWnd: HWND, dwo: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_short,
    pub dwExprDialog: unsafe extern "C" fn(
        hWnd: HWND,
        hWndOwner: HWND,
        pTitle: LPTSTR,
        pExpr: *mut LPTSTR,
        bAllowNULL: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwValidationExprDialog: unsafe extern "C" fn(
        hWnd: HWND,
        hWndOwner: HWND,
        pTitle: LPTSTR,
        pExpr: *mut LPTSTR,
        bAllowNULL: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwExprValidate: unsafe extern "C" fn(hWnd: HWND, fexpr: LPTSTR) -> ::std::os::raw::c_short,
    pub dwFind: unsafe extern "C" fn(
        hWnd: HWND,
        fexpr: LPTSTR,
        srow: ::std::os::raw::c_long,
        erow: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long,
    pub dwFindGroupChange: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        level: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_long,
    pub dwFindRequired: unsafe extern "C" fn(
        hWnd: HWND,
        queue: ::std::os::raw::c_short,
        rrow: *mut ::std::os::raw::c_long,
        col: *mut ::std::os::raw::c_short,
        lpName: LPTSTR,
        bUpdate: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwFitColumn: unsafe extern "C" fn(
        hWnd: HWND,
        col: ::std::os::raw::c_short,
        lCount: ::std::os::raw::c_long,
        bPrinter: BOOL,
        lpszSeparator: LPTSTR
    ) -> ::std::os::raw::c_short,
    pub dwGenTableName: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        bText: BOOL,
        row: ::std::os::raw::c_int,
        rows_per_detail: ::std::os::raw::c_int,
        col: *mut ::std::os::raw::c_void,
        tableCount: ::std::os::raw::c_short,
        bCrosstab: BOOL,
        ci: ::std::os::raw::c_short
    ) -> LPTSTR,
    pub dwGenXTableName: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        subpool: pbstg_subpool,
        bText: BOOL,
        row: ::std::os::raw::c_int,
        rows_per_detail: ::std::os::raw::c_int,
        pcol: *mut ::std::os::raw::c_void,
        bCrosstab: BOOL,
        ci: ::std::os::raw::c_short
    ) -> LPTSTR,
    pub dwGetBandUnderMouse: unsafe extern "C" fn(hWnd: HWND, name: LPTSTR) -> ::std::os::raw::c_short,
    pub dwGetRowColUnderPoint: unsafe extern "C" fn(
        hWnd: HWND,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        hrow: *mut ::std::os::raw::c_long,
        hcol: *mut ::std::os::raw::c_short
    ) -> BOOL,
    pub dwIsPointOutsideEditCol:
        unsafe extern "C" fn(hWnd: HWND, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> BOOL,
    pub dwGetColumnFormat: unsafe extern "C" fn(
        hWnd: HWND,
        col: ::std::os::raw::c_short,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetColumnFormatLength:
        unsafe extern "C" fn(hWnd: HWND, col: ::std::os::raw::c_short) -> ::std::os::raw::c_short,
    pub dwGetColumnNumber: unsafe extern "C" fn(hWnd: HWND, colname: LPTSTR) -> ::std::os::raw::c_short,
    pub dwGetColumnType:
        unsafe extern "C" fn(hWnd: HWND, col: ::std::os::raw::c_short) -> ::std::os::raw::c_short,
    pub dwGetColumnValidation: unsafe extern "C" fn(
        hWnd: HWND,
        col: ::std::os::raw::c_short,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetColumnValidationLength:
        unsafe extern "C" fn(hWnd: HWND, col: ::std::os::raw::c_short) -> ::std::os::raw::c_short,
    pub dwGetColumnValue: unsafe extern "C" fn(
        hWnd: HWND,
        col: ::std::os::raw::c_short,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short,
        item: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetColumnValueLength: unsafe extern "C" fn(
        hWnd: HWND,
        col: ::std::os::raw::c_short,
        item: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetCurrentCol:
        unsafe extern "C" fn(hWnd: HWND, bName: ::std::os::raw::c_short) -> ::std::os::raw::c_long,
    pub dwGetCurrentRowCol: unsafe extern "C" fn(
        hWnd: HWND,
        row: *mut ::std::os::raw::c_long,
        col: *mut ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetCurrentText: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR, maxsize: ULONG) -> ULONG,
    pub dwGetCurrentTextLength: unsafe extern "C" fn(hWnd: HWND) -> ULONG,
    pub dwGetDBvendor: unsafe extern "C" fn(
        hWnd: HWND,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetDBvendorLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetDBlogid: unsafe extern "C" fn(
        hWnd: HWND,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetDBlogidLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetDBlogpass: unsafe extern "C" fn(
        hWnd: HWND,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetDBlogpassLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetDBserver: unsafe extern "C" fn(
        hWnd: HWND,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetDBserverLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetDBdatabase: unsafe extern "C" fn(
        hWnd: HWND,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetDBdatabaseLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetDBuserid: unsafe extern "C" fn(
        hWnd: HWND,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetDBuseridLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetDBdatabasepass: unsafe extern "C" fn(
        hWnd: HWND,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetDBdatabasepassLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetDBparm: unsafe extern "C" fn(
        hWnd: HWND,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetDBparmLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetDBError: unsafe extern "C" fn(
        hWnd: HWND,
        errtext: LPTSTR,
        maxsize: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_long,
    pub dwGetDBErrorLength: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGetItemDateTime: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: PSH_TIME,
        queue: ::std::os::raw::c_short,
        original: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwGetItemDecimal: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: PSH_DEC,
        queue: ::std::os::raw::c_short,
        orig: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwGetItemDouble: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: *mut f64,
        queue: ::std::os::raw::c_short,
        original: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwGetItemLength: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        queue: ::std::os::raw::c_short,
        original: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwGetItemLengthLong: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        queue: ::std::os::raw::c_short,
        original: BOOL
    ) -> ::std::os::raw::c_long,
    pub dwGetItemStatus: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        queue: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwGetItemLengthAndString: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: *mut LPTSTR,
        lMaxsize: ::std::os::raw::c_long,
        queue: ::std::os::raw::c_short,
        original: BOOL,
        lLength: *mut ::std::os::raw::c_long,
        sa: ppbstg_anchor,
        pool: pbstg_subpool
    ) -> ::std::os::raw::c_short,
    pub dwGetItemString: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_short,
        queue: ::std::os::raw::c_short,
        original: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwGetItemStringLong: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: LPTSTR,
        maxsize: ::std::os::raw::c_long,
        queue: ::std::os::raw::c_short,
        original: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwGetItemType:
        unsafe extern "C" fn(hWnd: HWND, col: ::std::os::raw::c_short) -> ::std::os::raw::c_short,
    pub dwGetNextModifiedRow: unsafe extern "C" fn(
        hWnd: HWND,
        startrow: ::std::os::raw::c_long,
        q: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_long,
    pub dwGetNotifyArgs: unsafe extern "C" fn(
        hWnd: HWND,
        given_DW: *mut ::std::os::raw::c_void
    ) -> *mut *mut ::std::os::raw::c_void,
    pub dwGetObjectUnderMouse: unsafe extern "C" fn(hWnd: HWND, name: LPTSTR) -> ::std::os::raw::c_short,
    pub dwGroupCalc: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwOleActivateItem: unsafe extern "C" fn(
        hWnd: HWND,
        ri: ::std::os::raw::c_long,
        ci: ::std::os::raw::c_short,
        iVerb: WORD
    ) -> ::std::os::raw::c_short,
    pub dwOleGetControlInfo: unsafe extern "C" fn(hWnd: HWND) -> PDWOLER,
    pub dwOleDeactivate:
        unsafe extern "C" fn(hWnd: HWND, bFromPostMessage: BOOL, lParamForPost: LPARAM) -> BOOL,
    pub dwOleIPactivate: unsafe extern "C" fn(hWnd: HWND, bActive: BOOL),
    pub dwOleIPResize: unsafe extern "C" fn(hWnd: HWND),
    pub dwOleReset: unsafe extern "C" fn(hWnd: HWND, bDestroy: BOOL),
    pub dwGetProgram: unsafe extern "C" fn(hWnd: HWND) -> *mut ::std::os::raw::c_void,
    pub dwPosition: unsafe extern "C" fn(
        hWnd: HWND,
        gobname: LPTSTR,
        bandname: LPTSTR,
        bFront: BOOL
    ) -> ::std::os::raw::c_short,
    pub dwQryDialog: unsafe extern "C" fn(
        hWnd: HWND,
        sql: LPTSTR,
        gx: PDWEXPRTEST,
        pCriteria: LPTSTR,
        args: PDWBTBLARG
    ) -> LPTSTR,
    pub dwCanChangeQryMode: unsafe extern "C" fn(hWnd: HWND, given_DW: *mut ::std::os::raw::c_void) -> BOOL,
    pub dwResetColumnValues:
        unsafe extern "C" fn(hWnd: HWND, col: ::std::os::raw::c_short) -> ::std::os::raw::c_short,
    pub dwResetTrans: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwRetrieve:
        unsafe extern "C" fn(hWnd: HWND, args: PDWRETRIEVEARGS, bRepaint: BOOL) -> ::std::os::raw::c_long,
    pub dwSendMessage: unsafe extern "C" fn(
        hWnd: HWND,
        iMessage: ::std::os::raw::c_uint,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> ::std::os::raw::c_long,
    pub dwSetColumnFormat:
        unsafe extern "C" fn(hWnd: HWND, col: ::std::os::raw::c_short, m: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetColumnValidation:
        unsafe extern "C" fn(hWnd: HWND, col: ::std::os::raw::c_short, m: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetColumnValue: unsafe extern "C" fn(
        hWnd: HWND,
        col: ::std::os::raw::c_short,
        m: LPTSTR,
        item: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwSetCurrentRowCol: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwSetCurrentText: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDBvendor: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDBlogid: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDBlogpass: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDBserver: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDBdatabase: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDBuserid: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDBdatabasepass: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDBparm: unsafe extern "C" fn(hWnd: HWND, v: LPTSTR) -> ::std::os::raw::c_short,
    pub dwSetDetailHeight: unsafe extern "C" fn(
        hWnd: HWND,
        lStartRow: ::std::os::raw::c_long,
        lEndRow: ::std::os::raw::c_long,
        lHeight: ::std::os::raw::c_long
    ) -> ::std::os::raw::c_short,
    pub dwSetHTMLAction: unsafe extern "C" fn(
        hWnd: HWND,
        lpstrAction: LPTSTR,
        lpstrContext: LPTSTR
    ) -> ::std::os::raw::c_short,
    pub dwSetEventFlags: unsafe extern "C" fn(
        hWnd: HWND,
        evtF1: ULONG,
        evtF2: ULONG,
        given_DW: *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_short,
    pub dwSetItemDateTime: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: PSH_TIME
    ) -> ::std::os::raw::c_short,
    pub dwSetItemDecimal: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_short,
    pub dwSetItemDouble: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: *mut f64
    ) -> ::std::os::raw::c_short,
    pub dwSetItemLong: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: *mut ::std::os::raw::c_long
    ) -> ::std::os::raw::c_short,
    pub dwSetItemReal: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: *mut f32
    ) -> ::std::os::raw::c_short,
    pub dwSetItemStatus: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        queue: ::std::os::raw::c_short,
        status: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwSetItemString: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: LPTSTR
    ) -> ::std::os::raw::c_short,
    pub dwSetItemULong: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short,
        v: *mut ::std::os::raw::c_ulong
    ) -> ::std::os::raw::c_short,
    pub dwSetItemNull: unsafe extern "C" fn(
        hWnd: HWND,
        row: ::std::os::raw::c_long,
        col: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_short,
    pub dwSetLibrary: unsafe extern "C" fn(
        pDW: *mut ::std::os::raw::c_void,
        files: *mut LPTSTR,
        filecount: UINT
    ) -> ::std::os::raw::c_short,
    pub dwSetRC: unsafe extern "C" fn(
        hWnd: HWND,
        rc: ::std::os::raw::c_long,
        given_DW: *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_short,
    pub dwSetTrans:
        unsafe extern "C" fn(hWnd: HWND, trans: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_short,
    pub dwShareData: unsafe extern "C" fn(dw_primary: HWND, dw_secondary: HWND) -> ::std::os::raw::c_short,
    pub dwShareDataOff: unsafe extern "C" fn(hWndOff: HWND) -> ::std::os::raw::c_short,
    pub dwSQLGetSyntax: unsafe extern "C" fn(hWnd: HWND) -> LPTSTR,
    pub dwSQLSetSyntax: unsafe extern "C" fn(hWnd: HWND, syn: LPTSTR) -> ::std::os::raw::c_short,
    pub dwStorageDump: unsafe extern "C" fn(hWnd: HWND, lpFile: LPTSTR) -> ::std::os::raw::c_short,
    pub dwStyleChange:
        unsafe extern "C" fn(hWnd: HWND, lStyle: ::std::os::raw::c_long) -> ::std::os::raw::c_short,
    pub dwUpdateSQLGetRow: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_long,
    pub dwUpdateSQLGetQ: unsafe extern "C" fn(hWnd: HWND) -> ::std::os::raw::c_short,
    pub dwGenerateHTMLForm: unsafe extern "C" fn(
        hWnd: HWND,
        lpszSyntax: *mut LPTSTR,
        lpszStyle: *mut LPTSTR,
        lpszAction: LPTSTR,
        startRow: ::std::os::raw::c_long,
        endRow: ::std::os::raw::c_long,
        startCol: ::std::os::raw::c_short,
        endCol: ::std::os::raw::c_short,
        queue: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_long,
    pub dwSaveAsAscii: unsafe extern "C" fn(
        hWnd: HWND,
        lpszSyntax: LPTSTR,
        lpszSeparator: LPTSTR,
        lpszQuote: LPTSTR,
        lpszLineFeed: LPTSTR,
        bRetainNewLineChar: BOOL
    ) -> ::std::os::raw::c_long,
    pub dwBuildXHTMLTemplate: unsafe extern "C" fn(
        hWnd: HWND,
        ppITemplate: *mut *mut IPB_XMLDOMDocument,
        ppIStyle: *mut *mut IPB_XMLDOMDocument
    ) -> ::std::os::raw::c_long,
    pub dwSetFullState: unsafe extern "C" fn(
        hWnd: HWND,
        pBlob: *mut ::std::os::raw::c_void,
        obthis: *mut ::std::os::raw::c_void,
        ulLen: ULONG,
        ppStgOpen: *mut *mut ::std::os::raw::c_void,
        ppProg: *mut *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_long,
    pub dwGetFullState: unsafe extern "C" fn(
        hWnd: HWND,
        ppBlob: *mut *mut ::std::os::raw::c_void,
        pRowCount: *mut ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long,
    pub dwGetChanges: unsafe extern "C" fn(
        hWnd: HWND,
        ppBlob: *mut *mut ::std::os::raw::c_void,
        pCookie: *mut ::std::os::raw::c_void,
        pRowCount: *mut ::std::os::raw::c_long
    ) -> ::std::os::raw::c_long,
    pub dwSetChanges: unsafe extern "C" fn(
        hWnd: HWND,
        pBlob: *mut ::std::os::raw::c_void,
        ulLen: ULONG,
        CR: ::std::os::raw::c_short
    ) -> ::std::os::raw::c_long,
    pub dwGetStateStatus:
        unsafe extern "C" fn(hWnd: HWND, ppBlob: *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long,
    pub dwSetBlobStorageNULL: unsafe extern "C" fn(hWnd: HWND),
    pub dwFreeBlob:
        unsafe extern "C" fn(hWnd: HWND, pBlob: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long,
    pub dwSetWebLoadCallback:
        unsafe extern "C" fn(hWnd: HWND, pCallback: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long,
    pub dwPaintDC: unsafe extern "C" fn(hWnd: HWND, hDC: HDC, pRect: LPRECT) -> ::std::os::raw::c_long,
    pub DS_SetNotifyFunc: unsafe extern "C" fn(
        Given_pDW: *mut ::std::os::raw::c_void,
        GivenFunction: *mut ::std::os::raw::c_void
    ) -> ::std::os::raw::c_long,
    pub dwPrintObject: unsafe extern "C" fn(
        given_pDW: *mut ::std::os::raw::c_void,
        Show_Cancel_DialogBox: BOOL,
        Print_Job_DC: HDC
    ) -> ::std::os::raw::c_long,
    pub dwGraphSaveAsPdf: unsafe extern "C" fn(
        pdw: *mut ::std::os::raw::c_void,
        lpszGraphName: LPTSTR,
        lpszFileNamePdf: LPTSTR
    ) -> ::std::os::raw::c_long,
    pub dwGetPsrDataVersion: unsafe extern "C" fn(szFileName: LPOLESTR) -> ::std::os::raw::c_long,
    pub dwSetTransparency: unsafe extern "C" fn(hWnd: HWND, nTransparency: INT),
    pub dwGenNameExhaustive: unsafe extern "C" fn(
        sa: ppbstg_anchor,
        name_hash: *mut shhash,
        subpool: pbstg_subpool,
        lpszGivenName: LPTSTR,
        bText: BOOL
    ) -> LPTSTR,
    pub DS_Destroy: unsafe extern "C" fn(givenDW: *mut ::std::os::raw::c_void),
    pub dwXMLGetDefaultEncoding: unsafe extern "C" fn(ppEncoding: *mut *mut IPB_String) -> HRESULT,
    pub PBGetDBIProc: unsafe extern "C" fn(arg1: LPTSTR, arg2: HINSTANCE) -> DBI_FUNC,
    pub DBI_DatabaseLoad: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut *mut shlist) -> HRESULT,
    pub DBI_TableLoad:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut *mut shlist) -> HRESULT,
    pub DBI_TableExplode: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: DBI_TBLPTR, arg3: BOOL) -> HRESULT,
    pub DBI_DeleteDir: unsafe extern "C" fn(arg1: ppbstg_anchor, arg2: *mut shlist) -> HRESULT,
    pub DBI_DeleteTable: unsafe extern "C" fn(arg1: ppbstg_anchor, arg2: DBI_TBLPTR) -> HRESULT,
    pub DBI_ComboList: unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: HWND),
    pub DBI_FindComboString:
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int, arg3: HWND) -> HRESULT,
    pub DBI_MatchCombo: unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: HWND,
        arg3: *mut ::std::os::raw::c_int
    ) -> HRESULT,
    pub DBI_MatchString: unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        lpszValue: LPTSTR,
        arg2: *mut ::std::os::raw::c_int
    ) -> HRESULT,
    pub DBI_AttrInfo: unsafe extern "C" fn(arg1: PDBI_COMMAND),
    pub DBI_Command_Tran: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR) -> HRESULT,
    pub DBI_Step: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR) -> HRESULT,
    pub DBI_Commit: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_Connect: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: LPTSTR,
        arg5: LPTSTR
    ) -> HRESULT,
    pub DBI_DummyConnect: unsafe extern "C" fn(
        arg1: ppbstg_anchor,
        arg2: DWORD,
        arg3: LPTSTR,
        arg4: LPTSTR,
        arg5: *mut PDBI_COMMAND
    ) -> HRESULT,
    pub DBI_DialogConnect:
        unsafe extern "C" fn(arg1: PDBI_SIGNON, arg2: DWORD, arg3: *mut PDBI_COMMAND) -> HRESULT,
    pub DBI_LogIn: unsafe extern "C" fn(
        arg1: ppbstg_anchor,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: LPTSTR,
        arg5: LPTSTR,
        arg6: LPTSTR,
        arg7: DWORD,
        arg8: *mut PDBI_COMMAND
    ) -> HRESULT,
    pub DBI_SetLogIn: unsafe extern "C" fn(
        arg1: ppbstg_anchor,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: ::std::os::raw::c_long,
        arg5: ::std::os::raw::c_long,
        arg6: *mut PDBI_COMMAND
    ) -> HRESULT,
    pub DBI_LogInAdoConnection: unsafe extern "C" fn(
        arg1: ppbstg_anchor,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: ::std::os::raw::c_long,
        arg5: DWORD,
        arg6: *mut PDBI_COMMAND
    ) -> HRESULT,
    pub DBI_SetConnect: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: LPTSTR,
        arg5: LPTSTR
    ) -> HRESULT,
    pub DBI_DatabaseInfo: unsafe extern "C" fn(arg1: PDBI_COMMAND),
    pub DBI_Describe: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: INT, arg3: *mut *mut shlist) -> HRESULT,
    pub DBI_Disconnect: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_ErrorSQL: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> BOOL,
    pub DBI_Execute: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: BOOL, arg3: LPTSTR) -> HRESULT,
    pub DBI_FetchNext: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut *mut shlist) -> HRESULT,
    pub DBI_Prepare: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: INT) -> HRESULT,
    pub DBI_PrepareWithParms:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut shlist, arg4: LPTSTR) -> HRESULT,
    pub DBI_RollBack: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_TerminateSQL: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_Rows: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut ::std::os::raw::c_long) -> HRESULT,
    pub DBI_GetPBTypeString: unsafe extern "C" fn(arg1: PDBI_COMMAND),
    pub DBI_DescribeExtra:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut *mut shlist) -> HRESULT,
    pub DBI_DateString: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut LPTSTR) -> HRESULT,
    pub DBI_DecimalString: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR) -> HRESULT,
    pub DBI_DoubleString: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR) -> HRESULT,
    pub DBI_Numeri_tstring: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: SHORT,
        arg4: TCHAR,
        arg5: BOOL,
        arg6: BOOL
    ) -> HRESULT,
    pub DBI_CursorConnect: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut PDBI_COMMAND) -> HRESULT,
    pub DBI_GetSelectItems: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut INT) -> HRESULT,
    pub DBI_GetSelectInfo: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: PDBI_BIND, arg3: INT) -> HRESULT,
    pub DBI_BindSelectBuffer: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: PDBI_BIND, arg3: INT) -> HRESULT,
    pub DBI_RuntimeFetchNext: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_MPowerFetchNext: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_StartTran: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_UniqueKey:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: LPTSTR, arg4: *mut LPTSTR) -> HRESULT,
    pub DBI_ParseFrom: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: *mut *mut shlist,
        arg4: *mut BOOL
    ) -> HRESULT,
    pub DBI_ParseColList: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: *mut shlist,
        arg4: BOOL,
        arg5: *mut *mut shlist,
        pExplodeStarFunc: EXPLODESTAR_FUNC,
        bColAlias: *mut BOOL
    ) -> HRESULT,
    pub DBI_DoubleTheQuotes:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut LPTSTR) -> HRESULT,
    pub DBI_FreeMem: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut ::std::os::raw::c_void) -> HRESULT,
    pub DBI_FreePrepList: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut shlist) -> HRESULT,
    pub DBI_FreeColBlkList: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut shlist) -> HRESULT,
    pub DBI_DialogBoxCenter: unsafe extern "C" fn(arg1: HWND) -> HRESULT,
    pub DBI_CleanUpColumnList: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_Cancel: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_ProcInfo: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: *mut *mut shlist,
        arg3: DBI_PROCFILTER::Type
    ) -> HRESULT,
    pub DBI_ProcText:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: DBI_PROCPTR, arg3: *mut LPTSTR) -> HRESULT,
    pub DBI_EventText: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: DBI_EVENTPTR) -> HRESULT,
    pub DBI_ProcDescribe:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: DBI_PROCPTR, arg3: *mut *mut shlist) -> HRESULT,
    pub DBI_DeleteProcDir: unsafe extern "C" fn(arg1: ppbstg_anchor, arg2: *mut shlist) -> HRESULT,
    pub DBI_ProcPrepare: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: INT) -> HRESULT,
    pub DBI_Parse: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR) -> HRESULT,
    pub DBI_OuterJoinSyntax:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: PDBI_OUTER, arg3: *mut LPTSTR) -> HRESULT,
    pub DBI_RuntimeExecute: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: ::std::os::raw::c_long,
        arg4: LPTSTR
    ) -> HRESULT,
    pub DBI_FormatHash: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_ValidHash: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_FreeValidHash: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut *mut shhash) -> HRESULT,
    pub DBI_EditStyleHash: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_EditUpdate: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut shlist, arg3: HWND) -> HRESULT,
    pub DBI_EditStyleInfo: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: PPDWXEDT, arg3: LPTSTR) -> HRESULT,
    pub DBI_LoadString: unsafe extern "C" fn(arg1: INT) -> LPTSTR,
    pub DBI_DynamicBind: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_GetNextResultSet: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_ViewText:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: LPTSTR, arg4: *mut LPTSTR) -> HRESULT,
    pub DBI_ExecPlan: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut LPTSTR) -> HRESULT,
    pub DBI_GetTimestamp: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut LPTSTR) -> HRESULT,
    pub DBI_DoCompare: unsafe extern "C" fn(arg1: LPTSTR, arg2: LPTSTR) -> BOOL,
    pub DBI_DoCompareFirst: unsafe extern "C" fn(arg1: LPTSTR, arg2: LPTSTR) -> BOOL,
    pub DBI_DoCompareFirstWithSkip: unsafe extern "C" fn(arg1: LPTSTR, arg2: LPTSTR) -> BOOL,
    pub DBI_ReadBlob: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: ppbstg_anchor,
        arg3: pbstg_subpool,
        arg4: LPTSTR,
        arg5: LPTSTR,
        arg6: LPTSTR,
        arg7: *mut *mut ::std::os::raw::c_void
    ) -> HRESULT,
    pub DBI_WriteBlob: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: ppbstg_anchor,
        arg3: pbstg_subpool,
        arg4: LPTSTR,
        arg5: LPTSTR,
        arg6: LPTSTR,
        arg7: *mut ::std::os::raw::c_void
    ) -> HRESULT,
    pub DBI_FillBlanks: unsafe extern "C" fn(arg1: LPTSTR, arg2: ::std::os::raw::c_int) -> HRESULT,
    pub DBI_DeleteSyntaxList: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_ParseParms: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut *mut shlist) -> HRESULT,
    pub DBI_ParseKeyWords: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut BOOL) -> HRESULT,
    pub DBI_FreeParmList: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut *mut shlist) -> HRESULT,
    pub DBI_GetParm: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: *mut shlist,
        arg3: LPTSTR,
        arg4: *mut PDBI_PARMBLK
    ) -> HRESULT,
    pub DBI_FetchFirst: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_FetchPrev: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_FetchRandom: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_FetchRelative: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_FetchLast: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> HRESULT,
    pub DBI_NewDBParm: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR) -> HRESULT,
    pub DBI_ReplaceDbParm: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: LPTSTR) -> HRESULT,
    pub DBI_DelimitReservedWord: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: ::std::os::raw::c_long
    ) -> HRESULT,
    pub DBI_DescribeInput: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR) -> HRESULT,
    pub DBI_DescribeOutput: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR) -> HRESULT,
    pub DBI_DWCursorConnect: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut PDBI_COMMAND) -> HRESULT,
    pub DBI_ParseColSubset: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: *mut shlist,
        arg4: ::std::os::raw::c_long,
        arg5: BOOL,
        arg6: *mut *mut shlist
    ) -> HRESULT,
    pub DBI_ParseBasicSelect:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut LPTSTR) -> HRESULT,
    pub DBI_CreatePrimaryKeySyntax: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: PDBI_PRIMARY_KEY,
        arg3: HWND,
        arg4: *mut ::std::os::raw::c_long
    ) -> HRESULT,
    pub DBI_CreateNoLogPKeySyntax: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: PDBI_PRIMARY_KEY,
        arg3: LPTSTR,
        arg4: HWND,
        arg5: ::std::os::raw::c_long
    ) -> HRESULT,
    pub DBI_AlterPrimaryKeySyntax: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: PDBI_PRIMARY_KEY,
        arg3: PDBI_PRIMARY_KEY,
        arg4: HWND,
        arg5: *mut ::std::os::raw::c_long
    ) -> HRESULT,
    pub DBI_CreateForeignKeySyntax: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: *mut shlist,
        arg3: *mut ::std::os::raw::c_long
    ) -> HRESULT,
    pub DBI_AlterForeignKeySyntax: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: *mut shlist,
        arg3: *mut shlist,
        arg4: HWND,
        arg5: *mut ::std::os::raw::c_long
    ) -> HRESULT,
    pub DBI_GetForeignKYOptions:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: PDBI_FOREIGNKYOPTIONS) -> HRESULT,
    pub DBI_LibraryName: unsafe extern "C" fn(
        arg1: ppbstg_anchor,
        arg2: DWORD,
        arg3: LPTSTR,
        arg4: LPTSTR,
        arg5: LPTSTR
    ) -> HRESULT,
    pub DBI_LookForKeyWord: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: *mut BOOL,
        arg5: *mut ::std::os::raw::c_long,
        arg6: *mut ::std::os::raw::c_long
    ) -> HRESULT,
    pub DBI_ParseIdentifier: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: PDBI_IDENT,
        arg3: *mut LPTSTR,
        pPrepareTblList: *mut shlist
    ) -> HRESULT,
    pub DBI_GetColumnExpression:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: SHORT, arg4: *mut LPTSTR) -> HRESULT,
    pub DBI_ParseWhere: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: ::std::os::raw::c_long,
        arg4: *mut *mut shlist,
        arg5: *mut ::std::os::raw::c_long,
        arg6: *mut INT
    ) -> HRESULT,
    pub DBI_FreeWhereList: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut shlist) -> HRESULT,
    pub DBI_PrimaryKeyReferences: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: LPTSTR,
        arg5: *mut *mut shlist
    ) -> HRESULT,
    pub DBI_PBToSQL: unsafe extern "C" fn(
        arg1: LPTSTR,
        arg2: ppbstg_anchor,
        arg3: pbstg_subpool,
        arg4: PDBI_COMMAND,
        arg5: *mut LPTSTR
    ) -> HRESULT,
    pub DBI_PBToArgs: unsafe extern "C" fn(
        arg1: LPTSTR,
        arg2: ppbstg_anchor,
        arg3: pbstg_subpool,
        arg4: *mut PDWBTBLARG
    ) -> HRESULT,
    pub DBI_ReleaseInputParms: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut *mut shlist) -> HRESULT,
    pub DBI_SynText: unsafe extern "C" fn(arg1: PDBI_COMMAND),
    pub DBI_PBC_DialogBox: unsafe extern "C" fn(
        arg1: HINSTANCE,
        arg2: INT,
        arg3: HWND,
        arg4: DBI_DLGPROC,
        arg5: *mut INT
    ) -> HRESULT,
    pub DBI_PBC_DialogBoxParam: unsafe extern "C" fn(
        arg1: HINSTANCE,
        arg2: INT,
        arg3: HWND,
        arg4: DBI_DLGPROC,
        arg5: LPARAM,
        arg6: *mut INT
    ) -> HRESULT,
    pub DBI_PBC_ShowWindow: unsafe extern "C" fn(arg1: HWND, arg2: INT) -> HRESULT,
    pub DBI_DBHandle:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: WORD, arg3: *mut ::std::os::raw::c_long) -> HRESULT,
    pub DBI_GetAdoConnection:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: *mut ::std::os::raw::c_long) -> HRESULT,
    pub DBI_SearchReplace: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: LPTSTR,
        arg4: LPTSTR,
        arg5: LPTSTR,
        arg6: *mut LPTSTR
    ) -> HRESULT,
    pub DBI_CtrlChars2Text:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut LPTSTR) -> HRESULT,
    pub DBI_Text2CtrlChars:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: LPTSTR, arg3: *mut LPTSTR) -> HRESULT,
    pub DBI_SQLCacheBegin:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: UINT, arg3: *mut pCacheList) -> HRESULT,
    pub DBI_SQLCacheEnd: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: pCacheList) -> HRESULT,
    pub DBI_SQLCacheFlushEntries: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: pCacheList) -> HRESULT,
    pub DBI_SQLCacheRegisterSQLStatement: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: LPTSTR,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut pCacheEntry
    ) -> HRESULT,
    pub DBI_SQLCacheRegisterDescribe:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: pCacheEntry, arg3: *mut shlist) -> HRESULT,
    pub DBI_SQLCacheRequestSqlStatement: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: pCacheList,
        arg3: LPTSTR,
        arg4: *mut pCacheEntry
    ) -> HRESULT,
    pub DBI_SQLCacheRequestDescribe:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: pCacheEntry) -> *mut shlist,
    pub DBI_SQLCacheMakeSQLStatementAvailable:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: pCacheEntry, arg3: pCacheList) -> HRESULT,
    pub DBI_SQLCacheSetCacheSize:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: pCacheList, arg3: LPTSTR) -> HRESULT,
    pub DBI_SQLCacheDiscardEntry:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: pCacheEntry, arg3: BOOL) -> HRESULT,
    pub DBI_SQLCacheConnectSetup: unsafe extern "C" fn(pDB: PDBI_COMMAND) -> HRESULT,
    pub DBI_SQLCacheConnectDrop: unsafe extern "C" fn(pDB: PDBI_COMMAND) -> HRESULT,
    pub DBI_SQLCacheSetSelectCacheSize: unsafe extern "C" fn(pDB: PDBI_COMMAND) -> HRESULT,
    pub DBI_SQLCacheStatistics: unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: BOOL) -> LPTSTR,
    pub DBI_GetIdentityValue: unsafe extern "C" fn(
        arg1: PDBI_COMMAND,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_long,
        arg4: ::std::os::raw::c_long,
        arg5: LPTSTR,
        arg6: LPTSTR,
        arg7: SHORT,
        arg8: *mut BOOL
    ) -> HRESULT,
    pub DBI_FormatWindowsEOL: unsafe extern "C" fn(arg1: ppbstg_anchor, arg2: LPTSTR) -> LPTSTR,
    pub DBI_RegisterVendor: unsafe extern "C" fn(arg1: LPTSTR, arg2: LPTSTR) -> HRESULT,
    pub DBI_UnRegisterVendor: unsafe extern "C" fn(arg1: LPTSTR) -> HRESULT,
    pub DBI_IsCache: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> BOOL,
    pub DBI_IsConnInServerTrans: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> BOOL,
    pub DBI_IsConnFromServer: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> BOOL,
    pub DBI_ConvertHexStringToBlob:
        unsafe extern "C" fn(arg1: LPTSTR, arg2: LPBYTE, arg3: *mut ::std::os::raw::c_long) -> HRESULT,
    pub CreateDBIConnect: unsafe extern "C" fn(arg1: PDBI_COMMAND) -> *mut CPB_DBI_Connection,
    pub CreateDBITableSchema:
        unsafe extern "C" fn(arg1: PDBI_COMMAND, arg2: DBI_TBLPTR) -> *mut CPB_DBI_TableSchema,
    pub rtdb_close: unsafe extern "C" fn(
        obthis: POB_THIS,
        target_object: POB_RUNTIME_INST,
        cursor_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_commit: unsafe extern "C" fn(obthis: POB_THIS, transaction_object: POB_RUNTIME_INST) -> INT,
    pub rtdb_delete: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        transaction_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_deletewithcurs: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        target_object: POB_RUNTIME_INST,
        cursor_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_describe: unsafe extern "C" fn(
        obthis: POB_THIS,
        sqlsa_object: POB_RUNTIME_INST,
        sqlda_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_execute: unsafe extern "C" fn(
        obthis: POB_THIS,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        transaction_object: POB_RUNTIME_INST,
        procedure_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_executedyn: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        sqlsa_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_executeimmed: unsafe extern "C" fn(
        obthis: POB_THIS,
        syntax_string: LPTSTR,
        transaction_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_execdynproc: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        sqlsa_object: POB_RUNTIME_INST,
        procedure_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_execdynwithdesc: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        sqlsa_object: POB_RUNTIME_INST,
        sqlda_object: POB_RUNTIME_INST,
        procedure_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_fetch: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_outputs: UINT,
        output_expr_array: POB_DATA,
        target_object: POB_RUNTIME_INST,
        cursor_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_fetchwithdesc: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        sqlsa_object: POB_RUNTIME_INST,
        sqlda_object: POB_RUNTIME_INST,
        cursor_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_insert: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        transaction_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_open: unsafe extern "C" fn(
        obthis: POB_THIS,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        transaction_object: POB_RUNTIME_INST,
        cursor_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_opendyn: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        sqlsa_object: POB_RUNTIME_INST,
        cursor_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_opendynwithdesc: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        sqlsa_object: POB_RUNTIME_INST,
        sqlda_object: POB_RUNTIME_INST,
        cursor_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_prepare: unsafe extern "C" fn(
        obthis: POB_THIS,
        sqlsa_object: POB_RUNTIME_INST,
        syntax_string: LPTSTR,
        transaction_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_rollback: unsafe extern "C" fn(obthis: POB_THIS, transaction_object: POB_RUNTIME_INST) -> INT,
    pub rtdb_select: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        no_outputs: UINT,
        input_expr_array: POB_DATA,
        output_expr_array: POB_DATA,
        transaction_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_selectblob: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        no_outputs: UINT,
        input_expr_array: POB_DATA,
        output_expr_array: POB_DATA,
        transaction_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_start: unsafe extern "C" fn(obthis: POB_THIS, transaction_object: POB_RUNTIME_INST) -> INT,
    pub rtdb_stop: unsafe extern "C" fn(obthis: POB_THIS, transaction_object: POB_RUNTIME_INST) -> INT,
    pub rtdb_update: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        transaction_object: POB_RUNTIME_INST
    ) -> INT,
    pub rtdb_updatewithcurs: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        target_object: POB_RUNTIME_INST,
        cursor_statement_block: PRTDB_STATEMENT
    ) -> INT,
    pub rtdb_updateblob: unsafe extern "C" fn(
        obthis: POB_THIS,
        statement_offset: OB_CONST_REF,
        no_inputs: UINT,
        input_expr_array: POB_DATA,
        transaction_object: POB_RUNTIME_INST,
        blob_var: POB_DATA
    ) -> INT,
    pub rtdb_trans_pool_login: unsafe extern "C" fn(
        pThis: POB_THIS,
        lpszDBMS: LPTSTR,
        lpszServerName: LPTSTR,
        lpszLogId: LPTSTR,
        lpszLogPassWord: LPTSTR,
        lpszDbParm: LPTSTR,
        lpszDatabase: LPTSTR,
        lpszUserID: LPTSTR,
        lpszDBPass: LPTSTR,
        lpszLock: LPTSTR,
        pTransactionObject: POB_RUNTIME_INST,
        pbCalledLogIn: *mut BOOL
    ) -> PDBI_COMMAND,
    pub rtdb_trans_pool_disconnect: unsafe extern "C" fn(pThis: POB_THIS, pCommandBlock: PDBI_COMMAND),
    pub ob_set_curr_rtinst_and_return: unsafe extern "C" fn(obthis: POB_THIS, new_rtinst: POB_RUNTIME_INST),
    pub ob_unset_curr_rtinst_and_return: unsafe extern "C" fn(obthis: POB_THIS),
    pub ob_open_trace:
        unsafe extern "C" fn(obthis: POB_THIS, filename: LPTSTR, kind: OB_TIMERKIND) -> OB_ERROR_RETURN,
    pub ob_close_trace: unsafe extern "C" fn(obthis: POB_THIS) -> OB_ERROR_RETURN,
    pub ob_begin_trace: unsafe extern "C" fn(obthis: POB_THIS, message: LPTSTR) -> OB_ERROR_RETURN,
    pub ob_end_trace: unsafe extern "C" fn(obthis: POB_THIS) -> OB_ERROR_RETURN,
    pub ob_enable_event_trace: unsafe extern "C" fn(obthis: POB_THIS, event: OB_TRACEID) -> OB_ERROR_RETURN,
    pub ob_disable_event_trace: unsafe extern "C" fn(obthis: POB_THIS, event: OB_TRACEID) -> OB_ERROR_RETURN,
    pub otcg_string_cat: unsafe extern "C" fn(
        obthis: POB_THIS,
        string1: LPTSTR,
        string1_null: BOOL,
        string2: LPTSTR,
        string2_null: BOOL
    ) -> INT,
    pub otcg_binary_cat: unsafe extern "C" fn(
        obthis: POB_THIS,
        binary1: PSH_BINARY,
        binary1_null: BOOL,
        binary2: PSH_BINARY,
        binary2_null: BOOL
    ) -> INT,
    pub otcg_add_object_argument_reference: unsafe extern "C" fn(obthis: POB_THIS, data_node: POB_DATA),
    pub otcg_embedded_group:
        unsafe extern "C" fn(obthis: POB_THIS, curr_group: POB_GROUP, class_id: OB_CLASS_ID) -> POB_GROUP
}
impl Api {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>
    {
        let __library = library.into();
        let pbstg_begin = __library.get(b"pbstg_begin\0").map(|sym| *sym)?;
        let pbstg_begin_allocflags = __library.get(b"pbstg_begin_allocflags\0").map(|sym| *sym)?;
        let pbstg_begin_nofast = __library.get(b"pbstg_begin_nofast\0").map(|sym| *sym)?;
        let pbstg_end = __library.get(b"pbstg_end\0").map(|sym| *sym)?;
        let pbstg_free_pool = __library.get(b"pbstg_free_pool\0").map(|sym| *sym)?;
        let pbstg_new_pool = __library.get(b"pbstg_new_pool\0").map(|sym| *sym)?;
        let pbstg_new_pool_nofast = __library.get(b"pbstg_new_pool_nofast\0").map(|sym| *sym)?;
        let pbstg_new_pool_with_size = __library.get(b"pbstg_new_pool_with_size\0").map(|sym| *sym)?;
        let pbstg_new_pool_with_size_nofast =
            __library.get(b"pbstg_new_pool_with_size_nofast\0").map(|sym| *sym)?;
        let pbstg_set_pool_name = __library.get(b"pbstg_set_pool_name\0").map(|sym| *sym)?;
        let pbstg_set_poolpagesize = __library.get(b"pbstg_set_poolpagesize\0").map(|sym| *sym)?;
        let pbstg_write_debug = __library.get(b"pbstg_write_debug\0").map(|sym| *sym)?;
        let pbstg_stat = __library.get(b"pbstg_stat\0").map(|sym| *sym)?;
        let pbstg_shrink = __library.get(b"pbstg_shrink\0").map(|sym| *sym)?;
        let pbstg_nextGeneration = __library.get(b"pbstg_nextGeneration\0").map(|sym| *sym)?;
        let pbstg_dumpLeaks = __library.get(b"pbstg_dumpLeaks\0").map(|sym| *sym)?;
        let pbstg_dumpHeap = __library.get(b"pbstg_dumpHeap\0").map(|sym| *sym)?;
        let pbstg_alloc = __library.get(b"pbstg_alloc\0").map(|sym| *sym)?;
        let pbstg_free = __library.get(b"pbstg_free\0").map(|sym| *sym)?;
        let pbstg_realloc = __library.get(b"pbstg_realloc\0").map(|sym| *sym)?;
        let pbstg_size = __library.get(b"pbstg_size\0").map(|sym| *sym)?;
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
        let pbstg_dde_strdup = __library.get(b"pbstg_dde_strdup\0").map(|sym| *sym)?;
        let pbstg_huge_memcmp = __library.get(b"pbstg_huge_memcmp\0").map(|sym| *sym)?;
        let pbstg_huge_memcpy = __library.get(b"pbstg_huge_memcpy\0").map(|sym| *sym)?;
        let pbstg_huge_memmove = __library.get(b"pbstg_huge_memmove\0").map(|sym| *sym)?;
        let pbstg_huge_memset = __library.get(b"pbstg_huge_memset\0").map(|sym| *sym)?;
        let pbstg_huge_strchr = __library.get(b"pbstg_huge_strchr\0").map(|sym| *sym)?;
        let pbstg_huge_strcpy = __library.get(b"pbstg_huge_strcpy\0").map(|sym| *sym)?;
        let pbstg_huge_strlen = __library.get(b"pbstg_huge_strlen\0").map(|sym| *sym)?;
        let pbstg_huge_strncpy = __library.get(b"pbstg_huge_strncpy\0").map(|sym| *sym)?;
        let pbstg_huge_strstr = __library.get(b"pbstg_huge_strstr\0").map(|sym| *sym)?;
        let OS_UtilGetProfInt = __library.get(b"OS_UtilGetProfInt\0").map(|sym| *sym)?;
        let OS_UtilGetProfStr = __library.get(b"OS_UtilGetProfStr\0").map(|sym| *sym)?;
        let OS_UtilPutProfStr = __library.get(b"OS_UtilPutProfStr\0").map(|sym| *sym)?;
        let OS_NetworkGetProfInt = __library.get(b"OS_NetworkGetProfInt\0").map(|sym| *sym)?;
        let OS_NetworkGetProfStr = __library.get(b"OS_NetworkGetProfStr\0").map(|sym| *sym)?;
        let OS_NetworkPutProfStr = __library.get(b"OS_NetworkPutProfStr\0").map(|sym| *sym)?;
        let OS_NetworkSharedProfileAccessible =
            __library.get(b"OS_NetworkSharedProfileAccessible\0").map(|sym| *sym)?;
        let OS_UtilGetInitPath = __library.get(b"OS_UtilGetInitPath\0").map(|sym| *sym)?;
        let OS_UtilPutInitPath = __library.get(b"OS_UtilPutInitPath\0").map(|sym| *sym)?;
        let UtilIniFile = __library.get(b"UtilIniFile\0").map(|sym| *sym)?;
        let FDCC_Get_Registry_Entry = __library.get(b"FDCC_Get_Registry_Entry\0").map(|sym| *sym)?;
        let FDCC_Get_CSIDL_Path = __library.get(b"FDCC_Get_CSIDL_Path\0").map(|sym| *sym)?;
        let FDCC_Create_Registry_Entry = __library.get(b"FDCC_Create_Registry_Entry\0").map(|sym| *sym)?;
        let FDCC_Get_Install_Location = __library.get(b"FDCC_Get_Install_Location\0").map(|sym| *sym)?;
        let FDCC_New_INI_Installed = __library.get(b"FDCC_New_INI_Installed\0").map(|sym| *sym)?;
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
        let PBUNI_memchr = __library.get(b"PBUNI_memchr\0").map(|sym| *sym)?;
        let pbstg_lchrcmp = __library.get(b"pbstg_lchrcmp\0").map(|sym| *sym)?;
        let pbstg_lchrcmpi = __library.get(b"pbstg_lchrcmpi\0").map(|sym| *sym)?;
        let os_vabuild_start = __library.get(b"os_vabuild_start\0").map(|sym| *sym)?;
        let os_vabuild_ptrarg = __library.get(b"os_vabuild_ptrarg\0").map(|sym| *sym)?;
        let os_vabuild_end = __library.get(b"os_vabuild_end\0").map(|sym| *sym)?;
        let os_vabuild_add = __library.get(b"os_vabuild_add\0").map(|sym| *sym)?;
        let sh_dbg_console_init = __library.get(b"sh_dbg_console_init\0").map(|sym| *sym)?;
        let sh_dbg_console_out = __library.get(b"sh_dbg_console_out\0").map(|sym| *sym)?;
        let sh_dbg_console_lock = __library.get(b"sh_dbg_console_lock\0").map(|sym| *sym)?;
        let sh_dbg_console_unlock = __library.get(b"sh_dbg_console_unlock\0").map(|sym| *sym)?;
        let sh_dbg_init = __library.get(b"sh_dbg_init\0").map(|sym| *sym)?;
        let sh_dbg_this = __library.get(b"sh_dbg_this\0").map(|sym| *sym)?;
        let sh_dbg_term = __library.get(b"sh_dbg_term\0").map(|sym| *sym)?;
        let sh_dbg_read_input = __library.get(b"sh_dbg_read_input\0").map(|sym| *sym)?;
        let sh_dbg_outfile = __library.get(b"sh_dbg_outfile\0").map(|sym| *sym)?;
        let sh_dbg_open = __library.get(b"sh_dbg_open\0").map(|sym| *sym)?;
        let sh_dbg_close = __library.get(b"sh_dbg_close\0").map(|sym| *sym)?;
        let sh_dbg_set = __library.get(b"sh_dbg_set\0").map(|sym| *sym)?;
        let sh_dbg_del = __library.get(b"sh_dbg_del\0").map(|sym| *sym)?;
        let sh_dbg_header = __library.get(b"sh_dbg_header\0").map(|sym| *sym)?;
        let sh_dbg_indent = __library.get(b"sh_dbg_indent\0").map(|sym| *sym)?;
        let sh_dbg_set_this = __library.get(b"sh_dbg_set_this\0").map(|sym| *sym)?;
        let sh_dbg_out = __library.get(b"sh_dbg_out\0").map(|sym| *sym)?;
        let sh_dbg_start_indent = __library.get(b"sh_dbg_start_indent\0").map(|sym| *sym)?;
        let sh_dbg_end_indent = __library.get(b"sh_dbg_end_indent\0").map(|sym| *sym)?;
        let sh_dbg_enter = __library.get(b"sh_dbg_enter\0").map(|sym| *sym)?;
        let sh_dbg_leave = __library.get(b"sh_dbg_leave\0").map(|sym| *sym)?;
        let sh_dbg_on = __library.get(b"sh_dbg_on\0").map(|sym| *sym)?;
        let sh_dbg_off = __library.get(b"sh_dbg_off\0").map(|sym| *sym)?;
        let sh_dbg_query = __library.get(b"sh_dbg_query\0").map(|sym| *sym)?;
        let sh_dbg_is_hdr_on = __library.get(b"sh_dbg_is_hdr_on\0").map(|sym| *sym)?;
        let sh_dbg_is_indent_on = __library.get(b"sh_dbg_is_indent_on\0").map(|sym| *sym)?;
        let osAssert = __library.get(b"osAssert\0").map(|sym| *sym)?;
        let shlist_delete = __library.get(b"shlist_delete\0").map(|sym| *sym)?;
        let shlist_deleteFree = __library.get(b"shlist_deleteFree\0").map(|sym| *sym)?;
        let shlist_get_next = __library.get(b"shlist_get_next\0").map(|sym| *sym)?;
        let shlist_get_prev = __library.get(b"shlist_get_prev\0").map(|sym| *sym)?;
        let shlist_putafter = __library.get(b"shlist_putafter\0").map(|sym| *sym)?;
        let shlist_addafter = __library.get(b"shlist_addafter\0").map(|sym| *sym)?;
        let shlist_addbefore = __library.get(b"shlist_addbefore\0").map(|sym| *sym)?;
        let shlist_remove = __library.get(b"shlist_remove\0").map(|sym| *sym)?;
        let shlist_insert_at_curr = __library.get(b"shlist_insert_at_curr\0").map(|sym| *sym)?;
        let shlist_insert = __library.get(b"shlist_insert\0").map(|sym| *sym)?;
        let shlist_new = __library.get(b"shlist_new\0").map(|sym| *sym)?;
        let shlist_curr_node = __library.get(b"shlist_curr_node\0").map(|sym| *sym)?;
        let shlist_get_count = __library.get(b"shlist_get_count\0").map(|sym| *sym)?;
        let shlist_get_first = __library.get(b"shlist_get_first\0").map(|sym| *sym)?;
        let shlist_get_last = __library.get(b"shlist_get_last\0").map(|sym| *sym)?;
        let shlist_get_curr = __library.get(b"shlist_get_curr\0").map(|sym| *sym)?;
        let shlist_update = __library.get(b"shlist_update\0").map(|sym| *sym)?;
        let shlist_get_handle = __library.get(b"shlist_get_handle\0").map(|sym| *sym)?;
        let shlist_set_current = __library.get(b"shlist_set_current\0").map(|sym| *sym)?;
        let shlist_traversal = __library.get(b"shlist_traversal\0").map(|sym| *sym)?;
        let shlist_sort = __library.get(b"shlist_sort\0").map(|sym| *sym)?;
        let shlist_sort_param = __library.get(b"shlist_sort_param\0").map(|sym| *sym)?;
        let sh_grwblk_init = __library.get(b"sh_grwblk_init\0").map(|sym| *sym)?;
        let sh_new_grwblk = __library.get(b"sh_new_grwblk\0").map(|sym| *sym)?;
        let sh_set_grwblk_item = __library.get(b"sh_set_grwblk_item\0").map(|sym| *sym)?;
        let sh_add_to_grwblk = __library.get(b"sh_add_to_grwblk\0").map(|sym| *sym)?;
        let sh_append_to_grwblk = __library.get(b"sh_append_to_grwblk\0").map(|sym| *sym)?;
        let sh_grwblk_delete = __library.get(b"sh_grwblk_delete\0").map(|sym| *sym)?;
        let sh_grwblk_close = __library.get(b"sh_grwblk_close\0").map(|sym| *sym)?;
        let ob_set_session_icontext = __library.get(b"ob_set_session_icontext\0").map(|sym| *sym)?;
        let ob_set_main_obthis = __library.get(b"ob_set_main_obthis\0").map(|sym| *sym)?;
        let rt_move_thread = __library.get(b"rt_move_thread\0").map(|sym| *sym)?;
        let rt_clear_thread = __library.get(b"rt_clear_thread\0").map(|sym| *sym)?;
        let rt_get_current_this = __library.get(b"rt_get_current_this\0").map(|sym| *sym)?;
        let rt_set_current_this = __library.get(b"rt_set_current_this\0").map(|sym| *sym)?;
        let rt_add_task = __library.get(b"rt_add_task\0").map(|sym| *sym)?;
        let rt_free_task = __library.get(b"rt_free_task\0").map(|sym| *sym)?;
        let rt_get_current_task_info = __library.get(b"rt_get_current_task_info\0").map(|sym| *sym)?;
        let rt_set_current_task_info = __library.get(b"rt_set_current_task_info\0").map(|sym| *sym)?;
        let rt_get_free_task_slot = __library.get(b"rt_get_free_task_slot\0").map(|sym| *sym)?;
        let rt_is_running_exe = __library.get(b"rt_is_running_exe\0").map(|sym| *sym)?;
        let shhash_new = __library.get(b"shhash_new\0").map(|sym| *sym)?;
        let shhash_new_arg = __library.get(b"shhash_new_arg\0").map(|sym| *sym)?;
        let shhash_delete = __library.get(b"shhash_delete\0").map(|sym| *sym)?;
        let shhash_clear = __library.get(b"shhash_clear\0").map(|sym| *sym)?;
        let shhash_get_first = __library.get(b"shhash_get_first\0").map(|sym| *sym)?;
        let shhash_get_next = __library.get(b"shhash_get_next\0").map(|sym| *sym)?;
        let shhash_insert = __library.get(b"shhash_insert\0").map(|sym| *sym)?;
        let shhash_search = __library.get(b"shhash_search\0").map(|sym| *sym)?;
        let shhash_search_arg = __library.get(b"shhash_search_arg\0").map(|sym| *sym)?;
        let shhash_search_unique = __library.get(b"shhash_search_unique\0").map(|sym| *sym)?;
        let shhash_search_unique_arg = __library.get(b"shhash_search_unique_arg\0").map(|sym| *sym)?;
        let shhash_searchNext = __library.get(b"shhash_searchNext\0").map(|sym| *sym)?;
        let shhash_searchSlot = __library.get(b"shhash_searchSlot\0").map(|sym| *sym)?;
        let shhash_remove = __library.get(b"shhash_remove\0").map(|sym| *sym)?;
        let shhash_statistics = __library.get(b"shhash_statistics\0").map(|sym| *sym)?;
        let shhash_traversal = __library.get(b"shhash_traversal\0").map(|sym| *sym)?;
        let obAllocDebug = __library.get(b"obAllocDebug\0").map(|sym| *sym)?;
        let obFreeDebug = __library.get(b"obFreeDebug\0").map(|sym| *sym)?;
        let ob_add_const_data = __library.get(b"ob_add_const_data\0").map(|sym| *sym)?;
        let parse_stringvalue_hash = __library.get(b"parse_stringvalue_hash\0").map(|sym| *sym)?;
        let obconpool_stringvalue_del = __library.get(b"obconpool_stringvalue_del\0").map(|sym| *sym)?;
        let ob_looksym_keyfunc = __library.get(b"ob_looksym_keyfunc\0").map(|sym| *sym)?;
        let ob_looksym_reference = __library.get(b"ob_looksym_reference\0").map(|sym| *sym)?;
        let ob_looksym_delete = __library.get(b"ob_looksym_delete\0").map(|sym| *sym)?;
        let ob_dynarray_index = __library.get(b"ob_dynarray_index\0").map(|sym| *sym)?;
        let ob_dynarray_grow = __library.get(b"ob_dynarray_grow\0").map(|sym| *sym)?;
        let ob_narray_create_static = __library.get(b"ob_narray_create_static\0").map(|sym| *sym)?;
        let ob_narray_create_dynamic = __library.get(b"ob_narray_create_dynamic\0").map(|sym| *sym)?;
        let ob_narray_dynamic_item_init_callback =
            __library.get(b"ob_narray_dynamic_item_init_callback\0").map(|sym| *sym)?;
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
        let sh_MAX_DEC = __library.get(b"sh_MAX_DEC\0").map(|sym| *sym)?;
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
        let ConvertOldDecToString = __library.get(b"ConvertOldDecToString\0").map(|sym| *sym)?;
        let shdtDayName = __library.get(b"shdtDayName\0").map(|sym| *sym)?;
        let shdtDayOfWeek = __library.get(b"shdtDayOfWeek\0").map(|sym| *sym)?;
        let shdtBuildTime = __library.get(b"shdtBuildTime\0").map(|sym| *sym)?;
        let shdtDiffDate = __library.get(b"shdtDiffDate\0").map(|sym| *sym)?;
        let shdtDiffTime = __library.get(b"shdtDiffTime\0").map(|sym| *sym)?;
        let shdtDiffMSec = __library.get(b"shdtDiffMSec\0").map(|sym| *sym)?;
        let shdtNow = __library.get(b"shdtNow\0").map(|sym| *sym)?;
        let shdtParse = __library.get(b"shdtParse\0").map(|sym| *sym)?;
        let shdtParseEx = __library.get(b"shdtParseEx\0").map(|sym| *sym)?;
        let shdtParseStringEx = __library.get(b"shdtParseStringEx\0").map(|sym| *sym)?;
        let shdtParseStringExWithLcid = __library.get(b"shdtParseStringExWithLcid\0").map(|sym| *sym)?;
        let shdtParseToString = __library.get(b"shdtParseToString\0").map(|sym| *sym)?;
        let shdtRelativeDate = __library.get(b"shdtRelativeDate\0").map(|sym| *sym)?;
        let shdtToMJDDate = __library.get(b"shdtToMJDDate\0").map(|sym| *sym)?;
        let shdtToMJDTime = __library.get(b"shdtToMJDTime\0").map(|sym| *sym)?;
        let shdtToMJDTimestamp = __library.get(b"shdtToMJDTimestamp\0").map(|sym| *sym)?;
        let shMJDDateTodt = __library.get(b"shMJDDateTodt\0").map(|sym| *sym)?;
        let shMJDTimeTodt = __library.get(b"shMJDTimeTodt\0").map(|sym| *sym)?;
        let shMJDTimestampTodt = __library.get(b"shMJDTimestampTodt\0").map(|sym| *sym)?;
        let shdtString = __library.get(b"shdtString\0").map(|sym| *sym)?;
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
        let ob_group_is_normalized_window =
            __library.get(b"ob_group_is_normalized_window\0").map(|sym| *sym)?;
        let ob_group_set_normalized_window =
            __library.get(b"ob_group_set_normalized_window\0").map(|sym| *sym)?;
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
        let ob_add_liblist = __library.get(b"ob_add_liblist\0").map(|sym| *sym)?;
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
        let ob_reload_group_source = __library.get(b"ob_reload_group_source\0").map(|sym| *sym)?;
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
        let obIsClassAutoinstantiate = __library.get(b"obIsClassAutoinstantiate\0").map(|sym| *sym)?;
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
        let ob_get_runtime_group_hndl = __library.get(b"ob_get_runtime_group_hndl\0").map(|sym| *sym)?;
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
        let CreatePBITypeDef = __library.get(b"CreatePBITypeDef\0").map(|sym| *sym)?;
        let CreatePBIClassDef = __library.get(b"CreatePBIClassDef\0").map(|sym| *sym)?;
        let CreatePBIScriptDef = __library.get(b"CreatePBIScriptDef\0").map(|sym| *sym)?;
        let ob_create_interface_in_library =
            __library.get(b"ob_create_interface_in_library\0").map(|sym| *sym)?;
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
        let ob_insert_local_inst_ref_dbg =
            __library.get(b"ob_insert_local_inst_ref_dbg\0").map(|sym| *sym)?;
        let ob_open_typedef_group = __library.get(b"ob_open_typedef_group\0").map(|sym| *sym)?;
        let ob_load_pspp_dlls = __library.get(b"ob_load_pspp_dlls\0").map(|sym| *sym)?;
        let ob_save_dll_to_pbd = __library.get(b"ob_save_dll_to_pbd\0").map(|sym| *sym)?;
        let ob_convert_pbx_to_native_groups =
            __library.get(b"ob_convert_pbx_to_native_groups\0").map(|sym| *sym)?;
        let ObPsppCreateControl = __library.get(b"ObPsppCreateControl\0").map(|sym| *sym)?;
        let ObPsppGetEventID = __library.get(b"ObPsppGetEventID\0").map(|sym| *sym)?;
        let DrawPsppObject = __library.get(b"DrawPsppObject\0").map(|sym| *sym)?;
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
        let obJagResultsetNotify = __library.get(b"obJagResultsetNotify\0").map(|sym| *sym)?;
        let ob_class_delete_and_withinclass =
            __library.get(b"ob_class_delete_and_withinclass\0").map(|sym| *sym)?;
        let ob_find_orphan_class = __library.get(b"ob_find_orphan_class\0").map(|sym| *sym)?;
        let ob_nuke_orphan_class = __library.get(b"ob_nuke_orphan_class\0").map(|sym| *sym)?;
        let ob_is_ancestor_class_modified =
            __library.get(b"ob_is_ancestor_class_modified\0").map(|sym| *sym)?;
        let ob_rebuild_instance_image = __library.get(b"ob_rebuild_instance_image\0").map(|sym| *sym)?;
        let ob_build_compile_list = __library.get(b"ob_build_compile_list\0").map(|sym| *sym)?;
        let os_openlib = __library.get(b"os_openlib\0").map(|sym| *sym)?;
        let os_get_funcptr = __library.get(b"os_get_funcptr\0").map(|sym| *sym)?;
        let os_callc = __library.get(b"os_callc\0").map(|sym| *sym)?;
        let os_closelib = __library.get(b"os_closelib\0").map(|sym| *sym)?;
        let cm_init_script_compiler = __library.get(b"cm_init_script_compiler\0").map(|sym| *sym)?;
        let cm_enable_debug_symbol = __library.get(b"cm_enable_debug_symbol\0").map(|sym| *sym)?;
        let cm_disable_debug_symbol = __library.get(b"cm_disable_debug_symbol\0").map(|sym| *sym)?;
        let cm_set_compiler_context = __library.get(b"cm_set_compiler_context\0").map(|sym| *sym)?;
        let cm_terminate = __library.get(b"cm_terminate\0").map(|sym| *sym)?;
        let cm_compile_script = __library.get(b"cm_compile_script\0").map(|sym| *sym)?;
        let cm_free_error_list = __library.get(b"cm_free_error_list\0").map(|sym| *sym)?;
        let cm_get_error_list = __library.get(b"cm_get_error_list\0").map(|sym| *sym)?;
        let cm_keep_error_list = __library.get(b"cm_keep_error_list\0").map(|sym| *sym)?;
        let cm_combine_error_list = __library.get(b"cm_combine_error_list\0").map(|sym| *sym)?;
        let cm_compile_namespace_block = __library.get(b"cm_compile_namespace_block\0").map(|sym| *sym)?;
        let cm_group_compile = __library.get(b"cm_group_compile\0").map(|sym| *sym)?;
        let cm_is_word_reserved = __library.get(b"cm_is_word_reserved\0").map(|sym| *sym)?;
        let cm_stream_compile = __library.get(b"cm_stream_compile\0").map(|sym| *sym)?;
        let cm_src_block_compile = __library.get(b"cm_src_block_compile\0").map(|sym| *sym)?;
        let cm_describe_statement = __library.get(b"cm_describe_statement\0").map(|sym| *sym)?;
        let cm_compiler_error = __library.get(b"cm_compiler_error\0").map(|sym| *sym)?;
        let cm_compiler_error_ln = __library.get(b"cm_compiler_error_ln\0").map(|sym| *sym)?;
        let cm_get_curr_this = __library.get(b"cm_get_curr_this\0").map(|sym| *sym)?;
        let cm_rebuild_application = __library.get(b"cm_rebuild_application\0").map(|sym| *sym)?;
        let cm_rebuild_from_compile_list =
            __library.get(b"cm_rebuild_from_compile_list\0").map(|sym| *sym)?;
        let cm_regen_datawindow = __library.get(b"cm_regen_datawindow\0").map(|sym| *sym)?;
        let cm_read_alias_table = __library.get(b"cm_read_alias_table\0").map(|sym| *sym)?;
        let cm_find_alias = __library.get(b"cm_find_alias\0").map(|sym| *sym)?;
        let cm_add_alias_entry = __library.get(b"cm_add_alias_entry\0").map(|sym| *sym)?;
        let cm_remove_alias_entry = __library.get(b"cm_remove_alias_entry\0").map(|sym| *sym)?;
        let cm_write_alias_table = __library.get(b"cm_write_alias_table\0").map(|sym| *sym)?;
        let cm_add_global_var = __library.get(b"cm_add_global_var\0").map(|sym| *sym)?;
        let cm_import_pb_extension = __library.get(b"cm_import_pb_extension\0").map(|sym| *sym)?;
        let cm_convert_pbl_to_unicode = __library.get(b"cm_convert_pbl_to_unicode\0").map(|sym| *sym)?;
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
        let ot_clear_array_data = __library.get(b"ot_clear_array_data\0").map(|sym| *sym)?;
        let ob_get_local_session = __library.get(b"ob_get_local_session\0").map(|sym| *sym)?;
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
        let ob_lookup_routine_by_signature =
            __library.get(b"ob_lookup_routine_by_signature\0").map(|sym| *sym)?;
        let ob_proto_error_upgrade = __library.get(b"ob_proto_error_upgrade\0").map(|sym| *sym)?;
        let ob_get_proto_access_type = __library.get(b"ob_get_proto_access_type\0").map(|sym| *sym)?;
        let ob_type_process_protos = __library.get(b"ob_type_process_protos\0").map(|sym| *sym)?;
        let ob_type_reprocess_protos = __library.get(b"ob_type_reprocess_protos\0").map(|sym| *sym)?;
        let ob_type_proto_add = __library.get(b"ob_type_proto_add\0").map(|sym| *sym)?;
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
        let ob_create_proto_throws_list = __library.get(b"ob_create_proto_throws_list\0").map(|sym| *sym)?;
        let ob_create_proto_args = __library.get(b"ob_create_proto_args\0").map(|sym| *sym)?;
        let ob_proto_overload_search_src =
            __library.get(b"ob_proto_overload_search_src\0").map(|sym| *sym)?;
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
        let ob_find_type_ancestor_assign =
            __library.get(b"ob_find_type_ancestor_assign\0").map(|sym| *sym)?;
        let ob_find_common_ancestor = __library.get(b"ob_find_common_ancestor\0").map(|sym| *sym)?;
        let ob_get_ancestor_system_class =
            __library.get(b"ob_get_ancestor_system_class\0").map(|sym| *sym)?;
        let ob_get_runtime_class = __library.get(b"ob_get_runtime_class\0").map(|sym| *sym)?;
        let ob_get_pspp_class_name = __library.get(b"ob_get_pspp_class_name\0").map(|sym| *sym)?;
        let ob_get_func_vtable_entry = __library.get(b"ob_get_func_vtable_entry\0").map(|sym| *sym)?;
        let ob_find_method = __library.get(b"ob_find_method\0").map(|sym| *sym)?;
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
        let getCultureValueStr = __library.get(b"getCultureValueStr\0").map(|sym| *sym)?;
        let getCultureValueInt = __library.get(b"getCultureValueInt\0").map(|sym| *sym)?;
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
        let shregExprCmpl = __library.get(b"shregExprCmpl\0").map(|sym| *sym)?;
        let shregExprMatch = __library.get(b"shregExprMatch\0").map(|sym| *sym)?;
        let shIsValidReal = __library.get(b"shIsValidReal\0").map(|sym| *sym)?;
        let shNormalizeReal = __library.get(b"shNormalizeReal\0").map(|sym| *sym)?;
        let shNormalizeRealbyLocale = __library.get(b"shNormalizeRealbyLocale\0").map(|sym| *sym)?;
        let shIsValidRealWeb = __library.get(b"shIsValidRealWeb\0").map(|sym| *sym)?;
        let shNormalizeRealWeb = __library.get(b"shNormalizeRealWeb\0").map(|sym| *sym)?;
        let shNormalizeRealbyLocaleWeb = __library.get(b"shNormalizeRealbyLocaleWeb\0").map(|sym| *sym)?;
        let pbshr_intl = __library.get(b"pbshr_intl\0").map(|sym| *sym)?;
        let shIsValidRealNoLocale = __library.get(b"shIsValidRealNoLocale\0").map(|sym| *sym)?;
        let shGetRegProfileStringValue = __library.get(b"shGetRegProfileStringValue\0").map(|sym| *sym)?;
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
        let rt_build_exception_using_error =
            __library.get(b"rt_build_exception_using_error\0").map(|sym| *sym)?;
        let rt_handle_uncaught_exception =
            __library.get(b"rt_handle_uncaught_exception\0").map(|sym| *sym)?;
        let rt_populate_error_struct = __library.get(b"rt_populate_error_struct\0").map(|sym| *sym)?;
        let rt_populate_error_from_stack =
            __library.get(b"rt_populate_error_from_stack\0").map(|sym| *sym)?;
        let rt_call_error_callback = __library.get(b"rt_call_error_callback\0").map(|sym| *sym)?;
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
        let rtRoutineProtoInfoFree = __library.get(b"rtRoutineProtoInfoFree\0").map(|sym| *sym)?;
        let rtInitializeInfoForCall = __library.get(b"rtInitializeInfoForCall\0").map(|sym| *sym)?;
        let rtCleanupInfoAfterCall = __library.get(b"rtCleanupInfoAfterCall\0").map(|sym| *sym)?;
        let rtRoutineCount = __library.get(b"rtRoutineCount\0").map(|sym| *sym)?;
        let rtReferenceArgCreate = __library.get(b"rtReferenceArgCreate\0").map(|sym| *sym)?;
        let rtReferenceArgFree = __library.get(b"rtReferenceArgFree\0").map(|sym| *sym)?;
        let rtGetClassDescrip = __library.get(b"rtGetClassDescrip\0").map(|sym| *sym)?;
        let rtDataFree = __library.get(b"rtDataFree\0").map(|sym| *sym)?;
        let rtDataCopy = __library.get(b"rtDataCopy\0").map(|sym| *sym)?;
        let rt_hit_level_0 = __library.get(b"rt_hit_level_0\0").map(|sym| *sym)?;
        let rt_StartJaguarDebug = __library.get(b"rt_StartJaguarDebug\0").map(|sym| *sym)?;
        let rt_StopJaguarDebug = __library.get(b"rt_StopJaguarDebug\0").map(|sym| *sym)?;
        let rt_JagBreakpointHit = __library.get(b"rt_JagBreakpointHit\0").map(|sym| *sym)?;
        let rt_JaguarGetCurrentContext = __library.get(b"rt_JaguarGetCurrentContext\0").map(|sym| *sym)?;
        let obPsppNormalBody = __library.get(b"obPsppNormalBody\0").map(|sym| *sym)?;
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
        let obUpdateSrcLastEdit = __library.get(b"obUpdateSrcLastEdit\0").map(|sym| *sym)?;
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
        let ot_strict_type_check = __library.get(b"ot_strict_type_check\0").map(|sym| *sym)?;
        let ot_generateVarInfo = __library.get(b"ot_generateVarInfo\0").map(|sym| *sym)?;
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
        let ot_build_flditemupdate_refpak =
            __library.get(b"ot_build_flditemupdate_refpak\0").map(|sym| *sym)?;
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
        let dwCompile = __library.get(b"dwCompile\0").map(|sym| *sym)?;
        let dwCreateWindow = __library.get(b"dwCreateWindow\0").map(|sym| *sym)?;
        let dwCreateWindowEx = __library.get(b"dwCreateWindowEx\0").map(|sym| *sym)?;
        let dwCrosstabDLG = __library.get(b"dwCrosstabDLG\0").map(|sym| *sym)?;
        let dwDescribe = __library.get(b"dwDescribe\0").map(|sym| *sym)?;
        let dwFree = __library.get(b"dwFree\0").map(|sym| *sym)?;
        let dwSetDataStoreHWndBehavior = __library.get(b"dwSetDataStoreHWndBehavior\0").map(|sym| *sym)?;
        let dwLoadData = __library.get(b"dwLoadData\0").map(|sym| *sym)?;
        let dwLoadDataProg = __library.get(b"dwLoadDataProg\0").map(|sym| *sym)?;
        let dwLoadDataCrosstab = __library.get(b"dwLoadDataCrosstab\0").map(|sym| *sym)?;
        let dwLoadDataStorage = __library.get(b"dwLoadDataStorage\0").map(|sym| *sym)?;
        let dwLoadDataOle = __library.get(b"dwLoadDataOle\0").map(|sym| *sym)?;
        let dwLoadDW = __library.get(b"dwLoadDW\0").map(|sym| *sym)?;
        let dwLoadFile = __library.get(b"dwLoadFile\0").map(|sym| *sym)?;
        let dwLoadLibrary = __library.get(b"dwLoadLibrary\0").map(|sym| *sym)?;
        let dwLoadMemory = __library.get(b"dwLoadMemory\0").map(|sym| *sym)?;
        let dwLoadStorage = __library.get(b"dwLoadStorage\0").map(|sym| *sym)?;
        let dwModify = __library.get(b"dwModify\0").map(|sym| *sym)?;
        let dwSaveData = __library.get(b"dwSaveData\0").map(|sym| *sym)?;
        let dwSaveDataStorage = __library.get(b"dwSaveDataStorage\0").map(|sym| *sym)?;
        let dwSaveDataCopy = __library.get(b"dwSaveDataCopy\0").map(|sym| *sym)?;
        let dwSaveDataCopyStorage = __library.get(b"dwSaveDataCopyStorage\0").map(|sym| *sym)?;
        let dwSaveFile = __library.get(b"dwSaveFile\0").map(|sym| *sym)?;
        let dwSaveLibrary = __library.get(b"dwSaveLibrary\0").map(|sym| *sym)?;
        let dwSaveOle = __library.get(b"dwSaveOle\0").map(|sym| *sym)?;
        let dwSaveObjects = __library.get(b"dwSaveObjects\0").map(|sym| *sym)?;
        let dwSaveStorage = __library.get(b"dwSaveStorage\0").map(|sym| *sym)?;
        let dwFileSaveStorage = __library.get(b"dwFileSaveStorage\0").map(|sym| *sym)?;
        let dwSaveStorageStg = __library.get(b"dwSaveStorageStg\0").map(|sym| *sym)?;
        let dwSaveOleStg = __library.get(b"dwSaveOleStg\0").map(|sym| *sym)?;
        let dwSaveObjectsStg = __library.get(b"dwSaveObjectsStg\0").map(|sym| *sym)?;
        let dwSaveDataStg = __library.get(b"dwSaveDataStg\0").map(|sym| *sym)?;
        let dwLoadFileBlob = __library.get(b"dwLoadFileBlob\0").map(|sym| *sym)?;
        let dwStgFree = __library.get(b"dwStgFree\0").map(|sym| *sym)?;
        let dwGetMessageText = __library.get(b"dwGetMessageText\0").map(|sym| *sym)?;
        let dwGetExceedPageMessage = __library.get(b"dwGetExceedPageMessage\0").map(|sym| *sym)?;
        let DW_PluginStart = __library.get(b"DW_PluginStart\0").map(|sym| *sym)?;
        let DW_PluginStop = __library.get(b"DW_PluginStop\0").map(|sym| *sym)?;
        let dwSyntaxGen = __library.get(b"dwSyntaxGen\0").map(|sym| *sym)?;
        let dwSyntaxGenForm = __library.get(b"dwSyntaxGenForm\0").map(|sym| *sym)?;
        let dwSyntaxFree = __library.get(b"dwSyntaxFree\0").map(|sym| *sym)?;
        let dwGenDataListSyntax = __library.get(b"dwGenDataListSyntax\0").map(|sym| *sym)?;
        let dwGenDataFormSyntax = __library.get(b"dwGenDataFormSyntax\0").map(|sym| *sym)?;
        let dwSyntaxFromSQL = __library.get(b"dwSyntaxFromSQL\0").map(|sym| *sym)?;
        let dwSyntaxFromDesc = __library.get(b"dwSyntaxFromDesc\0").map(|sym| *sym)?;
        let dwClearOriginalValues = __library.get(b"dwClearOriginalValues\0").map(|sym| *sym)?;
        let dwRowsDiscard = __library.get(b"dwRowsDiscard\0").map(|sym| *sym)?;
        let dwRowsMove = __library.get(b"dwRowsMove\0").map(|sym| *sym)?;
        let dwCreateObjectStorage = __library.get(b"dwCreateObjectStorage\0").map(|sym| *sym)?;
        let dwDefaultArgs = __library.get(b"dwDefaultArgs\0").map(|sym| *sym)?;
        let dwDefaultArgsFree = __library.get(b"dwDefaultArgsFree\0").map(|sym| *sym)?;
        let dwCrosstabDef = __library.get(b"dwCrosstabDef\0").map(|sym| *sym)?;
        let dwCrosstabDefDynamic = __library.get(b"dwCrosstabDefDynamic\0").map(|sym| *sym)?;
        let dwCrosstabModifyDynamic = __library.get(b"dwCrosstabModifyDynamic\0").map(|sym| *sym)?;
        let dwCrosstabModifyStatic = __library.get(b"dwCrosstabModifyStatic\0").map(|sym| *sym)?;
        let dwCrosstabBuildModel = __library.get(b"dwCrosstabBuildModel\0").map(|sym| *sym)?;
        let dwInfoBegin = __library.get(b"dwInfoBegin\0").map(|sym| *sym)?;
        let dwInfoBitmap = __library.get(b"dwInfoBitmap\0").map(|sym| *sym)?;
        let dwInfoBlob = __library.get(b"dwInfoBlob\0").map(|sym| *sym)?;
        let dwInfoInkpic = __library.get(b"dwInfoInkpic\0").map(|sym| *sym)?;
        let dwInfoButton = __library.get(b"dwInfoButton\0").map(|sym| *sym)?;
        let dwInfoColumn = __library.get(b"dwInfoColumn\0").map(|sym| *sym)?;
        let dwInfoCompute = __library.get(b"dwInfoCompute\0").map(|sym| *sym)?;
        let dwInfoCrosstab = __library.get(b"dwInfoCrosstab\0").map(|sym| *sym)?;
        let dwInfoWpf = __library.get(b"dwInfoWpf\0").map(|sym| *sym)?;
        let dwInfoDataWindow = __library.get(b"dwInfoDataWindow\0").map(|sym| *sym)?;
        let dwInfoEnd = __library.get(b"dwInfoEnd\0").map(|sym| *sym)?;
        let dwInfoFunctions = __library.get(b"dwInfoFunctions\0").map(|sym| *sym)?;
        let dwInfoGetFirstColumn = __library.get(b"dwInfoGetFirstColumn\0").map(|sym| *sym)?;
        let dwInfoGetNextColumn = __library.get(b"dwInfoGetNextColumn\0").map(|sym| *sym)?;
        let dwInfoGetDWWidth = __library.get(b"dwInfoGetDWWidth\0").map(|sym| *sym)?;
        let dwInfoGetFirstObject = __library.get(b"dwInfoGetFirstObject\0").map(|sym| *sym)?;
        let dwInfoGetNextObject = __library.get(b"dwInfoGetNextObject\0").map(|sym| *sym)?;
        let dwInfoGetObjectType = __library.get(b"dwInfoGetObjectType\0").map(|sym| *sym)?;
        let dwInfoGraph = __library.get(b"dwInfoGraph\0").map(|sym| *sym)?;
        let dwInfoGroupBox = __library.get(b"dwInfoGroupBox\0").map(|sym| *sym)?;
        let dwInfoLine = __library.get(b"dwInfoLine\0").map(|sym| *sym)?;
        let dwInfoOle = __library.get(b"dwInfoOle\0").map(|sym| *sym)?;
        let dwInfoOval = __library.get(b"dwInfoOval\0").map(|sym| *sym)?;
        let dwInfoProcessType = __library.get(b"dwInfoProcessType\0").map(|sym| *sym)?;
        let dwInfoRect = __library.get(b"dwInfoRect\0").map(|sym| *sym)?;
        let dwInfoRegionFirst = __library.get(b"dwInfoRegionFirst\0").map(|sym| *sym)?;
        let dwInfoRegionNext = __library.get(b"dwInfoRegionNext\0").map(|sym| *sym)?;
        let dwInfoReport = __library.get(b"dwInfoReport\0").map(|sym| *sym)?;
        let dwInfoResourceNames = __library.get(b"dwInfoResourceNames\0").map(|sym| *sym)?;
        let dwInfoRoundRect = __library.get(b"dwInfoRoundRect\0").map(|sym| *sym)?;
        let dwInfoSparse = __library.get(b"dwInfoSparse\0").map(|sym| *sym)?;
        let dwInfoTable = __library.get(b"dwInfoTable\0").map(|sym| *sym)?;
        let dwInfoTableExtract = __library.get(b"dwInfoTableExtract\0").map(|sym| *sym)?;
        let dwInfoText = __library.get(b"dwInfoText\0").map(|sym| *sym)?;
        let dwInfoVerifyCompute = __library.get(b"dwInfoVerifyCompute\0").map(|sym| *sym)?;
        let dwInfoVerifyFilter = __library.get(b"dwInfoVerifyFilter\0").map(|sym| *sym)?;
        let dwInfoVerifyStmt = __library.get(b"dwInfoVerifyStmt\0").map(|sym| *sym)?;
        let dwInfoVerifyString = __library.get(b"dwInfoVerifyString\0").map(|sym| *sym)?;
        let dwInfoParseStmt = __library.get(b"dwInfoParseStmt\0").map(|sym| *sym)?;
        let dwInfoSetPSRFileName = __library.get(b"dwInfoSetPSRFileName\0").map(|sym| *sym)?;
        let dwClientToObject = __library.get(b"dwClientToObject\0").map(|sym| *sym)?;
        let dwDBCancel = __library.get(b"dwDBCancel\0").map(|sym| *sym)?;
        let dwDeleteCrosstabSourceList = __library.get(b"dwDeleteCrosstabSourceList\0").map(|sym| *sym)?;
        let dwDragObjectCreate = __library.get(b"dwDragObjectCreate\0").map(|sym| *sym)?;
        let dwDragObjectDestroy = __library.get(b"dwDragObjectDestroy\0").map(|sym| *sym)?;
        let dwExprDialog = __library.get(b"dwExprDialog\0").map(|sym| *sym)?;
        let dwValidationExprDialog = __library.get(b"dwValidationExprDialog\0").map(|sym| *sym)?;
        let dwExprValidate = __library.get(b"dwExprValidate\0").map(|sym| *sym)?;
        let dwFind = __library.get(b"dwFind\0").map(|sym| *sym)?;
        let dwFindGroupChange = __library.get(b"dwFindGroupChange\0").map(|sym| *sym)?;
        let dwFindRequired = __library.get(b"dwFindRequired\0").map(|sym| *sym)?;
        let dwFitColumn = __library.get(b"dwFitColumn\0").map(|sym| *sym)?;
        let dwGenTableName = __library.get(b"dwGenTableName\0").map(|sym| *sym)?;
        let dwGenXTableName = __library.get(b"dwGenXTableName\0").map(|sym| *sym)?;
        let dwGetBandUnderMouse = __library.get(b"dwGetBandUnderMouse\0").map(|sym| *sym)?;
        let dwGetRowColUnderPoint = __library.get(b"dwGetRowColUnderPoint\0").map(|sym| *sym)?;
        let dwIsPointOutsideEditCol = __library.get(b"dwIsPointOutsideEditCol\0").map(|sym| *sym)?;
        let dwGetColumnFormat = __library.get(b"dwGetColumnFormat\0").map(|sym| *sym)?;
        let dwGetColumnFormatLength = __library.get(b"dwGetColumnFormatLength\0").map(|sym| *sym)?;
        let dwGetColumnNumber = __library.get(b"dwGetColumnNumber\0").map(|sym| *sym)?;
        let dwGetColumnType = __library.get(b"dwGetColumnType\0").map(|sym| *sym)?;
        let dwGetColumnValidation = __library.get(b"dwGetColumnValidation\0").map(|sym| *sym)?;
        let dwGetColumnValidationLength = __library.get(b"dwGetColumnValidationLength\0").map(|sym| *sym)?;
        let dwGetColumnValue = __library.get(b"dwGetColumnValue\0").map(|sym| *sym)?;
        let dwGetColumnValueLength = __library.get(b"dwGetColumnValueLength\0").map(|sym| *sym)?;
        let dwGetCurrentCol = __library.get(b"dwGetCurrentCol\0").map(|sym| *sym)?;
        let dwGetCurrentRowCol = __library.get(b"dwGetCurrentRowCol\0").map(|sym| *sym)?;
        let dwGetCurrentText = __library.get(b"dwGetCurrentText\0").map(|sym| *sym)?;
        let dwGetCurrentTextLength = __library.get(b"dwGetCurrentTextLength\0").map(|sym| *sym)?;
        let dwGetDBvendor = __library.get(b"dwGetDBvendor\0").map(|sym| *sym)?;
        let dwGetDBvendorLength = __library.get(b"dwGetDBvendorLength\0").map(|sym| *sym)?;
        let dwGetDBlogid = __library.get(b"dwGetDBlogid\0").map(|sym| *sym)?;
        let dwGetDBlogidLength = __library.get(b"dwGetDBlogidLength\0").map(|sym| *sym)?;
        let dwGetDBlogpass = __library.get(b"dwGetDBlogpass\0").map(|sym| *sym)?;
        let dwGetDBlogpassLength = __library.get(b"dwGetDBlogpassLength\0").map(|sym| *sym)?;
        let dwGetDBserver = __library.get(b"dwGetDBserver\0").map(|sym| *sym)?;
        let dwGetDBserverLength = __library.get(b"dwGetDBserverLength\0").map(|sym| *sym)?;
        let dwGetDBdatabase = __library.get(b"dwGetDBdatabase\0").map(|sym| *sym)?;
        let dwGetDBdatabaseLength = __library.get(b"dwGetDBdatabaseLength\0").map(|sym| *sym)?;
        let dwGetDBuserid = __library.get(b"dwGetDBuserid\0").map(|sym| *sym)?;
        let dwGetDBuseridLength = __library.get(b"dwGetDBuseridLength\0").map(|sym| *sym)?;
        let dwGetDBdatabasepass = __library.get(b"dwGetDBdatabasepass\0").map(|sym| *sym)?;
        let dwGetDBdatabasepassLength = __library.get(b"dwGetDBdatabasepassLength\0").map(|sym| *sym)?;
        let dwGetDBparm = __library.get(b"dwGetDBparm\0").map(|sym| *sym)?;
        let dwGetDBparmLength = __library.get(b"dwGetDBparmLength\0").map(|sym| *sym)?;
        let dwGetDBError = __library.get(b"dwGetDBError\0").map(|sym| *sym)?;
        let dwGetDBErrorLength = __library.get(b"dwGetDBErrorLength\0").map(|sym| *sym)?;
        let dwGetItemDateTime = __library.get(b"dwGetItemDateTime\0").map(|sym| *sym)?;
        let dwGetItemDecimal = __library.get(b"dwGetItemDecimal\0").map(|sym| *sym)?;
        let dwGetItemDouble = __library.get(b"dwGetItemDouble\0").map(|sym| *sym)?;
        let dwGetItemLength = __library.get(b"dwGetItemLength\0").map(|sym| *sym)?;
        let dwGetItemLengthLong = __library.get(b"dwGetItemLengthLong\0").map(|sym| *sym)?;
        let dwGetItemStatus = __library.get(b"dwGetItemStatus\0").map(|sym| *sym)?;
        let dwGetItemLengthAndString = __library.get(b"dwGetItemLengthAndString\0").map(|sym| *sym)?;
        let dwGetItemString = __library.get(b"dwGetItemString\0").map(|sym| *sym)?;
        let dwGetItemStringLong = __library.get(b"dwGetItemStringLong\0").map(|sym| *sym)?;
        let dwGetItemType = __library.get(b"dwGetItemType\0").map(|sym| *sym)?;
        let dwGetNextModifiedRow = __library.get(b"dwGetNextModifiedRow\0").map(|sym| *sym)?;
        let dwGetNotifyArgs = __library.get(b"dwGetNotifyArgs\0").map(|sym| *sym)?;
        let dwGetObjectUnderMouse = __library.get(b"dwGetObjectUnderMouse\0").map(|sym| *sym)?;
        let dwGroupCalc = __library.get(b"dwGroupCalc\0").map(|sym| *sym)?;
        let dwOleActivateItem = __library.get(b"dwOleActivateItem\0").map(|sym| *sym)?;
        let dwOleGetControlInfo = __library.get(b"dwOleGetControlInfo\0").map(|sym| *sym)?;
        let dwOleDeactivate = __library.get(b"dwOleDeactivate\0").map(|sym| *sym)?;
        let dwOleIPactivate = __library.get(b"dwOleIPactivate\0").map(|sym| *sym)?;
        let dwOleIPResize = __library.get(b"dwOleIPResize\0").map(|sym| *sym)?;
        let dwOleReset = __library.get(b"dwOleReset\0").map(|sym| *sym)?;
        let dwGetProgram = __library.get(b"dwGetProgram\0").map(|sym| *sym)?;
        let dwPosition = __library.get(b"dwPosition\0").map(|sym| *sym)?;
        let dwQryDialog = __library.get(b"dwQryDialog\0").map(|sym| *sym)?;
        let dwCanChangeQryMode = __library.get(b"dwCanChangeQryMode\0").map(|sym| *sym)?;
        let dwResetColumnValues = __library.get(b"dwResetColumnValues\0").map(|sym| *sym)?;
        let dwResetTrans = __library.get(b"dwResetTrans\0").map(|sym| *sym)?;
        let dwRetrieve = __library.get(b"dwRetrieve\0").map(|sym| *sym)?;
        let dwSendMessage = __library.get(b"dwSendMessage\0").map(|sym| *sym)?;
        let dwSetColumnFormat = __library.get(b"dwSetColumnFormat\0").map(|sym| *sym)?;
        let dwSetColumnValidation = __library.get(b"dwSetColumnValidation\0").map(|sym| *sym)?;
        let dwSetColumnValue = __library.get(b"dwSetColumnValue\0").map(|sym| *sym)?;
        let dwSetCurrentRowCol = __library.get(b"dwSetCurrentRowCol\0").map(|sym| *sym)?;
        let dwSetCurrentText = __library.get(b"dwSetCurrentText\0").map(|sym| *sym)?;
        let dwSetDBvendor = __library.get(b"dwSetDBvendor\0").map(|sym| *sym)?;
        let dwSetDBlogid = __library.get(b"dwSetDBlogid\0").map(|sym| *sym)?;
        let dwSetDBlogpass = __library.get(b"dwSetDBlogpass\0").map(|sym| *sym)?;
        let dwSetDBserver = __library.get(b"dwSetDBserver\0").map(|sym| *sym)?;
        let dwSetDBdatabase = __library.get(b"dwSetDBdatabase\0").map(|sym| *sym)?;
        let dwSetDBuserid = __library.get(b"dwSetDBuserid\0").map(|sym| *sym)?;
        let dwSetDBdatabasepass = __library.get(b"dwSetDBdatabasepass\0").map(|sym| *sym)?;
        let dwSetDBparm = __library.get(b"dwSetDBparm\0").map(|sym| *sym)?;
        let dwSetDetailHeight = __library.get(b"dwSetDetailHeight\0").map(|sym| *sym)?;
        let dwSetHTMLAction = __library.get(b"dwSetHTMLAction\0").map(|sym| *sym)?;
        let dwSetEventFlags = __library.get(b"dwSetEventFlags\0").map(|sym| *sym)?;
        let dwSetItemDateTime = __library.get(b"dwSetItemDateTime\0").map(|sym| *sym)?;
        let dwSetItemDecimal = __library.get(b"dwSetItemDecimal\0").map(|sym| *sym)?;
        let dwSetItemDouble = __library.get(b"dwSetItemDouble\0").map(|sym| *sym)?;
        let dwSetItemLong = __library.get(b"dwSetItemLong\0").map(|sym| *sym)?;
        let dwSetItemReal = __library.get(b"dwSetItemReal\0").map(|sym| *sym)?;
        let dwSetItemStatus = __library.get(b"dwSetItemStatus\0").map(|sym| *sym)?;
        let dwSetItemString = __library.get(b"dwSetItemString\0").map(|sym| *sym)?;
        let dwSetItemULong = __library.get(b"dwSetItemULong\0").map(|sym| *sym)?;
        let dwSetItemNull = __library.get(b"dwSetItemNull\0").map(|sym| *sym)?;
        let dwSetLibrary = __library.get(b"dwSetLibrary\0").map(|sym| *sym)?;
        let dwSetRC = __library.get(b"dwSetRC\0").map(|sym| *sym)?;
        let dwSetTrans = __library.get(b"dwSetTrans\0").map(|sym| *sym)?;
        let dwShareData = __library.get(b"dwShareData\0").map(|sym| *sym)?;
        let dwShareDataOff = __library.get(b"dwShareDataOff\0").map(|sym| *sym)?;
        let dwSQLGetSyntax = __library.get(b"dwSQLGetSyntax\0").map(|sym| *sym)?;
        let dwSQLSetSyntax = __library.get(b"dwSQLSetSyntax\0").map(|sym| *sym)?;
        let dwStorageDump = __library.get(b"dwStorageDump\0").map(|sym| *sym)?;
        let dwStyleChange = __library.get(b"dwStyleChange\0").map(|sym| *sym)?;
        let dwUpdateSQLGetRow = __library.get(b"dwUpdateSQLGetRow\0").map(|sym| *sym)?;
        let dwUpdateSQLGetQ = __library.get(b"dwUpdateSQLGetQ\0").map(|sym| *sym)?;
        let dwGenerateHTMLForm = __library.get(b"dwGenerateHTMLForm\0").map(|sym| *sym)?;
        let dwSaveAsAscii = __library.get(b"dwSaveAsAscii\0").map(|sym| *sym)?;
        let dwBuildXHTMLTemplate = __library.get(b"dwBuildXHTMLTemplate\0").map(|sym| *sym)?;
        let dwSetFullState = __library.get(b"dwSetFullState\0").map(|sym| *sym)?;
        let dwGetFullState = __library.get(b"dwGetFullState\0").map(|sym| *sym)?;
        let dwGetChanges = __library.get(b"dwGetChanges\0").map(|sym| *sym)?;
        let dwSetChanges = __library.get(b"dwSetChanges\0").map(|sym| *sym)?;
        let dwGetStateStatus = __library.get(b"dwGetStateStatus\0").map(|sym| *sym)?;
        let dwSetBlobStorageNULL = __library.get(b"dwSetBlobStorageNULL\0").map(|sym| *sym)?;
        let dwFreeBlob = __library.get(b"dwFreeBlob\0").map(|sym| *sym)?;
        let dwSetWebLoadCallback = __library.get(b"dwSetWebLoadCallback\0").map(|sym| *sym)?;
        let dwPaintDC = __library.get(b"dwPaintDC\0").map(|sym| *sym)?;
        let DS_SetNotifyFunc = __library.get(b"DS_SetNotifyFunc\0").map(|sym| *sym)?;
        let dwPrintObject = __library.get(b"dwPrintObject\0").map(|sym| *sym)?;
        let dwGraphSaveAsPdf = __library.get(b"dwGraphSaveAsPdf\0").map(|sym| *sym)?;
        let dwGetPsrDataVersion = __library.get(b"dwGetPsrDataVersion\0").map(|sym| *sym)?;
        let dwSetTransparency = __library.get(b"dwSetTransparency\0").map(|sym| *sym)?;
        let dwGenNameExhaustive = __library.get(b"dwGenNameExhaustive\0").map(|sym| *sym)?;
        let DS_Destroy = __library.get(b"DS_Destroy\0").map(|sym| *sym)?;
        let dwXMLGetDefaultEncoding = __library.get(b"dwXMLGetDefaultEncoding\0").map(|sym| *sym)?;
        let PBGetDBIProc = __library.get(b"PBGetDBIProc\0").map(|sym| *sym)?;
        let DBI_DatabaseLoad = __library.get(b"DBI_DatabaseLoad\0").map(|sym| *sym)?;
        let DBI_TableLoad = __library.get(b"DBI_TableLoad\0").map(|sym| *sym)?;
        let DBI_TableExplode = __library.get(b"DBI_TableExplode\0").map(|sym| *sym)?;
        let DBI_DeleteDir = __library.get(b"DBI_DeleteDir\0").map(|sym| *sym)?;
        let DBI_DeleteTable = __library.get(b"DBI_DeleteTable\0").map(|sym| *sym)?;
        let DBI_ComboList = __library.get(b"DBI_ComboList\0").map(|sym| *sym)?;
        let DBI_FindComboString = __library.get(b"DBI_FindComboString\0").map(|sym| *sym)?;
        let DBI_MatchCombo = __library.get(b"DBI_MatchCombo\0").map(|sym| *sym)?;
        let DBI_MatchString = __library.get(b"DBI_MatchString\0").map(|sym| *sym)?;
        let DBI_AttrInfo = __library.get(b"DBI_AttrInfo\0").map(|sym| *sym)?;
        let DBI_Command_Tran = __library.get(b"DBI_Command_Tran\0").map(|sym| *sym)?;
        let DBI_Step = __library.get(b"DBI_Step\0").map(|sym| *sym)?;
        let DBI_Commit = __library.get(b"DBI_Commit\0").map(|sym| *sym)?;
        let DBI_Connect = __library.get(b"DBI_Connect\0").map(|sym| *sym)?;
        let DBI_DummyConnect = __library.get(b"DBI_DummyConnect\0").map(|sym| *sym)?;
        let DBI_DialogConnect = __library.get(b"DBI_DialogConnect\0").map(|sym| *sym)?;
        let DBI_LogIn = __library.get(b"DBI_LogIn\0").map(|sym| *sym)?;
        let DBI_SetLogIn = __library.get(b"DBI_SetLogIn\0").map(|sym| *sym)?;
        let DBI_LogInAdoConnection = __library.get(b"DBI_LogInAdoConnection\0").map(|sym| *sym)?;
        let DBI_SetConnect = __library.get(b"DBI_SetConnect\0").map(|sym| *sym)?;
        let DBI_DatabaseInfo = __library.get(b"DBI_DatabaseInfo\0").map(|sym| *sym)?;
        let DBI_Describe = __library.get(b"DBI_Describe\0").map(|sym| *sym)?;
        let DBI_Disconnect = __library.get(b"DBI_Disconnect\0").map(|sym| *sym)?;
        let DBI_ErrorSQL = __library.get(b"DBI_ErrorSQL\0").map(|sym| *sym)?;
        let DBI_Execute = __library.get(b"DBI_Execute\0").map(|sym| *sym)?;
        let DBI_FetchNext = __library.get(b"DBI_FetchNext\0").map(|sym| *sym)?;
        let DBI_Prepare = __library.get(b"DBI_Prepare\0").map(|sym| *sym)?;
        let DBI_PrepareWithParms = __library.get(b"DBI_PrepareWithParms\0").map(|sym| *sym)?;
        let DBI_RollBack = __library.get(b"DBI_RollBack\0").map(|sym| *sym)?;
        let DBI_TerminateSQL = __library.get(b"DBI_TerminateSQL\0").map(|sym| *sym)?;
        let DBI_Rows = __library.get(b"DBI_Rows\0").map(|sym| *sym)?;
        let DBI_GetPBTypeString = __library.get(b"DBI_GetPBTypeString\0").map(|sym| *sym)?;
        let DBI_DescribeExtra = __library.get(b"DBI_DescribeExtra\0").map(|sym| *sym)?;
        let DBI_DateString = __library.get(b"DBI_DateString\0").map(|sym| *sym)?;
        let DBI_DecimalString = __library.get(b"DBI_DecimalString\0").map(|sym| *sym)?;
        let DBI_DoubleString = __library.get(b"DBI_DoubleString\0").map(|sym| *sym)?;
        let DBI_Numeri_tstring = __library.get(b"DBI_Numeri_tstring\0").map(|sym| *sym)?;
        let DBI_CursorConnect = __library.get(b"DBI_CursorConnect\0").map(|sym| *sym)?;
        let DBI_GetSelectItems = __library.get(b"DBI_GetSelectItems\0").map(|sym| *sym)?;
        let DBI_GetSelectInfo = __library.get(b"DBI_GetSelectInfo\0").map(|sym| *sym)?;
        let DBI_BindSelectBuffer = __library.get(b"DBI_BindSelectBuffer\0").map(|sym| *sym)?;
        let DBI_RuntimeFetchNext = __library.get(b"DBI_RuntimeFetchNext\0").map(|sym| *sym)?;
        let DBI_MPowerFetchNext = __library.get(b"DBI_MPowerFetchNext\0").map(|sym| *sym)?;
        let DBI_StartTran = __library.get(b"DBI_StartTran\0").map(|sym| *sym)?;
        let DBI_UniqueKey = __library.get(b"DBI_UniqueKey\0").map(|sym| *sym)?;
        let DBI_ParseFrom = __library.get(b"DBI_ParseFrom\0").map(|sym| *sym)?;
        let DBI_ParseColList = __library.get(b"DBI_ParseColList\0").map(|sym| *sym)?;
        let DBI_DoubleTheQuotes = __library.get(b"DBI_DoubleTheQuotes\0").map(|sym| *sym)?;
        let DBI_FreeMem = __library.get(b"DBI_FreeMem\0").map(|sym| *sym)?;
        let DBI_FreePrepList = __library.get(b"DBI_FreePrepList\0").map(|sym| *sym)?;
        let DBI_FreeColBlkList = __library.get(b"DBI_FreeColBlkList\0").map(|sym| *sym)?;
        let DBI_DialogBoxCenter = __library.get(b"DBI_DialogBoxCenter\0").map(|sym| *sym)?;
        let DBI_CleanUpColumnList = __library.get(b"DBI_CleanUpColumnList\0").map(|sym| *sym)?;
        let DBI_Cancel = __library.get(b"DBI_Cancel\0").map(|sym| *sym)?;
        let DBI_ProcInfo = __library.get(b"DBI_ProcInfo\0").map(|sym| *sym)?;
        let DBI_ProcText = __library.get(b"DBI_ProcText\0").map(|sym| *sym)?;
        let DBI_EventText = __library.get(b"DBI_EventText\0").map(|sym| *sym)?;
        let DBI_ProcDescribe = __library.get(b"DBI_ProcDescribe\0").map(|sym| *sym)?;
        let DBI_DeleteProcDir = __library.get(b"DBI_DeleteProcDir\0").map(|sym| *sym)?;
        let DBI_ProcPrepare = __library.get(b"DBI_ProcPrepare\0").map(|sym| *sym)?;
        let DBI_Parse = __library.get(b"DBI_Parse\0").map(|sym| *sym)?;
        let DBI_OuterJoinSyntax = __library.get(b"DBI_OuterJoinSyntax\0").map(|sym| *sym)?;
        let DBI_RuntimeExecute = __library.get(b"DBI_RuntimeExecute\0").map(|sym| *sym)?;
        let DBI_FormatHash = __library.get(b"DBI_FormatHash\0").map(|sym| *sym)?;
        let DBI_ValidHash = __library.get(b"DBI_ValidHash\0").map(|sym| *sym)?;
        let DBI_FreeValidHash = __library.get(b"DBI_FreeValidHash\0").map(|sym| *sym)?;
        let DBI_EditStyleHash = __library.get(b"DBI_EditStyleHash\0").map(|sym| *sym)?;
        let DBI_EditUpdate = __library.get(b"DBI_EditUpdate\0").map(|sym| *sym)?;
        let DBI_EditStyleInfo = __library.get(b"DBI_EditStyleInfo\0").map(|sym| *sym)?;
        let DBI_LoadString = __library.get(b"DBI_LoadString\0").map(|sym| *sym)?;
        let DBI_DynamicBind = __library.get(b"DBI_DynamicBind\0").map(|sym| *sym)?;
        let DBI_GetNextResultSet = __library.get(b"DBI_GetNextResultSet\0").map(|sym| *sym)?;
        let DBI_ViewText = __library.get(b"DBI_ViewText\0").map(|sym| *sym)?;
        let DBI_ExecPlan = __library.get(b"DBI_ExecPlan\0").map(|sym| *sym)?;
        let DBI_GetTimestamp = __library.get(b"DBI_GetTimestamp\0").map(|sym| *sym)?;
        let DBI_DoCompare = __library.get(b"DBI_DoCompare\0").map(|sym| *sym)?;
        let DBI_DoCompareFirst = __library.get(b"DBI_DoCompareFirst\0").map(|sym| *sym)?;
        let DBI_DoCompareFirstWithSkip = __library.get(b"DBI_DoCompareFirstWithSkip\0").map(|sym| *sym)?;
        let DBI_ReadBlob = __library.get(b"DBI_ReadBlob\0").map(|sym| *sym)?;
        let DBI_WriteBlob = __library.get(b"DBI_WriteBlob\0").map(|sym| *sym)?;
        let DBI_FillBlanks = __library.get(b"DBI_FillBlanks\0").map(|sym| *sym)?;
        let DBI_DeleteSyntaxList = __library.get(b"DBI_DeleteSyntaxList\0").map(|sym| *sym)?;
        let DBI_ParseParms = __library.get(b"DBI_ParseParms\0").map(|sym| *sym)?;
        let DBI_ParseKeyWords = __library.get(b"DBI_ParseKeyWords\0").map(|sym| *sym)?;
        let DBI_FreeParmList = __library.get(b"DBI_FreeParmList\0").map(|sym| *sym)?;
        let DBI_GetParm = __library.get(b"DBI_GetParm\0").map(|sym| *sym)?;
        let DBI_FetchFirst = __library.get(b"DBI_FetchFirst\0").map(|sym| *sym)?;
        let DBI_FetchPrev = __library.get(b"DBI_FetchPrev\0").map(|sym| *sym)?;
        let DBI_FetchRandom = __library.get(b"DBI_FetchRandom\0").map(|sym| *sym)?;
        let DBI_FetchRelative = __library.get(b"DBI_FetchRelative\0").map(|sym| *sym)?;
        let DBI_FetchLast = __library.get(b"DBI_FetchLast\0").map(|sym| *sym)?;
        let DBI_NewDBParm = __library.get(b"DBI_NewDBParm\0").map(|sym| *sym)?;
        let DBI_ReplaceDbParm = __library.get(b"DBI_ReplaceDbParm\0").map(|sym| *sym)?;
        let DBI_DelimitReservedWord = __library.get(b"DBI_DelimitReservedWord\0").map(|sym| *sym)?;
        let DBI_DescribeInput = __library.get(b"DBI_DescribeInput\0").map(|sym| *sym)?;
        let DBI_DescribeOutput = __library.get(b"DBI_DescribeOutput\0").map(|sym| *sym)?;
        let DBI_DWCursorConnect = __library.get(b"DBI_DWCursorConnect\0").map(|sym| *sym)?;
        let DBI_ParseColSubset = __library.get(b"DBI_ParseColSubset\0").map(|sym| *sym)?;
        let DBI_ParseBasicSelect = __library.get(b"DBI_ParseBasicSelect\0").map(|sym| *sym)?;
        let DBI_CreatePrimaryKeySyntax = __library.get(b"DBI_CreatePrimaryKeySyntax\0").map(|sym| *sym)?;
        let DBI_CreateNoLogPKeySyntax = __library.get(b"DBI_CreateNoLogPKeySyntax\0").map(|sym| *sym)?;
        let DBI_AlterPrimaryKeySyntax = __library.get(b"DBI_AlterPrimaryKeySyntax\0").map(|sym| *sym)?;
        let DBI_CreateForeignKeySyntax = __library.get(b"DBI_CreateForeignKeySyntax\0").map(|sym| *sym)?;
        let DBI_AlterForeignKeySyntax = __library.get(b"DBI_AlterForeignKeySyntax\0").map(|sym| *sym)?;
        let DBI_GetForeignKYOptions = __library.get(b"DBI_GetForeignKYOptions\0").map(|sym| *sym)?;
        let DBI_LibraryName = __library.get(b"DBI_LibraryName\0").map(|sym| *sym)?;
        let DBI_LookForKeyWord = __library.get(b"DBI_LookForKeyWord\0").map(|sym| *sym)?;
        let DBI_ParseIdentifier = __library.get(b"DBI_ParseIdentifier\0").map(|sym| *sym)?;
        let DBI_GetColumnExpression = __library.get(b"DBI_GetColumnExpression\0").map(|sym| *sym)?;
        let DBI_ParseWhere = __library.get(b"DBI_ParseWhere\0").map(|sym| *sym)?;
        let DBI_FreeWhereList = __library.get(b"DBI_FreeWhereList\0").map(|sym| *sym)?;
        let DBI_PrimaryKeyReferences = __library.get(b"DBI_PrimaryKeyReferences\0").map(|sym| *sym)?;
        let DBI_PBToSQL = __library.get(b"DBI_PBToSQL\0").map(|sym| *sym)?;
        let DBI_PBToArgs = __library.get(b"DBI_PBToArgs\0").map(|sym| *sym)?;
        let DBI_ReleaseInputParms = __library.get(b"DBI_ReleaseInputParms\0").map(|sym| *sym)?;
        let DBI_SynText = __library.get(b"DBI_SynText\0").map(|sym| *sym)?;
        let DBI_PBC_DialogBox = __library.get(b"DBI_PBC_DialogBox\0").map(|sym| *sym)?;
        let DBI_PBC_DialogBoxParam = __library.get(b"DBI_PBC_DialogBoxParam\0").map(|sym| *sym)?;
        let DBI_PBC_ShowWindow = __library.get(b"DBI_PBC_ShowWindow\0").map(|sym| *sym)?;
        let DBI_DBHandle = __library.get(b"DBI_DBHandle\0").map(|sym| *sym)?;
        let DBI_GetAdoConnection = __library.get(b"DBI_GetAdoConnection\0").map(|sym| *sym)?;
        let DBI_SearchReplace = __library.get(b"DBI_SearchReplace\0").map(|sym| *sym)?;
        let DBI_CtrlChars2Text = __library.get(b"DBI_CtrlChars2Text\0").map(|sym| *sym)?;
        let DBI_Text2CtrlChars = __library.get(b"DBI_Text2CtrlChars\0").map(|sym| *sym)?;
        let DBI_SQLCacheBegin = __library.get(b"DBI_SQLCacheBegin\0").map(|sym| *sym)?;
        let DBI_SQLCacheEnd = __library.get(b"DBI_SQLCacheEnd\0").map(|sym| *sym)?;
        let DBI_SQLCacheFlushEntries = __library.get(b"DBI_SQLCacheFlushEntries\0").map(|sym| *sym)?;
        let DBI_SQLCacheRegisterSQLStatement =
            __library.get(b"DBI_SQLCacheRegisterSQLStatement\0").map(|sym| *sym)?;
        let DBI_SQLCacheRegisterDescribe =
            __library.get(b"DBI_SQLCacheRegisterDescribe\0").map(|sym| *sym)?;
        let DBI_SQLCacheRequestSqlStatement =
            __library.get(b"DBI_SQLCacheRequestSqlStatement\0").map(|sym| *sym)?;
        let DBI_SQLCacheRequestDescribe = __library.get(b"DBI_SQLCacheRequestDescribe\0").map(|sym| *sym)?;
        let DBI_SQLCacheMakeSQLStatementAvailable =
            __library.get(b"DBI_SQLCacheMakeSQLStatementAvailable\0").map(|sym| *sym)?;
        let DBI_SQLCacheSetCacheSize = __library.get(b"DBI_SQLCacheSetCacheSize\0").map(|sym| *sym)?;
        let DBI_SQLCacheDiscardEntry = __library.get(b"DBI_SQLCacheDiscardEntry\0").map(|sym| *sym)?;
        let DBI_SQLCacheConnectSetup = __library.get(b"DBI_SQLCacheConnectSetup\0").map(|sym| *sym)?;
        let DBI_SQLCacheConnectDrop = __library.get(b"DBI_SQLCacheConnectDrop\0").map(|sym| *sym)?;
        let DBI_SQLCacheSetSelectCacheSize =
            __library.get(b"DBI_SQLCacheSetSelectCacheSize\0").map(|sym| *sym)?;
        let DBI_SQLCacheStatistics = __library.get(b"DBI_SQLCacheStatistics\0").map(|sym| *sym)?;
        let DBI_GetIdentityValue = __library.get(b"DBI_GetIdentityValue\0").map(|sym| *sym)?;
        let DBI_FormatWindowsEOL = __library.get(b"DBI_FormatWindowsEOL\0").map(|sym| *sym)?;
        let DBI_RegisterVendor = __library.get(b"DBI_RegisterVendor\0").map(|sym| *sym)?;
        let DBI_UnRegisterVendor = __library.get(b"DBI_UnRegisterVendor\0").map(|sym| *sym)?;
        let DBI_IsCache = __library.get(b"DBI_IsCache\0").map(|sym| *sym)?;
        let DBI_IsConnInServerTrans = __library.get(b"DBI_IsConnInServerTrans\0").map(|sym| *sym)?;
        let DBI_IsConnFromServer = __library.get(b"DBI_IsConnFromServer\0").map(|sym| *sym)?;
        let DBI_ConvertHexStringToBlob = __library.get(b"DBI_ConvertHexStringToBlob\0").map(|sym| *sym)?;
        let CreateDBIConnect = __library.get(b"CreateDBIConnect\0").map(|sym| *sym)?;
        let CreateDBITableSchema = __library.get(b"CreateDBITableSchema\0").map(|sym| *sym)?;
        let rtdb_close = __library.get(b"rtdb_close\0").map(|sym| *sym)?;
        let rtdb_commit = __library.get(b"rtdb_commit\0").map(|sym| *sym)?;
        let rtdb_delete = __library.get(b"rtdb_delete\0").map(|sym| *sym)?;
        let rtdb_deletewithcurs = __library.get(b"rtdb_deletewithcurs\0").map(|sym| *sym)?;
        let rtdb_describe = __library.get(b"rtdb_describe\0").map(|sym| *sym)?;
        let rtdb_execute = __library.get(b"rtdb_execute\0").map(|sym| *sym)?;
        let rtdb_executedyn = __library.get(b"rtdb_executedyn\0").map(|sym| *sym)?;
        let rtdb_executeimmed = __library.get(b"rtdb_executeimmed\0").map(|sym| *sym)?;
        let rtdb_execdynproc = __library.get(b"rtdb_execdynproc\0").map(|sym| *sym)?;
        let rtdb_execdynwithdesc = __library.get(b"rtdb_execdynwithdesc\0").map(|sym| *sym)?;
        let rtdb_fetch = __library.get(b"rtdb_fetch\0").map(|sym| *sym)?;
        let rtdb_fetchwithdesc = __library.get(b"rtdb_fetchwithdesc\0").map(|sym| *sym)?;
        let rtdb_insert = __library.get(b"rtdb_insert\0").map(|sym| *sym)?;
        let rtdb_open = __library.get(b"rtdb_open\0").map(|sym| *sym)?;
        let rtdb_opendyn = __library.get(b"rtdb_opendyn\0").map(|sym| *sym)?;
        let rtdb_opendynwithdesc = __library.get(b"rtdb_opendynwithdesc\0").map(|sym| *sym)?;
        let rtdb_prepare = __library.get(b"rtdb_prepare\0").map(|sym| *sym)?;
        let rtdb_rollback = __library.get(b"rtdb_rollback\0").map(|sym| *sym)?;
        let rtdb_select = __library.get(b"rtdb_select\0").map(|sym| *sym)?;
        let rtdb_selectblob = __library.get(b"rtdb_selectblob\0").map(|sym| *sym)?;
        let rtdb_start = __library.get(b"rtdb_start\0").map(|sym| *sym)?;
        let rtdb_stop = __library.get(b"rtdb_stop\0").map(|sym| *sym)?;
        let rtdb_update = __library.get(b"rtdb_update\0").map(|sym| *sym)?;
        let rtdb_updatewithcurs = __library.get(b"rtdb_updatewithcurs\0").map(|sym| *sym)?;
        let rtdb_updateblob = __library.get(b"rtdb_updateblob\0").map(|sym| *sym)?;
        let rtdb_trans_pool_login = __library.get(b"rtdb_trans_pool_login\0").map(|sym| *sym)?;
        let rtdb_trans_pool_disconnect = __library.get(b"rtdb_trans_pool_disconnect\0").map(|sym| *sym)?;
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
        let otcg_string_cat = __library.get(b"otcg_string_cat\0").map(|sym| *sym)?;
        let otcg_binary_cat = __library.get(b"otcg_binary_cat\0").map(|sym| *sym)?;
        let otcg_add_object_argument_reference =
            __library.get(b"otcg_add_object_argument_reference\0").map(|sym| *sym)?;
        let otcg_embedded_group = __library.get(b"otcg_embedded_group\0").map(|sym| *sym)?;
        Ok(Api {
            __library,
            pbstg_begin,
            pbstg_begin_allocflags,
            pbstg_begin_nofast,
            pbstg_end,
            pbstg_free_pool,
            pbstg_new_pool,
            pbstg_new_pool_nofast,
            pbstg_new_pool_with_size,
            pbstg_new_pool_with_size_nofast,
            pbstg_set_pool_name,
            pbstg_set_poolpagesize,
            pbstg_write_debug,
            pbstg_stat,
            pbstg_shrink,
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
            pbstg_dde_strdup,
            pbstg_huge_memcmp,
            pbstg_huge_memcpy,
            pbstg_huge_memmove,
            pbstg_huge_memset,
            pbstg_huge_strchr,
            pbstg_huge_strcpy,
            pbstg_huge_strlen,
            pbstg_huge_strncpy,
            pbstg_huge_strstr,
            OS_UtilGetProfInt,
            OS_UtilGetProfStr,
            OS_UtilPutProfStr,
            OS_NetworkGetProfInt,
            OS_NetworkGetProfStr,
            OS_NetworkPutProfStr,
            OS_NetworkSharedProfileAccessible,
            OS_UtilGetInitPath,
            OS_UtilPutInitPath,
            UtilIniFile,
            FDCC_Get_Registry_Entry,
            FDCC_Get_CSIDL_Path,
            FDCC_Create_Registry_Entry,
            FDCC_Get_Install_Location,
            FDCC_New_INI_Installed,
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
            PBUNI_memchr,
            pbstg_lchrcmp,
            pbstg_lchrcmpi,
            os_vabuild_start,
            os_vabuild_ptrarg,
            os_vabuild_end,
            os_vabuild_add,
            sh_dbg_console_init,
            sh_dbg_console_out,
            sh_dbg_console_lock,
            sh_dbg_console_unlock,
            sh_dbg_init,
            sh_dbg_this,
            sh_dbg_term,
            sh_dbg_read_input,
            sh_dbg_outfile,
            sh_dbg_open,
            sh_dbg_close,
            sh_dbg_set,
            sh_dbg_del,
            sh_dbg_header,
            sh_dbg_indent,
            sh_dbg_set_this,
            sh_dbg_out,
            sh_dbg_start_indent,
            sh_dbg_end_indent,
            sh_dbg_enter,
            sh_dbg_leave,
            sh_dbg_on,
            sh_dbg_off,
            sh_dbg_query,
            sh_dbg_is_hdr_on,
            sh_dbg_is_indent_on,
            osAssert,
            shlist_delete,
            shlist_deleteFree,
            shlist_get_next,
            shlist_get_prev,
            shlist_putafter,
            shlist_addafter,
            shlist_addbefore,
            shlist_remove,
            shlist_insert_at_curr,
            shlist_insert,
            shlist_new,
            shlist_curr_node,
            shlist_get_count,
            shlist_get_first,
            shlist_get_last,
            shlist_get_curr,
            shlist_update,
            shlist_get_handle,
            shlist_set_current,
            shlist_traversal,
            shlist_sort,
            shlist_sort_param,
            sh_grwblk_init,
            sh_new_grwblk,
            sh_set_grwblk_item,
            sh_add_to_grwblk,
            sh_append_to_grwblk,
            sh_grwblk_delete,
            sh_grwblk_close,
            ob_set_session_icontext,
            ob_set_main_obthis,
            rt_move_thread,
            rt_clear_thread,
            rt_get_current_this,
            rt_set_current_this,
            rt_add_task,
            rt_free_task,
            rt_get_current_task_info,
            rt_set_current_task_info,
            rt_get_free_task_slot,
            rt_is_running_exe,
            shhash_new,
            shhash_new_arg,
            shhash_delete,
            shhash_clear,
            shhash_get_first,
            shhash_get_next,
            shhash_insert,
            shhash_search,
            shhash_search_arg,
            shhash_search_unique,
            shhash_search_unique_arg,
            shhash_searchNext,
            shhash_searchSlot,
            shhash_remove,
            shhash_statistics,
            shhash_traversal,
            obAllocDebug,
            obFreeDebug,
            ob_add_const_data,
            parse_stringvalue_hash,
            obconpool_stringvalue_del,
            ob_looksym_keyfunc,
            ob_looksym_reference,
            ob_looksym_delete,
            ob_dynarray_index,
            ob_dynarray_grow,
            ob_narray_create_static,
            ob_narray_create_dynamic,
            ob_narray_dynamic_item_init_callback,
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
            sh_MAX_DEC,
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
            ConvertOldDecToString,
            shdtDayName,
            shdtDayOfWeek,
            shdtBuildTime,
            shdtDiffDate,
            shdtDiffTime,
            shdtDiffMSec,
            shdtNow,
            shdtParse,
            shdtParseEx,
            shdtParseStringEx,
            shdtParseStringExWithLcid,
            shdtParseToString,
            shdtRelativeDate,
            shdtToMJDDate,
            shdtToMJDTime,
            shdtToMJDTimestamp,
            shMJDDateTodt,
            shMJDTimeTodt,
            shMJDTimestampTodt,
            shdtString,
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
            ob_group_is_normalized_window,
            ob_group_set_normalized_window,
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
            ob_add_liblist,
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
            ob_reload_group_source,
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
            obIsClassAutoinstantiate,
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
            ob_get_runtime_group_hndl,
            ob_check_for_locked_menu,
            ob_create_obinst,
            ob_instantiate_child_object,
            ob_instantiate_system_object,
            ob_destroy_obinst,
            ob_set_runtime,
            ob_create_executable,
            ob_create_library,
            ob_create_consolidated_library,
            CreatePBITypeDef,
            CreatePBIClassDef,
            CreatePBIScriptDef,
            ob_create_interface_in_library,
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
            ob_insert_local_inst_ref_dbg,
            ob_open_typedef_group,
            ob_load_pspp_dlls,
            ob_save_dll_to_pbd,
            ob_convert_pbx_to_native_groups,
            ObPsppCreateControl,
            ObPsppGetEventID,
            DrawPsppObject,
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
            obJagResultsetNotify,
            ob_class_delete_and_withinclass,
            ob_find_orphan_class,
            ob_nuke_orphan_class,
            ob_is_ancestor_class_modified,
            ob_rebuild_instance_image,
            ob_build_compile_list,
            os_openlib,
            os_get_funcptr,
            os_callc,
            os_closelib,
            cm_init_script_compiler,
            cm_enable_debug_symbol,
            cm_disable_debug_symbol,
            cm_set_compiler_context,
            cm_terminate,
            cm_compile_script,
            cm_free_error_list,
            cm_get_error_list,
            cm_keep_error_list,
            cm_combine_error_list,
            cm_compile_namespace_block,
            cm_group_compile,
            cm_is_word_reserved,
            cm_stream_compile,
            cm_src_block_compile,
            cm_describe_statement,
            cm_compiler_error,
            cm_compiler_error_ln,
            cm_get_curr_this,
            cm_rebuild_application,
            cm_rebuild_from_compile_list,
            cm_regen_datawindow,
            cm_read_alias_table,
            cm_find_alias,
            cm_add_alias_entry,
            cm_remove_alias_entry,
            cm_write_alias_table,
            cm_add_global_var,
            cm_import_pb_extension,
            cm_convert_pbl_to_unicode,
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
            ot_clear_array_data,
            ob_get_local_session,
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
            ob_lookup_routine_by_signature,
            ob_proto_error_upgrade,
            ob_get_proto_access_type,
            ob_type_process_protos,
            ob_type_reprocess_protos,
            ob_type_proto_add,
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
            ob_create_proto_throws_list,
            ob_create_proto_args,
            ob_proto_overload_search_src,
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
            ob_find_type_ancestor_assign,
            ob_find_common_ancestor,
            ob_get_ancestor_system_class,
            ob_get_runtime_class,
            ob_get_pspp_class_name,
            ob_get_func_vtable_entry,
            ob_find_method,
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
            getCultureValueStr,
            getCultureValueInt,
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
            shregExprCmpl,
            shregExprMatch,
            shIsValidReal,
            shNormalizeReal,
            shNormalizeRealbyLocale,
            shIsValidRealWeb,
            shNormalizeRealWeb,
            shNormalizeRealbyLocaleWeb,
            pbshr_intl,
            shIsValidRealNoLocale,
            shGetRegProfileStringValue,
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
            rt_build_exception_using_error,
            rt_handle_uncaught_exception,
            rt_populate_error_struct,
            rt_populate_error_from_stack,
            rt_call_error_callback,
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
            rtRoutineProtoInfoFree,
            rtInitializeInfoForCall,
            rtCleanupInfoAfterCall,
            rtRoutineCount,
            rtReferenceArgCreate,
            rtReferenceArgFree,
            rtGetClassDescrip,
            rtDataFree,
            rtDataCopy,
            rt_hit_level_0,
            rt_StartJaguarDebug,
            rt_StopJaguarDebug,
            rt_JagBreakpointHit,
            rt_JaguarGetCurrentContext,
            obPsppNormalBody,
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
            obUpdateSrcLastEdit,
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
            ot_strict_type_check,
            ot_generateVarInfo,
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
            ot_build_flditemupdate_refpak,
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
            dwCompile,
            dwCreateWindow,
            dwCreateWindowEx,
            dwCrosstabDLG,
            dwDescribe,
            dwFree,
            dwSetDataStoreHWndBehavior,
            dwLoadData,
            dwLoadDataProg,
            dwLoadDataCrosstab,
            dwLoadDataStorage,
            dwLoadDataOle,
            dwLoadDW,
            dwLoadFile,
            dwLoadLibrary,
            dwLoadMemory,
            dwLoadStorage,
            dwModify,
            dwSaveData,
            dwSaveDataStorage,
            dwSaveDataCopy,
            dwSaveDataCopyStorage,
            dwSaveFile,
            dwSaveLibrary,
            dwSaveOle,
            dwSaveObjects,
            dwSaveStorage,
            dwFileSaveStorage,
            dwSaveStorageStg,
            dwSaveOleStg,
            dwSaveObjectsStg,
            dwSaveDataStg,
            dwLoadFileBlob,
            dwStgFree,
            dwGetMessageText,
            dwGetExceedPageMessage,
            DW_PluginStart,
            DW_PluginStop,
            dwSyntaxGen,
            dwSyntaxGenForm,
            dwSyntaxFree,
            dwGenDataListSyntax,
            dwGenDataFormSyntax,
            dwSyntaxFromSQL,
            dwSyntaxFromDesc,
            dwClearOriginalValues,
            dwRowsDiscard,
            dwRowsMove,
            dwCreateObjectStorage,
            dwDefaultArgs,
            dwDefaultArgsFree,
            dwCrosstabDef,
            dwCrosstabDefDynamic,
            dwCrosstabModifyDynamic,
            dwCrosstabModifyStatic,
            dwCrosstabBuildModel,
            dwInfoBegin,
            dwInfoBitmap,
            dwInfoBlob,
            dwInfoInkpic,
            dwInfoButton,
            dwInfoColumn,
            dwInfoCompute,
            dwInfoCrosstab,
            dwInfoWpf,
            dwInfoDataWindow,
            dwInfoEnd,
            dwInfoFunctions,
            dwInfoGetFirstColumn,
            dwInfoGetNextColumn,
            dwInfoGetDWWidth,
            dwInfoGetFirstObject,
            dwInfoGetNextObject,
            dwInfoGetObjectType,
            dwInfoGraph,
            dwInfoGroupBox,
            dwInfoLine,
            dwInfoOle,
            dwInfoOval,
            dwInfoProcessType,
            dwInfoRect,
            dwInfoRegionFirst,
            dwInfoRegionNext,
            dwInfoReport,
            dwInfoResourceNames,
            dwInfoRoundRect,
            dwInfoSparse,
            dwInfoTable,
            dwInfoTableExtract,
            dwInfoText,
            dwInfoVerifyCompute,
            dwInfoVerifyFilter,
            dwInfoVerifyStmt,
            dwInfoVerifyString,
            dwInfoParseStmt,
            dwInfoSetPSRFileName,
            dwClientToObject,
            dwDBCancel,
            dwDeleteCrosstabSourceList,
            dwDragObjectCreate,
            dwDragObjectDestroy,
            dwExprDialog,
            dwValidationExprDialog,
            dwExprValidate,
            dwFind,
            dwFindGroupChange,
            dwFindRequired,
            dwFitColumn,
            dwGenTableName,
            dwGenXTableName,
            dwGetBandUnderMouse,
            dwGetRowColUnderPoint,
            dwIsPointOutsideEditCol,
            dwGetColumnFormat,
            dwGetColumnFormatLength,
            dwGetColumnNumber,
            dwGetColumnType,
            dwGetColumnValidation,
            dwGetColumnValidationLength,
            dwGetColumnValue,
            dwGetColumnValueLength,
            dwGetCurrentCol,
            dwGetCurrentRowCol,
            dwGetCurrentText,
            dwGetCurrentTextLength,
            dwGetDBvendor,
            dwGetDBvendorLength,
            dwGetDBlogid,
            dwGetDBlogidLength,
            dwGetDBlogpass,
            dwGetDBlogpassLength,
            dwGetDBserver,
            dwGetDBserverLength,
            dwGetDBdatabase,
            dwGetDBdatabaseLength,
            dwGetDBuserid,
            dwGetDBuseridLength,
            dwGetDBdatabasepass,
            dwGetDBdatabasepassLength,
            dwGetDBparm,
            dwGetDBparmLength,
            dwGetDBError,
            dwGetDBErrorLength,
            dwGetItemDateTime,
            dwGetItemDecimal,
            dwGetItemDouble,
            dwGetItemLength,
            dwGetItemLengthLong,
            dwGetItemStatus,
            dwGetItemLengthAndString,
            dwGetItemString,
            dwGetItemStringLong,
            dwGetItemType,
            dwGetNextModifiedRow,
            dwGetNotifyArgs,
            dwGetObjectUnderMouse,
            dwGroupCalc,
            dwOleActivateItem,
            dwOleGetControlInfo,
            dwOleDeactivate,
            dwOleIPactivate,
            dwOleIPResize,
            dwOleReset,
            dwGetProgram,
            dwPosition,
            dwQryDialog,
            dwCanChangeQryMode,
            dwResetColumnValues,
            dwResetTrans,
            dwRetrieve,
            dwSendMessage,
            dwSetColumnFormat,
            dwSetColumnValidation,
            dwSetColumnValue,
            dwSetCurrentRowCol,
            dwSetCurrentText,
            dwSetDBvendor,
            dwSetDBlogid,
            dwSetDBlogpass,
            dwSetDBserver,
            dwSetDBdatabase,
            dwSetDBuserid,
            dwSetDBdatabasepass,
            dwSetDBparm,
            dwSetDetailHeight,
            dwSetHTMLAction,
            dwSetEventFlags,
            dwSetItemDateTime,
            dwSetItemDecimal,
            dwSetItemDouble,
            dwSetItemLong,
            dwSetItemReal,
            dwSetItemStatus,
            dwSetItemString,
            dwSetItemULong,
            dwSetItemNull,
            dwSetLibrary,
            dwSetRC,
            dwSetTrans,
            dwShareData,
            dwShareDataOff,
            dwSQLGetSyntax,
            dwSQLSetSyntax,
            dwStorageDump,
            dwStyleChange,
            dwUpdateSQLGetRow,
            dwUpdateSQLGetQ,
            dwGenerateHTMLForm,
            dwSaveAsAscii,
            dwBuildXHTMLTemplate,
            dwSetFullState,
            dwGetFullState,
            dwGetChanges,
            dwSetChanges,
            dwGetStateStatus,
            dwSetBlobStorageNULL,
            dwFreeBlob,
            dwSetWebLoadCallback,
            dwPaintDC,
            DS_SetNotifyFunc,
            dwPrintObject,
            dwGraphSaveAsPdf,
            dwGetPsrDataVersion,
            dwSetTransparency,
            dwGenNameExhaustive,
            DS_Destroy,
            dwXMLGetDefaultEncoding,
            PBGetDBIProc,
            DBI_DatabaseLoad,
            DBI_TableLoad,
            DBI_TableExplode,
            DBI_DeleteDir,
            DBI_DeleteTable,
            DBI_ComboList,
            DBI_FindComboString,
            DBI_MatchCombo,
            DBI_MatchString,
            DBI_AttrInfo,
            DBI_Command_Tran,
            DBI_Step,
            DBI_Commit,
            DBI_Connect,
            DBI_DummyConnect,
            DBI_DialogConnect,
            DBI_LogIn,
            DBI_SetLogIn,
            DBI_LogInAdoConnection,
            DBI_SetConnect,
            DBI_DatabaseInfo,
            DBI_Describe,
            DBI_Disconnect,
            DBI_ErrorSQL,
            DBI_Execute,
            DBI_FetchNext,
            DBI_Prepare,
            DBI_PrepareWithParms,
            DBI_RollBack,
            DBI_TerminateSQL,
            DBI_Rows,
            DBI_GetPBTypeString,
            DBI_DescribeExtra,
            DBI_DateString,
            DBI_DecimalString,
            DBI_DoubleString,
            DBI_Numeri_tstring,
            DBI_CursorConnect,
            DBI_GetSelectItems,
            DBI_GetSelectInfo,
            DBI_BindSelectBuffer,
            DBI_RuntimeFetchNext,
            DBI_MPowerFetchNext,
            DBI_StartTran,
            DBI_UniqueKey,
            DBI_ParseFrom,
            DBI_ParseColList,
            DBI_DoubleTheQuotes,
            DBI_FreeMem,
            DBI_FreePrepList,
            DBI_FreeColBlkList,
            DBI_DialogBoxCenter,
            DBI_CleanUpColumnList,
            DBI_Cancel,
            DBI_ProcInfo,
            DBI_ProcText,
            DBI_EventText,
            DBI_ProcDescribe,
            DBI_DeleteProcDir,
            DBI_ProcPrepare,
            DBI_Parse,
            DBI_OuterJoinSyntax,
            DBI_RuntimeExecute,
            DBI_FormatHash,
            DBI_ValidHash,
            DBI_FreeValidHash,
            DBI_EditStyleHash,
            DBI_EditUpdate,
            DBI_EditStyleInfo,
            DBI_LoadString,
            DBI_DynamicBind,
            DBI_GetNextResultSet,
            DBI_ViewText,
            DBI_ExecPlan,
            DBI_GetTimestamp,
            DBI_DoCompare,
            DBI_DoCompareFirst,
            DBI_DoCompareFirstWithSkip,
            DBI_ReadBlob,
            DBI_WriteBlob,
            DBI_FillBlanks,
            DBI_DeleteSyntaxList,
            DBI_ParseParms,
            DBI_ParseKeyWords,
            DBI_FreeParmList,
            DBI_GetParm,
            DBI_FetchFirst,
            DBI_FetchPrev,
            DBI_FetchRandom,
            DBI_FetchRelative,
            DBI_FetchLast,
            DBI_NewDBParm,
            DBI_ReplaceDbParm,
            DBI_DelimitReservedWord,
            DBI_DescribeInput,
            DBI_DescribeOutput,
            DBI_DWCursorConnect,
            DBI_ParseColSubset,
            DBI_ParseBasicSelect,
            DBI_CreatePrimaryKeySyntax,
            DBI_CreateNoLogPKeySyntax,
            DBI_AlterPrimaryKeySyntax,
            DBI_CreateForeignKeySyntax,
            DBI_AlterForeignKeySyntax,
            DBI_GetForeignKYOptions,
            DBI_LibraryName,
            DBI_LookForKeyWord,
            DBI_ParseIdentifier,
            DBI_GetColumnExpression,
            DBI_ParseWhere,
            DBI_FreeWhereList,
            DBI_PrimaryKeyReferences,
            DBI_PBToSQL,
            DBI_PBToArgs,
            DBI_ReleaseInputParms,
            DBI_SynText,
            DBI_PBC_DialogBox,
            DBI_PBC_DialogBoxParam,
            DBI_PBC_ShowWindow,
            DBI_DBHandle,
            DBI_GetAdoConnection,
            DBI_SearchReplace,
            DBI_CtrlChars2Text,
            DBI_Text2CtrlChars,
            DBI_SQLCacheBegin,
            DBI_SQLCacheEnd,
            DBI_SQLCacheFlushEntries,
            DBI_SQLCacheRegisterSQLStatement,
            DBI_SQLCacheRegisterDescribe,
            DBI_SQLCacheRequestSqlStatement,
            DBI_SQLCacheRequestDescribe,
            DBI_SQLCacheMakeSQLStatementAvailable,
            DBI_SQLCacheSetCacheSize,
            DBI_SQLCacheDiscardEntry,
            DBI_SQLCacheConnectSetup,
            DBI_SQLCacheConnectDrop,
            DBI_SQLCacheSetSelectCacheSize,
            DBI_SQLCacheStatistics,
            DBI_GetIdentityValue,
            DBI_FormatWindowsEOL,
            DBI_RegisterVendor,
            DBI_UnRegisterVendor,
            DBI_IsCache,
            DBI_IsConnInServerTrans,
            DBI_IsConnFromServer,
            DBI_ConvertHexStringToBlob,
            CreateDBIConnect,
            CreateDBITableSchema,
            rtdb_close,
            rtdb_commit,
            rtdb_delete,
            rtdb_deletewithcurs,
            rtdb_describe,
            rtdb_execute,
            rtdb_executedyn,
            rtdb_executeimmed,
            rtdb_execdynproc,
            rtdb_execdynwithdesc,
            rtdb_fetch,
            rtdb_fetchwithdesc,
            rtdb_insert,
            rtdb_open,
            rtdb_opendyn,
            rtdb_opendynwithdesc,
            rtdb_prepare,
            rtdb_rollback,
            rtdb_select,
            rtdb_selectblob,
            rtdb_start,
            rtdb_stop,
            rtdb_update,
            rtdb_updatewithcurs,
            rtdb_updateblob,
            rtdb_trans_pool_login,
            rtdb_trans_pool_disconnect,
            ob_set_curr_rtinst_and_return,
            ob_unset_curr_rtinst_and_return,
            ob_open_trace,
            ob_close_trace,
            ob_begin_trace,
            ob_end_trace,
            ob_enable_event_trace,
            ob_disable_event_trace,
            otcg_string_cat,
            otcg_binary_cat,
            otcg_add_object_argument_reference,
            otcg_embedded_group
        })
    }
}
