use crate::{
    pbx::{bindings::*, value::FromValue, *}, primitive::*
};

/// 过程调用上下文
pub struct CallInfo {
    ci: _PBCallInfo,
    mid: MethodId,
    session: Session
}

impl CallInfo {
    pub(crate) unsafe fn new(cls: pbclass, mid: MethodId, session: Session) -> Result<CallInfo> {
        let mut ci = _PBCallInfo {
            pArgs: NonNull::dangling(),
            returnValue: NonNull::dangling(),
            returnClass: NonNull::dangling()
        };
        let pbxr = ffi::pbsession_InitCallInfo(session.as_ptr(), cls, mid, (&mut ci).into());
        if pbxr != PBXRESULT::OK {
            return Err(pbxr);
        }
        Ok(CallInfo {
            ci,
            mid,
            session
        })
    }
    pub(crate) fn as_ptr(&self) -> pbcallinfo { (&self.ci).into() }
    pub(crate) fn mid(&self) -> MethodId { self.mid }

    /// 获取取引用对象
    pub fn as_ref(&self) -> CallInfoRef {
        unsafe { CallInfoRef::from_ptr((&self.ci).into(), self.session.clone()) }
    }

    /// 获取关联的`Session`
    pub fn session(&self) -> &Session { &self.session }

    /// 获取指定参数
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn arg(&self, idx: pbint) -> Value { self.args().get(idx) }

    /// 获取参数数量
    pub fn arg_count(&self) -> pbint { self.args().count() }

    /// 获取参数列表对象
    pub fn args(&self) -> Arguments {
        unsafe { Arguments::from_ptr((&self.ci).into(), self.session.clone()) }
    }

    /// 获取返回值对象
    pub fn return_value(&self) -> Value {
        unsafe { Value::from_ptr(self.ci.returnValue, self.session.clone()) }
    }
}

impl Drop for CallInfo {
    fn drop(&mut self) {
        unsafe {
            ffi::pbsession_FreeCallInfo(self.session.as_ptr(), (&self.ci).into());
        }
    }
}

/// 调用上下文引用
pub struct CallInfoRef<'ci> {
    ptr: pbcallinfo,
    session: Session,
    _marker: PhantomData<&'ci pbcallinfo>
}

impl<'ci> CallInfoRef<'ci> {
    pub(crate) unsafe fn from_ptr(ptr: pbcallinfo, session: Session) -> CallInfoRef<'ci> {
        CallInfoRef {
            ptr,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn as_ptr(&self) -> pbcallinfo { self.ptr }
    pub(crate) fn clone(&self) -> CallInfoRef<'ci> {
        CallInfoRef {
            ptr: self.ptr,
            session: self.session.clone(),
            _marker: PhantomData
        }
    }

    /// 获取关联的`Session`
    pub fn session(&self) -> &Session { &self.session }

    /// 获取指定参数
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn arg(&self, idx: pbint) -> Value<'ci> { self.args().get(idx) }

    /// 获取参数数量
    pub fn arg_count(&self) -> pbint { self.args().count() }

    /// 获取参数列表对象
    pub fn args(&self) -> ArgumentsRef<'ci> {
        unsafe { ArgumentsRef::from_ptr(self.ptr.as_ref().pArgs, self.session.clone()) }
    }

    /// 获取返回值对象
    pub fn return_value(&self) -> Value<'ci> {
        unsafe { Value::from_ptr(self.ptr.as_ref().returnValue, self.session.clone()) }
    }
}

/// 提取参数列表
pub trait FromCallInfo<'ci>: Sized {
    fn from_callinfo(ci: &CallInfoRef<'ci>) -> Result<Self>;
}

pub trait FromArg<'ci>: Sized {
    fn from_arg(
        ci: &CallInfoRef<'ci>,
        args: &ArgumentsRef<'ci>,
        idx: &mut pbint,
        complete: &mut bool
    ) -> Result<Self>;
}

impl<'ci> FromArg<'ci> for CallInfoRef<'ci> {
    fn from_arg(
        ci: &CallInfoRef<'ci>,
        _: &ArgumentsRef<'ci>,
        _: &mut pbint,
        complete: &mut bool
    ) -> Result<Self> {
        *complete = true;
        Ok(ci.clone())
    }
}
impl<'ci> FromArg<'ci> for ArgumentsRef<'ci> {
    fn from_arg(
        _: &CallInfoRef<'ci>,
        args: &ArgumentsRef<'ci>,
        _: &mut pbint,
        complete: &mut bool
    ) -> Result<Self> {
        *complete = true;
        Ok(args.clone())
    }
}
impl FromArg<'_> for Session {
    fn from_arg(ci: &CallInfoRef, _: &ArgumentsRef, _: &mut pbint, _: &mut bool) -> Result<Session> {
        Ok(ci.session().clone())
    }
}

impl<'ci, T: FromValue<'ci>> FromArg<'ci> for T {
    fn from_arg(_: &CallInfoRef, args: &ArgumentsRef<'ci>, idx: &mut pbint, _: &mut bool) -> Result<Self> {
        *idx += 1;
        T::from_value(if *idx < args.count() {
            Some(args.try_get(*idx)?)
        } else {
            None
        })
    }
}

#[rustfmt::skip]
mod m {
    use super::*;

    /// 提取CallInfo参数为tuple
    macro_rules! tuple_from_ci {
        ($($T:ident),+) => {
            #[doc(hidden)]
            impl<'ci,$($T:FromArg<'ci>),+> FromCallInfo<'ci> for ($($T,)+)
            {
                fn from_callinfo(ci: &CallInfoRef<'ci>) -> Result<Self> {
                    let args = ci.args();
                    let mut idx = -1;
                    let mut complete = false;
                    let rv = ($($T::from_arg(ci,&args,&mut idx,&mut complete)?,)+);
                    if !complete && idx + 1 < args.count() {
                        return Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS);
                    }
                    Ok(rv)
                }
            }
        }
    }

    #[doc(hidden)]
    impl FromCallInfo<'_> for () {
        fn from_callinfo(_: &CallInfoRef) -> Result<Self> { Ok(()) }
    }

    tuple_from_ci!(A);
    tuple_from_ci!(A, B);
    tuple_from_ci!(A, B, C);
    tuple_from_ci!(A, B, C, D);
    tuple_from_ci!(A, B, C, D, E);
    tuple_from_ci!(A, B, C, D, E, F);
    tuple_from_ci!(A, B, C, D, E, F, G);
    tuple_from_ci!(A, B, C, D, E, F, G, H);
    tuple_from_ci!(A, B, C, D, E, F, G, H, I);
    tuple_from_ci!(A, B, C, D, E, F, G, H, I, J);
    tuple_from_ci!(A, B, C, D, E, F, G, H, I, J, K);
    tuple_from_ci!(A, B, C, D, E, F, G, H, I, J, K, L);
    tuple_from_ci!(A, B, C, D, E, F, G, H, I, J, K, L, M);
    tuple_from_ci!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    tuple_from_ci!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);

}
