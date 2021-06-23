use crate::{
    bindings::*, invoker::{ObjectEvent, ObjectMethod}, session::AsMethodName, value::FromValueOwned, *
};
use std::{
    cell::{Cell, RefCell}, ops::{Deref, DerefMut}, rc::Rc
};

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

/*
    Instance variable
*/

/// 实例变量抽象
pub trait VarId {
    fn var_id(&self, obj: &Object) -> FieldId;
}

impl<T: AsPBStr> VarId for T {
    #[inline]
    fn var_id(&self, obj: &Object) -> FieldId {
        let pbstr = self.as_pbstr();
        obj.get_var_id(pbstr.as_ref())
            .ok_or_else(|| format!("invalid var {}", pbstr.to_string_lossy()))
            .unwrap()
    }
}
impl VarId for FieldId {
    #[inline]
    fn var_id(&self, _obj: &Object) -> FieldId { *self }
}
impl VarId for Option<FieldId> {
    #[inline]
    fn var_id(&self, _obj: &Object) -> FieldId { self.unwrap() }
}

impl<'obj> Object<'obj> {
    /// 获取实例变量数量
    ///
    /// # Examples
    ///
    /// ```
    /// let cnt = obj.get_var_count("is_var");
    /// //遍历所有变量并输出变量名
    /// for id in 0..cnt {
    ///     let fid = unsafe { FieldId::new(id) };
    ///     println!("field id: {}, name: {}", id, obj.get_var_name(fid));
    /// }
    /// ```
    pub fn get_var_count(&self) -> pbulong {
        unsafe { ffi::pbsession_GetNumOfFields(self.session.as_ptr(), self.cls) }
    }

    /// 查找实例变量ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = obj.get_var_id("is_var").unwrap();
    /// obj.set_var_str(fid,"rust");
    /// ```
    pub fn get_var_id(&self, name: impl AsPBStr) -> Option<FieldId> {
        unsafe {
            let fid = ffi::pbsession_GetFieldID(self.session.as_ptr(), self.cls, name.as_pbstr().as_ptr());
            if fid.is_undefined() {
                None
            } else {
                Some(fid)
            }
        }
    }

    /// 通过实例变量ID获取变量名
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = obj.get_var_id("is_var").unwrap();
    /// assert_eq!(pbstr!("is_var"),obj.get_var_name(fid));
    /// ```
    pub fn get_var_name(&self, fid: impl VarId) -> &PBStr {
        unsafe {
            PBStr::from_ptr_str(ffi::pbsession_GetFieldName(
                self.session.as_ptr(),
                self.cls,
                fid.var_id(self)
            ))
        }
    }

    /// 判断是否存在指定实例变量
    ///
    /// # Examples
    ///
    /// ```
    /// if object.has_var("is_var") {
    ///     object.set_var_str("is_var","rust");
    /// }
    /// ```
    pub fn has_var(self, name: impl AsPBStr) -> bool { self.get_var_id(name).is_some() }

    /// 获取变量类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// if object.get_var_type("is_var") == ValueType::String {
    ///     object.set_var_str("is_var","rust");
    /// }
    /// ```
    pub fn get_var_type(&self, fid: impl VarId) -> ValueType {
        unsafe { ffi::pbsession_GetFieldType(self.session.as_ptr(), self.cls, fid.var_id(self)) }
    }

