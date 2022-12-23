//! 全局变量访问接口实现
//!
use super::*;

/*
    Global variable
*/

/// 全局变量ID抽象
pub trait AsGlobalVarId {
    fn as_var_id(&self, session: &Session) -> FieldId;
    fn try_as_var_id(&self, session: &Session) -> Result<FieldId>;
}

impl<T: AsPBStr> AsGlobalVarId for T {
    #[inline]
    fn as_var_id(&self, session: &Session) -> FieldId {
        AsGlobalVarId::try_as_var_id(self, session)
            .map_err(|_| format!("invalid global var {}", self.as_pbstr().to_string_lossy()))
            .unwrap()
    }
    fn try_as_var_id(&self, session: &Session) -> Result<FieldId> {
        let pbstr = self.as_pbstr();
        session.get_global_var_id(pbstr.as_ref()).ok_or(PBXRESULT::E_INVALID_FIELD_ID)
    }
}
impl AsGlobalVarId for FieldId {
    #[inline]
    fn as_var_id(&self, _session: &Session) -> FieldId { *self }
    #[inline]
    fn try_as_var_id(&self, _session: &Session) -> Result<FieldId> { Ok(*self) }
}
impl AsGlobalVarId for Option<FieldId> {
    #[inline]
    fn as_var_id(&self, _session: &Session) -> FieldId { self.unwrap() }
    #[inline]
    fn try_as_var_id(&self, _session: &Session) -> Result<FieldId> {
        self.ok_or(PBXRESULT::E_INVALID_FIELD_ID)
    }
}

impl Session {
    /// 获取全局变量ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = session.get_global_var_id("gs_var").unwrap();
    /// session.set_global_var_str(fid,"rust");
    /// ```
    pub fn get_global_var_id(&self, name: impl AsPBStr) -> Option<FieldId> {
        unsafe {
            let fid = ffi::pbsession_GetGlobalVarID(self.ptr, name.as_pbstr().as_ptr());
            if fid.is_undefined() {
                None
            } else {
                Some(fid)
            }
        }
    }

    /// 判断是否存在指定全局变量
    ///
    /// # Examples
    ///
    /// ```
    /// if session.has_global_var("gs_var") {
    ///     session.set_global_var_str("gs_var","rust");
    /// }
    /// ```
    pub fn has_global_var(&self, name: impl AsPBStr) -> bool { self.get_global_var_id(name).is_some() }

    /// 获取指定全局变量类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// if session.get_global_var_type("gs_var") == ValueType::String {
    ///     session.set_global_var_str("gs_var","rust");
    /// }
    /// ```
    pub fn get_global_var_type(&self, fid: impl AsGlobalVarId) -> ValueType {
        unsafe { ffi::pbsession_GetGlobalVarType(self.ptr, fid.as_var_id(self)) }
    }

    /// 判断指定全局变量是否为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_global_var_null(&self, fid: impl AsGlobalVarId) -> bool {
        unsafe { ffi::pbsession_IsGlobalVarNull(self.ptr, fid.as_var_id(self)).into() }
    }

    /// 判断指定全局变量是否为数组类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_global_var_array(&self, fid: impl AsGlobalVarId) -> bool {
        unsafe { ffi::pbsession_IsGlobalVarArray(self.ptr, fid.as_var_id(self)).into() }
    }

    /// 判断指定全局变量是否为对象类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_global_var_object(&self, fid: impl AsGlobalVarId) -> bool {
        unsafe { ffi::pbsession_IsGlobalVarObject(self.ptr, fid.as_var_id(self)).into() }
    }

    /// 设置全局变量的值为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn set_var_to_null(&self, fid: impl AsGlobalVarId) {
        unsafe {
            ffi::pbsession_SetGlobalVarToNull(self.ptr, fid.as_var_id(self));
        }
    }
}

/*
    Global variable getter/setter
*/

