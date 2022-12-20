use crate::{
    prelude::*, syslib::{bindings::*, *}
};
use std::ops::{Deref, DerefMut};

mod array;
pub use array::Array;

/// 值的封装
pub struct Value<'val> {
    inner: ValueInner,
    session: Session,
    _marker: PhantomData<&'val mut OB_DATA>
}

impl<'val> Value<'val> {
    pub(crate) unsafe fn from_ptr(ptr: POB_DATA, session: Session) -> Value<'val> {
        let inner = ValueInner::from_ptr(ptr, session.as_ptr());
        Value {
            inner,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) unsafe fn new(session: Session) -> Value<'val> {
        let inner = ValueInner::new();
        Value {
            inner,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn as_ptr(&self) -> POB_DATA { &*self.inner as *const OB_DATA as _ }
    pub(crate) fn forget(mut self) { self.inner = ValueInner::LValue(ptr::null_mut()); }

    //pub(crate) fn get_class(&self) -> Option<pbclass> { unsafe { pbvalue_GetClass(self.ptr) } }

    /// 获取值的类型
    pub fn get_type(&self) -> ValueType { self.inner.type_.into() }

    /// 判断值是否为NULL
    pub fn is_null(&self) -> bool { self.inner.info & DATA_NULLVAL_MASK == 1 }

    /// 判断值的类型是否为对象
    pub fn is_object(&self) -> bool {
        self.get_type_kind() != OB_TYPE_KIND::OB_SIMPLE_TYPE &&
            self.get_info_style() == OB_DATASTYLE::PTR_STYLE
    }

    /// 判断值的类型是否为数组
    pub fn is_array(&self) -> bool { self.get_info_group() == OB_GROUPTYPE::OB_ARRAY }

    /// 判断值的类型是否为枚举
    pub fn is_enum(&self) -> bool {
        self.get_type_kind() != OB_TYPE_KIND::OB_SIMPLE_TYPE &&
            self.get_info_style() != OB_DATASTYLE::PTR_STYLE
    }

    /// 判断值是否为引用传递
    pub fn is_ref(&self) -> bool { self.get_info_reftype() == OB_REFTYPE::OB_ARGUMENT_REF }

    /// 判断值是否为只读传递
    pub fn is_readonly(&self) -> bool { self.get_info_reftype() == OB_REFTYPE::OB_ARGUMENT_READONLY }

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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_int(&self) -> Result<Option<pbint>> {
        if self.get_type() == ValueType::Int {
            unsafe { Ok(self.get_int_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.inner.val.int_val)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_uint(&self) -> Result<Option<pbuint>> {
        if self.get_type() == ValueType::Uint {
            unsafe { Ok(self.get_uint_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.inner.val.uint_val)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_long(&self) -> Result<Option<pblong>> {
        if self.get_type() == ValueType::Long {
            unsafe { Ok(self.get_long_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.inner.val.long_val)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_ulong(&self) -> Result<Option<pbulong>> {
        if self.get_type() == ValueType::Ulong {
            unsafe { Ok(self.get_ulong_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.inner.val.ulong_val)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_longlong(&self) -> Result<Option<pblonglong>> {
        if self.get_type() == ValueType::LongLong {
            unsafe { Ok(self.get_longlong_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            let val = self.inner.val.ptr as *const pblonglong;
            Some(*val)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_real(&self) -> Result<Option<pbreal>> {
        if self.get_type() == ValueType::Real {
            unsafe { Ok(self.get_real_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.inner.val.fl_val)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_double(&self) -> Result<Option<pbdouble>> {
        if self.get_type() == ValueType::Double {
            unsafe { Ok(self.get_double_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            let val = self.inner.val.ptr as *const pbdouble;
            Some(*val)
        }
    }

    /// 获取`decimal`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_dec(&self) -> Option<Decimal> { self.try_get_dec().unwrap() }

    /// 尝试获取`decimal`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_dec(&self) -> Result<Option<Decimal>> {
        if self.get_type() == ValueType::Decimal {
            unsafe { Ok(self.get_dec_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`decimal`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_dec_unchecked(&self) -> Option<Decimal> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_dec_unchecked(self.inner.val.ptr as _))
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_bool(&self) -> Result<Option<bool>> {
        if self.get_type() == ValueType::Boolean {
            unsafe { Ok(self.get_bool_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.inner.val.int_val != 0)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_byte(&self) -> Result<Option<pbbyte>> {
        if self.get_type() == ValueType::Byte {
            unsafe { Ok(self.get_byte_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.inner.val.byte_val)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_char(&self) -> Result<Option<PBChar>> {
        if self.get_type() == ValueType::Char {
            unsafe { Ok(self.get_char_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.inner.val.int_val as _)
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub unsafe fn try_get_str(&self) -> Result<Option<&'val PBStr>> {
        if self.get_type() == ValueType::String {
            Ok(self.get_str_unchecked())
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(PBStr::from_ptr_str(self.inner.val.ptr as _))
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_string(&self) -> Result<Option<PBString>> {
        if self.get_type() == ValueType::String {
            unsafe { Ok(self.get_string_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`date`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_date(&self) -> Option<NaiveDate> { self.try_get_date().unwrap() }

    /// 尝试获取`date`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_date(&self) -> Result<Option<NaiveDate>> {
        if self.get_type() == ValueType::Date {
            unsafe { Ok(self.get_date_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`date`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_date_unchecked(&self) -> Option<NaiveDate> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_date_unchecked(self.inner.val.ptr as _))
        }
    }

    /// 获取`time`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_time(&self) -> Option<NaiveTime> { self.try_get_time().unwrap() }

    /// 尝试获取`time`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_time(&self) -> Result<Option<NaiveTime>> {
        if self.get_type() == ValueType::Time {
            unsafe { Ok(self.get_time_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`time`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_time_unchecked(&self) -> Option<NaiveTime> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_time_unchecked(self.inner.val.ptr as _))
        }
    }

    /// 获取`datetime`类型值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_datetime(&self) -> Option<NaiveDateTime> { self.try_get_datetime().unwrap() }

    /// 尝试获取`datetime`类型值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn try_get_datetime(&self) -> Result<Option<NaiveDateTime>> {
        if self.get_type() == ValueType::DateTime {
            unsafe { Ok(self.get_datetime_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 获取`datetime`类型值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_datetime_unchecked(&self) -> Option<NaiveDateTime> {
        if self.is_null() {
            None
        } else {
            Some(self.session.get_datetime_unchecked(self.inner.val.ptr as _))
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

    /*
    TODO
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub unsafe fn try_get_object(&self) -> Result<Option<Object<'val>>> {
        if self.is_object() {
            Ok(self.get_object_unchecked())
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(Object::from_ptr(pbvalue_GetObject(self.ptr), self.obthis.clone()))
        }
    }
    */

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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub unsafe fn try_get_array(&self) -> Result<Option<Array<'val>>> {
        if self.is_array() {
            Ok(self.get_array_unchecked())
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(Array::from_ptr(self.inner.val.ptr as _, self.is_object(), self.session.clone()))
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
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub unsafe fn try_get_blob(&self) -> Result<Option<&'val [u8]>> {
        if self.get_type() == ValueType::Blob {
            Ok(self.get_blob_unchecked())
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(self.session.get_blob_unchecked(self.inner.val.ptr as _))
        }
    }

    /// 设置值为NULL
    pub fn set_to_null(&mut self) -> Result<()> {
        self.set_info_flag(1, DATA_NULLVAL_SHIFT, DATA_NULLVAL_MASK);
        Ok(())
    }

    /// 设置`int`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_int(&mut self, v: pbint) -> Result<()> {
        if matches!(self.get_type(), ValueType::Int | ValueType::NoType) {
            unsafe { self.set_int_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`int`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_int_unchecked(&mut self, v: pbint) -> Result<()> {
        self.free_val_ptr();
        self.inner.val.int_val = v;
        self.set_info(OB_DATASTYLE::INT_STYLE, INT_TYPE, OB_GROUPTYPE::OB_SIMPLE);
        Ok(())
    }

    /// 设置`uint`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_uint(&mut self, v: pbuint) -> Result<()> {
        if matches!(self.get_type(), ValueType::Uint | ValueType::NoType) {
            unsafe { self.set_uint_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`uint`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_uint_unchecked(&mut self, v: pbuint) -> Result<()> {
        self.free_val_ptr();
        self.inner.val.uint_val = v;
        self.set_info(OB_DATASTYLE::INT_STYLE, UINT_TYPE, OB_GROUPTYPE::OB_SIMPLE);
        Ok(())
    }

    /// 设置`long`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_long(&mut self, v: pblong) -> Result<()> {
        if matches!(self.get_type(), ValueType::Long | ValueType::NoType) {
            unsafe { self.set_long_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`long`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_long_unchecked(&mut self, v: pblong) -> Result<()> {
        self.free_val_ptr();
        self.inner.val.long_val = v;
        self.set_info(OB_DATASTYLE::LONG_STYLE, LONG_TYPE, OB_GROUPTYPE::OB_SIMPLE);
        Ok(())
    }

    /// 设置`ulong`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_ulong(&mut self, v: pbulong) -> Result<()> {
        if matches!(self.get_type(), ValueType::Ulong | ValueType::NoType) {
            unsafe { self.set_ulong_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`ulong`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_ulong_unchecked(&mut self, v: pbulong) -> Result<()> {
        self.free_val_ptr();
        self.inner.val.ulong_val = v;
        self.set_info(OB_DATASTYLE::LONG_STYLE, ULONG_TYPE, OB_GROUPTYPE::OB_SIMPLE);
        Ok(())
    }

    /// 设置`longlong`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_longlong(&mut self, v: pblonglong) -> Result<()> {
        if matches!(self.get_type(), ValueType::LongLong | ValueType::NoType) {
            unsafe { self.set_longlong_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`longlong`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_longlong_unchecked(&mut self, v: pblonglong) -> Result<()> {
        let v = API.ob_dup_longlong(self.session.as_ptr(), &v as *const pblonglong as _);
        self.set_ptr(v as _, LONGLONG_TYPE);
        Ok(())
    }

    /// 设置`real`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_real(&mut self, v: pbreal) -> Result<()> {
        if matches!(self.get_type(), ValueType::Real | ValueType::NoType) {
            unsafe { self.set_real_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`real`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_real_unchecked(&mut self, v: pbreal) -> Result<()> {
        self.free_val_ptr();
        self.inner.val.fl_val = v;
        self.set_info(OB_DATASTYLE::FLOAT_STYLE, FLOAT_TYPE, OB_GROUPTYPE::OB_SIMPLE);
        Ok(())
    }

    /// 设置`double`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_double(&mut self, v: pbdouble) -> Result<()> {
        if matches!(self.get_type(), ValueType::Double | ValueType::NoType) {
            unsafe { self.set_double_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`double`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_double_unchecked(&mut self, v: pbdouble) -> Result<()> {
        let v = API.ob_dup_double(self.session.as_ptr(), &v as *const pbdouble as _);
        self.set_ptr(v as _, DOUBLE_TYPE);
        Ok(())
    }

    /// 设置`decimal`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_dec(&mut self, v: Decimal) -> Result<()> {
        if matches!(self.get_type(), ValueType::Decimal | ValueType::NoType) {
            unsafe { self.set_dec_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`decimal`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_dec_unchecked(&mut self, v: Decimal) -> Result<()> {
        self.set_ptr(self.session.new_pbdec(v) as _, DEC_TYPE);
        Ok(())
    }

    /// 设置`boolean`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_bool(&mut self, v: bool) -> Result<()> {
        if matches!(self.get_type(), ValueType::Boolean | ValueType::NoType) {
            unsafe { self.set_bool_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`boolean`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_bool_unchecked(&mut self, v: bool) -> Result<()> {
        self.free_val_ptr();
        self.inner.val.int_val = v as _;
        self.set_info(OB_DATASTYLE::INT_STYLE, BOOL_TYPE, OB_GROUPTYPE::OB_SIMPLE);
        Ok(())
    }

    /// 设置`byte`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_byte(&mut self, v: pbbyte) -> Result<()> {
        if matches!(self.get_type(), ValueType::Byte | ValueType::NoType) {
            unsafe { self.set_byte_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`byte`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_byte_unchecked(&mut self, v: pbbyte) -> Result<()> {
        self.free_val_ptr();
        self.inner.val.int_val = v as _;
        self.set_info(OB_DATASTYLE::INT_STYLE, BYTE_TYPE, OB_GROUPTYPE::OB_SIMPLE);
        Ok(())
    }

    /// 设置`char`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_char(&mut self, v: PBChar) -> Result<()> {
        if matches!(self.get_type(), ValueType::Char | ValueType::NoType) {
            unsafe { self.set_char_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`char`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_char_unchecked(&mut self, v: PBChar) -> Result<()> {
        self.free_val_ptr();
        self.inner.val.uint_val = v;
        self.set_info(OB_DATASTYLE::INT_STYLE, CHAR_TYPE, OB_GROUPTYPE::OB_SIMPLE);
        Ok(())
    }

    /// 设置`string`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_str(&mut self, v: impl AsPBStr) -> Result<()> {
        if matches!(self.get_type(), ValueType::String | ValueType::NoType) {
            unsafe { self.set_str_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`string`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_str_unchecked(&mut self, v: impl AsPBStr) -> Result<()> {
        let v = v.as_pbstr();
        let v = API.ob_dup_string(self.session.as_ptr(), v.as_ptr() as _);
        self.set_ptr(v as _, STRING_TYPE);
        Ok(())
    }

    /// 设置`date`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_date(&mut self, v: NaiveDate) -> Result<()> {
        if matches!(self.get_type(), ValueType::Date | ValueType::NoType) {
            unsafe { self.set_date_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`date`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_date_unchecked(&mut self, v: NaiveDate) -> Result<()> {
        self.set_ptr(self.session.new_pbdate(v) as _, DATE_TYPE);
        Ok(())
    }

    /// 设置`time`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_time(&mut self, v: NaiveTime) -> Result<()> {
        if matches!(self.get_type(), ValueType::Time | ValueType::NoType) {
            unsafe { self.set_time_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`time`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_time_unchecked(&mut self, v: NaiveTime) -> Result<()> {
        self.set_ptr(self.session.new_pbtime(v) as _, TIME_TYPE);
        Ok(())
    }

    /// 设置`datetime`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_datetime(&mut self, v: NaiveDateTime) -> Result<()> {
        if matches!(self.get_type(), ValueType::DateTime | ValueType::NoType) {
            unsafe { self.set_datetime_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`datetime`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_datetime_unchecked(&mut self, v: NaiveDateTime) -> Result<()> {
        self.set_ptr(self.session.new_pbdatetime(v) as _, DATETIME_TYPE);
        Ok(())
    }

    /// 设置`blob`类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_blob(&mut self, v: impl AsRef<[u8]>) -> Result<()> {
        if matches!(self.get_type(), ValueType::Blob | ValueType::NoType) {
            unsafe { self.set_blob_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置`blob`类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_blob_unchecked(&mut self, v: impl AsRef<[u8]>) -> Result<()> {
        let v = v.as_ref();
        if v.is_empty() {
            return Err(PBRESULT::E_OUTOF_MEMORY);
        }
        self.set_ptr(self.session.new_pbblob(v) as _, BINARY_TYPE);
        Ok(())
    }
    /*
    TODO
    /// 设置对象类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_object(&mut self, v: &Object) -> Result<()> {
        if self.is_object() {
            unsafe { self.set_object_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置对象类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_object_unchecked(&mut self, v: &Object) -> Result<()> {
        //TODO
        Ok(())
    }
    */

    /// 设置数组类型的值
    ///
    /// # Returns
    ///
    /// 类型不匹配时返回`Err(PBRESULT::E_MISMATCHED_DATA_TYPE)`
    pub fn set_array(&mut self, v: &Array) -> Result<()> {
        if self.is_array() {
            unsafe { self.set_array_unchecked(v) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置数组类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_array_unchecked(&mut self, v: &Array) -> Result<()> {
        self.set_ptr(
            API.ot_copy_array(self.session.as_ptr(), v.as_ptr() as _) as _,
            v.info().value_type() as OB_CLASS_ID
        );
        self.set_info_group(OB_GROUPTYPE::OB_ARRAY);
        Ok(())
    }

    /// 从`src`参数拷贝并覆盖现有值
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_value_unchecked(&mut self, src: &Value) {
        API.rtDataCopy(self.session.as_ptr(), self.as_ptr(), src.as_ptr(), true as BOOL);
    }

    /*
    TODO
    /// 拷贝并转移所有权,`self`将被消耗
    pub fn acquire(self) -> OwnedValue {
        unsafe {
            let new_value = pbobthis_AcquireValue(self.obthis.as_ptr(), self.ptr);
            OwnedValue::from_ptr(new_value, self.obthis.clone())
        }
    }
    */
}

impl<'val> Value<'val> {
    #[inline(always)]
    fn get_info_flag(&self, shift: u32, mask: OB_INFO_FLAGS) -> OB_INFO_FLAGS {
        bitfield!(@get self.inner.info,shift,mask)
    }

    #[inline(always)]
    fn get_this_info_flag(&self, shift: u32, mask: OB_INFO_FLAGS) -> OB_INFO_FLAGS {
        bitfield!(@get self.inner.this().info,shift,mask)
    }

    #[inline(always)]
    fn get_info_typeargs(&self) -> UINT { self.get_info_flag(DATA_TYPEARGS_SHIFT, DATA_TYPEARGS_MASK) as _ }

    #[inline(always)]
    fn get_info_reftype(&self) -> OB_REFTYPE {
        unsafe { mem::transmute(self.get_this_info_flag(DATA_REFTYPE_SHIFT, DATA_REFTYPE_MASK) as i32) }
    }

    #[inline(always)]
    fn get_info_status(&self) -> OB_STATUS {
        unsafe { mem::transmute(self.get_info_flag(DATA_STATUS_SHIFT, DATA_STATUS_MASK) as i32) }
    }

    #[inline(always)]
    fn get_info_fieldtype(&self) -> OB_FIELD_TYPE {
        unsafe { mem::transmute(self.get_info_flag(DATA_FIELDTYPE_SHIFT, DATA_FIELDTYPE_MASK) as i32) }
    }

    #[inline(always)]
    fn get_info_style(&self) -> OB_DATASTYLE {
        unsafe { mem::transmute(self.get_info_flag(DATA_STYLE_SHIFT, DATA_STYLE_MASK) as i32) }
    }

    #[inline(always)]
    fn get_info_group(&self) -> OB_GROUPTYPE {
        unsafe { mem::transmute(self.get_info_flag(DATA_GROUP_SHIFT, DATA_GROUP_MASK) as i32) }
    }

    #[inline(always)]
    fn get_info_access(&self) -> OB_MEMBER_ACCESS {
        unsafe { mem::transmute(self.get_info_flag(DATA_ACCESS_SHIFT, DATA_ACCESS_MASK) as i32) }
    }

    #[inline(always)]
    fn get_type_kind(&self) -> OB_TYPE_KIND {
        unsafe {
            mem::transmute(
                bitfield!(@get self.inner.type_,TYPE_KIND_SHIFT,TYPE_KIND_MASK as OB_CLASS_ID) as i32
            )
        }
    }

    #[inline(always)]
    fn set_info_flag(&mut self, value: OB_INFO_FLAGS, shift: u32, mask: OB_INFO_FLAGS) {
        self.inner.info = bitfield!(@modify self.inner.info,value,shift,mask);
    }

    #[inline(always)]
    fn set_info_group(&mut self, group: OB_GROUPTYPE) {
        self.set_info_flag(group as OB_INFO_FLAGS, DATA_GROUP_SHIFT, DATA_GROUP_MASK);
    }

    #[inline(always)]
    fn set_info(&mut self, style: OB_DATASTYLE, typ: OB_CLASS_ID, group: OB_GROUPTYPE) {
        self.inner.info = (((OB_MEMBER_ACCESS::OB_PUBLIC_MEMBER as OB_INFO_FLAGS) << DATA_ACCESS_SHIFT) |
            ((group as OB_INFO_FLAGS) << DATA_GROUP_SHIFT) |
            (0 << DATA_FIELDTYPE_SHIFT) |
            ((style as OB_INFO_FLAGS) << DATA_STYLE_SHIFT) |
            ((OB_STATUS::USED as OB_INFO_FLAGS) << DATA_STATUS_SHIFT) |
            ((OB_REFTYPE::OB_DIRECT_REF as OB_INFO_FLAGS) << DATA_REFTYPE_SHIFT) |
            (0 << DATA_TYPEARGS_SHIFT)) as OB_INFO_FLAGS;
        self.inner.type_ = typ;
    }

    #[inline(always)]
    fn set_ptr(&mut self, val: LPVOID, typ: OB_CLASS_ID) {
        self.free_val_ptr();
        self.inner.val.ptr = val;
        self.set_info(OB_DATASTYLE::PTR_STYLE, typ, OB_GROUPTYPE::OB_SIMPLE);
    }

    #[inline(always)]
    fn free_val_ptr(&mut self) {
        unsafe {
            if self.get_info_group() == OB_GROUPTYPE::OB_ARRAY {
                API.ot_free_array(self.session.as_ptr(), self.as_ptr());
            } else {
                API.ot_free_val_ptr(self.session.as_ptr(), self.as_ptr());
            }
        }
    }
}

impl Drop for Value<'_> {
    fn drop(&mut self) {
        if matches!(self.inner, ValueInner::Owned(_)) {
            self.free_val_ptr();
        }
    }
}

enum ValueInner {
    LValue(POB_DATA),
    RefValue(POB_DATA /* this */, POB_DATA /* ref */),
    Owned(OB_DATA)
}

impl ValueInner {
    unsafe fn new() -> ValueInner { ValueInner::Owned(MaybeUninit::zeroed().assume_init()) }
    unsafe fn from_ptr(ptr: POB_DATA, obthis: POB_THIS) -> ValueInner {
        let data = &*ptr;
        if bitfield!(@get data.info,DATA_REFTYPE_SHIFT, DATA_REFTYPE_MASK) == OB_REFTYPE::OB_ARGUMENT_REF as _
        {
            let refpak = &*(data.val.ptr as POT_REF_PAK);
            let refval = match refpak.style {
                OT_REFPAK_STYLE::OT_SIMPLE_REF => refpak.ref_.simple.lvalue,
                OT_REFPAK_STYLE::OT_FIELD_REF => {
                    API.ot_get_field_lv(obthis, refpak.ref_.field.obinst, refpak.ref_.field.field_id)
                },
                OT_REFPAK_STYLE::OT_FIELD_ITEM_REF => {
                    API.ot_get_field_item_lv(
                        obthis,
                        refpak.ref_.field.obinst,
                        refpak.ref_.field.field_id,
                        refpak.ref_.field.item_index
                    )
                },
            };
            ValueInner::RefValue(ptr, refval)
        } else {
            ValueInner::LValue(ptr)
        }
    }

    /// 获取此值的直接引用
    #[inline(always)]
    fn this(&self) -> &OB_DATA {
        match self {
            ValueInner::LValue(ptr) => unsafe { &**ptr },
            ValueInner::RefValue(this_ptr, _) => unsafe { &**this_ptr },
            ValueInner::Owned(data) => data
        }
    }

    /// 获取此值的直接可变引用
    #[inline(always)]
    fn this_mut(&mut self) -> &mut OB_DATA {
        match self {
            ValueInner::LValue(ptr) => unsafe { &mut **ptr },
            ValueInner::RefValue(this_ptr, _) => unsafe { &mut **this_ptr },
            ValueInner::Owned(data) => data
        }
    }
}

impl Deref for ValueInner {
    type Target = OB_DATA;
    fn deref(&self) -> &Self::Target {
        match self {
            ValueInner::LValue(ptr) => unsafe { &**ptr },
            ValueInner::RefValue(_, ref_ptr) => unsafe { &**ref_ptr },
            ValueInner::Owned(data) => data
        }
    }
}

impl DerefMut for ValueInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ValueInner::LValue(ptr) => unsafe { &mut **ptr },
            ValueInner::RefValue(_, ref_ptr) => unsafe { &mut **ref_ptr },
            ValueInner::Owned(data) => data
        }
    }
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
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for &'val PBStr {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            unsafe { val.try_get_str()?.ok_or(PBRESULT::E_VALUE_IS_NULL) }
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for PBString {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_string()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for String {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_string()?.map(|v| v.to_string_lossy()).ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbint {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_int()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbuint {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_uint()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pblong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_long()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbulong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_ulong()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pblonglong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_longlong()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbdouble {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_double()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbreal {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_real()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for Decimal {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_dec()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveDate {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_date()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveTime {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_time()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveDateTime {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_datetime()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbbyte {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_byte()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for bool {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_bool()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}

impl<'val> FromValue<'val> for &'val [u8] {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            unsafe { val.try_get_blob()?.ok_or(PBRESULT::E_VALUE_IS_NULL) }
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for Vec<u8> {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            unsafe { val.try_get_blob()?.ok_or(PBRESULT::E_VALUE_IS_NULL).map(Vec::from) }
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
/*
TODO
impl<'val> FromValue<'val> for Object<'val> {
fn from_value(val: Option<Value<'val>>) -> Result<Self> {
if let Some(val) = val {
unsafe { val.try_get_object()?.ok_or(PBRESULT::E_NULL_ERROR) }
} else {
Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
}
}
}
*/
impl<'val> FromValue<'val> for Array<'val> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            unsafe { val.try_get_array()?.ok_or(PBRESULT::E_VALUE_IS_NULL) }
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
/*
TODO
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
impl<'val, T: UserObject> FromValue<'val> for &'val T {
fn from_value(val: Option<Value<'val>>) -> Result<Self> {
if let Some(val) = val {
if let Some(obj) = unsafe { val.try_get_object()? } {
Ok(unsafe { obj.get_native_ref()? })
} else {
Err(PBRESULT::E_NULL_ERROR)
}
} else {
Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
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
Err(PBRESULT::E_NULL_ERROR)
}
} else {
Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
}
}
}
*/
impl<'val> FromValue<'val> for Value<'val> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            Ok(val)
        } else {
            Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
        }
    }
}
/*
TODO
impl FromValue<'_> for OwnedValue {
fn from_value(val: Option<Value>) -> Result<Self> {
if let Some(val) = val {
Ok(val.acquire())
} else {
Err(PBRESULT::E_INVOKE_WRONG_NUM_ARGS)
}
}
}
*/
impl<'val, T: FromValue<'val>> FromValue<'val> for Option<T> {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        T::from_value(val).map(Some).or_else(|e| {
            if e == PBRESULT::E_INVOKE_WRONG_NUM_ARGS || e == PBRESULT::E_VALUE_IS_NULL {
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
impl ToValue for Decimal {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_dec(self) }
}
impl ToValue for NaiveDate {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_date(self) }
}
impl ToValue for NaiveTime {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_time(self) }
}
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
/*
TODO
impl ToValue for Object<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_object(&self) }
}
*/
impl ToValue for Array<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.set_array(&self) }
}
impl ToValue for Value<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> {
        unsafe { val.set_value_unchecked(&self) };
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
