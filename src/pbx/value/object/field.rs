//! 对象实例变量与共享(静态)变量访问接口实现
//!
use super::*;

/*
    Instance variable
*/

/// 实例变量抽象
pub trait AsFieldId {
    fn as_field_id(&self, obj: &Object) -> FieldId;
    fn try_as_field_id(&self, obj: &Object) -> Result<FieldId>;
}

impl<T: AsPBStr> AsFieldId for T {
    #[inline]
    fn as_field_id(&self, obj: &Object) -> FieldId {
        AsFieldId::try_as_field_id(self, obj)
            .map_err(|_| format!("invalid var {}", self.as_pbstr().to_string_lossy()))
            .unwrap()
    }
    #[inline]
    fn try_as_field_id(&self, obj: &Object) -> Result<FieldId> {
        let pbstr = self.as_pbstr();
        obj.get_field_id(pbstr.as_ref()).ok_or(PBXRESULT::E_INVALID_FIELD_ID)
    }
}
impl AsFieldId for FieldId {
    #[inline]
    fn as_field_id(&self, _obj: &Object) -> FieldId { *self }
    #[inline]
    fn try_as_field_id(&self, _obj: &Object) -> Result<FieldId> { Ok(*self) }
}
impl AsFieldId for Option<FieldId> {
    #[inline]
    fn as_field_id(&self, _obj: &Object) -> FieldId { self.unwrap() }
    #[inline]
    fn try_as_field_id(&self, _obj: &Object) -> Result<FieldId> { self.ok_or(PBXRESULT::E_INVALID_FIELD_ID) }
}

impl<'obj> Object<'obj> {
    /// 获取实例变量数量
    ///
    /// # Examples
    ///
    /// ```
    /// let cnt = obj.get_field_count("is_var");
    /// //遍历所有变量并输出变量名
    /// for id in 0..cnt {
    ///     let fid = unsafe { FieldId::new(id) };
    ///     println!("field id: {}, name: {}", id, obj.get_var_name(fid));
    /// }
    /// ```
    pub fn get_field_count(&self) -> pbulong {
        unsafe { ffi::pbsession_GetNumOfFields(self.session.as_raw(), self.cls) }
    }