    /// 判断实例变量是否为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_null(&self, fid: impl VarId) -> bool {
        unsafe { ffi::pbsession_IsFieldNull(self.session.as_ptr(), self.ptr, fid.var_id(self)).into() }
    }

    /// 判断实例变量类型是否为数组
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_array(&self, fid: impl VarId) -> bool {
        unsafe { ffi::pbsession_IsFieldArray(self.session.as_ptr(), self.cls, fid.var_id(self)).into() }
    }

    /// 判断实例变量类型是否为对象
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_object(&self, fid: impl VarId) -> bool {
        unsafe { ffi::pbsession_IsFieldObject(self.session.as_ptr(), self.cls, fid.var_id(self)).into() }
    }

    /// 刷新实例变量关联的UI状态,如窗口的`title`变量修改后需要调用此函数以刷新UI
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// object.set_var_str("title","rust");
    /// object.update_field("title");
    /// ```
    pub fn update_field(&self, fid: impl VarId) -> Result<()> {
        unsafe { ffi::pbsession_UpdateField(self.session.as_ptr(), self.ptr, fid.var_id(self)).into() }
    }

    /// 获取`int`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_int(&self, fid: impl VarId) -> Option<pbint> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Int);
        unsafe { self.get_var_int_unchecked(fid) }
    }

    /// 获取`int`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_int_unchecked(&self, fid: impl VarId) -> Option<pbint> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetIntField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`uint`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_uint(&self, fid: impl VarId) -> Option<pbuint> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Uint);
        unsafe { self.get_var_uint_unchecked(fid) }
    }

    /// 获取`uint`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_uint_unchecked(&self, fid: impl VarId) -> Option<pbuint> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetUintField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`long`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_long(&self, fid: impl VarId) -> Option<pblong> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Long);
        unsafe { self.get_var_long_unchecked(fid) }
    }

    /// 获取`long`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_long_unchecked(&self, fid: impl VarId) -> Option<pblong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetLongField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`ulong`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_ulong(&self, fid: impl VarId) -> Option<pbulong> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Ulong);
        unsafe { self.get_var_ulong_unchecked(fid) }
    }

    /// 获取`ulong`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_ulong_unchecked(&self, fid: impl VarId) -> Option<pbulong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetUlongField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`longlong`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_longlong(&self, fid: impl VarId) -> Option<pblonglong> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::LongLong);
        unsafe { self.get_var_longlong_unchecked(fid) }
    }

    /// 获取`longlong`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_longlong_unchecked(&self, fid: impl VarId) -> Option<pblonglong> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetLongLongField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`real`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_real(&self, fid: impl VarId) -> Option<pbreal> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Real);
        unsafe { self.get_var_real_unchecked(fid) }
    }

    /// 获取`real`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_real_unchecked(&self, fid: impl VarId) -> Option<pbreal> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetRealField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`double`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_double(&self, fid: impl VarId) -> Option<pbdouble> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Double);
        unsafe { self.get_var_double_unchecked(fid) }
    }

    /// 获取`double`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_double_unchecked(&self, fid: impl VarId) -> Option<pbdouble> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetDoubleField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`decimal`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn get_var_dec(&self, fid: impl VarId) -> Option<Decimal> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Decimal);
        unsafe { self.get_var_dec_unchecked(fid) }
    }

    /// 获取`decimal`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn get_var_dec_unchecked(&self, fid: impl VarId) -> Option<Decimal> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDecField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_dec_unchecked(v))
        }
    }

    /// 获取`string`类型实例变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_str(&self, fid: impl VarId) -> Option<&'obj PBStr> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::String);
        self.get_var_str_unchecked(fid)
    }

    /// 获取`string`类型实例变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_str_unchecked(&self, fid: impl VarId) -> Option<&'obj PBStr> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetStringField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            self.session.get_string_unchecked(v)
        }
    }

    /// 获取`string`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_string(&self, fid: impl VarId) -> Option<PBString> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::String);
        unsafe { self.get_var_string_unchecked(fid) }
    }

    /// 获取`string`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_string_unchecked(&self, fid: impl VarId) -> Option<PBString> {
        self.get_var_str_unchecked(fid).map(|v| v.to_ucstring())
    }

    /// 获取`boolean`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_bool(&self, fid: impl VarId) -> Option<bool> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Boolean);
        unsafe { self.get_var_bool_unchecked(fid) }
    }

    /// 获取`boolean`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_bool_unchecked(&self, fid: impl VarId) -> Option<bool> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetBoolField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v.into())
        }
    }

    /// 获取`blob`类型实例变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_blob(&self, fid: impl VarId) -> Option<&'obj [u8]> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Blob);
        self.get_var_blob_unchecked(fid)
    }

    /// 获取`blob`类型实例变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_blob_unchecked(&self, fid: impl VarId) -> Option<&'obj [u8]> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetBlobField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_blob_unchecked(v))
        }
    }

    /// 获取`date`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_var_date(&self, fid: impl VarId) -> Option<NaiveDate> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Date);
        unsafe { self.get_var_date_unchecked(fid) }
    }

    /// 获取`date`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_var_date_unchecked(&self, fid: impl VarId) -> Option<NaiveDate> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDateField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_date_unchecked(v))
        }
    }

    /// 获取`time`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_var_time(&self, fid: impl VarId) -> Option<NaiveTime> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Time);
        unsafe { self.get_var_time_unchecked(fid) }
    }

    /// 获取`time`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_var_time_unchecked(&self, fid: impl VarId) -> Option<NaiveTime> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetTimeField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_time_unchecked(v))
        }
    }

    /// 获取`datetime`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_var_datetime(&self, fid: impl VarId) -> Option<NaiveDateTime> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::DateTime);
        unsafe { self.get_var_datetime_unchecked(fid) }
    }

    /// 获取`datetime`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_var_datetime_unchecked(&self, fid: impl VarId) -> Option<NaiveDateTime> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetDateTimeField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_datetime_unchecked(v))
        }
    }

    /// 获取`char`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_char(&self, fid: impl VarId) -> Option<PBChar> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Char);
        unsafe { self.get_var_char_unchecked(fid) }
    }

    /// 获取`char`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_char_unchecked(&self, fid: impl VarId) -> Option<PBChar> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetCharField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`byte`类型实例变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_byte(&self, fid: impl VarId) -> Option<pbbyte> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Byte);
        unsafe { self.get_var_byte_unchecked(fid) }
    }

    /// 获取`byte`类型实例变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_byte_unchecked(&self, fid: impl VarId) -> Option<pbbyte> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetByteField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取对象类型实例变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_object(&self, fid: impl VarId) -> Option<Object<'obj>> {
        let fid = fid.var_id(self);
        assert!(self.is_var_object(fid));
        self.get_var_object_unchecked(fid)
    }

    /// 获取对象类型实例变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_object_unchecked(&self, fid: impl VarId) -> Option<Object<'obj>> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetObjectField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Object::from_ptr(v, self.session.clone()))
        }
    }

    /// 获取数组类型实例变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_array(&self, fid: impl VarId) -> Option<Array<'obj>> {
        let fid = fid.var_id(self);
        assert!(self.is_var_array(fid));
        self.get_var_array_unchecked(fid)
    }

    /// 获取数组类型实例变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_array_unchecked(&self, fid: impl VarId) -> Option<Array<'obj>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetArrayField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Array::from_ptr(v, self.is_var_object(fid), self.session.clone()))
        }
    }

    /// 获取`any`类型实例变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_any(&self, fid: impl VarId) -> Option<Value<'obj>> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Any);
        self.get_var_any_unchecked(fid)
    }

    /// 获取`any`类型实例变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_any_unchecked(&self, fid: impl VarId) -> Option<Value<'obj>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetPBAnyField(self.session.as_ptr(), self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Value::from_ptr(v, self.session.clone()))
        }
    }

    /// 设置实例变量的值为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn set_var_to_null(&self, fid: impl VarId) {
        unsafe { ffi::pbsession_SetFieldToNull(self.session.as_ptr(), self.ptr, fid.var_id(self)) }
    }

    /// 设置`int`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_int(&mut self, fid: impl VarId, value: pbint) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Int);
        unsafe { ffi::pbsession_SetIntField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`int`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_int_unchecked(&mut self, fid: impl VarId, value: pbint) -> Result<()> {
        ffi::pbsession_SetIntField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`uint`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_uint(&mut self, fid: impl VarId, value: pbuint) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Uint);
        unsafe { ffi::pbsession_SetUintField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`uint`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_uint_unchecked(&mut self, fid: impl VarId, value: pbuint) -> Result<()> {
        ffi::pbsession_SetUintField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`long`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_long(&mut self, fid: impl VarId, value: pblong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Long);
        unsafe { ffi::pbsession_SetLongField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`long`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_long_unchecked(&mut self, fid: impl VarId, value: pblong) -> Result<()> {
        ffi::pbsession_SetLongField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`ulong`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_ulong(&mut self, fid: impl VarId, value: pbulong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Ulong);
        unsafe { ffi::pbsession_SetUlongField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`ulong`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_ulong_unchecked(&mut self, fid: impl VarId, value: pbulong) -> Result<()> {
        ffi::pbsession_SetUlongField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`longlong`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_longlong(&mut self, fid: impl VarId, value: pblonglong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::LongLong);
        unsafe { ffi::pbsession_SetLongLongField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`longlong`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_longlong_unchecked(&mut self, fid: impl VarId, value: pblonglong) -> Result<()> {
        ffi::pbsession_SetLongLongField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`real`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_real(&mut self, fid: impl VarId, value: pbreal) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Real);
        unsafe { ffi::pbsession_SetRealField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`real`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_real_unchecked(&mut self, fid: impl VarId, value: pbreal) -> Result<()> {
        ffi::pbsession_SetRealField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`double`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_double(&mut self, fid: impl VarId, value: pbdouble) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Double);
        unsafe { ffi::pbsession_SetDoubleField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`double`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_double_unchecked(&mut self, fid: impl VarId, value: pbdouble) -> Result<()> {
        ffi::pbsession_SetDoubleField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`decimal`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn set_var_dec(&mut self, fid: impl VarId, value: Decimal) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Decimal);
        unsafe {
            ffi::pbsession_SetDecField(self.session.as_ptr(), self.ptr, fid, self.session.new_pbdec(value))
                .into()
        }
    }

    /// 设置`decimal`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn set_var_dec_unchecked(&mut self, fid: impl VarId, value: Decimal) -> Result<()> {
        ffi::pbsession_SetDecField(
            self.session.as_ptr(),
            self.ptr,
            fid.var_id(self),
            self.session.new_pbdec(value)
        )
        .into()
    }

    /// 设置`string`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_str(&mut self, fid: impl VarId, value: impl AsPBStr) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::String);
        unsafe {
            ffi::pbsession_SetStringField(self.session.as_ptr(), self.ptr, fid, value.as_pbstr().as_ptr())
                .into()
        }
    }

    /// 设置`string`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_str_unchecked(&mut self, fid: impl VarId, value: impl AsPBStr) -> Result<()> {
        ffi::pbsession_SetStringField(
            self.session.as_ptr(),
            self.ptr,
            fid.var_id(self),
            value.as_pbstr().as_ptr()
        )
        .into()
    }

    /// 设置`boolean`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_bool(&mut self, fid: impl VarId, value: bool) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Boolean);
        unsafe { ffi::pbsession_SetBoolField(self.session.as_ptr(), self.ptr, fid, value.into()).into() }
    }

    /// 设置`boolean`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_bool_unchecked(&mut self, fid: impl VarId, value: bool) -> Result<()> {
        ffi::pbsession_SetBoolField(self.session.as_ptr(), self.ptr, fid.var_id(self), value.into()).into()
    }

    /// 设置`blob`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_blob(&mut self, fid: impl VarId, value: impl AsRef<[u8]>) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Blob);
        unsafe {
            ffi::pbsession_SetBlobField(self.session.as_ptr(), self.ptr, fid, self.session.new_pbblob(value))
                .into()
        }
    }

    /// 设置`blob`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_blob_unchecked(&mut self, fid: impl VarId, value: impl AsRef<[u8]>) -> Result<()> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(PBXRESULT::E_OUTOF_MEMORY);
        }
        ffi::pbsession_SetBlobField(
            self.session.as_ptr(),
            self.ptr,
            fid.var_id(self),
            self.session.new_pbblob(value)
        )
        .into()
    }

    /// 设置`date`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_var_date(&mut self, fid: impl VarId, value: NaiveDate) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Date);
        unsafe {
            ffi::pbsession_SetDateField(self.session.as_ptr(), self.ptr, fid, self.session.new_pbdate(value))
                .into()
        }
    }

    /// 设置`date`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_var_date_unchecked(&mut self, fid: impl VarId, value: NaiveDate) -> Result<()> {
        ffi::pbsession_SetDateField(
            self.session.as_ptr(),
            self.ptr,
            fid.var_id(self),
            self.session.new_pbdate(value)
        )
        .into()
    }

    /// 设置`time`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_var_time(&mut self, fid: impl VarId, value: NaiveTime) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Time);
        unsafe {
            ffi::pbsession_SetTimeField(self.session.as_ptr(), self.ptr, fid, self.session.new_pbtime(value))
                .into()
        }
    }

    /// 设置`time`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_var_time_unchecked(&mut self, fid: impl VarId, value: NaiveTime) -> Result<()> {
        ffi::pbsession_SetTimeField(
            self.session.as_ptr(),
            self.ptr,
            fid.var_id(self),
            self.session.new_pbtime(value)
        )
        .into()
    }

    /// 设置`datetime`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_var_datetime(&mut self, fid: impl VarId, value: NaiveDateTime) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::DateTime);
        unsafe {
            ffi::pbsession_SetDateTimeField(
                self.session.as_ptr(),
                self.ptr,
                fid,
                self.session.new_pbdatetime(value)
            )
            .into()
        }
    }

    /// 设置`datetime`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_var_datetime_unchecked(&mut self, fid: impl VarId, value: NaiveDateTime) -> Result<()> {
        ffi::pbsession_SetDateTimeField(
            self.session.as_ptr(),
            self.ptr,
            fid.var_id(self),
            self.session.new_pbdatetime(value)
        )
        .into()
    }

    /// 设置`char`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_char(&mut self, fid: impl VarId, value: PBChar) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Char);
        unsafe { ffi::pbsession_SetCharField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`char`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_char_unchecked(&mut self, fid: impl VarId, value: PBChar) -> Result<()> {
        ffi::pbsession_SetCharField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`byte`类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_byte(&mut self, fid: impl VarId, value: pbbyte) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Byte);
        unsafe { ffi::pbsession_SetByteField(self.session.as_ptr(), self.ptr, fid, value).into() }
    }

    /// 设置`byte`类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_byte_unchecked(&mut self, fid: impl VarId, value: pbbyte) -> Result<()> {
        ffi::pbsession_SetByteField(self.session.as_ptr(), self.ptr, fid.var_id(self), value).into()
    }

    /// 设置对象类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_object(&mut self, fid: impl VarId, value: &Object) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.is_var_object(fid));
        unsafe { ffi::pbsession_SetObjectField(self.session.as_ptr(), self.ptr, fid, value.as_ptr()).into() }
    }

    /// 设置对象类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_object_unchecked(&mut self, fid: impl VarId, value: &Object) -> Result<()> {
        ffi::pbsession_SetObjectField(self.session.as_ptr(), self.ptr, fid.var_id(self), value.as_ptr())
            .into()
    }

    /// 设置数组类型实例变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_array(&mut self, fid: impl VarId, value: &Array) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.is_var_array(fid));
        unsafe { ffi::pbsession_SetArrayField(self.session.as_ptr(), self.ptr, fid, value.as_ptr()).into() }
    }

    /// 设置数组类型实例变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_array_unchecked(&mut self, fid: impl VarId, value: &Array) -> Result<()> {
        ffi::pbsession_SetArrayField(self.session.as_ptr(), self.ptr, fid.var_id(self), value.as_ptr()).into()
    }
}

