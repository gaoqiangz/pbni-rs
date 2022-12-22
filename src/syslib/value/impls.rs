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
        $type_name: ty, $type: ty, $type_check: pat,
        $field: ident, $field_style: expr, $field_type: expr
    ) => {
        impl_value!(
            @simple_getter
            $type_name, $type, $type_check,
            $field
        );
        impl_value!(
            @simple_setter
            $type_name, $type, $type_check,
            $field, $field_style, $field_type
        );
    };
    (
        @simple_getter
        $type_name: ty, $type: ty, $type_check: pat,
        $field: ident
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
                    Some(impl_value!(@simple_get_val self.inner.val.$field, $type_name) as _)
                }
            }
        }
    };
    (
        @simple_setter
        $type_name: ty, $type: ty, $type_check: pat,
        $field: ident, $field_style: expr, $field_type: expr
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
                self.free_val_ptr();
                self.inner.val.$field = impl_value!(@simple_set_val value, $type_name) as _;
                self.set_info($field_style, $field_type, OB_GROUPTYPE::OB_SIMPLE);
            }
        }
    };
    (@simple_get_val $value: expr, bool) => {
        if $value == 1 {
            true
        } else {
            false
        }
    };
    (@simple_get_val $value: expr, $type_name: ty) => {
        $value
    };
    (@simple_set_val $value: expr, bool) => {
        if $value {
            1
        } else {
            0
        }
    };
    (@simple_set_val $value: expr, $type_name: ty) => {
        $value
    };

    /*
        复杂类型
    */
    (
        @complex
        $type_name: ty, $type: ty, $type_check: pat,
        $field_type: expr
    ) => {
        impl_value!(
            @complex_getter
            $type_name, $type, $type_check
        );
        impl_value!(
            @complex_setter
            $type_name, $type, $type_check,
            $field_type
        );
    };
    (
        @complex_getter
        $type_name: ty, $type: ty, $type_check: pat
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
                    Some(impl_value!(@complex_get_val self, self.inner.val.ptr, $type_name))
                }
            }
        }
    };
    (
        @complex_setter
        $type_name: ty, $type: ty, $type_check: pat,
        $field_type: expr
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
                impl_value!(@complex_set_val self, value, $field_type, $type_name);
            }
        }
    };
    (@complex_get_val $self: expr, $value: expr, longlong) => {
        *($value as *const pblonglong)
    };
    (@complex_get_val $self: expr, $value: expr, double) => {
        *($value as *const pbdouble)
    };
    (@complex_get_val $self: expr, $value: expr, str) => {
        PBStr::from_ptr_str($value as _)
    };
    (@complex_get_val $self: expr, $value: expr, string) => {
        PBString::from_ptr_str($value as _)
    };
    (@complex_get_val $self: expr, $value: expr, array) => {
        Array::from_ptr($value as _, $self.is_object(), $self.session.clone())
    };
    (@complex_get_val $self: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            $self.session.[<get_ $type_name _unchecked>]($value as _)
        }
    };
    (@complex_set_val $self: expr, $value: expr, $field_type: expr, longlong) => {
        $self.set_ptr(API.ob_dup_longlong($self.session.as_ptr(), &$value as *const pblonglong as _) as _, $field_type);
    };
    (@complex_set_val $self: expr, $value: expr, $field_type: expr, double) => {
        $self.set_ptr(API.ob_dup_double($self.session.as_ptr(), &$value as *const pbdouble as _) as _, $field_type);
    };
    (@complex_set_val $self: expr, $value: expr, $field_type: expr, str) => {
        $self.set_ptr(API.ob_dup_string($self.session.as_ptr(), $value.as_pbstr().as_ptr() as _) as _, $field_type);
    };
    (@complex_set_val $self: expr, $value: expr, $field_type: expr, array) => {
        {
            $self.set_ptr(API.ot_copy_array($self.session.as_ptr(), $value.as_ptr() as _) as _, $value.info().value_type() as OB_CLASS_ID);
            $self.set_info_group(OB_GROUPTYPE::OB_ARRAY);
        }
    };
    (@complex_set_val $self: expr, $value: expr, $field_type: expr, value) => {
        API.rtDataCopy($self.session.as_ptr(), $self.as_ptr(), $value.as_ptr(), true as BOOL);
    };
    (@complex_set_val $self: expr, $value: expr, $field_type: expr, $type_name: ty) => {
        ::paste::paste! {
            $self.set_ptr($self.session.[<new_pb $type_name>]($value) as _, $field_type);
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
                    Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }
        }
    };
    (
        @checked_setter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型值"]
            ///
            /// # Panics
            ///
            /// 类型不匹配时会触发Panic
            pub fn [<set_ $type_name>](&mut self, value: $type) {
                self.[<try_set_ $type_name>](value).unwrap();
            }

            #[doc = "设置`" $type_name "`类型值"]
            pub fn [<try_set_ $type_name>](&mut self, value: $type) -> Result<()> {
                if impl_value!(@check_type_set self, value, $type_check, $type_name) {
                    unsafe {
                        self.[<set_ $type_name _unchecked>](value);
                    }
                    Ok(())
                } else {
                    return Err(PBRESULT::E_MISMATCHED_DATA_TYPE);
                }
            }
        }
    };
    (@check_type_get $self: expr, $type_check: pat, array) => {
        $self.is_array()
    };
    (@check_type_get $self: expr, $type_check: pat, $type_name: ty) => {
        matches!($self.get_type(), $type_check)
    };
    (@check_type_set $self: expr, $value: expr, $type_check: pat, array) => {
        $self.is_array() || $self.get_type() == ValueType::NoType
    };
    (@check_type_set $self: expr, $value: expr, $type_check: pat, value) => {
        $self.get_type() == $value.get_type() &&
        $self.get_type_kind() == $value.get_type_kind() &&
        $self.get_info_group() == $value.get_info_group() &&
        $self.get_info_style() == $value.get_info_style()
    };
    (@check_type_set $self: expr, $value: expr, $type_check: pat, $type_name: ty) => {
        matches!($self.get_type(), $type_check | ValueType::NoType)
    };
}