    /// 查找实例变量ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = obj.get_field_id("is_var").unwrap();
    /// obj.set_field_str(fid,"rust");
    /// ```
    pub fn get_field_id(&self, name: impl AsPBStr) -> Option<FieldId> {
        unsafe {
            let fid = ffi::pbsession_GetFieldID(self.session.as_raw(), self.cls, name.as_pbstr().as_ptr());
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
    /// let fid = obj.get_field_id("is_var").unwrap();
    /// assert_eq!(pbstr!("is_var"),obj.get_var_name(fid));
    /// ```
    pub fn get_field_name(&self, fid: impl AsFieldId) -> &PBStr {
        unsafe {
            PBStr::from_ptr_str(ffi::pbsession_GetFieldName(
                self.session.as_raw(),
                self.cls,
                fid.as_field_id(self)
            ))
        }
    }

    /// 判断是否存在指定实例变量
    ///
    /// # Examples
    ///
    /// ```
    /// if object.has_field("is_var") {
    ///     object.set_field_str("is_var","rust");
    /// }
    /// ```
    pub fn has_var(self, name: impl AsPBStr) -> bool { self.get_field_id(name).is_some() }

    /// 获取变量类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// if object.get_field_type("is_var") == ValueType::String {
    ///     object.set_field_str("is_var","rust");
    /// }
    /// ```
    pub fn get_field_type(&self, fid: impl AsFieldId) -> ValueType {
        unsafe { ffi::pbsession_GetFieldType(self.session.as_raw(), self.cls, fid.as_field_id(self)) }
    }

    /// 判断实例变量是否为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_field_null(&self, fid: impl AsFieldId) -> bool {
        unsafe { ffi::pbsession_IsFieldNull(self.session.as_raw(), self.ptr, fid.as_field_id(self)).into() }
    }

    /// 判断实例变量类型是否为数组
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_field_array(&self, fid: impl AsFieldId) -> bool {
        unsafe { ffi::pbsession_IsFieldArray(self.session.as_raw(), self.cls, fid.as_field_id(self)).into() }
    }

    /// 判断实例变量类型是否为对象
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_field_object(&self, fid: impl AsFieldId) -> bool {
        unsafe { ffi::pbsession_IsFieldObject(self.session.as_raw(), self.cls, fid.as_field_id(self)).into() }
    }

    /// 设置实例变量的值为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn set_field_to_null(&self, fid: impl AsFieldId) {
        unsafe { ffi::pbsession_SetFieldToNull(self.session.as_raw(), self.ptr, fid.as_field_id(self)) }
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
    pub fn update_field(&self, fid: impl AsFieldId) -> Result<()> {
        unsafe { ffi::pbsession_UpdateField(self.session.as_raw(), self.ptr, fid.as_field_id(self)).into() }
    }
}

/*
    Shared variable
*/

/// 共享(静态)变量ID抽象
pub trait AsSharedVarId {
    fn as_var_id(&self, obj: &Object) -> FieldId;
    fn try_as_var_id(&self, obj: &Object) -> Result<FieldId>;
}

impl<T: AsPBStr> AsSharedVarId for T {
    #[inline]
    fn as_var_id(&self, obj: &Object) -> FieldId {
        AsSharedVarId::try_as_var_id(self, obj)
            .map_err(|_| format!("invalid shared var {}", self.as_pbstr().to_string_lossy()))
            .unwrap()
    }
    #[inline]
    fn try_as_var_id(&self, obj: &Object) -> Result<FieldId> {
        let pbstr = self.as_pbstr();
        obj.get_shared_var_id(pbstr.as_ref()).ok_or(PBXRESULT::E_INVALID_FIELD_ID)
    }
}
impl AsSharedVarId for FieldId {
    #[inline]
    fn as_var_id(&self, _obj: &Object) -> FieldId { *self }
    #[inline]
    fn try_as_var_id(&self, _obj: &Object) -> Result<FieldId> { Ok(*self) }
}
impl AsSharedVarId for Option<FieldId> {
    #[inline]
    fn as_var_id(&self, _obj: &Object) -> FieldId { self.unwrap() }
    #[inline]
    fn try_as_var_id(&self, _obj: &Object) -> Result<FieldId> { self.ok_or(PBXRESULT::E_INVALID_FIELD_ID) }
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
                self.session.as_raw(),
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
    pub fn get_shared_var_type(&self, fid: impl AsSharedVarId) -> ValueType {
        unsafe {
            ffi::pbsession_GetSharedVarType(self.session.as_raw(), self.get_group(), fid.as_var_id(self))
        }
    }

    /// 判断指定共享(静态)变量是否为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_null(&self, fid: impl AsSharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarNull(self.session.as_raw(), self.get_group(), fid.as_var_id(self))
                .into()
        }
    }

    /// 判断指定共享(静态)变量是否为数组类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_array(&self, fid: impl AsSharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarArray(self.session.as_raw(), self.get_group(), fid.as_var_id(self))
                .into()
        }
    }

    /// 判断指定共享(静态)变量是否为对象类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_object(&self, fid: impl AsSharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarObject(self.session.as_raw(), self.get_group(), fid.as_var_id(self))
                .into()
        }
    }

    /// 设置共享(静态)变量的值为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn set_shared_var_to_null(&self, fid: impl AsFieldId) {
        unsafe {
            ffi::pbsession_SetSharedVarToNull(self.session.as_raw(), self.get_group(), fid.as_field_id(self))
        }
    }
}

/*
   Field getter/setter
*/

