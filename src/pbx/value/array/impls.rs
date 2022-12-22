use super::*;

/*
    Getter/Setter
*/

macro_rules! impl_array {
    /*
        简单类型
    */
    (
        @simple
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_array!(
            @simple_getter
            $type_name, $type, $type_check
        );
        impl_array!(
            @simple_setter
            $type_name, $type, $type_check
        );
    };
    (
        @simple_getter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_array!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型元素值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_item_ $type_name _unchecked>](&self, dim: impl AsArrayIndex) -> Option<$type> {
                let dim = dim.as_array_index();
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel ArrayItem>](self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
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
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_array!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型元素值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_item_ $type_name _unchecked>](&mut self, dim: impl AsArrayIndex, value: $type) {
                let dim = dim.as_array_index();
                assert_eq!(ffi::[<pbsession_Set $type_name:camel ArrayItem>](self.session.as_ptr(), self.ptr, dim.as_ptr(), value.into()), PBXRESULT::OK);
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
        impl_array!(
            @complex_getter
            $type_name, $type, $type_check
        );
        impl_array!(
            @complex_setter
            $type_name, $type, $type_check
        );
    };
    (
        @complex_getter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_array!(
            @checked_getter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型元素值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_item_ $type_name _unchecked>](&self, dim: impl AsArrayIndex) -> Option<$type> {
                let dim = dim.as_array_index();
                let mut is_null = Default::default();
                let v = ffi::[<pbsession_Get $type_name:camel ArrayItem>](self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
                if is_null == true {
                    None
                } else {
                    impl_array!(@complex_get_val self, v, $type_name)
                }
            }
        }
    };
    (
        @complex_setter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_array!(
            @checked_setter
            $type_name, $type, $type_check
        );
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型元素值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_item_ $type_name _unchecked>](&mut self, dim: impl AsArrayIndex, value: $type) {
                let dim = dim.as_array_index();
                impl_array!(@complex_set_val self, dim, value, $type_name);
            }
        }
    };
    (@complex_get_val $self: expr, $value: expr, str) => {
        $self.session.get_string_unchecked($value)
    };
    (@complex_get_val $self: expr, $value: expr, string) => {
        $self.session.get_string_unchecked($value).map(PBStr::to_ucstring)
    };
    (@complex_get_val $self: expr, $value: expr, object) => {
        Some(Object::from_ptr($value, $self.session.clone()))
    };
    (@complex_get_val $self: expr, $value: expr, any) => {
        Some(Value::from_ptr($value, $self.session.clone()))
    };
    (@complex_get_val $self: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            Some($self.session.[<get_ $type_name _unchecked>]($value))
        }
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, str) => {
        assert_eq!(ffi::pbsession_SetStrArrayItem($self.session.as_ptr(), $self.ptr, $dim.as_ptr(), $value.as_pbstr().as_ptr()), PBXRESULT::OK);
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, object) => {
        assert_eq!(ffi::pbsession_SetObjectArrayItem($self.session.as_ptr(), $self.ptr, $dim.as_ptr(), $value.as_ptr()), PBXRESULT::OK);
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, any) => {
        ffi::pbsession_SetArrayItemValue($self.session.as_ptr(), $self.ptr, $dim.as_ptr(), $value.as_ptr());
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            assert_eq!(ffi::[<pbsession_Set $type_name:camel ArrayItem>]($self.session.as_ptr(), $self.ptr, $dim.as_ptr(), $self.session.[<new_pb $type_name>]($value)), PBXRESULT::OK);
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
            #[doc = "获取`" $type_name "`类型元素值"]
            ///
            /// # Panics
            ///
            /// 索引越界或类型不匹配时会触发Panic
            pub fn [<get_item_ $type_name>](&self, dim: impl AsArrayIndex) -> Option<$type> {
                self.[<try_get_item_ $type_name>](dim).unwrap()
            }

            #[doc = "获取`" $type_name "`类型元素值"]
            pub fn [<try_get_item_ $type_name>](&self, dim: impl AsArrayIndex) -> Result<Option<$type>> {
                let dim = dim.as_array_index();
                impl_array!(@check_type_get self, dim, $type_check, $type_name);
                unsafe {
                    Ok(self.[<get_item_ $type_name _unchecked>](dim))
                }
            }
        }
    };
    (
        @checked_setter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型元素值"]
            ///
            /// # Panics
            ///
            /// 索引越界或类型不匹配时会触发Panic
            pub fn [<set_item_ $type_name>](&mut self, dim: impl AsArrayIndex, value: $type) {
                self.[<try_set_item_ $type_name>](dim, value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型元素值"]
            ///
            /// # Panics
            ///
            /// 索引越界或类型不匹配时会触发Panic
            pub fn [<try_set_item_ $type_name>](&mut self, dim: impl AsArrayIndex, value: $type) -> Result<()> {
                let dim = dim.as_array_index();
                impl_array!(@check_type_set self, dim, value, $type_check, $type_name);
                unsafe {
                    self.[<set_item_ $type_name _unchecked>](dim, value);
                }
                Ok(())
            }
        }
    };
    (@check_type_get $self: expr, $dim: expr, $type_check: expr, object) => {
        if !$self.is_item_object($dim) {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }
    };
    (@check_type_get $self: expr, $dim: expr, $type_check: expr, $type_name: ty) => {
        $self.check_get($dim, $type_check)?;
    };
    (@check_type_set $self: expr, $dim: expr, $value: expr, $type_check: expr, object) => {
        $self.check_set_index($dim)?;
        if !$self.is_item_object($dim) && $self.info.value_type() != ValueType::Any {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }
    };
    (@check_type_set $self: expr, $dim: expr, $value: expr, $type_check: expr, any) => {
        $self.check_set($dim, $value.get_type())?;
    };
    (@check_type_set $self: expr, $dim: expr, $value: expr, $type_check: expr, $type_name: ty) => {
        $self.check_set($dim, $type_check)?;
    };

}