impl<'val> Value<'val> {
    impl_value!(
        @simple
        int, pbint, ValueType::Int,
        int_val, OB_DATASTYLE::INT_STYLE, INT_TYPE
    );
    impl_value!(
        @simple
        uint, pbuint, ValueType::Uint,
        uint_val, OB_DATASTYLE::INT_STYLE, UINT_TYPE
    );
    impl_value!(
        @simple
        long, pblong, ValueType::Long,
        long_val, OB_DATASTYLE::LONG_STYLE, LONG_TYPE
    );
    impl_value!(
        @simple
        ulong, pbulong, ValueType::Ulong,
        ulong_val, OB_DATASTYLE::LONG_STYLE, ULONG_TYPE
    );
    impl_value!(
        @simple
        real, pbreal, ValueType::Real,
        fl_val, OB_DATASTYLE::FLOAT_STYLE, FLOAT_TYPE
    );
    impl_value!(
        @simple
        byte, pbbyte, ValueType::Byte,
        uint_val, OB_DATASTYLE::INT_STYLE, BYTE_TYPE
    );
    impl_value!(
        @simple
        bool, bool, ValueType::Boolean,
        int_val, OB_DATASTYLE::INT_STYLE, BOOL_TYPE
    );
    impl_value!(
        @simple
        char, PBChar, ValueType::Char,
        uint_val, OB_DATASTYLE::INT_STYLE, CHAR_TYPE
    );

    impl_value!(
        @complex
        longlong, pblonglong, ValueType::LongLong,
        LONGLONG_TYPE
    );
    impl_value!(
        @complex
        double, pbdouble, ValueType::Double,
        DOUBLE_TYPE
    );
    impl_value!(
        @complex
        dec, Decimal, ValueType::Decimal,
        DEC_TYPE
    );
    impl_value!(
        @complex
        date, NaiveDate, ValueType::Date,
        DATE_TYPE
    );
    impl_value!(
        @complex
        time, NaiveTime, ValueType::Time,
        TIME_TYPE
    );
    impl_value!(
        @complex
        datetime, NaiveDateTime, ValueType::DateTime,
        DATETIME_TYPE
    );
    impl_value!(
        @complex_getter
        blob, &'val [u8], ValueType::Blob
    );
    impl_value!(
        @complex_setter
        blob, &[u8], ValueType::Blob,
        BINARY_TYPE
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
        str, impl AsPBStr, ValueType::String,
        STRING_TYPE
    );
    impl_value!(
        @complex_getter
        array, Array<'val>, ValueType::NoType
    );
    impl_value!(
        @complex_setter
        array, &Array, ValueType::NoType,
        NO_TYPE
    );
    impl_value!(
        @complex_setter
        value, &Value, ValueType::NoType,
        NO_TYPE
    );
}