macro_rules! impl_field {
    /*
        简单类型
    */
    (
        @simple
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_field!(
            @simple_getter
            $type_name, $type, $type_check
        );
        impl_field!(
            @simple_setter
            $type_name, $type, $type_check
        );
    };
    (
        @simple_getter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_field!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型实例变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_field_ $type_name _unchecked>](&self, fid: impl AsFieldId) -> Option<$type> {
                let fid = fid.as_field_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel Field>](self.session.as_raw(), self.ptr, fid, &mut is_null);
                if is_null == true {
                    None
                } else {
                    Some(v.into())
                }
            }

            #[doc = "获取`" $type_name "`类型共享(静态)变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_shared_var_ $type_name _unchecked>](&self, fid: impl AsSharedVarId) -> Option<$type> {
                let fid = fid.as_var_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel SharedVar>](self.session.as_raw(), self.get_group(), fid, &mut is_null);
                if is_null == true {
                    None
                } else {
                    Some(v.into())
                }
            }
        }
    };
    (
        @simple_setter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_field!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型实例变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_field_ $type_name _unchecked>](&mut self, fid: impl AsFieldId, value: $type) {
                let fid = fid.as_field_id(self);
                debug_assert_eq!(ffi::[<pbsession_Set $type_name:camel Field>](self.session.as_raw(), self.ptr, fid, value.into()), PBXRESULT::OK);
            }

            #[doc = "设置`" $type_name "`类型共享(静态)变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_shared_var_ $type_name _unchecked>](&mut self, fid: impl AsSharedVarId, value: $type) {
                let fid = fid.as_var_id(self);
                debug_assert_eq!(ffi::[<pbsession_Set $type_name:camel SharedVar>](self.session.as_raw(), self.get_group(), fid, value.into()), PBXRESULT::OK);
            }
        }
    };

    /*
        复杂类型
    */
    (
        @complex
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_field!(
            @complex_getter
            $type_name, $type, $type_check
        );
        impl_field!(
            @complex_setter
            $type_name, $type, $type_check
        );
    };
    (
        @complex_getter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_field!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型实例变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_field_ $type_name _unchecked>](&self, fid: impl AsFieldId) -> Option<$type> {
                let fid = fid.as_field_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel Field>](self.session.as_raw(), self.ptr, fid, &mut is_null);
                if is_null == true {
                    None
                } else {
                    impl_field!(@complex_get_val field, self, fid, v, $type_name)
                }
            }

            #[doc = "获取`" $type_name "`类型共享(静态)变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_shared_var_ $type_name _unchecked>](&self, fid: impl AsSharedVarId) -> Option<$type> {
                let fid = fid.as_var_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel SharedVar>](self.session.as_raw(), self.get_group(), fid, &mut is_null);
                if is_null == true {
                    None
                } else {
                    impl_field!(@complex_get_val shared_var, self, fid, v, $type_name)
                }
            }
        }
    };
    (
        @complex_setter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_field!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型实例变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_field_ $type_name _unchecked>](&mut self, fid: impl AsFieldId, value: $type) {
                let fid = fid.as_field_id(self);
                debug_assert_eq!(ffi::[<pbsession_Set $type_name:camel Field>](self.session.as_raw(), self.ptr, fid, impl_field!(@complex_set_val self, fid, value, $type_name)), PBXRESULT::OK);
            }

            #[doc = "设置`" $type_name "`类型共享(静态)变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_shared_var_ $type_name _unchecked>](&mut self, fid: impl AsSharedVarId, value: $type) {
                let fid = fid.as_var_id(self);
                debug_assert_eq!(ffi::[<pbsession_Set $type_name:camel SharedVar>](self.session.as_raw(), self.get_group(), fid, impl_field!(@complex_set_val self, fid, value, $type_name)), PBXRESULT::OK);
            }
        }
    };
    (@complex_get_val $field_kind: ty, $self: expr, $fid: expr, $value: expr, str) => {
        $self.session.get_string_unchecked($value)
    };
    (@complex_get_val $field_kind: ty, $self: expr, $fid: expr, $value: expr, string) => {
        $self.session.get_string_unchecked($value).map(PBStr::to_ucstring)
    };
    (@complex_get_val $field_kind: ty, $self: expr, $fid: expr, $value: expr, array) => {
        ::paste::paste! {
            Some(Array::from_raw($value, $self.[<is_ $field_kind _object>]($fid), $self.session.clone()))
        }
    };
    (@complex_get_val $field_kind: ty, $self: expr, $fid: expr, $value: expr, object) => {
        Some(Object::from_raw($value, $self.session.clone()))
    };
    (@complex_get_val $field_kind: ty, $self: expr, $fid: expr, $value: expr, any) => {
        Some(Value::from_raw($value, $self.session.clone()))
    };
    (@complex_get_val $field_kind: ty, $self: expr, $fid: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            Some($self.session.[<get_ $type_name _unchecked>]($value))
        }
    };
    (@complex_set_val $self: expr, $fid: expr, $value: expr, str) => {
        $value.as_pbstr().as_ptr()
    };
    (@complex_set_val $self: expr, $fid: expr, $value: expr, array) => {
        $value.as_raw()
    };
    (@complex_set_val $self: expr, $fid: expr, $value: expr, object) => {
        $value.as_raw()
    };
    (@complex_set_val $self: expr, $fid: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            $self.session.[<new_pb $type_name>]($value)
        }
    };

    /*
        通用类型检查接口
    */
    (
        @checked_getter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型实例变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<get_field_ $type_name>](&self, fid: impl AsFieldId) -> Option<$type> {
                self.[<try_get_field_ $type_name>](fid).unwrap()
            }

            #[doc = "获取`" $type_name "`类型实例变量值"]
            pub fn [<try_get_field_ $type_name>](&self, fid: impl AsFieldId) -> Result<Option<$type>> {
                let fid = fid.try_as_field_id(self)?;
                if impl_field!(@check_type_get field, self, fid, $type_check, $type_name) {
                    unsafe {
                        Ok(self.[<get_field_ $type_name _unchecked>](fid))
                    }
                } else {
                    Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }

            #[doc = "获取`" $type_name "`类型共享(静态)变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<get_shared_var_ $type_name>](&self, fid: impl AsSharedVarId) -> Option<$type> {
                self.[<try_get_shared_var_ $type_name>](fid).unwrap()
            }

            #[doc = "获取`" $type_name "`类型共享(静态)变量值"]
            pub fn [<try_get_shared_var_ $type_name>](&self, fid: impl AsSharedVarId) -> Result<Option<$type>> {
                let fid = fid.try_as_var_id(self)?;
                if impl_field!(@check_type_get shared_var, self, fid, $type_check, $type_name) {
                    unsafe {
                        Ok(self.[<get_shared_var_ $type_name _unchecked>](fid))
                    }
                } else {
                    Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }
        }
    };
    (
        @checked_setter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型实例变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<set_field_ $type_name>](&mut self, fid: impl AsFieldId, value: $type) {
                self.[<try_set_field_ $type_name>](fid, value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型实例变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<try_set_field_ $type_name>](&mut self, fid: impl AsFieldId, value: $type) -> Result<()> {
                let fid = fid.try_as_field_id(self)?;
                if impl_field!(@check_type_set field, self, fid, value, $type_check, $type_name) {
                    unsafe {
                        self.[<set_field_ $type_name _unchecked>](fid, value);
                    }
                    Ok(())
                } else {
                    Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }

            #[doc = "设置`" $type_name "`类型共享(静态)变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<set_shared_var_ $type_name>](&mut self, fid: impl AsSharedVarId, value: $type) {
                self.[<try_set_shared_var_ $type_name>](fid, value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型共享(静态)变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<try_set_shared_var_ $type_name>](&mut self, fid: impl AsSharedVarId, value: $type) -> Result<()> {
                let fid = fid.try_as_var_id(self)?;
                if impl_field!(@check_type_set shared_var, self, fid, value, $type_check, $type_name) {
                    unsafe {
                        self.[<set_shared_var_ $type_name _unchecked>](fid, value);
                    }
                    Ok(())
                } else {
                    Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }
        }
    };
    (@check_type_get $field_kind: ty, $self: expr, $fid: expr, $type_check: pat, array) => {
        ::paste::paste! {
            $self.[<is_ $field_kind _array>]($fid)
        }
    };
    (@check_type_get $field_kind: ty, $self: expr, $fid: expr, $type_check: pat, object) => {
        ::paste::paste! {
            $self.[<is_ $field_kind _object>]($fid)
        }
    };
    (@check_type_get $field_kind: ty, $self: expr, $fid: expr, $type_check: pat, $type_name: ty) => {
        ::paste::paste! {
            matches!($self.[<get_ $field_kind _type>]($fid), $type_check)
        }
    };
    (@check_type_set $field_kind: ty, $self: expr, $fid: expr, $value: expr, $type_check: pat, array) => {
        ::paste::paste! {
            $self.[<is_ $field_kind _array>]($fid) || $self.[<get_ $field_kind _type>]($fid) == ValueType::Any
        }
    };
    (@check_type_set $field_kind: ty, $self: expr, $fid: expr, $value: expr, $type_check: pat, object) => {
        ::paste::paste! {
            $self.[<is_ $field_kind _object>]($fid) || $self.[<get_ $field_kind _type>]($fid) == ValueType::Any
        }
    };
    (@check_type_set $field_kind: ty, $self: expr, $fid: expr, $value: expr, $type_check: pat, $type_name: ty) => {
        ::paste::paste! {
            matches!($self.[<get_ $field_kind _type>]($fid), $type_check | ValueType::NoType)
        }
    };
}