/*
    Shared variable
*/

/// 共享(静态)变量ID抽象
pub trait SharedVarId {
    fn var_id(&self, obj: &Object) -> FieldId;
}

impl<T: AsPBStr> SharedVarId for T {
    #[inline]
    fn var_id(&self, obj: &Object) -> FieldId {
        let pbstr = self.as_pbstr();
        obj.get_shared_var_id(pbstr.as_ref())
            .ok_or_else(|| format!("invalid shared var {}", pbstr.to_string_lossy()))
            .unwrap()
    }
}
impl SharedVarId for FieldId {
    #[inline]
    fn var_id(&self, _obj: &Object) -> FieldId { *self }
}
impl SharedVarId for Option<FieldId> {
    #[inline]
    fn var_id(&self, _obj: &Object) -> FieldId { self.unwrap() }
}

impl<'obj> Object<'obj> {
    /// 获取共享(静态)变量ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = object.get_shared_var_id("ss_var").unwrap();
    /// object.set_shared_var_str(fid,"rust");
    /// ```
    pub fn get_shared_var_id(&self, name: impl AsPBStr) -> Option<FieldId> {
        unsafe {
            let fid = ffi::pbsession_GetSharedVarID(
                self.session.as_ptr(),
                self.get_group(),
                name.as_pbstr().as_ptr()
            );
            if fid.is_undefined() {
                None
            } else {
                Some(fid)
            }
        }
    }

