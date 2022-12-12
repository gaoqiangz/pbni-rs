use crate::pbni::{bindings::*, session::OwnedSession, *};
use libloading::Library;
use std::{borrow::Cow, ffi::OsStr};

/// 虚拟机对象
///
/// # Examples
/// ```
/// let vm = VM::new(r#"C:\Program Files (x86)\Appeon\Shared\PowerBuilder\PBVM190.DLL"#).unwrap();
/// let session = vm.new_session("pbrs", &[r#"pbrs\pbw\pbrs.pbl"#]).unwrap();
/// ```
pub struct VM {
    ptr: pbvm,
    lib: Library
}

impl VM {
    /// 从指定`PBVM`dll路径加载虚拟机对象
    pub fn new(file_name: impl AsRef<OsStr>) -> Result<VM> {
        unsafe {
            let lib = Library::new(file_name).map_err(|_| PBXRESULT::E_GET_PBVM_FAILED)?;
            let fn_get_vm = lib
                .get::<fn(*mut Option<pbvm>) -> PBXRESULT>(b"PB_GetVM\0")
                .map_err(|_| PBXRESULT::E_GET_PBVM_FAILED)?;
            let mut pvm = None;
            let pbxr = fn_get_vm(&mut pvm);
            if pbxr.is_err() {
                return Err(pbxr);
            }
            if pvm.is_none() {
                return Err(PBXRESULT::E_GET_PBVM_FAILED);
            }
            Ok(VM {
                ptr: pvm.unwrap(),
                lib
            })
        }
    }

    /// 新建Session对象
    pub fn new_session<'a>(
        &'a self,
        applicationName: impl AsPBStr,
        libraryList: &[impl AsPBStr]
    ) -> Result<OwnedSession<'a>> {
        if libraryList.is_empty() {
            return Err(PBXRESULT::E_INVALID_ARGUMENT);
        }
        let applicationName = applicationName.as_pbstr();
        if applicationName.is_empty() {
            return Err(PBXRESULT::E_INVALID_ARGUMENT);
        }
        unsafe {
            let libraryList: Vec<Cow<'_, PBStr>> = libraryList.iter().map(|item| item.as_pbstr()).collect();
            let libraryList: Vec<LPCTSTR> = libraryList.iter().map(|item| item.as_ptr()).collect();
            let mut session = NonNull::dangling();
            let pbxr = ffi::pbvm_CreateSession(
                self.ptr,
                applicationName.as_ptr(),
                libraryList.as_ptr(),
                libraryList.len() as pbuint,
                &mut session
            );
            if pbxr.is_ok() {
                Ok(OwnedSession::from_ptr(session))
            } else {
                Err(pbxr)
            }
        }
    }

    /// 运行PB工程
    pub fn run_application<'a>(
        &'a self,
        applicationName: impl AsPBStr,
        libraryList: &[impl AsPBStr],
        commandLine: impl AsPBStr
    ) -> Result<OwnedSession<'a>> {
        if libraryList.is_empty() {
            return Err(PBXRESULT::E_INVALID_ARGUMENT);
        }
        let applicationName = applicationName.as_pbstr();
        if applicationName.is_empty() {
            return Err(PBXRESULT::E_INVALID_ARGUMENT);
        }
        unsafe {
            let libraryList: Vec<Cow<'_, PBStr>> = libraryList.iter().map(|item| item.as_pbstr()).collect();
            let libraryList: Vec<LPCTSTR> = libraryList.iter().map(|item| item.as_ptr()).collect();
            let mut session = NonNull::dangling();
            let pbxr = ffi::pbvm_RunApplication(
                self.ptr,
                applicationName.as_ptr(),
                libraryList.as_ptr(),
                libraryList.len() as pbuint,
                commandLine.as_pbstr().as_ptr(),
                &mut session
            );
            if pbxr.is_ok() {
                Ok(OwnedSession::from_ptr(session))
            } else {
                Err(pbxr)
            }
        }
    }
}
