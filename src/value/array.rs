use crate::{bindings::*, *};

/// 数组的引用
///
/// # Parameters
///
/// 数组函数的`dims`参数可指定不同维度的索引: `&[dim_1_index,dim_2_index,...]`
pub struct Array<'arr> {
    ptr: pbarray,
    info: ArrayInfo,
    is_object: bool,
    session: Session,
    _marker: PhantomData<&'arr pbarray>
}

impl<'arr> Array<'arr> {
    pub(crate) unsafe fn from_ptr(ptr: pbarray, is_object: bool, session: Session) -> Array<'arr> {
        let info = ArrayInfo::new(ptr, session.clone());
        Array {
            ptr,
            info,
            is_object,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn as_ptr(&self) -> pbarray { self.ptr }

    #[inline]
    fn check_get_index(&self, dim: &[pblong]) {
        if dim.is_empty() {
            panic!("invalid index");
        }
        if !self.info.bounded() {
            if dim.len() > 1 {
                panic!("invalid index dimensions {} [1]", dim.len());
            }
            if dim[0] < 1 || dim[0] > self.len() {
                panic!("index {} is out of bound [{},{}]", dim[0], 1, self.len());
            }
        } else {
            if dim.len() != self.info.dimensions() as usize {
                panic!("invalid index dimensions {} [{}]", dim.len(), self.info.dimensions());
            }
            for (dim, &idx) in dim.iter().enumerate() {
                let (lower, upper) = self.info.bound(dim as pbuint);
                if idx < lower || idx > upper {
                    panic!("index {} is out of bound [{},{}]", idx, lower, upper);
                }
            }
        }
    }

    #[inline]
    fn check_get(&self, dim: &[pblong], get_type: ValueType) {
        //get_item_type包含check_get_index
        let item_type = self.get_item_type(dim);
        assert!(
            item_type == get_type,
            "value type missmatched: item_type = {:?}, get_type = {:?}",
            item_type,
            get_type
        );
    }

    #[inline]
    fn check_set_index(&self, dim: &[pblong]) {
        if dim.is_empty() {
            panic!("invalid index");
        }
        if !self.info.bounded() {
            if dim.len() > 1 {
                panic!("invalid index dimensions {} [1]", dim.len());
            }
            if dim[0] < 1 {
                panic!("index {} is out of bound [{},{}]", dim[0], 1, self.len());
            }
        } else {
            if dim.len() != self.info.dimensions() as usize {
                panic!("invalid index dimensions {} [{}]", dim.len(), self.info.dimensions());
            }
            for (dim, &idx) in dim.iter().enumerate() {
                let (lower, upper) = self.info.bound(dim as pbuint);
                if idx < lower || idx > upper {
                    panic!("index {} is out of bound [{},{}]", idx, lower, upper);
                }
            }
        }
    }

    #[inline]
    fn check_set(&self, dim: &[pblong], set_type: ValueType) {
        self.check_set_index(dim);
        let item_type = self.info.value_type();
        assert!(
            item_type == set_type || item_type == ValueType::Any,
            "value type missmatched: item_type = {:?}, set_type = {:?}",
            item_type,
            set_type
        );
    }

    /// 获取数组信息
    pub fn info(&self) -> &ArrayInfo { &self.info }

    /// 获取数组长度(仅一维数组有效)
    pub fn len(&self) -> pblong { unsafe { ffi::pbsession_GetArrayLength(self.session.as_ptr(), self.ptr) } }

    /// 获取元素迭代器,仅支持一维数组
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// for item in arr.iter::<pbint>() {
    ///     println!("item: {:?}", item);
    /// }
    /// ```
    pub fn iter<'a, T: ArrayIterItem<'arr>>(&'a self) -> ArrayIter<'a, 'arr, T> {
        if self.info.dimensions() > 1 {
            panic!("invalid dimensions {}", self.info.dimensions());
        }
        assert!(T::check_type(self), "type mismatched");
        ArrayIter::new(self)
    }

    /// 获取元素类型
    pub fn get_item_type(&self, dim: &[pblong]) -> ValueType {
        self.check_get_index(dim);
        unsafe { ffi::pbsession_GetArrayItemType(self.session.as_ptr(), self.ptr, dim.as_ptr()) }
    }

    /// 判断元素是否为对象类型
    pub fn is_item_object(&self, _dim: &[pblong]) -> bool { self.is_object }

    /// 判断元素是否为NULL
    pub fn is_item_null(&self, dim: &[pblong]) -> bool {
        self.check_get_index(dim);
        unsafe { ffi::pbsession_IsArrayItemNull(self.session.as_ptr(), self.ptr, dim.as_ptr()).into() }
    }

    /// 获取`int`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_int(&self, dim: &[pblong]) -> Option<pbint> {
        self.check_get(dim, ValueType::Int);
        unsafe { self.get_item_int_unchecked(dim) }
    }