    /// 判断是否存在指定共享(静态)变量
    ///
    /// # Examples
    ///
    /// ```
    /// if object.has_shared_var("ss_var") {
    ///     object.set_shared_var_str("ss_var","rust");
    /// }
    /// ```
    pub fn has_shared_var(self, name: impl AsPBStr) -> bool { self.get_shared_var_id(name).is_some() }

    /// 获取指定共享(静态)变量类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// if object.get_shared_var_type("ss_var") == ValueType::String {
    ///     object.set_shared_var_str("ss_var","rust");
    /// }
    /// ```
    pub fn get_shared_var_type(&self, fid: impl SharedVarId) -> ValueType {
        unsafe { ffi::pbsession_GetSharedVarType(self.session.as_ptr(), self.get_group(), fid.var_id(self)) }
    }

    /// 判断指定共享(静态)变量是否为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_null(&self, fid: impl SharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarNull(self.session.as_ptr(), self.get_group(), fid.var_id(self)).into()
        }
    }

    /// 判断指定共享(静态)变量是否为数组类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_array(&self, fid: impl SharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarArray(self.session.as_ptr(), self.get_group(), fid.var_id(self)).into()
        }
    }

    /// 判断指定共享(静态)变量是否为对象类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_object(&self, fid: impl SharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarObject(self.session.as_ptr(), self.get_group(), fid.var_id(self)).into()
        }
    }

    /// 获取`int`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_int(&self, fid: impl VarId) -> Option<pbint> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Int);
        unsafe { self.get_shared_var_int_unchecked(fid) }
    }

    /// 获取`int`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_int_unchecked(&self, fid: impl VarId) -> Option<pbint> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetIntSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`uint`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_uint(&self, fid: impl VarId) -> Option<pbuint> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Uint);
        unsafe { self.get_shared_var_uint_unchecked(fid) }
    }

    /// 获取`uint`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_uint_unchecked(&self, fid: impl VarId) -> Option<pbuint> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetUintSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`long`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_long(&self, fid: impl VarId) -> Option<pblong> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Long);
        unsafe { self.get_shared_var_long_unchecked(fid) }
    }

    /// 获取`long`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_long_unchecked(&self, fid: impl VarId) -> Option<pblong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetLongSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`ulong`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_ulong(&self, fid: impl VarId) -> Option<pbulong> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Ulong);
        unsafe { self.get_shared_var_ulong_unchecked(fid) }
    }

    /// 获取`ulong`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_ulong_unchecked(&self, fid: impl VarId) -> Option<pbulong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetUlongSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`longlong`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_longlong(&self, fid: impl VarId) -> Option<pblonglong> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::LongLong);
        unsafe { self.get_shared_var_longlong_unchecked(fid) }
    }

    /// 获取`longlong`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_longlong_unchecked(&self, fid: impl VarId) -> Option<pblonglong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetLongLongSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`real`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_real(&self, fid: impl VarId) -> Option<pbreal> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Real);
        unsafe { self.get_shared_var_real_unchecked(fid) }
    }

    /// 获取`real`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_real_unchecked(&self, fid: impl VarId) -> Option<pbreal> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetRealSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`double`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_double(&self, fid: impl VarId) -> Option<pbdouble> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Double);
        unsafe { self.get_shared_var_double_unchecked(fid) }
    }

    /// 获取`double`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_double_unchecked(&self, fid: impl VarId) -> Option<pbdouble> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDoubleSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`decimal`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn get_shared_var_dec(&self, fid: impl VarId) -> Option<Decimal> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Decimal);
        unsafe { self.get_shared_var_dec_unchecked(fid) }
    }

    /// 获取`decimal`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn get_shared_var_dec_unchecked(&self, fid: impl VarId) -> Option<Decimal> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDecSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(self.session.get_dec_unchecked(v))
        }
    }

    /// 获取`string`类型共享(静态)变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_str(&self, fid: impl VarId) -> Option<&'obj PBStr> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::String);
        self.get_shared_var_str_unchecked(fid)
    }

    /// 获取`string`类型共享(静态)变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_str_unchecked(&self, fid: impl VarId) -> Option<&'obj PBStr> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetStringSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            self.session.get_string_unchecked(v)
        }
    }

    /// 获取`string`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_string(&self, fid: impl VarId) -> Option<PBString> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::String);
        unsafe { self.get_shared_var_string_unchecked(fid) }
    }

    /// 获取`string`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_string_unchecked(&self, fid: impl VarId) -> Option<PBString> {
        self.get_shared_var_str_unchecked(fid).map(|v| v.to_ucstring())
    }

    /// 获取`boolean`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_bool(&self, fid: impl VarId) -> Option<bool> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Boolean);
        unsafe { self.get_shared_var_bool_unchecked(fid) }
    }

    /// 获取`boolean`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_bool_unchecked(&self, fid: impl VarId) -> Option<bool> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetBoolSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v.into())
        }
    }

    /// 获取`blob`类型共享(静态)变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_blob(&self, fid: impl VarId) -> Option<&'obj [u8]> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Blob);
        self.get_shared_var_blob_unchecked(fid)
    }

    /// 获取`blob`类型共享(静态)变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_blob_unchecked(&self, fid: impl VarId) -> Option<&'obj [u8]> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetBlobSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(self.session.get_blob_unchecked(v))
        }
    }

    /// 获取`date`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_shared_var_date(&self, fid: impl VarId) -> Option<NaiveDate> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Date);
        unsafe { self.get_shared_var_date_unchecked(fid) }
    }

    /// 获取`date`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_shared_var_date_unchecked(&self, fid: impl VarId) -> Option<NaiveDate> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDateSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(self.session.get_date_unchecked(v))
        }
    }

    /// 获取`time`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_shared_var_time(&self, fid: impl VarId) -> Option<NaiveTime> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Time);
        unsafe { self.get_shared_var_time_unchecked(fid) }
    }

    /// 获取`time`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_shared_var_time_unchecked(&self, fid: impl VarId) -> Option<NaiveTime> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetTimeSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(self.session.get_time_unchecked(v))
        }
    }

    /// 获取`datetime`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_shared_var_datetime(&self, fid: impl VarId) -> Option<NaiveDateTime> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::DateTime);
        unsafe { self.get_shared_var_datetime_unchecked(fid) }
    }

    /// 获取`datetime`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_shared_var_datetime_unchecked(&self, fid: impl VarId) -> Option<NaiveDateTime> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDateTimeSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(self.session.get_datetime_unchecked(v))
        }
    }

    /// 获取`char`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_char(&self, fid: impl VarId) -> Option<PBChar> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Char);
        unsafe { self.get_shared_var_char_unchecked(fid) }
    }

    /// 获取`char`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_char_unchecked(&self, fid: impl VarId) -> Option<PBChar> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetCharSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`byte`类型共享(静态)变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_shared_var_byte(&self, fid: impl VarId) -> Option<pbbyte> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Byte);
        unsafe { self.get_shared_var_byte_unchecked(fid) }
    }

    /// 获取`byte`类型共享(静态)变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_shared_var_byte_unchecked(&self, fid: impl VarId) -> Option<pbbyte> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetByteSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取对象类型共享(静态)变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_object(&self, fid: impl VarId) -> Option<Object<'obj>> {
        let fid = fid.var_id(self);
        assert!(self.is_var_object(fid));
        self.get_shared_var_object_unchecked(fid)
    }

    /// 获取对象类型共享(静态)变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_object_unchecked(&self, fid: impl VarId) -> Option<Object<'obj>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetObjectSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(Object::from_ptr(v, self.session.clone()))
        }
    }

    /// 获取数组类型共享(静态)变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_array(&self, fid: impl VarId) -> Option<Array<'obj>> {
        let fid = fid.var_id(self);
        assert!(self.is_var_array(fid));
        self.get_shared_var_array_unchecked(fid)
    }

    /// 获取数组类型共享(静态)变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_array_unchecked(&self, fid: impl VarId) -> Option<Array<'obj>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetArraySharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(Array::from_ptr(v, self.is_var_object(fid), self.session.clone()))
        }
    }

    /// 获取`any`类型共享(静态)变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_any(&self, fid: impl VarId) -> Option<Value<'obj>> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Any);
        self.get_shared_var_any_unchecked(fid)
    }

    /// 获取`any`类型共享(静态)变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_shared_var_any_unchecked(&self, fid: impl VarId) -> Option<Value<'obj>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetPBAnySharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            &mut is_null
        );
        if is_null == true {
            None
        } else {
            Some(Value::from_ptr(v, self.session.clone()))
        }
    }

    /// 设置共享(静态)变量的值为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn set_shared_var_to_null(&self, fid: impl VarId) {
        unsafe {
            ffi::pbsession_SetSharedVarToNull(self.session.as_ptr(), self.get_group(), fid.var_id(self))
        }
    }

    /// 设置`int`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_int(&mut self, fid: impl VarId, value: pbint) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Int);
        unsafe { ffi::pbsession_SetIntSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into() }
    }

    /// 设置`int`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_int_unchecked(&mut self, fid: impl VarId, value: pbint) -> Result<()> {
        ffi::pbsession_SetIntSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置`uint`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_uint(&mut self, fid: impl VarId, value: pbuint) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Uint);
        unsafe { ffi::pbsession_SetUintSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into() }
    }

    /// 设置`uint`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_uint_unchecked(&mut self, fid: impl VarId, value: pbuint) -> Result<()> {
        ffi::pbsession_SetUintSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置`long`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_long(&mut self, fid: impl VarId, value: pblong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Long);
        unsafe { ffi::pbsession_SetLongSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into() }
    }

    /// 设置`long`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_long_unchecked(&mut self, fid: impl VarId, value: pblong) -> Result<()> {
        ffi::pbsession_SetLongSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置`ulong`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_ulong(&mut self, fid: impl VarId, value: pbulong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Ulong);
        unsafe {
            ffi::pbsession_SetUlongSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into()
        }
    }

    /// 设置`ulong`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_ulong_unchecked(&mut self, fid: impl VarId, value: pbulong) -> Result<()> {
        ffi::pbsession_SetUlongSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置`longlong`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_longlong(&mut self, fid: impl VarId, value: pblonglong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::LongLong);
        unsafe {
            ffi::pbsession_SetLongLongSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into()
        }
    }

    /// 设置`longlong`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_longlong_unchecked(
        &mut self,
        fid: impl VarId,
        value: pblonglong
    ) -> Result<()> {
        ffi::pbsession_SetLongLongSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置`real`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_real(&mut self, fid: impl VarId, value: pbreal) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Real);
        unsafe { ffi::pbsession_SetRealSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into() }
    }

    /// 设置`real`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_real_unchecked(&mut self, fid: impl VarId, value: pbreal) -> Result<()> {
        ffi::pbsession_SetRealSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置`double`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_double(&mut self, fid: impl VarId, value: pbdouble) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Double);
        unsafe {
            ffi::pbsession_SetDoubleSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into()
        }
    }

    /// 设置`double`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_double_unchecked(&mut self, fid: impl VarId, value: pbdouble) -> Result<()> {
        ffi::pbsession_SetDoubleSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置`decimal`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn set_shared_var_dec(&mut self, fid: impl VarId, value: Decimal) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Decimal);
        unsafe {
            ffi::pbsession_SetDecSharedVar(
                self.session.as_ptr(),
                self.get_group(),
                fid,
                self.session.new_pbdec(value)
            )
            .into()
        }
    }

    /// 设置`decimal`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn set_shared_var_dec_unchecked(&mut self, fid: impl VarId, value: Decimal) -> Result<()> {
        ffi::pbsession_SetDecSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            self.session.new_pbdec(value)
        )
        .into()
    }

    /// 设置`string`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_str(&mut self, fid: impl VarId, value: impl AsPBStr) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::String);
        unsafe {
            ffi::pbsession_SetStringSharedVar(
                self.session.as_ptr(),
                self.get_group(),
                fid,
                value.as_pbstr().as_ptr()
            )
            .into()
        }
    }

    /// 设置`string`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_str_unchecked(
        &mut self,
        fid: impl VarId,
        value: impl AsPBStr
    ) -> Result<()> {
        ffi::pbsession_SetStringSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            value.as_pbstr().as_ptr()
        )
        .into()
    }

    /// 设置`boolean`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_bool(&mut self, fid: impl VarId, value: bool) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Boolean);
        unsafe {
            ffi::pbsession_SetBoolSharedVar(self.session.as_ptr(), self.get_group(), fid, value.into()).into()
        }
    }

    /// 设置`boolean`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_bool_unchecked(&mut self, fid: impl VarId, value: bool) -> Result<()> {
        ffi::pbsession_SetBoolSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            value.into()
        )
        .into()
    }

    /// 设置`blob`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_blob(&mut self, fid: impl VarId, value: impl AsRef<[u8]>) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Blob);
        unsafe {
            ffi::pbsession_SetBlobSharedVar(
                self.session.as_ptr(),
                self.get_group(),
                fid,
                self.session.new_pbblob(value)
            )
            .into()
        }
    }

    /// 设置`blob`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_blob_unchecked(
        &mut self,
        fid: impl VarId,
        value: impl AsRef<[u8]>
    ) -> Result<()> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(PBXRESULT::E_OUTOF_MEMORY);
        }
        ffi::pbsession_SetBlobSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            self.session.new_pbblob(value)
        )
        .into()
    }

    /// 设置`date`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_shared_var_date(&mut self, fid: impl VarId, value: NaiveDate) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Date);
        unsafe {
            ffi::pbsession_SetDateSharedVar(
                self.session.as_ptr(),
                self.get_group(),
                fid,
                self.session.new_pbdate(value)
            )
            .into()
        }
    }

    /// 设置`date`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_shared_var_date_unchecked(&mut self, fid: impl VarId, value: NaiveDate) -> Result<()> {
        ffi::pbsession_SetDateSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            self.session.new_pbdate(value)
        )
        .into()
    }

    /// 设置`time`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_shared_var_time(&mut self, fid: impl VarId, value: NaiveTime) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Time);
        unsafe {
            ffi::pbsession_SetTimeSharedVar(
                self.session.as_ptr(),
                self.get_group(),
                fid,
                self.session.new_pbtime(value)
            )
            .into()
        }
    }

    /// 设置`time`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_shared_var_time_unchecked(&mut self, fid: impl VarId, value: NaiveTime) -> Result<()> {
        ffi::pbsession_SetTimeSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            self.session.new_pbtime(value)
        )
        .into()
    }

    /// 设置`datetime`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_shared_var_datetime(&mut self, fid: impl VarId, value: NaiveDateTime) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::DateTime);
        unsafe {
            ffi::pbsession_SetDateTimeSharedVar(
                self.session.as_ptr(),
                self.get_group(),
                fid,
                self.session.new_pbdatetime(value)
            )
            .into()
        }
    }

    /// 设置`datetime`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_shared_var_datetime_unchecked(
        &mut self,
        fid: impl VarId,
        value: NaiveDateTime
    ) -> Result<()> {
        ffi::pbsession_SetDateTimeSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            self.session.new_pbdatetime(value)
        )
        .into()
    }

    /// 设置`char`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_char(&mut self, fid: impl VarId, value: PBChar) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Char);
        unsafe { ffi::pbsession_SetCharSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into() }
    }

    /// 设置`char`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_char_unchecked(&mut self, fid: impl VarId, value: PBChar) -> Result<()> {
        ffi::pbsession_SetCharSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置`byte`类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_byte(&mut self, fid: impl VarId, value: pbbyte) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_shared_var_type(fid) == ValueType::Byte);
        unsafe { ffi::pbsession_SetByteSharedVar(self.session.as_ptr(), self.get_group(), fid, value).into() }
    }

    /// 设置`byte`类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_byte_unchecked(&mut self, fid: impl VarId, value: pbbyte) -> Result<()> {
        ffi::pbsession_SetByteSharedVar(self.session.as_ptr(), self.get_group(), fid.var_id(self), value)
            .into()
    }

    /// 设置对象类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_object(&mut self, fid: impl VarId, value: &Object) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.is_var_object(fid));
        unsafe {
            ffi::pbsession_SetObjectSharedVar(self.session.as_ptr(), self.get_group(), fid, value.as_ptr())
                .into()
        }
    }

    /// 设置对象类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_object_unchecked(&mut self, fid: impl VarId, value: &Object) -> Result<()> {
        ffi::pbsession_SetObjectSharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            value.as_ptr()
        )
        .into()
    }

    /// 设置数组类型共享(静态)变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_shared_var_array(&mut self, fid: impl VarId, value: &Array) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.is_var_array(fid));
        unsafe {
            ffi::pbsession_SetArraySharedVar(self.session.as_ptr(), self.get_group(), fid, value.as_ptr())
                .into()
        }
    }

    /// 设置数组类型共享(静态)变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_shared_var_array_unchecked(&mut self, fid: impl VarId, value: &Array) -> Result<()> {
        ffi::pbsession_SetArraySharedVar(
            self.session.as_ptr(),
            self.get_group(),
            fid.var_id(self),
            value.as_ptr()
        )
        .into()
    }
}

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