macro_rules! impl_global_var {
    /*
        简单类型
    */
    (
        @simple
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_global_var!(
            @simple_getter
            $type_name, $type, $type_check
        );
        impl_global_var!(
            @simple_setter
            $type_name, $type, $type_check
        );
    };
    (
        @simple_getter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_global_var!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型全局变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_global_var_ $type_name _unchecked>](&self, fid: impl AsGlobalVarId) -> Option<$type> {
                let fid = fid.as_var_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel GlobalVar>](self.ptr, fid, &mut is_null);
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
        impl_global_var!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型全局变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_global_var_ $type_name _unchecked>](&mut self, fid: impl AsGlobalVarId, value: $type) {
                let fid = fid.as_var_id(self);
                assert_eq!(ffi::[<pbsession_Set $type_name:camel GlobalVar>](self.ptr, fid, value.into()), PBXRESULT::OK);
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
        impl_global_var!(
            @complex_getter
            $type_name, $type, $type_check
        );
        impl_global_var!(
            @complex_setter
            $type_name, $type, $type_check
        );
    };
    (
        @complex_getter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_global_var!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型全局变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_global_var_ $type_name _unchecked>](&self, fid: impl AsGlobalVarId) -> Option<$type> {
                let fid = fid.as_var_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel GlobalVar>](self.ptr, fid, &mut is_null);
                if is_null == true {
                    None
                } else {
                    impl_global_var!(@complex_get_val self, fid, v, $type_name)
                }
            }
        }
    };
    (
        @complex_setter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_global_var!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型全局变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_global_var_ $type_name _unchecked>](&mut self, fid: impl AsGlobalVarId, value: $type) {
                let fid = fid.as_var_id(self);
                assert_eq!(ffi::[<pbsession_Set $type_name:camel GlobalVar>](self.ptr, fid, impl_global_var!(@complex_set_val self, fid, value, $type_name)), PBXRESULT::OK);
            }
        }
    };
    (@complex_get_val $self: expr, $fid: expr, $value: expr, str) => {
        $self.get_string_unchecked($value)
    };
    (@complex_get_val $self: expr, $fid: expr, $value: expr, string) => {
        $self.get_string_unchecked($value).map(PBStr::to_ucstring)
    };
    (@complex_get_val $self: expr, $fid: expr, $value: expr, array) => {
        Some(Array::from_ptr($value, $self.is_global_var_object($fid), $self.clone()))
    };
    (@complex_get_val $self: expr, $fid: expr, $value: expr, object) => {
        Some(Object::from_ptr($value, $self.clone()))
    };
    (@complex_get_val $self: expr, $fid: expr, $value: expr, any) => {
        Some(Value::from_ptr($value, $self.clone()))
    };
    (@complex_get_val $self: expr, $fid: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            Some($self.[<get_ $type_name _unchecked>]($value))
        }
    };
    (@complex_set_val $self: expr, $fid: expr, $value: expr, str) => {
        $value.as_pbstr().as_ptr()
    };
    (@complex_set_val $self: expr, $fid: expr, $value: expr, array) => {
        $value.as_ptr()
    };
    (@complex_set_val $self: expr, $fid: expr, $value: expr, object) => {
        $value.as_ptr()
    };
    (@complex_set_val $self: expr, $fid: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            $self.[<new_pb $type_name>]($value)
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
            #[doc = "获取`" $type_name "`类型全局变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<get_global_var_ $type_name>](&self, fid: impl AsGlobalVarId) -> Option<$type> {
                self.[<try_get_global_var_ $type_name>](fid).unwrap()
            }

            #[doc = "获取`" $type_name "`类型全局变量值"]
            pub fn [<try_get_global_var_ $type_name>](&self, fid: impl AsGlobalVarId) -> Result<Option<$type>> {
                let fid = fid.try_as_var_id(self)?;
                if impl_global_var!(@check_type_get self, fid, $type_check, $type_name) {
                    unsafe {
                        Ok(self.[<get_global_var_ $type_name _unchecked>](fid))
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
            #[doc = "设置`" $type_name "`类型全局变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<set_global_var_ $type_name>](&mut self, fid: impl AsGlobalVarId, value: $type) {
                self.[<try_set_global_var_ $type_name>](fid, value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型全局变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<try_set_global_var_ $type_name>](&mut self, fid: impl AsGlobalVarId, value: $type) -> Result<()> {
                let fid = fid.try_as_var_id(self)?;
                if impl_global_var!(@check_type_set self, fid, value, $type_check, $type_name) {
                    unsafe {
                        self.[<set_global_var_ $type_name _unchecked>](fid, value);
                    }
                    Ok(())
                } else {
                    Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }
        }
    };
    (@check_type_get $self: expr, $fid: expr, $type_check: pat, array) => {
        $self.is_global_var_array($fid)
    };
    (@check_type_get $self: expr, $fid: expr, $type_check: pat, object) => {
        $self.is_global_var_object($fid)
    };
    (@check_type_get $self: expr, $fid: expr, $type_check: pat, $type_name: ty) => {
        matches!($self.get_global_var_type($fid), $type_check)
    };
    (@check_type_set $self: expr, $fid: expr, $value: expr, $type_check: pat, array) => {
        $self.is_global_var_array($fid) || $self.get_global_var_type($fid) == ValueType::Any
    };
    (@check_type_set $self: expr, $fid: expr, $value: expr, $type_check: pat, object) => {
        $self.is_global_var_object($fid) || $self.get_global_var_type($fid) == ValueType::Any
    };
    (@check_type_set $self: expr, $fid: expr, $value: expr, $type_check: pat, $type_name: ty) => {
        matches!($self.get_global_var_type($fid), $type_check | ValueType::NoType)
    };
}

impl Session {
    impl_global_var!(
        @simple
        int, pbint, ValueType::Int
    );
    impl_global_var!(
        @simple
        uint, pbuint, ValueType::Uint
    );
    impl_global_var!(
        @simple
        long, pblong, ValueType::Long
    );
    impl_global_var!(
        @simple
        ulong, pbulong, ValueType::Ulong
    );
    impl_global_var!(
        @simple
        longlong, pblonglong, ValueType::LongLong
    );
    impl_global_var!(
        @simple
        real, pbreal, ValueType::Real
    );
    impl_global_var!(
        @simple
        double, pbdouble, ValueType::Double
    );
    impl_global_var!(
        @simple
        byte, pbbyte, ValueType::Byte
    );
    impl_global_var!(
        @simple
        bool, bool, ValueType::Boolean
    );
    impl_global_var!(
        @simple
        char, PBChar, ValueType::Char
    );

    impl_global_var!(
        @complex
        dec, Decimal, ValueType::Decimal
    );
    impl_global_var!(
        @complex
        date, NaiveDate, ValueType::Date
    );
    impl_global_var!(
        @complex
        time, NaiveTime, ValueType::Time
    );
    impl_global_var!(
        @complex
        datetime, NaiveDateTime, ValueType::DateTime
    );
    impl_global_var!(
        @complex_getter
        blob, &[u8], ValueType::Blob
    );
    impl_global_var!(
        @complex_setter
        blob, &[u8], ValueType::Blob
    );
    impl_global_var!(
        @complex_getter
        str, &PBStr, ValueType::String
    );
    impl_global_var!(
        @complex_getter
        string, PBString, ValueType::String
    );
    impl_global_var!(
        @complex_setter
        str, impl AsPBStr, ValueType::String
    );
    impl_global_var!(
        @complex_getter
        array, Array, ValueType::NoType
    );
    impl_global_var!(
        @complex_setter
        array, &Array, ValueType::NoType
    );
    impl_global_var!(
        @complex_getter
        object, Object, ValueType::NoType
    );
    impl_global_var!(
        @complex_setter
        object, &Object, ValueType::NoType
    );
    impl_global_var!(
        @complex_getter
        any, Value, ValueType::Any
    );
}
