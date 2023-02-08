//! 数组元素访问接口实现
//!
use super::*;
use crate::comm::*;

impl<'arr> Array<'arr> {
    /// 获取元素类型
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn get_item_type(&self, dim: impl AsArrayIndex) -> ValueType { self.try_get_item_type(dim).unwrap() }

    /// 获取元素类型
    pub fn try_get_item_type(&self, dim: impl AsArrayIndex) -> Result<ValueType> {
        let dim = dim.as_array_index();
        self.check_get_index(dim)?;
        unsafe { Ok(ffi::pbsession_GetArrayItemType(self.session.as_raw(), self.ptr, dim.as_ptr())) }
    }

    /// 判断元素是否为对象类型
    pub fn is_item_object(&self, _dim: impl AsArrayIndex) -> bool { self.is_object }

    /// 判断元素是否为NULL
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn is_item_null(&self, dim: impl AsArrayIndex) -> bool { self.try_is_item_null(dim).unwrap() }

    /// 判断元素是否为NULL
    pub fn try_is_item_null(&self, dim: impl AsArrayIndex) -> Result<bool> {
        let dim = dim.as_array_index();
        self.check_get_index(dim)?;
        unsafe { Ok(ffi::pbsession_IsArrayItemNull(self.session.as_raw(), self.ptr, dim.as_ptr()).into()) }
    }

    /// 设置元素为NULL
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn set_item_to_null(&mut self, dim: impl AsArrayIndex) { self.try_set_item_to_null(dim).unwrap(); }

    /// 设置元素为NULL
    pub fn try_set_item_to_null(&mut self, dim: impl AsArrayIndex) -> Result<()> {
        let dim = dim.as_array_index();
        self.check_set_index(dim)?;
        unsafe {
            self.set_item_to_null_unchecked(dim);
        }
        Ok(())
    }

    /// 设置元素为NULL
    ///
    /// # Safety
    ///
    /// 索引越界时可能会出现未定义行为
    pub unsafe fn set_item_to_null_unchecked(&mut self, dim: impl AsArrayIndex) {
        let dim = dim.as_array_index();
        ffi::pbsession_SetArrayItemToNull(self.session.as_raw(), self.ptr, dim.as_ptr());
    }

    /// 拷贝元素的值
    pub fn acquire_item_value(&mut self, dim: impl AsArrayIndex) -> OwnedValue {
        let dim = dim.as_array_index();
        unsafe {
            let new_value =
                ffi::pbsession_AcquireArrayItemValue(self.session.as_raw(), self.ptr, dim.as_ptr());
            OwnedValue::from_raw(new_value, self.session.clone())
        }
    }

