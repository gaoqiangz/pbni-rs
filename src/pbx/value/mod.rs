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

    /// 获取`int`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_int(&self) -> Option<pbint> { self.try_get_int().unwrap() }

    /// 尝试获取`int`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_int(&self) -> Result<Option<pbint>> {
        if self.get_type() == ValueType::Int {
            unsafe { Ok(self.get_int_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`int`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_int_unchecked(&self) -> Option<pbint> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetInt(self.ptr))
        }
    }

    /// 获取`uint`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_uint(&self) -> Option<pbuint> { self.try_get_uint().unwrap() }

    /// 尝试获取`uint`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_uint(&self) -> Result<Option<pbuint>> {
        if self.get_type() == ValueType::Uint {
            unsafe { Ok(self.get_uint_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`uint`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_uint_unchecked(&self) -> Option<pbuint> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetUint(self.ptr))
        }
    }

    /// 获取`long`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_long(&self) -> Option<pblong> { self.try_get_long().unwrap() }

    /// 尝试获取`long`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_long(&self) -> Result<Option<pblong>> {
        if self.get_type() == ValueType::Long {
            unsafe { Ok(self.get_long_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`long`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_long_unchecked(&self) -> Option<pblong> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetLong(self.ptr))
        }
    }

    /// 获取`ulong`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_ulong(&self) -> Option<pbulong> { self.try_get_ulong().unwrap() }

    /// 尝试获取`ulong`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_ulong(&self) -> Result<Option<pbulong>> {
        if self.get_type() == ValueType::Ulong {
            unsafe { Ok(self.get_ulong_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`ulong`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_ulong_unchecked(&self) -> Option<pbulong> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetUlong(self.ptr))
        }
    }

    /// 获取`longlong`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_longlong(&self) -> Option<pblonglong> { self.try_get_longlong().unwrap() }

    /// 尝试获取`longlong`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_longlong(&self) -> Result<Option<pblonglong>> {
        if self.get_type() == ValueType::LongLong {
            unsafe { Ok(self.get_longlong_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`longlong`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_longlong_unchecked(&self) -> Option<pblonglong> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetLongLong(self.ptr))
        }
    }

    /// 获取`real`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_real(&self) -> Option<pbreal> { self.try_get_real().unwrap() }

    /// 尝试获取`real`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_real(&self) -> Result<Option<pbreal>> {
        if self.get_type() == ValueType::Real {
            unsafe { Ok(self.get_real_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`real`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_real_unchecked(&self) -> Option<pbreal> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetReal(self.ptr))
        }
    }

    /// 获取`double`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_double(&self) -> Option<pbdouble> { self.try_get_double().unwrap() }

    /// 尝试获取`double`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_double(&self) -> Result<Option<pbdouble>> {
        if self.get_type() == ValueType::Double {
            unsafe { Ok(self.get_double_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`double`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_double_unchecked(&self) -> Option<pbdouble> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetDouble(self.ptr))
        }
    }

    /// 获取`decimal`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn get_dec(&self) -> Option<Decimal> { self.try_get_dec().unwrap() }

    /// 尝试获取`decimal`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    #[cfg(feature = "decimal")]
    pub fn try_get_dec(&self) -> Result<Option<Decimal>> {
        if self.get_type() == ValueType::Decimal {
            unsafe { Ok(self.get_dec_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`decimal`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn get_dec_unchecked(&self) -> Option<Decimal> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_dec_unchecked(ffi::pbvalue_GetDecimal(self.ptr)))
        }
    }

    /// 获取`boolean`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_bool(&self) -> Option<bool> { self.try_get_bool().unwrap() }

    /// 尝试获取`boolean`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_bool(&self) -> Result<Option<bool>> {
        if self.get_type() == ValueType::Boolean {
            unsafe { Ok(self.get_bool_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`boolean`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_bool_unchecked(&self) -> Option<bool> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetBool(self.ptr).into())
        }
    }

    /// 获取`byte`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_byte(&self) -> Option<pbbyte> { self.try_get_byte().unwrap() }

    /// 尝试获取`byte`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_byte(&self) -> Result<Option<pbbyte>> {
        if self.get_type() == ValueType::Byte {
            unsafe { Ok(self.get_byte_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`byte`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_byte_unchecked(&self) -> Option<pbbyte> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetByte(self.ptr))
        }
    }

    /// 获取`char`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_char(&self) -> Option<PBChar> { self.try_get_char().unwrap() }

    /// 尝试获取`char`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_char(&self) -> Result<Option<PBChar>> {
        if self.get_type() == ValueType::Char {
            unsafe { Ok(self.get_char_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`char`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_char_unchecked(&self) -> Option<PBChar> {
        if self.is_null() {
            None
        } else {
            Some(ffi::pbvalue_GetChar(self.ptr))
        }
    }

    /// 获取`string`类型值的引用
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_str(&self) -> Option<&'val PBStr> { self.try_get_str().unwrap() }

    /// 尝试获取`string`类型值的引用
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub unsafe fn try_get_str(&self) -> Result<Option<&'val PBStr>> {
        if self.get_type() == ValueType::String {
            Ok(self.get_str_unchecked())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`char`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_str_unchecked(&self) -> Option<&'val PBStr> {
        if self.is_null() {
            None
        } else {
            self.session.get_string_unchecked(ffi::pbvalue_GetString(self.ptr))
        }
    }

    /// 获取`string`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_string(&self) -> Option<PBString> { self.try_get_string().unwrap() }

    /// 尝试获取`string`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_string(&self) -> Result<Option<PBString>> {
        if self.get_type() == ValueType::String {
            unsafe { Ok(self.get_string_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`date`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_date(&self) -> Option<NaiveDate> { self.try_get_date().unwrap() }

    /// 尝试获取`date`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    #[cfg(feature = "datetime")]
    pub fn try_get_date(&self) -> Result<Option<NaiveDate>> {
        if self.get_type() == ValueType::Date {
            unsafe { Ok(self.get_date_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`date`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_date_unchecked(&self) -> Option<NaiveDate> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_date_unchecked(ffi::pbvalue_GetDate(self.ptr)))
        }
    }

    /// 获取`time`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_time(&self) -> Option<NaiveTime> { self.try_get_time().unwrap() }

    /// 尝试获取`time`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    #[cfg(feature = "datetime")]
    pub fn try_get_time(&self) -> Result<Option<NaiveTime>> {
        if self.get_type() == ValueType::Time {
            unsafe { Ok(self.get_time_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`time`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_time_unchecked(&self) -> Option<NaiveTime> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_time_unchecked(ffi::pbvalue_GetTime(self.ptr)))
        }
    }

    /// 获取`datetime`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_datetime(&self) -> Option<NaiveDateTime> { self.try_get_datetime().unwrap() }

    /// 尝试获取`datetime`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    #[cfg(feature = "datetime")]
    pub fn try_get_datetime(&self) -> Result<Option<NaiveDateTime>> {
        if self.get_type() == ValueType::DateTime {
            unsafe { Ok(self.get_datetime_unchecked()) }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`datetime`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_datetime_unchecked(&self) -> Option<NaiveDateTime> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_datetime_unchecked(ffi::pbvalue_GetDateTime(self.ptr)))
        }
    }

    /// 获取`string`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_string_unchecked(&self) -> Option<PBString> {
        self.get_str_unchecked().map(PBStr::to_ucstring)
    }

    /// 获取对象类型值的引用
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_object(&self) -> Option<Object<'val>> { self.try_get_object().unwrap() }

    /// 尝试获取对象类型值的引用
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub unsafe fn try_get_object(&self) -> Result<Option<Object<'val>>> {
        if self.is_object() {
            Ok(self.get_object_unchecked())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取对象类型值,不检查类型
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
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
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_array(&self) -> Option<Array<'val>> { self.try_get_array().unwrap() }

    /// 尝试获取数组类型值的引用
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub unsafe fn try_get_array(&self) -> Result<Option<Array<'val>>> {
        if self.is_array() {
            Ok(self.get_array_unchecked())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取数组类型值,不检查类型
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_array_unchecked(&self) -> Option<Array<'val>> {
        if self.is_null() {
            None
        } else {
            Some(Array::from_ptr(ffi::pbvalue_GetArray(self.ptr), self.is_object(), self.session.clone()))
        }
    }

    /// 获取`blob`类型值的引用
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_blob(&self) -> Option<&'val [u8]> { self.try_get_blob().unwrap() }

    /// 尝试获取`blob`类型值的引用
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub unsafe fn try_get_blob(&self) -> Result<Option<&'val [u8]>> {
        if self.get_type() == ValueType::Blob {
            Ok(self.get_blob_unchecked())
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取数组`blob`值,不检查类型
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_blob_unchecked(&self) -> Option<&'val [u8]> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_blob_unchecked(ffi::pbvalue_GetBlob(self.ptr)))
        }
    }

    /// 设置值为NULL
    pub fn set_to_null(&mut self) -> Result<()> { unsafe { ffi::pbvalue_SetToNull(self.ptr).into() } }

    /// 设置`int`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_int(&mut self, v: pbint) -> Result<()> {
        if self.get_type() == ValueType::Int {
            unsafe { ffi::pbvalue_SetInt(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`int`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_int_unchecked(&mut self, v: pbint) -> Result<()> {
        ffi::pbvalue_SetInt(self.ptr, v).into()
    }

    /// 设置`uint`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_uint(&mut self, v: pbuint) -> Result<()> {
        if self.get_type() == ValueType::Uint {
            unsafe { ffi::pbvalue_SetUint(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`uint`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_uint_unchecked(&mut self, v: pbuint) -> Result<()> {
        ffi::pbvalue_SetUint(self.ptr, v).into()
    }

    /// 设置`long`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_long(&mut self, v: pblong) -> Result<()> {
        if self.get_type() == ValueType::Long {
            unsafe { ffi::pbvalue_SetLong(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`long`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_long_unchecked(&mut self, v: pblong) -> Result<()> {
        ffi::pbvalue_SetLong(self.ptr, v).into()
    }

    /// 设置`ulong`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_ulong(&mut self, v: pbulong) -> Result<()> {
        if self.get_type() == ValueType::Ulong {
            unsafe { ffi::pbvalue_SetUlong(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`ulong`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_ulong_unchecked(&mut self, v: pbulong) -> Result<()> {
        ffi::pbvalue_SetUlong(self.ptr, v).into()
    }

    /// 设置`longlong`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_longlong(&mut self, v: pblonglong) -> Result<()> {
        if self.get_type() == ValueType::LongLong {
            unsafe { ffi::pbvalue_SetLongLong(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`longlong`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_longlong_unchecked(&mut self, v: pblonglong) -> Result<()> {
        ffi::pbvalue_SetLongLong(self.ptr, v).into()
    }

    /// 设置`real`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_real(&mut self, v: pbreal) -> Result<()> {
        if self.get_type() == ValueType::Real {
            unsafe { ffi::pbvalue_SetReal(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`real`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_real_unchecked(&mut self, v: pbreal) -> Result<()> {
        ffi::pbvalue_SetReal(self.ptr, v).into()
    }

    /// 设置`double`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_double(&mut self, v: pbdouble) -> Result<()> {
        if self.get_type() == ValueType::Double {
            unsafe { ffi::pbvalue_SetDouble(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`double`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_double_unchecked(&mut self, v: pbdouble) -> Result<()> {
        ffi::pbvalue_SetDouble(self.ptr, v).into()
    }

    /// 设置`decimal`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    #[cfg(feature = "decimal")]
    pub fn set_dec(&mut self, v: Decimal) -> Result<()> {
        unsafe {
            if self.get_type() == ValueType::Decimal {
                ffi::pbvalue_SetDecimal(self.ptr, self.session.new_pbdec(v)).into()
            } else {
                Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
            }
        }
    }

    /// 设置`decimal`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn set_dec_unchecked(&mut self, v: Decimal) -> Result<()> {
        ffi::pbvalue_SetDecimal(self.ptr, self.session.new_pbdec(v)).into()
    }

    /// 设置`boolean`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_bool(&mut self, v: bool) -> Result<()> {
        if self.get_type() == ValueType::Boolean {
            unsafe { ffi::pbvalue_SetBool(self.ptr, v.into()).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`boolean`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_bool_unchecked(&mut self, v: bool) -> Result<()> {
        ffi::pbvalue_SetBool(self.ptr, v.into()).into()
    }

    /// 设置`byte`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_byte(&mut self, v: pbbyte) -> Result<()> {
        if self.get_type() == ValueType::Byte {
            unsafe { ffi::pbvalue_SetByte(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`byte`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_byte_unchecked(&mut self, v: pbbyte) -> Result<()> {
        ffi::pbvalue_SetByte(self.ptr, v).into()
    }

    /// 设置`char`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_char(&mut self, v: PBChar) -> Result<()> {
        if self.get_type() == ValueType::Char {
            unsafe { ffi::pbvalue_SetChar(self.ptr, v).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`char`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_char_unchecked(&mut self, v: PBChar) -> Result<()> {
        ffi::pbvalue_SetChar(self.ptr, v).into()
    }

    /// 设置`string`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_str(&mut self, v: impl AsPBStr) -> Result<()> {
        if self.get_type() == ValueType::String {
            unsafe { ffi::pbvalue_SetString(self.ptr, v.as_pbstr().as_ptr()).into() }
        } else {
            Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`string`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_str_unchecked(&mut self, v: impl AsPBStr) -> Result<()> {
        ffi::pbvalue_SetString(self.ptr, v.as_pbstr().as_ptr()).into()
    }

    /// 设置`date`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    #[cfg(feature = "datetime")]
    pub fn set_date(&mut self, v: NaiveDate) -> Result<()> {
        unsafe {
            if self.get_type() == ValueType::Date {
                ffi::pbvalue_SetDate(self.ptr, self.session.new_pbdate(v)).into()
            } else {
                Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
            }
        }
    }

    /// 设置`date`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_date_unchecked(&mut self, v: NaiveDate) -> Result<()> {
        ffi::pbvalue_SetDate(self.ptr, self.session.new_pbdate(v)).into()
    }

    /// 设置`time`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    #[cfg(feature = "datetime")]
    pub fn set_time(&mut self, v: NaiveTime) -> Result<()> {
        unsafe {
            if self.get_type() == ValueType::Time {
                ffi::pbvalue_SetTime(self.ptr, self.session.new_pbtime(v)).into()
            } else {
                Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
            }
        }
    }

    /// 设置`time`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_time_unchecked(&mut self, v: NaiveTime) -> Result<()> {
        ffi::pbvalue_SetTime(self.ptr, self.session.new_pbtime(v)).into()
    }

    /// 设置`datetime`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    #[cfg(feature = "datetime")]
    pub fn set_datetime(&mut self, v: NaiveDateTime) -> Result<()> {
        unsafe {
            if self.get_type() == ValueType::DateTime {
                ffi::pbvalue_SetDateTime(self.ptr, self.session.new_pbdatetime(v)).into()
            } else {
                Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
            }
        }
    }

    /// 设置`datetime`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_datetime_unchecked(&mut self, v: NaiveDateTime) -> Result<()> {
        ffi::pbvalue_SetDateTime(self.ptr, self.session.new_pbdatetime(v)).into()
    }

    /// 设置`blob`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_blob(&mut self, v: impl AsRef<[u8]>) -> Result<()> {
        unsafe {
            if self.get_type() == ValueType::Blob {
                ffi::pbvalue_SetBlob(self.ptr, self.session.new_pbblob(v)).into()
            } else {
                Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
            }
        }
    }

    /// 设置`blob`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_blob_unchecked(&mut self, v: impl AsRef<[u8]>) -> Result<()> {
        ffi::pbvalue_SetBlob(self.ptr, self.session.new_pbblob(v)).into()
    }

    /// 设置对象类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_object(&mut self, v: &Object) -> Result<()> {
        unsafe {
            if self.is_object() {
                ffi::pbvalue_SetObject(self.ptr, v.as_ptr()).into()
            } else {
                Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
            }
        }
    }

    /// 设置对象类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_object_unchecked(&mut self, v: &Object) -> Result<()> {
        ffi::pbvalue_SetObject(self.ptr, v.as_ptr()).into()
    }

    /// 设置数组类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_array(&mut self, v: &Array) -> Result<()> {
        unsafe {
            if self.is_array() {
                ffi::pbvalue_SetArray(self.ptr, v.as_ptr()).into()
            } else {
                Err(PBXRESULT::E_MISMATCHED_DATA_TYPE)
            }
        }
    }

    /// 设置数组类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_array_unchecked(&mut self, v: &Array) -> Result<()> {
        ffi::pbvalue_SetArray(self.ptr, v.as_ptr()).into()
    }

    /// 从`src`参数拷贝并覆盖现有值
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_value(&mut self, src: &Value) {
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
            unsafe { val.try_get_str()?.ok_or(PBXRESULT::E_NULL_ERROR) }
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for PBString {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_string()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for String {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_string()?.map(|v| v.to_string_lossy()).ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbint {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_int()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbuint {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_uint()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pblong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_long()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbulong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_ulong()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pblonglong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_longlong()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbdouble {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_double()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbreal {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_real()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
#[cfg(feature = "decimal")]
impl FromValue<'_> for Decimal {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_dec()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
#[cfg(feature = "datetime")]
impl FromValue<'_> for NaiveDate {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_date()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
#[cfg(feature = "datetime")]
impl FromValue<'_> for NaiveTime {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_time()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
#[cfg(feature = "datetime")]
impl FromValue<'_> for NaiveDateTime {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_datetime()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbbyte {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_byte()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for bool {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_bool()?.ok_or(PBXRESULT::E_NULL_ERROR)
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for &'val [u8] {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            unsafe { val.try_get_blob()?.ok_or(PBXRESULT::E_NULL_ERROR) }
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for Vec<u8> {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            unsafe { val.try_get_blob()?.ok_or(PBXRESULT::E_NULL_ERROR).map(Vec::from) }
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for Object<'val> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            unsafe { val.try_get_object()?.ok_or(PBXRESULT::E_NULL_ERROR) }
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for Array<'val> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            unsafe { val.try_get_array()?.ok_or(PBXRESULT::E_NULL_ERROR) }
        } else {
            Err(PBXRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
impl<'val, T: UserObject> FromValue<'val> for &'val T {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            if let Some(obj) = unsafe { val.try_get_object()? } {
                Ok(unsafe { obj.get_native_ref()? })
            } else {
                Err(PBXRESULT::E_NULL_ERROR)
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
            if let Some(mut obj) = unsafe { val.try_get_object()? } {
                Ok(unsafe { obj.get_native_mut()? })
            } else {
                Err(PBXRESULT::E_NULL_ERROR)
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
            if e == PBXRESULT::E_INVOKE_WRONG_NUM_ARGS || e == PBXRESULT::E_NULL_ERROR {
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
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_str(self) }
}
impl ToValue for pbint {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_int(self) }
}
impl ToValue for pbuint {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_uint(self) }
}
impl ToValue for pblong {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_long(self) }
}
impl ToValue for pbulong {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_ulong(self) }
}
impl ToValue for pblonglong {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_longlong(self) }
}
impl ToValue for pbdouble {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_double(self) }
}
impl ToValue for pbreal {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_real(self) }
}
#[cfg(feature = "decimal")]
impl ToValue for Decimal {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_dec(self) }
}
#[cfg(feature = "datetime")]
impl ToValue for NaiveDate {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_date(self) }
}
#[cfg(feature = "datetime")]
impl ToValue for NaiveTime {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_time(self) }
}
#[cfg(feature = "datetime")]
impl ToValue for NaiveDateTime {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_datetime(self) }
}
impl ToValue for pbbyte {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_byte(self) }
}
impl ToValue for bool {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_bool(self) }
}
impl ToValue for &[u8] {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_blob(self) }
}
impl ToValue for Vec<u8> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_blob(self.as_slice()) }
}
impl ToValue for Object<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_object(&self) }
}
impl ToValue for Array<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_array(&self) }
}
impl ToValue for Value<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> {
        unsafe { val.set_value(&self) };
        Ok(())
    }
}
impl ToValue for OwnedValue {
    fn to_value(self, val: &mut Value) -> Result<()> {
        unsafe { val.set_value(&self.value()) };
        Ok(())
    }
}

impl<T: ToValue> ToValue for Option<T> {
    fn to_value(self, val: &mut Value) -> Result<()> {
        if let Some(v) = self {
            T::to_value(v, val)
        } else {
            val.set_to_null()
        }
    }
}
impl<T: ToValue> ToValue for Result<T> {
    fn to_value(self, val: &mut Value) -> Result<()> { T::to_value(self?, val) }
}
