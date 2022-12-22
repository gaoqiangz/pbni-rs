use super::*;

/*
    Getter/Setter
*/

macro_rules! impl_value {
    /*
        简单类型
    */
    (
        @simple
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_value!(
            @simple_getter
            $type_name, $type, $type_check
        );
        impl_value!(
            @simple_setter
            $type_name, $type, $type_check
        );
    };
    (
        @simple_getter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_value!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_ $type_name _unchecked>](&self) -> Option<$type> {
                if self.is_null() {
                    None
                } else {
                    let v = ffi::[<pbvalue_Get $type_name:camel>](self.ptr);
                    Some(v.into())
                }
            }
        }
    };
    (
        @simple_setter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_value!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_ $type_name _unchecked>](&mut self, value: $type) {
                assert_eq!(ffi::[<pbvalue_Set $type_name:camel>](self.ptr, value.into()), PBXRESULT::OK);
            }
        }
    };

    /*
        复杂类型
    */
    (
        @complex
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_value!(
            @complex_getter
            $type_name, $type, $type_check
        );
        impl_value!(
            @complex_setter
            $type_name, $type, $type_check
        );
    };
    (
        @complex_getter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_value!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_ $type_name _unchecked>](&self) -> Option<$type> {
                if self.is_null() {
                    None
                } else {
                    let v = ffi::[<pbvalue_Get $type_name:camel>](self.ptr);
                    impl_value!(@complex_get_val self, v, $type_name)
                }
            }
        }
    };
    (
        @complex_setter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_value!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_ $type_name _unchecked>](&mut self, value: $type) {
                impl_value!(@complex_set_val self, value, $type_name);
            }
        }
    };
    (@complex_get_val $self: expr, $value: expr, str) => {
        $self.session.get_string_unchecked($value)
    };
    (@complex_get_val $self: expr, $value: expr, string) => {
        $self.session.get_string_unchecked($value).map(PBStr::to_ucstring)
    };
    (@complex_get_val $self: expr, $value: expr, array) => {
        Some(Array::from_ptr($value, $self.is_object(), $self.session.clone()))
    };
    (@complex_get_val $self: expr, $value: expr, object) => {
        Some(Object::from_ptr($value, $self.session.clone()))
    };
    (@complex_get_val $self: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            Some($self.session.[<get_ $type_name _unchecked>]($value))
        }
    };
    (@complex_set_val $self: expr, $value: expr, str) => {
        assert_eq!(ffi::pbvalue_SetStr($self.ptr, $value.as_pbstr().as_ptr()), PBXRESULT::OK);
    };
    (@complex_set_val $self: expr, $value: expr, array) => {
        assert_eq!(ffi::pbvalue_SetArray($self.ptr, $value.as_ptr()), PBXRESULT::OK);
    };
    (@complex_set_val $self: expr, $value: expr, object) => {
        assert_eq!(ffi::pbvalue_SetObject($self.ptr, $value.as_ptr()), PBXRESULT::OK);
    };
    (@complex_set_val $self: expr, $value: expr, value) => {
        ffi::pbsession_SetValue($self.session.as_ptr(), $self.ptr, $value.ptr);
    };
    (@complex_set_val $self: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            assert_eq!(ffi::[<pbvalue_Set $type_name:camel>]($self.ptr, $self.session.[<new_pb $type_name>]($value)), PBXRESULT::OK);
        }
    };

    /*
        通用类型检查接口
    */
    (
        @checked_getter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型值"]
            ///
            /// # Panics
            ///
            /// 类型不匹配时会触发Panic
            pub fn [<get_ $type_name>](&self) -> Option<$type> {
                self.[<try_get_ $type_name>]().unwrap()
            }

            #[doc = "获取`" $type_name "`类型值"]
            pub fn [<try_get_ $type_name>](&self) -> Result<Option<$type>> {
                if impl_value!(@check_type_get self, $type_check, $type_name) {
                    unsafe {
                        Ok(self.[<get_ $type_name _unchecked>]())
                    }
                } else {
                    Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }
        }
    };
    (
        @checked_setter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型值"]
            ///
            /// # Panics
            ///
            /// 类型不匹配时会触发Panic
            pub fn [<set_ $type_name>](&mut self, value: $type) {
                self.[<try_set_ $type_name>](value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型值"]
            pub fn [<try_set_ $type_name>](&mut self, value: $type) -> Result<()> {
                if impl_value!(@check_type_set self, value, $type_check, $type_name) {
                    unsafe {
                        self.[<set_ $type_name _unchecked>](value);
                    }
                    Ok(())
                } else {
                    Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }
        }
    };
    (@check_type_get $self: expr, $type_check: pat, array) => {
        $self.is_array()
    };
    (@check_type_get $self: expr, $type_check: pat, object) => {
        $self.is_object()
    };
    (@check_type_get $self: expr, $type_check: pat, $type_name: ty) => {
        matches!($self.get_type(), $type_check)
    };
    (@check_type_set $self: expr, $value: expr, $type_check: pat, array) => {
        $self.is_array() || $self.get_type() == ValueType::NoType
    };
    (@check_type_set $self: expr, $value: expr, $type_check: pat, object) => {
        $self.is_object() || $self.get_type() == ValueType::NoType
    };
    (@check_type_set $self: expr, $value: expr, $type_check: pat, value) => {
        $self.get_type() == $value.get_type() &&
        $self.is_object() == $value.is_object() &&
        $self.is_array() == $value.is_array() &&
        $self.is_enum() == $value.is_enum()
    };
    (@check_type_set $self: expr, $value: expr, $type_check: pat, $type_name: ty) => {
        matches!($self.get_type(), $type_check | ValueType::NoType)
    };
}

impl<'val> Value<'val> {
    impl_value!(
        @simple
        int, pbint, ValueType::Int
    );
    impl_value!(
        @simple
        uint, pbuint, ValueType::Uint
    );
    impl_value!(
        @simple
        long, pblong, ValueType::Long
    );
    impl_value!(
        @simple
        ulong, pbulong, ValueType::Ulong
    );
    impl_value!(
        @simple
        longlong, pblonglong, ValueType::LongLong
    );
    impl_value!(
        @simple
        real, pbreal, ValueType::Real
    );
    impl_value!(
        @simple
        double, pbdouble, ValueType::Double
    );
    impl_value!(
        @simple
        byte, pbbyte, ValueType::Byte
    );
    impl_value!(
        @simple
        bool, bool, ValueType::Boolean
    );
    impl_value!(
        @simple
        char, PBChar, ValueType::Char
    );

    impl_value!(
        @complex
        dec, Decimal, ValueType::Decimal
    );
    impl_value!(
        @complex
        date, NaiveDate, ValueType::Date
    );
    impl_value!(
        @complex
        time, NaiveTime, ValueType::Time
    );
    impl_value!(
        @complex
        datetime, NaiveDateTime, ValueType::DateTime
    );
    impl_value!(
        @complex_getter
        blob, &'val [u8], ValueType::Blob
    );
    impl_value!(
        @complex_setter
        blob, &[u8], ValueType::Blob
    );
    impl_value!(
        @complex_getter
        str, &'val PBStr, ValueType::String
    );
    impl_value!(
        @complex_getter
        string, PBString, ValueType::String
    );
    impl_value!(
        @complex_setter
        str, impl AsPBStr, ValueType::String
    );
    impl_value!(
        @complex_getter
        array, Array<'val>, ValueType::NoType
    );
    impl_value!(
        @complex_setter
        array, &Array, ValueType::NoType
    );
    impl_value!(
        @complex_getter
        object, Object<'val>, ValueType::NoType
    );
    impl_value!(
        @complex_setter
        object, &Object, ValueType::NoType
    );
    impl_value!(
        @complex_setter
        value, &Value, ValueType::NoType
    );
}
