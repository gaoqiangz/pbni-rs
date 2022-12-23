//! 全局函数调用接口实现
//!
use super::*;
use crate::comm::*;

/*
    Method calling
*/

/// 全局函数ID
#[derive(Clone, Copy)]
pub struct GlobalFunctionId {
    pub(crate) cls: pbclass,
    pub(crate) mid: MethodId
}

/// 全局函数ID抽象
pub trait AsGlobalFunctionId {
    fn as_mid(&self, session: &Session) -> Result<GlobalFunctionId>;
}

impl<T: AsMethodName> AsGlobalFunctionId for T {
    #[inline]
    fn as_mid(&self, session: &Session) -> Result<GlobalFunctionId> {
        session.get_function_id(self.as_method_name()).ok_or(PBXRESULT::E_INVALID_METHOD_ID)
    }
}
impl AsGlobalFunctionId for GlobalFunctionId {
    #[inline]
    fn as_mid(&self, _session: &Session) -> Result<GlobalFunctionId> { Ok(*self) }
}
impl AsGlobalFunctionId for Option<GlobalFunctionId> {
    #[inline]
    fn as_mid(&self, _session: &Session) -> Result<GlobalFunctionId> {
        self.ok_or(PBXRESULT::E_INVALID_METHOD_ID)
    }
}

impl Session {
    /// 获取用户定义全局函数ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = session.get_user_function_id("gf_test").unwrap();
    /// let invoker = session.begin_invoke_function(fid).unwrap();
    /// invoker.invoke();
    /// ```
    pub fn get_user_function_id(&self, name: impl AsMethodName) -> Option<GlobalFunctionId> {
        let (name, sign) = name.as_method_name();
        if let Some(group) = self.find_group(name.as_ref(), GroupType::Function) {
            self.find_class(group, name.as_ref()).and_then(|cls| {
                self.get_method_id(cls, name.as_ref(), RoutineType::Function, sign, true).map(|mid| {
                    GlobalFunctionId {
                        cls,
                        mid
                    }
                })
            })
        } else {
            None
        }
    }

    /// 获取系统全局函数ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = session.get_system_function_id(("MessageBox","ISS")).unwrap();
    /// let invoker = session.begin_invoke_function(fid).unwrap();
    /// invoker.arg(0).set_str("title");
    /// invoker.arg(1).set_str("content");
    /// invoker.invoke();
    /// ```
    pub fn get_system_function_id(&self, name: impl AsMethodName) -> Option<GlobalFunctionId> {
        let (name, sign) = name.as_method_name();
        self.find_class(self.get_system_group(), "SystemFunctions").and_then(|cls| {
            self.get_method_id(cls, name, RoutineType::Function, sign, true).map(|mid| {
                GlobalFunctionId {
                    cls,
                    mid
                }
            })
        })
    }

    /// 获取任意全局函数ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = session.get_function_id(("MessageBox","ISS")).unwrap();
    /// let invoker = session.begin_invoke_function(fid).unwrap();
    /// invoker.arg(0).set_str("title");
    /// invoker.arg(1).set_str("content");
    /// invoker.invoke();
    /// ```
    pub fn get_function_id(&self, name: impl AsMethodName) -> Option<GlobalFunctionId> {
        let (name, sign) = name.as_method_name();
        self.get_user_function_id((name.as_ref(), sign.as_ref()))
            .or_else(|| self.get_system_function_id((name, sign)))
    }

    /// 调用全局函数
    ///
    /// # Examples
    ///
    /// ```
    /// let rv: pbint = session.invoke_function(("MessageBox","ISS"),pbargs!["title","content"]).unwrap();
    /// ```
    pub fn invoke_function<F, R>(&self, mid: impl AsGlobalFunctionId, arg_cb: F) -> Result<R>
    where
        F: FnOnce(Arguments) -> Result<()>,
        R: FromValueOwned
    {
        let invoker = self.begin_invoke_function(mid)?;
        arg_cb(invoker.args())?;
        let rv = invoker.invoke()?;
        R::from_value(Some(rv))
    }

    /// 初始化全局函数调用上下文
    ///
    /// # Examples
    ///
    /// ```
    /// let invoker = session.begin_invoke_function(("MessageBox","ISS")).unwrap();
    /// invoker.arg(0).set_str("title");
    /// invoker.arg(1).set_str("content");
    /// invoker.invoke();
    /// ```
    pub fn begin_invoke_function<'a>(
        &'a self,
        mid: impl AsGlobalFunctionId
    ) -> Result<Invoker<GlobalFunction<'a>>> {
        let mid = mid.as_mid(self)?;
        let ci = unsafe { CallInfo::new(mid.cls, mid.mid, self.clone())? };
        Ok(Invoker::<GlobalFunction>::new(GlobalFunction::new(mid.cls), ci))
    }

    pub(crate) fn get_method_id(
        &self,
        cls: pbclass,
        methodName: impl AsPBStr,
        rt: RoutineType,
        signature: impl AsPBStr,
        publicOnly: bool
    ) -> Option<MethodId> {
        unsafe {
            let mid = ffi::pbsession_GetMethodID(
                self.ptr,
                cls,
                methodName.as_pbstr().as_ptr(),
                rt,
                signature.as_pbstr().as_ptr(),
                publicOnly.into()
            );
            if mid.is_undefined() {
                None
            } else {
                Some(mid)
            }
        }
    }
    pub(crate) fn find_matching_function(
        &self,
        cls: pbclass,
        methodName: impl AsPBStr,
        rt: RoutineType,
        readableSignature: impl AsPBStr
    ) -> Option<MethodId> {
        unsafe {
            let mid = ffi::pbsession_FindMatchingFunction(
                self.ptr,
                cls,
                methodName.as_pbstr().as_ptr(),
                rt,
                readableSignature.as_pbstr().as_ptr()
            );
            if mid.is_undefined() {
                None
            } else {
                Some(mid)
            }
        }
    }
    pub(crate) fn get_method_id_by_event_id(&self, cls: pbclass, eventID: impl AsPBStr) -> Option<MethodId> {
        unsafe {
            let mid = ffi::pbsession_GetMethodIDByEventID(self.ptr, cls, eventID.as_pbstr().as_ptr());
            if mid.is_undefined() {
                None
            } else {
                Some(mid)
            }
        }
    }
}