    /// 获取`int`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_int_unchecked(&self, dim: &[pblong]) -> Option<pbint> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetIntArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`uint`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_uint(&self, dim: &[pblong]) -> Option<pbuint> {
        self.check_get(dim, ValueType::Uint);
        unsafe { self.get_item_uint_unchecked(dim) }
    }

    /// 获取`uint`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_uint_unchecked(&self, dim: &[pblong]) -> Option<pbuint> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetUintArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`long`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_long(&self, dim: &[pblong]) -> Option<pblong> {
        self.check_get(dim, ValueType::Long);
        unsafe { self.get_item_long_unchecked(dim) }
    }

    /// 获取`long`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_long_unchecked(&self, dim: &[pblong]) -> Option<pblong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetLongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`ulong`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_ulong(&self, dim: &[pblong]) -> Option<pbulong> {
        self.check_get(dim, ValueType::Ulong);
        unsafe { self.get_item_ulong_unchecked(dim) }
    }

    /// 获取`ulong`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_ulong_unchecked(&self, dim: &[pblong]) -> Option<pbulong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetUlongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`longlong`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_longlong(&self, dim: &[pblong]) -> Option<pblonglong> {
        self.check_get(dim, ValueType::LongLong);
        unsafe { self.get_item_longlong_unchecked(dim) }
    }

    /// 获取`longlong`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_longlong_unchecked(&self, dim: &[pblong]) -> Option<pblonglong> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetLongLongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`real`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_real(&self, dim: &[pblong]) -> Option<pbreal> {
        self.check_get(dim, ValueType::Real);
        unsafe { self.get_item_real_unchecked(dim) }
    }

    /// 获取`real`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_real_unchecked(&self, dim: &[pblong]) -> Option<pbreal> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetRealArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`double`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_double(&self, dim: &[pblong]) -> Option<pbdouble> {
        self.check_get(dim, ValueType::Double);
        unsafe { self.get_item_double_unchecked(dim) }
    }

    /// 获取`double`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_double_unchecked(&self, dim: &[pblong]) -> Option<pbdouble> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetDoubleArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`decimal`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn get_item_dec(&self, dim: &[pblong]) -> Option<Decimal> {
        self.check_get(dim, ValueType::Decimal);
        unsafe { self.get_item_dec_unchecked(dim) }
    }

    /// 获取`decimal`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn get_item_dec_unchecked(&self, dim: &[pblong]) -> Option<Decimal> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDecArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_dec_unchecked(v))
        }
    }

    /// 获取`string`类型元素值
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
    pub unsafe fn get_item_str(&self, dim: &[pblong]) -> Option<&'arr PBStr> {
        self.check_get(dim, ValueType::String);
        self.get_item_str_unchecked(dim)
    }

    /// 获取`string`类型元素值的引用,不检查类型
    ///
    /// # Safety
    ///
    /// - 索引越界或类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_item_str_unchecked(&self, dim: &[pblong]) -> Option<&'arr PBStr> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetStringArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            self.session.get_string_unchecked(v)
        }
    }

    /// 获取`string`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_string(&self, dim: &[pblong]) -> Option<PBString> {
        self.check_get(dim, ValueType::String);
        unsafe { self.get_item_string_unchecked(dim) }
    }

    /// 获取`string`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_string_unchecked(&self, dim: &[pblong]) -> Option<PBString> {
        self.get_item_str_unchecked(dim).map(|v| v.to_ucstring())
    }

    /// 获取`boolean`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_bool(&self, dim: &[pblong]) -> Option<bool> {
        self.check_get(dim, ValueType::Boolean);
        unsafe { self.get_item_bool_unchecked(dim) }
    }

    /// 获取`boolean`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_bool_unchecked(&self, dim: &[pblong]) -> Option<bool> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetBoolArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v.into())
        }
    }

    /// 获取`char`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_char(&self, dim: &[pblong]) -> Option<PBChar> {
        self.check_get(dim, ValueType::Char);
        unsafe { self.get_item_char_unchecked(dim) }
    }

    /// 获取`char`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_char_unchecked(&self, dim: &[pblong]) -> Option<PBChar> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetCharArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`byte`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_item_byte(&self, dim: &[pblong]) -> Option<pbbyte> {
        self.check_get(dim, ValueType::Byte);
        unsafe { self.get_item_byte_unchecked(dim) }
    }

    /// 获取`byte`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_byte_unchecked(&self, dim: &[pblong]) -> Option<pbbyte> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetByteArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`byte`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_item_date(&self, dim: &[pblong]) -> Option<NaiveDate> {
        self.check_get(dim, ValueType::Date);
        unsafe { self.get_item_date_unchecked(dim) }
    }

    /// 获取`date`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_item_date_unchecked(&self, dim: &[pblong]) -> Option<NaiveDate> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDateArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_date_unchecked(v))
        }
    }

    /// 获取`time`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_item_time(&self, dim: &[pblong]) -> Option<NaiveTime> {
        self.check_get(dim, ValueType::Time);
        unsafe { self.get_item_time_unchecked(dim) }
    }

    /// 获取`time`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_item_time_unchecked(&self, dim: &[pblong]) -> Option<NaiveTime> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetTimeArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_time_unchecked(v))
        }
    }

    /// 获取`datetime`类型元素值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_item_datetime(&self, dim: &[pblong]) -> Option<NaiveDateTime> {
        self.check_get(dim, ValueType::DateTime);
        unsafe { self.get_item_datetime_unchecked(dim) }
    }

    /// 获取`datetime`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_item_datetime_unchecked(&self, dim: &[pblong]) -> Option<NaiveDateTime> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetDateTimeArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_datetime_unchecked(v))
        }
    }

    /// 获取`blob`类型元素值的引用
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
    pub fn get_item_blob(&self, dim: &[pblong]) -> Option<&'arr [u8]> {
        self.check_get(dim, ValueType::Blob);
        unsafe { self.get_item_blob_unchecked(dim) }
    }

    /// 获取`blob`类型元素值的引用,不检查类型
    ///
    /// # Safety
    ///
    /// - 索引越界或类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_item_blob_unchecked(&self, dim: &[pblong]) -> Option<&'arr [u8]> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetBlobArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.session.get_blob_unchecked(v))
        }
    }

    /// 获取对象类型元素值的引用
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
    pub fn get_item_object(&self, dim: &[pblong]) -> Option<Object<'arr>> {
        assert!(self.is_item_object(dim));
        unsafe { self.get_item_object_unchecked(dim) }
    }

    /// 获取对象类型元素值的引用,不检查类型
    ///
    /// # Safety
    ///
    /// - 索引越界或类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_item_object_unchecked(&self, dim: &[pblong]) -> Option<Object<'arr>> {
        let mut is_null = Default::default();
        let v =
            ffi::pbsession_GetObjectArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Object::from_ptr(v, self.session.clone()))
        }
    }

    /// 获取`any`类型元素值的引用
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
    pub fn get_item_any(&self, dim: &[pblong]) -> Option<Value<'arr>> {
        self.check_get(dim, ValueType::Any);
        unsafe { self.get_item_any_unchecked(dim) }
    }

    /// 获取`any`类型元素值的引用,不检查类型
    ///
    /// # Safety
    ///
    /// - 索引越界或类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_item_any_unchecked(&self, dim: &[pblong]) -> Option<Value<'arr>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetPBAnyArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Value::from_ptr(v, self.session.clone()))
        }
    }

    /// 设置`int`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_int(&mut self, dim: &[pblong], value: pbint) -> Result<()> {
        self.check_set(dim, ValueType::Int);
        unsafe { ffi::pbsession_SetIntArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into() }
    }

    /// 设置`int`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_int_unchecked(&mut self, dim: &[pblong], value: pbint) -> Result<()> {
        ffi::pbsession_SetIntArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`uint`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_uint(&mut self, dim: &[pblong], value: pbuint) -> Result<()> {
        self.check_set(dim, ValueType::Uint);
        unsafe {
            ffi::pbsession_SetUintArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
        }
    }

    /// 设置`uint`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_uint_unchecked(&mut self, dim: &[pblong], value: pbuint) -> Result<()> {
        ffi::pbsession_SetUintArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`long`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_long(&mut self, dim: &[pblong], value: pblong) -> Result<()> {
        self.check_set(dim, ValueType::Long);
        unsafe {
            ffi::pbsession_SetLongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
        }
    }

    /// 设置`long`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_long_unchecked(&mut self, dim: &[pblong], value: pblong) -> Result<()> {
        ffi::pbsession_SetLongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`ulong`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_ulong(&mut self, dim: &[pblong], value: pbulong) -> Result<()> {
        self.check_set(dim, ValueType::Ulong);
        unsafe {
            ffi::pbsession_SetUlongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
        }
    }

    /// 设置`ulong`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_ulong_unchecked(&mut self, dim: &[pblong], value: pbulong) -> Result<()> {
        ffi::pbsession_SetUlongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`longlong`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_longlong(&mut self, dim: &[pblong], value: pblonglong) -> Result<()> {
        self.check_set(dim, ValueType::LongLong);
        unsafe {
            ffi::pbsession_SetLongLongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
        }
    }

    /// 设置`longlong`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_longlong_unchecked(&mut self, dim: &[pblong], value: pblonglong) -> Result<()> {
        ffi::pbsession_SetLongLongArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`real`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_real(&mut self, dim: &[pblong], value: pbreal) -> Result<()> {
        self.check_set(dim, ValueType::Real);
        unsafe {
            ffi::pbsession_SetRealArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
        }
    }

    /// 设置`real`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_real_unchecked(&mut self, dim: &[pblong], value: pbreal) -> Result<()> {
        ffi::pbsession_SetRealArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`double`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_double(&mut self, dim: &[pblong], value: pbdouble) -> Result<()> {
        self.check_set(dim, ValueType::Double);
        unsafe {
            ffi::pbsession_SetDoubleArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
        }
    }

    /// 设置`double`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_double_unchecked(&mut self, dim: &[pblong], value: pbdouble) -> Result<()> {
        ffi::pbsession_SetDoubleArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`decimal`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn set_item_dec(&mut self, dim: &[pblong], value: Decimal) -> Result<()> {
        self.check_set(dim, ValueType::Decimal);
        unsafe {
            ffi::pbsession_SetDecArrayItem(
                self.session.as_ptr(),
                self.ptr,
                dim.as_ptr(),
                self.session.new_pbdec(value)
            )
            .into()
        }
    }

    /// 设置`decimal`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn set_item_dec_unchecked(&mut self, dim: &[pblong], value: Decimal) -> Result<()> {
        ffi::pbsession_SetDecArrayItem(
            self.session.as_ptr(),
            self.ptr,
            dim.as_ptr(),
            self.session.new_pbdec(value)
        )
        .into()
    }

    /// 设置`string`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_str(&mut self, dim: &[pblong], value: impl AsPBStr) -> Result<()> {
        self.check_set(dim, ValueType::String);
        unsafe {
            ffi::pbsession_SetStringArrayItem(
                self.session.as_ptr(),
                self.ptr,
                dim.as_ptr(),
                value.as_pbstr().as_ptr()
            )
            .into()
        }
    }

    /// 设置`string`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_str_unchecked(&mut self, dim: &[pblong], value: impl AsPBStr) -> Result<()> {
        ffi::pbsession_SetStringArrayItem(
            self.session.as_ptr(),
            self.ptr,
            dim.as_ptr(),
            value.as_pbstr().as_ptr()
        )
        .into()
    }

    /// 设置`boolean`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_bool(&mut self, dim: &[pblong], value: bool) -> Result<()> {
        self.check_set(dim, ValueType::Boolean);
        unsafe {
            ffi::pbsession_SetBoolArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.into())
                .into()
        }
    }

    /// 设置`boolean`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_bool_unchecked(&mut self, dim: &[pblong], value: bool) -> Result<()> {
        ffi::pbsession_SetBoolArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.into()).into()
    }

    /// 设置`date`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_item_date(&mut self, dim: &[pblong], value: NaiveDate) -> Result<()> {
        self.check_set(dim, ValueType::Date);
        unsafe {
            ffi::pbsession_SetDateArrayItem(
                self.session.as_ptr(),
                self.ptr,
                dim.as_ptr(),
                self.session.new_pbdate(value)
            )
            .into()
        }
    }

    /// 设置`date`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_item_date_unchecked(&mut self, dim: &[pblong], value: NaiveDate) -> Result<()> {
        ffi::pbsession_SetDateArrayItem(
            self.session.as_ptr(),
            self.ptr,
            dim.as_ptr(),
            self.session.new_pbdate(value)
        )
        .into()
    }

    /// 设置`time`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_item_time(&mut self, dim: &[pblong], value: NaiveTime) -> Result<()> {
        self.check_set(dim, ValueType::Time);
        unsafe {
            ffi::pbsession_SetTimeArrayItem(
                self.session.as_ptr(),
                self.ptr,
                dim.as_ptr(),
                self.session.new_pbtime(value)
            )
            .into()
        }
    }

    /// 设置`time`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_item_time_unchecked(&mut self, dim: &[pblong], value: NaiveTime) -> Result<()> {
        ffi::pbsession_SetTimeArrayItem(
            self.session.as_ptr(),
            self.ptr,
            dim.as_ptr(),
            self.session.new_pbtime(value)
        )
        .into()
    }

    /// 设置`datetime`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_item_datetime(&mut self, dim: &[pblong], value: NaiveDateTime) -> Result<()> {
        self.check_set(dim, ValueType::DateTime);
        unsafe {
            ffi::pbsession_SetDateTimeArrayItem(
                self.session.as_ptr(),
                self.ptr,
                dim.as_ptr(),
                self.session.new_pbdatetime(value)
            )
            .into()
        }
    }

    /// 设置`datetime`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_item_datetime_unchecked(&mut self, dim: &[pblong], value: NaiveDateTime) -> Result<()> {
        ffi::pbsession_SetDateTimeArrayItem(
            self.session.as_ptr(),
            self.ptr,
            dim.as_ptr(),
            self.session.new_pbdatetime(value)
        )
        .into()
    }

    /// 设置`char`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_char(&mut self, dim: &[pblong], value: PBChar) -> Result<()> {
        self.check_set(dim, ValueType::Char);
        unsafe {
            ffi::pbsession_SetCharArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
        }
    }

    /// 设置`char`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_char_unchecked(&mut self, dim: &[pblong], value: PBChar) -> Result<()> {
        ffi::pbsession_SetCharArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`byte`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_byte(&mut self, dim: &[pblong], value: pbbyte) -> Result<()> {
        self.check_set(dim, ValueType::Byte);
        unsafe {
            ffi::pbsession_SetByteArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
        }
    }

    /// 设置`byte`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_byte_unchecked(&mut self, dim: &[pblong], value: pbbyte) -> Result<()> {
        ffi::pbsession_SetByteArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value).into()
    }

    /// 设置`blob`类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_blob(&mut self, dim: &[pblong], value: impl AsRef<[u8]>) -> Result<()> {
        self.check_set(dim, ValueType::Blob);
        unsafe { self.set_item_blob_unchecked(dim, value) }
    }

    /// 设置`blob`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_blob_unchecked(&mut self, dim: &[pblong], value: impl AsRef<[u8]>) -> Result<()> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(PBXRESULT::E_OUTOF_MEMORY);
        }
        ffi::pbsession_SetBlobArrayItem(
            self.session.as_ptr(),
            self.ptr,
            dim.as_ptr(),
            self.session.new_pbblob(value)
        )
        .into()
    }

    /// 设置对象类型元素的值
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_object(&mut self, dim: &[pblong], value: &Object) -> Result<()> {
        assert!(self.is_item_object(dim) || self.info.value_type() == ValueType::Any);
        self.check_set_index(dim);
        unsafe { self.set_item_object_unchecked(dim, value) }
    }

    /// 设置对象类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_object_unchecked(&mut self, dim: &[pblong], value: &Object) -> Result<()> {
        ffi::pbsession_SetObjectArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.as_ptr())
            .into()
    }

    /// 设置`any`类型元素的值,`value`参数将被消耗
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn set_item_any(&mut self, dim: &[pblong], value: Value) {
        self.check_set(dim, ValueType::Any);
        unsafe {
            ffi::pbsession_SetArrayItemValue(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.as_ptr())
        }
    }

    /// 设置`any`类型元素的值,`value`参数将被消耗
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_any_unchecked(&mut self, dim: &[pblong], value: Value) {
        ffi::pbsession_SetArrayItemValue(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.as_ptr())
    }

    /// 设置元素为NULL
    pub fn set_item_to_null(&mut self, dim: &[pblong]) {
        self.check_set_index(dim);
        unsafe {
            ffi::pbsession_SetArrayItemToNull(self.session.as_ptr(), self.ptr, dim.as_ptr());
        }
    }

    /// 拷贝元素的值
    pub fn acquire_item_value(&mut self, dim: &[pblong]) -> OwnedValue {
        unsafe {
            let new_value =
                ffi::pbsession_AcquireArrayItemValue(self.session.as_ptr(), self.ptr, dim.as_ptr());
            OwnedValue::from_ptr(new_value, self.session.clone())
        }
    }
}

