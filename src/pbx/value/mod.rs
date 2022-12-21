use crate::{
    pbx::{bindings::*, *}, prelude::*
};

mod array;
pub use array::Array;

/// 值的引用
pub struct Value<'val> {
    ptr: pbvalue,
    session: Session,
    _marker: PhantomData<&'val mut pbvalue>
}

impl<'val> Value<'val> {
    pub(crate) unsafe fn from_ptr(ptr: pbvalue, session: Session) -> Value<'val> {
        Value {
            ptr,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn as_ptr(&self) -> pbvalue { self.ptr }

    pub(crate) fn get_class(&self) -> Option<pbclass> { unsafe { ffi::pbvalue_GetClass(self.ptr) } }

    /// 获取值的类型
    pub fn get_type(&self) -> ValueType { unsafe { ffi::pbvalue_GetType(self.ptr).into() } }

    /// 判断值是否为NULL
    pub fn is_null(&self) -> bool { unsafe { ffi::pbvalue_IsNull(self.ptr).into() } }

    /// 判断值的类型是否为对象
    pub fn is_object(&self) -> bool { unsafe { ffi::pbvalue_IsObject(self.ptr).into() } }

    /// 判断值的类型是否为数组
    pub fn is_array(&self) -> bool { unsafe { ffi::pbvalue_IsArray(self.ptr).into() } }

    /// 判断值的类型是否为枚举
    pub fn is_enum(&self) -> bool { unsafe { ffi::pbvalue_IsEnum(self.ptr).into() } }

    /// 判断值是否为引用传递
    pub fn is_ref(&self) -> bool { unsafe { ffi::pbvalue_IsByRef(self.ptr).into() } }

    /// 判断值是否为只读传递
    pub fn is_readonly(&self) -> bool { unsafe { ffi::pbvalue_IsReadOnly(self.ptr).into() } }

    /// 获取对象类型值的引用
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_object(&self) -> Option<Object<'val>> { self.try_get_object().unwrap() }

    /// 尝试获取对象类型值的引用
    pub fn try_get_object(&self) -> Result<Option<Object<'val>>> {
        if self.is_object() {
            unsafe { Ok(self.get_object_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取对象类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_object_unchecked(&self) -> Option<Object<'val>> {
        if self.is_null() {
            None
        } else {
            Some(Object::from_ptr(ffi::pbvalue_GetObject(self.ptr), self.session.clone()))
        }
    }

    /// 获取数组类型值的引用
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_array(&self) -> Option<Array<'val>> { self.try_get_array().unwrap() }

    /// 尝试获取数组类型值的引用
    pub fn try_get_array(&self) -> Result<Option<Array<'val>>> {
        if self.is_array() {
            unsafe { Ok(self.get_array_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取数组类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_array_unchecked(&self) -> Option<Array<'val>> {
        if self.is_null() {
            None
        } else {
            Some(Array::from_ptr(ffi::pbvalue_GetArray(self.ptr), self.is_object(), self.session.clone()))
        }
    }

    /// 设置值为NULL
    pub fn set_to_null(&mut self) { unsafe { assert_eq!(ffi::pbvalue_SetToNull(self.ptr), PBXRESULT::OK) } }

    /// 设置对象类型的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_object(&mut self, value: &Object) { self.try_set_object(value).unwrap(); }

    /// 设置对象类型的值
    pub fn try_set_object(&mut self, value: &Object) -> Result<()> {
        if self.is_object() {
            unsafe {
                self.set_object_unchecked(value);
            }
            Ok(())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置对象类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_object_unchecked(&mut self, v: &Object) {
        assert_eq!(ffi::pbvalue_SetObject(self.ptr, v.as_ptr()), PBXRESULT::OK);
    }

    /// 设置数组类型的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_array(&mut self, value: &Array) { self.try_set_array(value).unwrap(); }

    /// 设置数组类型的值
    pub fn try_set_array(&mut self, value: &Array) -> Result<()> {
        if self.is_array() {
            unsafe {
                self.set_array_unchecked(value);
            }
            Ok(())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置数组类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_array_unchecked(&mut self, v: &Array) {
        assert_eq!(ffi::pbvalue_SetArray(self.ptr, v.as_ptr()), PBXRESULT::OK);
    }

    /// 从参数拷贝并覆盖现有值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_value(&mut self, value: &Value) { self.try_set_value(value).unwrap(); }

    /// 从参数拷贝并覆盖现有值
    pub fn try_set_value(&mut self, value: &Value) -> Result<()> {
        if self.get_type() == value.get_type() &&
            self.is_object() == value.is_object() &&
            self.is_array() == value.is_array() &&
            self.is_enum() == value.is_enum()
        {
            unsafe {
                self.set_value_unchecked(value);
            }
            Ok(())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 从参数拷贝并覆盖现有值
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_value_unchecked(&mut self, src: &Value) {
        ffi::pbsession_SetValue(self.session.as_ptr(), self.ptr, src.ptr)
    }

    /// 拷贝并转移所有权,`self`将被消耗
    pub fn acquire(self) -> OwnedValue {
        unsafe {
            let new_value = ffi::pbsession_AcquireValue(self.session.as_ptr(), self.ptr);
            OwnedValue::from_ptr(new_value, self.session.clone())
        }
    }
}

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
            /// 索引越界或类型不兼容时可能会出现未定义行为
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
            /// 索引越界或类型不兼容时可能会出现未定义行为
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
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_ $type_name _unchecked>](&self) -> Option<$type> {
                if self.is_null() {
                    None
                } else {
                    let v = ffi::[<pbvalue_Get $type_name:camel>](self.ptr);
                    impl_value!(@complex_get_val self.session, v, $type_name)
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
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_ $type_name _unchecked>](&mut self, value: $type) {
                assert_eq!(ffi::[<pbvalue_Set $type_name:camel>](self.ptr, impl_value!(@complex_set_val self.session, value, $type_name)), PBXRESULT::OK);
            }
        }
    };
    (@complex_get_val $session: expr, $value: expr, str) => {
        $session.get_string_unchecked($value)
    };
    (@complex_get_val $session: expr, $value: expr, string) => {
        $session.get_string_unchecked($value).map(PBStr::to_ucstring)
    };
    (@complex_get_val $session: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            Some($session.[<get_ $type_name _unchecked>]($value))
        }
    };
    (@complex_set_val $session: expr, $value: expr, str) => {
        $value.as_pbstr().as_ptr()
    };
    (@complex_set_val $session: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            $session.[<new_pb $type_name>]($value)
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
            /// 索引越界或类型不匹配时会触发Panic
            pub fn [<get_ $type_name>](&self) -> Option<$type> {
                self.[<try_get_ $type_name>]().unwrap()
            }

            #[doc = "获取`" $type_name "`类型值"]
            pub fn [<try_get_ $type_name>](&self) -> Result<Option<$type>> {
                if matches!(self.get_type(), $type_check) {
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
            /// 索引越界或类型不匹配时会触发Panic
            pub fn [<set_ $type_name>](&mut self, value: $type) {
                self.[<try_set_ $type_name>](value).unwrap()
            }

            #[doc = "设置`" $type_name "`类型值"]
            pub fn [<try_set_ $type_name>](&mut self, value: $type) -> Result<()> {
                if matches!(self.get_type(), $type_check) {
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
}

/// 拥有所有权的值
pub struct OwnedValue {
    ptr: pbvalue,
    session: Session
}

impl OwnedValue {
    pub(crate) unsafe fn from_ptr(ptr: pbvalue, session: Session) -> OwnedValue {
        OwnedValue {
            ptr,
            session
        }
    }
    pub(crate) fn as_ptr(&self) -> pbvalue { self.ptr }

    /// 获取值的引用
    pub fn value(&self) -> Value { unsafe { Value::from_ptr(self.ptr, self.session.clone()) } }
}

impl Drop for OwnedValue {
    fn drop(&mut self) { unsafe { ffi::pbsession_ReleaseValue(self.session.as_ptr(), self.ptr) } }
}

/// 参数值提取
pub trait FromValue<'val>: Sized {
    fn from_value(val: Option<Value<'val>>) -> Result<Self>;
}

impl FromValue<'_> for () {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(_) = val {
            Ok(())
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for &'val PBStr {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_str()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for PBString {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_string()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for String {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_string()?.map(|v| v.to_string_lossy()).ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbint {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_int()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbuint {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_uint()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pblong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_long()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbulong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_ulong()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pblonglong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_longlong()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbdouble {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_double()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbreal {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_real()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for Decimal {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_dec()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveDate {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_date()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveTime {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_time()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveDateTime {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_datetime()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbbyte {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_byte()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for bool {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_bool()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for &'val [u8] {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_blob()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for Vec<u8> {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_blob()?.ok_or(PBXRESULT::E_VALUE_IS_NULL).map(Vec::from)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for Object<'val> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_object()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for Array<'val> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_array()?.ok_or(PBXRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
impl<'val, T: UserObject> FromValue<'val> for &'val T {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            if let Some(obj) = val.try_get_object()? {
                Ok(unsafe { obj.get_native_ref()? })
            } else {
                Err(PBXRESULT::E_VALUE_IS_NULL)
            }
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
impl<'val, T: UserObject> FromValue<'val> for &'val mut T {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            if let Some(mut obj) = val.try_get_object()? {
                Ok(unsafe { obj.get_native_mut()? })
            } else {
                Err(PBXRESULT::E_VALUE_IS_NULL)
            }
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}

impl<'val> FromValue<'val> for Value<'val> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            Ok(val)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for OwnedValue {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            Ok(val.acquire())
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}

impl<'val, T: FromValue<'val>> FromValue<'val> for Option<T> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        T::from_value(val).map(Some).or_else(|e| {
            if e == PBXRESULT::E_INVOKE_WRONG_NUM_ARGS || e == PBXRESULT::E_VALUE_IS_NULL {
                Ok(None)
            } else {
                Err(e)
            }
        })
    }
}

/// 参数值提取,无引用版本
pub trait FromValueOwned: Sized + for<'val> FromValue<'val> {}

impl<T> FromValueOwned for T where T: for<'val> FromValue<'val> {}

/// 设置参数值
pub trait ToValue: Sized {
    fn to_value(self, val: &mut Value) -> Result<()>;
}

impl ToValue for () {
    fn to_value(self, _: &mut Value) -> Result<()> { Ok(()) }
}
impl<T: AsPBStr> ToValue for T {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_str(self) }
}
impl ToValue for pbint {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_int(self) }
}
impl ToValue for pbuint {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_uint(self) }
}
impl ToValue for pblong {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_long(self) }
}
impl ToValue for pbulong {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_ulong(self) }
}
impl ToValue for pblonglong {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_longlong(self) }
}
impl ToValue for pbdouble {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_double(self) }
}
impl ToValue for pbreal {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_real(self) }
}
impl ToValue for Decimal {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_dec(self) }
}
impl ToValue for NaiveDate {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_date(self) }
}
impl ToValue for NaiveTime {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_time(self) }
}
impl ToValue for NaiveDateTime {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_datetime(self) }
}
impl ToValue for pbbyte {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_byte(self) }
}
impl ToValue for bool {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_bool(self) }
}
impl ToValue for &[u8] {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_blob(self) }
}
impl ToValue for Vec<u8> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_blob(self.as_slice()) }
}
impl ToValue for Object<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_object(&self) }
}
impl ToValue for Array<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_array(&self) }
}
impl ToValue for Value<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_value(&self) }
}
impl ToValue for OwnedValue {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_value(&self.value()) }
}

impl<T: ToValue> ToValue for Option<T> {
    fn to_value(self, val: &mut Value) -> Result<()> {
        if let Some(v) = self {
            T::to_value(v, val)
        } else {
            val.set_to_null();
            Ok(())
        }
    }
}
impl<T: ToValue> ToValue for Result<T> {
    fn to_value(self, val: &mut Value) -> Result<()> { T::to_value(self?, val) }
}