/// 共享的对象
/// 增加了全局引用计数
#[derive(Clone)]
pub struct SharedObject {
    inner: Rc<RefCell<SharedObjectInner>>
}

impl SharedObject {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: Session) -> SharedObject {
        SharedObject {
            inner: Rc::new(RefCell::new(SharedObjectInner::from_ptr(ptr, session)))
        }
    }
}

impl Deref for SharedObject {
    type Target = RefCell<SharedObjectInner>;
    fn deref(&self) -> &Self::Target { &self.inner }
}

impl From<Object<'_>> for SharedObject {
    fn from(obj: Object) -> Self { unsafe { Self::from_ptr(obj.ptr, obj.session) } }
}

pub struct SharedObjectInner {
    obj: Object<'static>
}

impl SharedObjectInner {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: Session) -> SharedObjectInner {
        ffi::pbsession_AddGlobalRef(session.as_ptr(), ptr);
        SharedObjectInner {
            obj: Object::from_ptr(ptr, session)
        }
    }
}

impl Drop for SharedObjectInner {
    fn drop(&mut self) { unsafe { ffi::pbsession_RemoveGlobalRef(self.obj.session.as_ptr(), self.obj.ptr) } }
}

impl Deref for SharedObjectInner {
    type Target = Object<'static>;
    fn deref(&self) -> &Self::Target { &self.obj }
}