    #[inline]
    fn check_get_index(&self, dim: &[pblong]) -> Result<()> {
        if dim.is_empty() {
            return Err(PBXRESULT::E_OUTOF_MEMORY);
        }
        if !self.info.bounded() {
            if dim.len() > 1 {
                return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
            }
            if dim[0] < 1 || dim[0] > self.len() {
                return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
            }
        } else {
            if dim.len() != self.info.dimensions() as usize {
                return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
            }
            for (dim, &idx) in dim.iter().enumerate() {
                let (lower, upper) = self.info.bound(dim as pbuint + 1);
                if idx < lower || idx > upper {
                    return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn check_get(&self, dim: &[pblong], get_type: ValueType) -> Result<()> {
        //get_item_type包含check_get_index
        let item_type = self.get_item_type(dim);
        if item_type == get_type {
            Ok(())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    #[inline]
    fn check_set_index(&self, dim: &[pblong]) -> Result<()> {
        if dim.is_empty() {
            return Err(PBXRESULT::E_OUTOF_MEMORY);
        }
        if !self.info.bounded() {
            if dim.len() > 1 {
                return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
            }
            if dim[0] < 1 {
                return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
            }
        } else {
            if dim.len() != self.info.dimensions() as usize {
                return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
            }
            for (dim, &idx) in dim.iter().enumerate() {
                let (lower, upper) = self.info.bound(dim as pbuint);
                if idx < lower || idx > upper {
                    return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn check_set(&self, dim: &[pblong], set_type: ValueType) -> Result<()> {
        self.check_set_index(dim)?;
        let item_type = self.info.value_type();
        if item_type == set_type || item_type == ValueType::Any {
            Ok(())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }
}

/*
   Getter/Setter
*/

macro_rules! impl_item {
    /*
        简单类型
    */
    (
        @simple
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_item!(
            @simple_getter
            $type_name, $type, $type_check
        );
        impl_item!(
            @simple_setter
            $type_name, $type, $type_check
        );
    };
    (
        @simple_getter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_item!(
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
                let v = ffi::[<pbsession_Get $type_name:camel ArrayItem>](self.session.as_raw(), self.ptr, dim.as_ptr(), &mut is_null);
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
        impl_item!(
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
                debug_assert_eq!(ffi::[<pbsession_Set $type_name:camel ArrayItem>](self.session.as_raw(), self.ptr, dim.as_ptr(), value.into()), PBXRESULT::OK);
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
        impl_item!(
            @complex_getter
            $type_name, $type, $type_check
        );
        impl_item!(
            @complex_setter
            $type_name, $type, $type_check
        );
    };
    (
        @complex_getter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_item!(
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
                let v = ffi::[<pbsession_Get $type_name:camel ArrayItem>](self.session.as_raw(), self.ptr, dim.as_ptr(), &mut is_null);
                if is_null == true {
                    None
                } else {
                    impl_item!(@complex_get_val self, v, $type_name)
                }
            }
        }
    };
    (
        @complex_setter
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_item!(
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
                debug_assert_eq!(ffi::[<pbsession_Set $type_name:camel ArrayItem>](self.session.as_raw(), self.ptr, dim.as_ptr(), impl_item!(@complex_set_val self, dim, value, $type_name)), PBXRESULT::OK);
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
        Some(Object::from_raw($value, $self.session.clone()))
    };
    (@complex_get_val $self: expr, $value: expr, any) => {
        Some(Value::from_raw($value, $self.session.clone()))
    };
    (@complex_get_val $self: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            Some($self.session.[<get_ $type_name _unchecked>]($value))
        }
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, str) => {
        $value.as_pbstr().as_ptr()
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, object) => {
        $value.as_raw()
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, any) => {
        $value.as_raw()
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            $self.session.[<new_pb $type_name>]($value)
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
                impl_item!(@check_type_get self, dim, $type_check, $type_name);
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
                impl_item!(@check_type_set self, dim, value, $type_check, $type_name);
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
    impl_item!(
        @simple
        int, pbint, ValueType::Int
    );
    impl_item!(
        @simple
        uint, pbuint, ValueType::Uint
    );
    impl_item!(
        @simple
        long, pblong, ValueType::Long
    );
    impl_item!(
        @simple
        ulong, pbulong, ValueType::Ulong
    );
    impl_item!(
        @simple
        longlong, pblonglong, ValueType::LongLong
    );
    impl_item!(
        @simple
        real, pbreal, ValueType::Real
    );
    impl_item!(
        @simple
        double, pbdouble, ValueType::Double
    );
    impl_item!(
        @simple
        byte, pbbyte, ValueType::Byte
    );
    impl_item!(
        @simple
        bool, bool, ValueType::Boolean
    );
    impl_item!(
        @simple
        char, PBChar, ValueType::Char
    );

    impl_item!(
        @complex
        dec, Decimal, ValueType::Decimal
    );
    impl_item!(
        @complex
        date, NaiveDate, ValueType::Date
    );
    impl_item!(
        @complex
        time, NaiveTime, ValueType::Time
    );
    impl_item!(
        @complex
        datetime, NaiveDateTime, ValueType::DateTime
    );
    impl_item!(
        @complex_getter
        blob, &'arr [u8], ValueType::Blob
    );
    impl_item!(
        @complex_setter
        blob, &[u8], ValueType::Blob
    );
    impl_item!(
        @complex_getter
        str, &'arr PBStr, ValueType::String
    );
    impl_item!(
        @complex_getter
        string, PBString, ValueType::String
    );
    impl_item!(
        @complex_setter
        str, impl AsPBStr, ValueType::String
    );
    impl_item!(
        @complex_getter
        object, Object<'arr>, ValueType::NoType
    );
    impl_item!(
        @complex_setter
        object, &Object, ValueType::NoType
    );
    impl_item!(
        @complex_getter
        any, Value<'arr>, ValueType::Any
    );
    impl_item!(
        @complex_setter
        any, Value, ValueType::Any
    );
}
