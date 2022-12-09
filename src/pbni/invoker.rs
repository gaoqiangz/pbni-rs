use crate::pbni::{bindings::*, *};

/// 全局函数调用参数
pub struct GlobalFunction<'session> {
    cls: pbclass,
    _marker: PhantomData<&'session pbclass>
}

impl<'session> GlobalFunction<'session> {
    pub(crate) fn new(cls: pbclass) -> Self {
        GlobalFunction {
            cls,
            _marker: PhantomData
        }
    }
}

/// 对象函数调用参数
pub struct ObjectMethod<'a, 'obj> {
    obj: &'a mut Object<'obj>
}

impl<'a, 'obj> ObjectMethod<'a, 'obj> {
    pub(crate) fn new(obj: &'a mut Object<'obj>) -> Self {
        ObjectMethod {
            obj
        }
    }
}

/// 对象事件调用参数
pub struct ObjectEvent<'a, 'obj> {
    obj: &'a mut Object<'obj>
}

impl<'a, 'obj> ObjectEvent<'a, 'obj> {
    pub(crate) fn new(obj: &'a mut Object<'obj>) -> Self {
        ObjectEvent {
            obj
        }
    }
}

/// 过程调用对象
pub struct Invoker<T> {
    inner: T,
    ci: CallInfo
}

impl<T> Invoker<T> {
    /// 获取指定参数
    pub fn arg(&self, idx: pbint) -> Value { self.ci.arg(idx) }

    /// 获取参数数量
    pub fn arg_count(&self) -> pbint { self.ci.arg_count() }

    /// 获取参数对象
    pub fn args(&self) -> Arguments { self.ci.args() }
}

impl<'session> Invoker<GlobalFunction<'session>> {
    pub(crate) fn new(inner: GlobalFunction<'session>, ci: CallInfo) -> Self {
        Invoker {
            inner,
            ci
        }
    }

    /// 函数调用
    pub fn invoke(&self) -> Result<Value> {
        unsafe {
            let session = self.ci.session().clone();
            let pbxr = ffi::pbsession_InvokeClassFunction(
                session.as_ptr(),
                self.inner.cls,
                self.ci.mid(),
                self.ci.as_ptr()
            );
            if session.has_exception() {
                //传播异常
                Err(PBXRESULT::OK)
            } else {
                if pbxr.is_ok() {
                    Ok(self.ci.return_value())
                } else {
                    Err(pbxr)
                }
            }
        }
    }
}

impl<'a, 'obj> Invoker<ObjectMethod<'a, 'obj>> {
    pub(crate) fn new(inner: ObjectMethod<'a, 'obj>, ci: CallInfo) -> Self {
        Invoker {
            inner,
            ci
        }
    }

    /// 函数调用
    pub fn invoke(&self) -> Result<Value> {
        unsafe {
            let session = self.ci.session().clone();
            let pbxr = ffi::pbsession_InvokeObjectFunction(
                session.as_ptr(),
                self.inner.obj.as_ptr(),
                self.ci.mid(),
                self.ci.as_ptr()
            );
            if session.has_exception() {
                //传播异常
                Err(PBXRESULT::OK)
            } else {
                if pbxr.is_ok() {
                    Ok(self.ci.return_value())
                } else {
                    Err(pbxr)
                }
            }
        }
    }
}

impl<'a, 'obj> Invoker<ObjectEvent<'a, 'obj>> {
    pub(crate) fn new(inner: ObjectEvent<'a, 'obj>, ci: CallInfo) -> Self {
        Invoker {
            inner,
            ci
        }
    }

    /// 事件调用
    pub fn trigger(&self) -> Result<Value> {
        unsafe {
            let session = self.ci.session().clone();
            let pbxr = ffi::pbsession_TriggerEvent(
                session.as_ptr(),
                self.inner.obj.as_ptr(),
                self.ci.mid(),
                self.ci.as_ptr()
            );
            if session.has_exception() {
                //传播异常
                Err(PBXRESULT::OK)
            } else {
                if pbxr.is_ok() {
                    Ok(self.ci.return_value())
                } else {
                    Err(pbxr)
                }
            }
        }
    }
}