impl DerefMut for SharedObjectInner {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.obj }
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
}

impl Deref for ContextObject {
    type Target = Object<'static>;
    fn deref(&self) -> &Self::Target { &self.obj }
}

impl DerefMut for ContextObject {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.obj }
}

/// 用户自定义对象抽象
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub trait UserObject: Sized + 'static {
    /// 类名(小写)
    const CLASS_NAME: &'static PBStr;

    /// 创建对象
    fn new(session: Session, ctx: ContextObject) -> Result<Self>;

    /// 接口调用
    ///
    /// # Returns
    ///
    /// - 调用的方法ID被处理后返回`Ok(None)`
    /// - 调用的方法ID未处理则返回`Ok(Some(mid))`,`mid`为最后一个方法ID的偏移,此设计用于实现继承
    fn invoke(&mut self, mid: MethodId, ci: &CallInfoRef) -> Result<Option<MethodId>>;
}

/// 不可视对象
#[cfg(feature = "nonvisualobject")]
pub trait NonVisualObject: UserObject {
    /// 注册
    fn register() { export::register_nonvisualobject::<Self>() }
}

/// 可视对象
#[cfg(feature = "visualobject")]
pub trait VisualObject: UserObject {
    /// 窗口类名
    const WINDOW_CLASS_NAME: &'static PBStr;

    /// 创建窗口
    fn create_control(
        &mut self,
        dwExStyle: u32,
        window_name: &PBStr,
        dwStyle: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: HWND,
        instance: HINSTANCE
    ) -> HWND;

    /// 窗口消息与PB事件ID映射
    #[allow(unused_variables)]
    fn get_event_id(&self, hwnd: HWND, msg: u16, wparam: u32, lparam: u32) -> Option<i32> { None }

    /// 注册
    fn register() { export::register_visualobject::<Self>() }
}
