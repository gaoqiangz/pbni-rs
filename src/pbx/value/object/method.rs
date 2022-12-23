//! 对象函数与事件调用接口实现
//!
use super::*;
use crate::{
    comm::*, pbx::invoker::{ObjectEvent, ObjectMethod}
};

/*
    Method calling
*/

/// 函数ID抽象
pub trait AsMethodId {
    fn as_mid(&self, obj: &Object) -> Result<MethodId>;
    fn as_eid(&self, obj: &Object) -> Result<MethodId>;
}

impl<T: AsMethodName> AsMethodId for T {
    #[inline]
    fn as_mid(&self, obj: &Object) -> Result<MethodId> {
        obj.get_method_id(self.as_method_name()).ok_or(PBXRESULT::E_INVALID_METHOD_ID)
    }
    #[inline]
    fn as_eid(&self, obj: &Object) -> Result<MethodId> {
        obj.get_event_id(self.as_method_name()).ok_or(PBXRESULT::E_INVALID_METHOD_ID)
    }
}
impl AsMethodId for MethodId {
    #[inline]
    fn as_mid(&self, _obj: &Object) -> Result<MethodId> { Ok(*self) }
    #[inline]
    fn as_eid(&self, _obj: &Object) -> Result<MethodId> { Ok(*self) }
}
impl AsMethodId for Option<MethodId> {
    #[inline]
    fn as_mid(&self, _obj: &Object) -> Result<MethodId> { self.ok_or(PBXRESULT::E_INVALID_METHOD_ID) }
    #[inline]
    fn as_eid(&self, _obj: &Object) -> Result<MethodId> { self.ok_or(PBXRESULT::E_INVALID_METHOD_ID) }
}

impl<'obj> Object<'obj> {
    /// 查找函数ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = object.get_method_id("of_test").unwrap();
    /// let invoker = object.begin_invoke_method(fid).unwrap();
    /// invoker.invoke();
    /// ```
    pub fn get_method_id(&self, name: impl AsMethodName) -> Option<MethodId> {
        let (name, sign) = name.as_method_name();
        self.session.get_method_id(self.cls, name, RoutineType::Function, sign, false)
    }

    /// 查找事件ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = object.get_event_id("on_test").unwrap();
    /// let invoker = object.begin_invoke_event(fid).unwrap();
    /// invoker.trigger();
    /// ```
    pub fn get_event_id(&self, name: impl AsMethodName) -> Option<MethodId> {
        let (name, sign) = name.as_method_name();
        self.session.get_method_id(self.cls, name, RoutineType::Event, sign, false)
    }

    /// 调用函数
    ///
    /// # Examples
    ///
    /// ```
    /// let rv: pbint = obj.invoke_method("of_Test", pbargs!["abcd", 123]).unwrap();
    /// ```
    pub fn invoke_method<F, R>(&mut self, mid: impl AsMethodId, arg_cb: F) -> Result<R>
    where
        F: FnOnce(Arguments) -> Result<()>,
        R: FromValueOwned
    {
        let invoker = self.begin_invoke_method(mid)?;
        arg_cb(invoker.args())?;
        let rv = invoker.invoke()?;
        R::from_value(Some(rv))
    }

    /// 调用事件
    ///
    /// # Examples
    ///
    /// ```
    /// let rv: pbint = obj.trigger_event("onTest", pbargs!["abcd", 123]).unwrap();
    /// ```
    pub fn trigger_event<F, R>(&mut self, mid: impl AsMethodId, arg_cb: F) -> Result<R>
    where
        F: FnOnce(Arguments) -> Result<()>,
        R: FromValueOwned
    {
        let invoker = self.begin_invoke_event(mid)?;
        arg_cb(invoker.args())?;
        let rv = invoker.trigger()?;
        R::from_value(Some(rv))
    }

    /// 初始化函数调用上下文
    ///
    /// # Examples
    ///
    /// ```
    /// let invoker = object.begin_invoke_method("of_test").unwrap();
    /// invoker.invoke();
    /// ```
    pub fn begin_invoke_method<'a>(
        &'a mut self,
        mid: impl AsMethodId
    ) -> Result<Invoker<ObjectMethod<'a, 'obj>>> {
        let mid = mid.as_mid(self)?;
        let ci = unsafe { CallInfo::new(self.cls, mid, self.session.clone())? };
        Ok(Invoker::<ObjectMethod>::new(ObjectMethod::new(self), ci))
    }

    /// 初始化事件调用上下文
    ///
    /// # Examples
    ///
    /// ```
    /// let invoker = object.begin_invoke_event("on_test").unwrap();
    /// invoker.trigger();
    /// ```
    pub fn begin_invoke_event<'a>(
        &'a mut self,
        mid: impl AsMethodId
    ) -> Result<Invoker<ObjectEvent<'a, 'obj>>> {
        let mid = mid.as_eid(self)?;
        let ci = unsafe { CallInfo::new(self.cls, mid, self.session.clone())? };
        Ok(Invoker::<ObjectEvent>::new(ObjectEvent::new(self), ci))
    }
}