impl<'arr> Array<'arr> {
    impl_array!(
        @simple
        int, pbint, ValueType::Int
    );
    impl_array!(
        @simple
        uint, pbuint, ValueType::Uint
    );
    impl_array!(
        @simple
        long, pblong, ValueType::Long
    );
    impl_array!(
        @simple
        ulong, pbulong, ValueType::Ulong
    );
    impl_array!(
        @simple
        longlong, pblonglong, ValueType::LongLong
    );
    impl_array!(
        @simple
        real, pbreal, ValueType::Real
    );
    impl_array!(
        @simple
        double, pbdouble, ValueType::Double
    );
    impl_array!(
        @simple
        byte, pbbyte, ValueType::Byte
    );
    impl_array!(
        @simple
        bool, bool, ValueType::Boolean
    );
    impl_array!(
        @simple
        char, PBChar, ValueType::Char
    );

    impl_array!(
        @complex
        dec, Decimal, ValueType::Decimal
    );
    impl_array!(
        @complex
        date, NaiveDate, ValueType::Date
    );
    impl_array!(
        @complex
        time, NaiveTime, ValueType::Time
    );
    impl_array!(
        @complex
        datetime, NaiveDateTime, ValueType::DateTime
    );
    impl_array!(
        @complex_getter
        blob, &'arr [u8], ValueType::Blob
    );
    impl_array!(
        @complex_setter
        blob, &[u8], ValueType::Blob
    );
    impl_array!(
        @complex_getter
        str, &'arr PBStr, ValueType::String
    );
    impl_array!(
        @complex_getter
        string, PBString, ValueType::String
    );
    impl_array!(
        @complex_setter
        str, impl AsPBStr, ValueType::String
    );
    impl_array!(
        @complex_getter
        object, Object<'arr>, ValueType::NoType
    );
    impl_array!(
        @complex_setter
        object, &Object, ValueType::NoType
    );
    impl_array!(
        @complex_getter
        any, Value<'arr>, ValueType::Any
    );
    impl_array!(
        @complex_setter
        any, Value, ValueType::Any
    );
}
