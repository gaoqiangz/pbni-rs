use super::*;

/*
    Getter/Setter
*/

macro_rules! impl_object {
    /*
        简单类型
    */
    (
        @simple
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_object!(
            @simple_getter
            $type_name, $type, $type_check
        );
        impl_object!(
            @simple_setter
            $type_name, $type, $type_check
        );
    };
    (
        @simple_getter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_object!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型实例变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_var_ $type_name _unchecked>](&self, fid: impl VarId) -> Option<$type> {
                let fid = fid.var_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel Field>](self.session.as_ptr(), self.ptr, fid, &mut is_null);
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
            pub unsafe fn [<get_shared_var_ $type_name _unchecked>](&self, fid: impl SharedVarId) -> Option<$type> {
                let fid = fid.var_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel SharedVar>](self.session.as_ptr(), self.get_group(), fid, &mut is_null);
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
        impl_object!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型实例变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_var_ $type_name _unchecked>](&mut self, fid: impl VarId, value: $type) {
                let fid = fid.var_id(self);
                assert_eq!(ffi::[<pbsession_Set $type_name:camel Field>](self.session.as_ptr(), self.ptr, fid, value.into()), PBXRESULT::OK);
            }

            #[doc = "设置`" $type_name "`类型共享(静态)变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_shared_var_ $type_name _unchecked>](&mut self, fid: impl SharedVarId, value: $type) {
                let fid = fid.var_id(self);
                assert_eq!(ffi::[<pbsession_Set $type_name:camel SharedVar>](self.session.as_ptr(), self.get_group(), fid, value.into()), PBXRESULT::OK);
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
        impl_object!(
            @complex_getter
            $type_name, $type, $type_check
        );
        impl_object!(
            @complex_setter
            $type_name, $type, $type_check
        );
    };
    (
        @complex_getter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_object!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型实例变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_var_ $type_name _unchecked>](&self, fid: impl VarId) -> Option<$type> {
                let fid = fid.var_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel Field>](self.session.as_ptr(), self.ptr, fid, &mut is_null);
                if is_null == true {
                    None
                } else {
                    impl_object!(@complex_get_val var, self, fid, v, $type_name)
                }
            }

            #[doc = "获取`" $type_name "`类型共享(静态)变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_shared_var_ $type_name _unchecked>](&self, fid: impl SharedVarId) -> Option<$type> {
                let fid = fid.var_id(self);
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel SharedVar>](self.session.as_ptr(), self.get_group(), fid, &mut is_null);
                if is_null == true {
                    None
                } else {
                    impl_object!(@complex_get_val shared_var, self, fid, v, $type_name)
                }
            }
        }
    };
    (
        @complex_setter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        impl_object!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型实例变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_var_ $type_name _unchecked>](&mut self, fid: impl VarId, value: $type) {
                let fid = fid.var_id(self);
                impl_object!(@complex_set_val Field, self, self.ptr, fid, value, $type_name);
            }

            #[doc = "设置`" $type_name "`类型共享(静态)变量值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 变量无效或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_shared_var_ $type_name _unchecked>](&mut self, fid: impl SharedVarId, value: $type) {
                let fid = fid.var_id(self);
                impl_object!(@complex_set_val SharedVar, self, self.get_group(), fid, value, $type_name);
            }
        }
    };
    (@complex_get_val $var_kind: ty, $self: expr, $fid: expr, $value: expr, str) => {
        $self.session.get_string_unchecked($value)
    };
    (@complex_get_val $var_kind: ty, $self: expr, $fid: expr, $value: expr, string) => {
        $self.session.get_string_unchecked($value).map(PBStr::to_ucstring)
    };
    (@complex_get_val $var_kind: ty, $self: expr, $fid: expr, $value: expr, array) => {
        ::paste::paste! {
            Some(Array::from_ptr($value, $self.[<is_ $var_kind _object>]($fid), $self.session.clone()))
        }
    };
    (@complex_get_val $var_kind: ty, $self: expr, $fid: expr, $value: expr, object) => {
        Some(Object::from_ptr($value, $self.session.clone()))
    };
    (@complex_get_val $var_kind: ty, $self: expr, $fid: expr, $value: expr, any) => {
        Some(Value::from_ptr($value, $self.session.clone()))
    };
    (@complex_get_val $var_kind: ty, $self: expr, $fid: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            Some($self.session.[<get_ $type_name _unchecked>]($value))
        }
    };
    (@complex_set_val $var_kind: ty, $self: expr, $dst: expr, $fid: expr, $value: expr, str) => {
        ::paste::paste! {
            assert_eq!(ffi::[<pbsession_SetStr $var_kind>]($self.session.as_ptr(), $dst, $fid, $value.as_pbstr().as_ptr()), PBXRESULT::OK);
        }
    };
    (@complex_set_val $var_kind: ty, $self: expr, $dst: expr, $fid: expr, $value: expr, array) => {
        ::paste::paste! {
            assert_eq!(ffi::[<pbsession_SetArray $var_kind>]($self.session.as_ptr(), $dst, $fid, $value.as_ptr()), PBXRESULT::OK);
        }
    };
    (@complex_set_val $var_kind: ty, $self: expr, $dst: expr, $fid: expr, $value: expr, object) => {
        ::paste::paste! {
            assert_eq!(ffi::[<pbsession_SetObject $var_kind>]($self.session.as_ptr(), $dst, $fid, $value.as_ptr()), PBXRESULT::OK);
        }
    };
    (@complex_set_val $var_kind: ty, $self: expr, $dst: expr, $fid: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            assert_eq!(ffi::[<pbsession_Set $type_name:camel $var_kind>]($self.session.as_ptr(), $dst, $fid, $self.session.[<new_pb $type_name>]($value)), PBXRESULT::OK);
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
            pub fn [<get_var_ $type_name>](&self, fid: impl VarId) -> Option<$type> {
                self.[<try_get_var_ $type_name>](fid).unwrap()
            }

            #[doc = "获取`" $type_name "`类型实例变量值"]
            pub fn [<try_get_var_ $type_name>](&self, fid: impl VarId) -> Result<Option<$type>> {
                let fid = fid.try_var_id(self)?;
                if impl_object!(@check_type_get var, self, fid, $type_check, $type_name) {
                    unsafe {
                        Ok(self.[<get_var_ $type_name _unchecked>](fid))
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
            pub fn [<get_shared_var_ $type_name>](&self, fid: impl SharedVarId) -> Option<$type> {
                self.[<try_get_shared_var_ $type_name>](fid).unwrap()
            }

            #[doc = "获取`" $type_name "`类型共享(静态)变量值"]
            pub fn [<try_get_shared_var_ $type_name>](&self, fid: impl SharedVarId) -> Result<Option<$type>> {
                let fid = fid.try_var_id(self)?;
                if impl_object!(@check_type_get shared_var, self, fid, $type_check, $type_name) {
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
            pub fn [<set_var_ $type_name>](&mut self, fid: impl VarId, value: $type) {
                self.[<try_set_var_ $type_name>](fid, value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型实例变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<try_set_var_ $type_name>](&mut self, fid: impl VarId, value: $type) -> Result<()> {
                let fid = fid.try_var_id(self)?;
                if impl_object!(@check_type_set var, self, fid, value, $type_check, $type_name) {
                    unsafe {
                        self.[<set_var_ $type_name _unchecked>](fid, value);
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
            pub fn [<set_shared_var_ $type_name>](&mut self, fid: impl SharedVarId, value: $type) {
                self.[<try_set_shared_var_ $type_name>](fid, value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型共享(静态)变量值"]
            ///
            /// # Panics
            ///
            /// 变量无效或类型不匹配时会触发Panic
            pub fn [<try_set_shared_var_ $type_name>](&mut self, fid: impl SharedVarId, value: $type) -> Result<()> {
                let fid = fid.try_var_id(self)?;
                if impl_object!(@check_type_set shared_var, self, fid, value, $type_check, $type_name) {
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
    (@check_type_get $var_kind: ty, $self: expr, $fid: expr, $type_check: pat, array) => {
        ::paste::paste! {
            $self.[<is_ $var_kind _array>]($fid)
        }
    };
    (@check_type_get $var_kind: ty, $self: expr, $fid: expr, $type_check: pat, object) => {
        ::paste::paste! {
            $self.[<is_ $var_kind _object>]($fid)
        }
    };
    (@check_type_get $var_kind: ty, $self: expr, $fid: expr, $type_check: pat, $type_name: ty) => {
        ::paste::paste! {
            matches!($self.[<get_ $var_kind _type>]($fid), $type_check)
        }
    };
    (@check_type_set $var_kind: ty, $self: expr, $fid: expr, $value: expr, $type_check: pat, array) => {
        ::paste::paste! {
            $self.[<is_ $var_kind _array>]($fid) || $self.[<get_ $var_kind _type>]($fid) == ValueType::Any
        }
    };
    (@check_type_set $var_kind: ty, $self: expr, $fid: expr, $value: expr, $type_check: pat, object) => {
        ::paste::paste! {
            $self.[<is_ $var_kind _object>]($fid) || $self.[<get_ $var_kind _type>]($fid) == ValueType::Any
        }
    };
    (@check_type_set $var_kind: ty, $self: expr, $fid: expr, $value: expr, $type_check: pat, $type_name: ty) => {
        ::paste::paste! {
            matches!($self.[<get_ $var_kind _type>]($fid), $type_check | ValueType::NoType)
        }
    };
}

impl<'obj> Object<'obj> {
    impl_object!(
        @simple
        int, pbint, ValueType::Int
    );
    impl_object!(
        @simple
        uint, pbuint, ValueType::Uint
    );
    impl_object!(
        @simple
        long, pblong, ValueType::Long
    );
    impl_object!(
        @simple
        ulong, pbulong, ValueType::Ulong
    );
    impl_object!(
        @simple
        longlong, pblonglong, ValueType::LongLong
    );
    impl_object!(
        @simple
        real, pbreal, ValueType::Real
    );
    impl_object!(
        @simple
        double, pbdouble, ValueType::Double
    );
    impl_object!(
        @simple
        byte, pbbyte, ValueType::Byte
    );
    impl_object!(
        @simple
        bool, bool, ValueType::Boolean
    );
    impl_object!(
        @simple
        char, PBChar, ValueType::Char
    );

    impl_object!(
        @complex
        dec, Decimal, ValueType::Decimal
    );
    impl_object!(
        @complex
        date, NaiveDate, ValueType::Date
    );
    impl_object!(
        @complex
        time, NaiveTime, ValueType::Time
    );
    impl_object!(
        @complex
        datetime, NaiveDateTime, ValueType::DateTime
    );
    impl_object!(
        @complex_getter
        blob, &'obj [u8], ValueType::Blob
    );
    impl_object!(
        @complex_setter
        blob, &[u8], ValueType::Blob
    );
    impl_object!(
        @complex_getter
        str, &'obj PBStr, ValueType::String
    );
    impl_object!(
        @complex_getter
        string, PBString, ValueType::String
    );
    impl_object!(
        @complex_setter
        str, impl AsPBStr, ValueType::String
    );
    impl_object!(
        @complex_getter
        array, Array<'obj>, ValueType::NoType
    );
    impl_object!(
        @complex_setter
        array, &Array, ValueType::NoType
    );
    impl_object!(
        @complex_getter
        object, Object<'obj>, ValueType::NoType
    );
    impl_object!(
        @complex_setter
        object, &Object, ValueType::NoType
    );
    impl_object!(
        @complex_getter
        any, Value<'obj>, ValueType::Any
    );
}