/// 数组信息
pub struct ArrayInfo {
    ptr: pbarrayinfo,
    session: Session
}

impl ArrayInfo {
    pub(crate) unsafe fn new(arr: pbarray, session: Session) -> ArrayInfo {
        let ptr = ffi::pbsession_GetArrayInfo(session.as_ptr(), arr);
        ArrayInfo {
            ptr,
            session
        }
    }

    pub(crate) fn item_group(&self) -> pbgroup { unsafe { self.ptr.as_ref().itemGroup } }

    /// 是否固定长度
    pub fn bounded(&self) -> bool { unsafe { self.ptr.as_ref().arrayType == ArrayType::BoundedArray } }

    /// 元素类型
    pub fn value_type(&self) -> ValueType { unsafe { self.ptr.as_ref().valueType } }

    /// 维度数量
    pub fn dimensions(&self) -> pbuint { unsafe { self.ptr.as_ref().numDimensions } }

    /// 获取指定维度的边界
    pub fn bound(&self, dim: pbuint) -> (pblong, pblong) {
        unsafe {
            assert!(dim < self.ptr.as_ref().numDimensions);
            let bound = self.ptr.as_ref().bounds.get_unchecked(dim as usize);
            (bound.lowerBound, bound.upperBound)
        }
    }
}