impl<'obj> Object<'obj> {
    impl_field!(
        @simple
        int, pbint, ValueType::Int
    );
    impl_field!(
        @simple
        uint, pbuint, ValueType::Uint
    );
    impl_field!(
        @simple
        long, pblong, ValueType::Long
    );
    impl_field!(
        @simple
        ulong, pbulong, ValueType::Ulong
    );
    impl_field!(
        @simple
        longlong, pblonglong, ValueType::LongLong
    );
    impl_field!(
        @simple
        real, pbreal, ValueType::Real
    );
    impl_field!(
        @simple
        double, pbdouble, ValueType::Double
    );
    impl_field!(
        @simple
        byte, pbbyte, ValueType::Byte
    );
    impl_field!(
        @simple
        bool, bool, ValueType::Boolean
    );
    impl_field!(
        @simple
        char, PBChar, ValueType::Char
    );

    impl_field!(
        @complex
        dec, Decimal, ValueType::Decimal
    );
    impl_field!(
        @complex
        date, NaiveDate, ValueType::Date
    );
    impl_field!(
        @complex
        time, NaiveTime, ValueType::Time
    );
    impl_field!(
        @complex
        datetime, NaiveDateTime, ValueType::DateTime
    );
    impl_field!(
        @complex_getter
        blob, &'obj [u8], ValueType::Blob
    );
    impl_field!(
        @complex_setter
        blob, &[u8], ValueType::Blob
    );
    impl_field!(
        @complex_getter
        str, &'obj PBStr, ValueType::String
    );
    impl_field!(
        @complex_getter
        string, PBString, ValueType::String
    );
    impl_field!(
        @complex_setter
        str, impl AsPBStr, ValueType::String
    );
    impl_field!(
        @complex_getter
        array, Array<'obj>, ValueType::NoType
    );
    impl_field!(
        @complex_setter
        array, &Array, ValueType::NoType
    );
    impl_field!(
        @complex_getter
        object, Object<'obj>, ValueType::NoType
    );
    impl_field!(
        @complex_setter
        object, &Object, ValueType::NoType
    );
    impl_field!(
        @complex_getter
        any, Value<'obj>, ValueType::Any
    );
}
