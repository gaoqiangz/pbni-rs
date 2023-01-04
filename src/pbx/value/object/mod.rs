use crate::{
    pbx::{bindings::*, value::FromValueOwned, *}, prelude::*
};
use std::{
    cell::Cell, ops::{Deref, DerefMut}
};

mod field;
mod method;

/// 对象实例的引用
pub struct Object<'obj> {
    ptr: pbobject,
    group: Cell<Option<pbgroup>>,
    cls: pbclass,
    session: Session,
    _marker: PhantomData<&'obj pbobject>
}

impl<'obj> Object<'obj> {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: Session) -> Object<'obj> {
        let group = Cell::new(None);
        let cls = ffi::pbsession_GetClass(session.as_ptr(), ptr).unwrap();
        Object {
            ptr,
            group,
            cls,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn as_ptr(&self) -> pbobject { self.ptr }
    pub(crate) fn get_group(&self) -> pbgroup {
        match self.group.get() {
            Some(group) => group,
            None => {
                let group = self.session.get_group(self.cls);
                self.group.set(group);
                group.unwrap()
            }
        }
    }
    pub(crate) fn get_class(&self) -> pbclass { self.cls }
    pub(crate) unsafe fn clone(&self) -> Object<'obj> {
        Object {
            ptr: self.ptr,
            group: self.group.clone(),
            cls: self.cls,
            session: self.session.clone(),
            _marker: PhantomData
        }
    }

    /// 是否为原生对象 (由pbni-rs导出的对象)
    /// FIXME: 始终返回false?
    pub fn is_native(&self) -> bool {
        unsafe { ffi::pbsession_IsNativeObject(self.session.as_ptr(), self.ptr).into() }
    }

    /// 获取原生对象的引用
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    ///
    /// # Examples
    ///
    /// ```
    /// let obj = obj.get_native_ref::<RustObject>().uwnrap();
    /// ```
    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    pub unsafe fn get_native_ref<T: UserObject>(&self) -> Result<&'obj T> {
        /*if !self.is_native() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }*/
        let obj = ffi::pbsession_GetNativeInterface(self.session.as_ptr(), self.ptr);
        if obj.is_none() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }
        let ctx = ffi::GetSafeContext(obj.unwrap(), type_id::<T>());
        if ctx.is_null() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        } else {
            Ok(&*(ctx as *const T))
        }
    }

    /// 获取原生对象的可变引用
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    ///
    /// # Examples
    ///
    /// ```
    /// let mut obj = obj.get_native_mut::<RustObject>().uwnrap();
    /// obj.set_var("is_test","rust").unwrap();
    /// ```
    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    pub unsafe fn get_native_mut<T: UserObject>(&mut self) -> Result<&'obj mut T> {
        /*if !self.is_native() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }*/
        let obj = ffi::pbsession_GetNativeInterface(self.session.as_ptr(), self.ptr);
        if obj.is_none() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }
        let ctx = ffi::GetSafeContext(obj.unwrap(), type_id::<T>());
        if ctx.is_null() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        } else {
            Ok(&mut *(ctx as *mut T))
        }
    }

    /// 共享对象
    pub fn share(&self) -> SharedObject { unsafe { SharedObject::from_ptr(self.ptr, self.session.clone()) } }

    /// 转换为共享对象
    pub fn into_shared(self) -> SharedObject { self.into() }
}

/// 共享的对象
/// 增加了全局引用计数
pub struct SharedObject {
    obj: Object<'static>
}

impl SharedObject {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: Session) -> SharedObject {
        ffi::pbsession_AddGlobalRef(session.as_ptr(), ptr);
        SharedObject {
            obj: Object::from_ptr(ptr, session)
        }
    }
}

impl Drop for SharedObject {
    fn drop(&mut self) { unsafe { ffi::pbsession_RemoveGlobalRef(self.obj.session.as_ptr(), self.obj.ptr) } }
}

impl Clone for SharedObject {
    fn clone(&self) -> Self { unsafe { Self::from_ptr(self.obj.ptr, self.obj.session.clone()) } }
}

impl Deref for SharedObject {
    type Target = Object<'static>;
    fn deref(&self) -> &Self::Target { &self.obj }
}

impl DerefMut for SharedObject {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.obj }
}

impl From<Object<'_>> for SharedObject {
    fn from(obj: Object) -> Self { unsafe { Self::from_ptr(obj.ptr, obj.session) } }
}

/// Rust对象绑定的PB对象
pub struct ContextObject {
    obj: Object<'static>
}

impl ContextObject {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: &Session) -> ContextObject {
        ContextObject {
            obj: Object::from_ptr(ptr, session.clone())
        }
    }

    /// 克隆对象
    ///
    /// # Safety
    ///
    /// 此方法不能延长对象的生命周期,因此不能保证克隆后的对象始终有效,生命周期将始终与此对象一样
    pub unsafe fn clone(&self) -> ContextObject {
        ContextObject {
            obj: self.obj.clone()
        }
    }
}

impl Deref for ContextObject {
    type Target = Object<'static>;
    fn deref(&self) -> &Self::Target { &self.obj }
}

impl DerefMut for ContextObject {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.obj }
}