impl Drop for ArrayInfo {
    fn drop(&mut self) {
        unsafe {
            ffi::pbsession_ReleaseArrayInfo(self.session.as_ptr(), self.ptr);
        }
    }
}

/// 数组迭代器元素抽象
pub trait ArrayIterItem<'arr>: Sized {
    fn check_type(arr: &Array) -> bool;
    fn get_value(arr: &Array<'arr>, index: pblong) -> Option<Self>;
}

macro_rules! arr_iter_item {
    ($type: ty, $pbtype: expr, $fn: ident) => {
        impl<'arr> ArrayIterItem<'arr> for $type {
            fn check_type(arr: &Array) -> bool { arr.info.value_type() == $pbtype }
            fn get_value(arr: &Array<'arr>, index: pblong) -> Option<Self> { unsafe { arr.$fn(&[index]) } }
        }
    };
    ($type: ty, @object, $fn: ident) => {
        impl<'arr> ArrayIterItem<'arr> for $type {
            fn check_type(arr: &Array) -> bool { arr.is_object }
            fn get_value(arr: &Array<'arr>, index: pblong) -> Option<Self> { unsafe { arr.$fn(&[index]) } }
        }
    };
}

arr_iter_item!(pbint, ValueType::Int, get_item_int_unchecked);
arr_iter_item!(pbuint, ValueType::Uint, get_item_uint_unchecked);
arr_iter_item!(pblong, ValueType::Long, get_item_long_unchecked);
arr_iter_item!(pbulong, ValueType::Ulong, get_item_ulong_unchecked);
arr_iter_item!(pblonglong, ValueType::LongLong, get_item_longlong_unchecked);
arr_iter_item!(pbreal, ValueType::Real, get_item_real_unchecked);
arr_iter_item!(pbdouble, ValueType::Double, get_item_double_unchecked);
#[cfg(feature = "decimal")]
arr_iter_item!(Decimal, ValueType::Decimal, get_item_dec_unchecked);
#[cfg(feature = "datetime")]
arr_iter_item!(NaiveDate, ValueType::Date, get_item_date_unchecked);
#[cfg(feature = "datetime")]
arr_iter_item!(NaiveTime, ValueType::Time, get_item_time_unchecked);
#[cfg(feature = "datetime")]
arr_iter_item!(NaiveDateTime, ValueType::DateTime, get_item_datetime_unchecked);
arr_iter_item!(bool, ValueType::Boolean, get_item_bool_unchecked);
arr_iter_item!(pbbyte, ValueType::Byte, get_item_byte_unchecked);
//arr_iter_item!(PBChar, ValueType::Char, get_item_char_unchecked);
arr_iter_item!(&'arr PBStr, ValueType::String, get_item_str_unchecked);
arr_iter_item!(PBString, ValueType::String, get_item_string_unchecked);
arr_iter_item!(&'arr [u8], ValueType::Blob, get_item_blob_unchecked);
arr_iter_item!(Object<'arr>, @object, get_item_object_unchecked);
arr_iter_item!(Value<'arr>, ValueType::Any, get_item_any_unchecked);

/// 一维数组迭代器
///
/// # Examples
///
/// ```
/// for item in arr.iter::<pbint>() {
///     println!("item: {:?}", item);
/// }
/// ```
pub struct ArrayIter<'a, 'arr, T: ArrayIterItem<'arr>> {
    arr: &'a Array<'arr>,
    current: pblong,
    lower_bound: pblong,
    upper_bound: pblong,
    _marker: PhantomData<T>
}

impl<'a, 'arr, T: ArrayIterItem<'arr>> ArrayIter<'a, 'arr, T> {
    fn new(arr: &'a Array<'arr>) -> Self {
        let (lower_bound, upper_bound) = if arr.info.bounded() {
            arr.info.bound(0)
        } else {
            (1, arr.len())
        };
        ArrayIter {
            arr,
            current: lower_bound,
            lower_bound,
            upper_bound,
            _marker: PhantomData
        }
    }
}

impl<'a, 'arr, T: ArrayIterItem<'arr>> Iterator for ArrayIter<'a, 'arr, T> {
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.upper_bound {
            return None;
        }
        let current = self.current;
        self.current += 1;
        Some(T::get_value(self.arr, current))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.lower_bound as usize, Some(self.upper_bound as usize))
    }
}
