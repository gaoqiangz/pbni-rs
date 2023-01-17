use crate::{
    pbx::{bindings::*, userobject::UserObjectWrap, value::FromValueOwned, *}, prelude::*
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
    pub(crate) unsafe fn from_raw(ptr: pbobject, session: Session) -> Object<'obj> {
        let group = Cell::new(None);
        let cls = ffi::pbsession_GetClass(session.as_raw(), ptr).expect("invalid object");
        Object {
            ptr,
            group,
            cls,
            session,
            _marker: PhantomData
        }
    }
    pub fn as_raw(&self) -> pbobject { self.ptr }
    pub(crate) fn get_group(&self) -> pbgroup {
        match self.group.get() {
            Some(group) => group,
            None => {
                let group = self.session.get_group(self.cls);
                self.group.set(group);
                group.expect("invalid class")
            }
        }
    }
    pub(crate) fn get_class(&self) -> pbclass { self.cls }

    pub fn get_class_name(&self) -> String {
        unsafe {
            let cls_name = ffi::pbsession_GetClassName(self.session.as_raw(), self.cls);
            if !cls_name.is_null() {
                PBStr::from_ptr_str(cls_name).to_string_lossy()
            } else {
                "".to_string()
            }
        }
    }

    pub fn get_session(&self) -> Session { self.session }

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
        unsafe { ffi::pbsession_IsNativeObject(self.session.as_raw(), self.ptr).into() }
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
    pub fn get_native_ref<T: UserObject>(&self) -> Result<&'obj T> {
        /*if !self.is_native() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }*/
        unsafe {
            let obj = ffi::pbsession_GetNativeInterface(self.session.as_raw(), self.ptr);
            if obj.is_none() {
                return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
            }
            let ctx = ffi::GetSafeContext(obj.unwrap(), type_id::<T>());
            if ctx.is_null() {
                return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
            } else {
                Ok(&*(ctx as *const UserObjectWrap<T>))
            }
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
    pub fn get_native_mut<T: UserObject>(&mut self) -> Result<&'obj mut T> {
        /*if !self.is_native() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }*/
        unsafe {
            let obj = ffi::pbsession_GetNativeInterface(self.session.as_raw(), self.ptr);
            if obj.is_none() {
                return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
            }
            let ctx = ffi::GetSafeContext(obj.unwrap(), type_id::<T>());
            if ctx.is_null() {
                return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
            } else {
                Ok(&mut *(ctx as *mut UserObjectWrap<T>))
            }
        }
    }

    /// 共享对象
    pub fn share(&self) -> SharedObject { unsafe { SharedObject::from_raw(self.ptr, self.session.clone()) } }

    /// 转换为共享对象
    pub fn into_shared(self) -> SharedObject { self.into() }
}

/// 共享的对象
/// 增加了全局引用计数
pub struct SharedObject {
    obj: Object<'static>
}

impl SharedObject {
    pub(crate) unsafe fn from_raw(ptr: pbobject, session: Session) -> SharedObject {
        ffi::pbsession_AddGlobalRef(session.as_raw(), ptr);
        SharedObject {
            obj: Object::from_raw(ptr, session)
        }
    }
}

impl Drop for SharedObject {
    fn drop(&mut self) { unsafe { ffi::pbsession_RemoveGlobalRef(self.obj.session.as_raw(), self.obj.ptr) } }
}

impl Clone for SharedObject {
    fn clone(&self) -> Self { unsafe { Self::from_raw(self.obj.ptr, self.obj.session.clone()) } }
}

impl Deref for SharedObject {
    type Target = Object<'static>;
    fn deref(&self) -> &Self::Target { &self.obj }
}

impl DerefMut for SharedObject {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.obj }
}

impl From<Object<'_>> for SharedObject {
    fn from(obj: Object) -> Self { unsafe { Self::from_raw(obj.ptr, obj.session) } }
}
