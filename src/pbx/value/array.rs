use crate::{
    pbx::{bindings::*, *}, prelude::*
};

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
    /// 索引越界或类型不匹配时会触发Panic
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
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn get_item_type(&self, dim: impl AsArrayIndex) -> ValueType {
        let dim = dim.as_array_index();
        self.check_get_index(dim);
        unsafe { ffi::pbsession_GetArrayItemType(self.session.as_ptr(), self.ptr, dim.as_ptr()) }
    }

    /// 判断元素是否为对象类型
    pub fn is_item_object(&self, _dim: impl AsArrayIndex) -> bool { self.is_object }

    /// 判断元素是否为NULL
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn is_item_null(&self, dim: impl AsArrayIndex) -> bool {
        let dim = dim.as_array_index();
        self.check_get_index(dim);
        unsafe { ffi::pbsession_IsArrayItemNull(self.session.as_ptr(), self.ptr, dim.as_ptr()).into() }
    }

    /// 获取`string`类型元素值
    ///
    /// # Panics
    ///
    /// 索引越界或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_item_str(&self, dim: impl AsArrayIndex) -> Option<&'arr PBStr> {
        let dim = dim.as_array_index();
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
    pub unsafe fn get_item_str_unchecked(&self, dim: impl AsArrayIndex) -> Option<&'arr PBStr> {
        let dim = dim.as_array_index();
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
    /// 索引越界或类型不匹配时会触发Panic
    pub fn get_item_string(&self, dim: impl AsArrayIndex) -> Option<PBString> {
        let dim = dim.as_array_index();
        self.check_get(dim, ValueType::String);
        unsafe { self.get_item_string_unchecked(dim) }
    }

    /// 获取`string`类型元素值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_string_unchecked(&self, dim: impl AsArrayIndex) -> Option<PBString> {
        let dim = dim.as_array_index();
        self.get_item_str_unchecked(dim).map(|v| v.to_ucstring())
    }

    /// 获取对象类型元素值的引用
    ///
    /// # Panics
    ///
    /// 索引越界或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub fn get_item_object(&self, dim: impl AsArrayIndex) -> Option<Object<'arr>> {
        let dim = dim.as_array_index();
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
    pub unsafe fn get_item_object_unchecked(&self, dim: impl AsArrayIndex) -> Option<Object<'arr>> {
        let dim = dim.as_array_index();
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
    /// 索引越界时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub fn get_item_any(&self, dim: impl AsArrayIndex) -> Option<Value<'arr>> {
        let dim = dim.as_array_index();
        self.check_get(dim, ValueType::Any);
        unsafe { self.get_item_any_unchecked(dim) }
    }

    /// 获取`any`类型元素值的引用,不检查类型
    ///
    /// # Safety
    ///
    /// - 索引越界时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_item_any_unchecked(&self, dim: impl AsArrayIndex) -> Option<Value<'arr>> {
        let dim = dim.as_array_index();
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetPBAnyArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Value::from_ptr(v, self.session.clone()))
        }
    }

    /// 设置`string`类型元素的值
    ///
    /// # Panics
    ///
    /// 索引越界或类型不匹配时会触发Panic
    pub fn set_item_str(&mut self, dim: impl AsArrayIndex, value: impl AsPBStr) -> Result<()> {
        let dim = dim.as_array_index();
        self.check_set(dim, ValueType::String);
        unsafe { self.set_item_str_unchecked(dim, value) }
    }

    /// 设置`string`类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_str_unchecked(
        &mut self,
        dim: impl AsArrayIndex,
        value: impl AsPBStr
    ) -> Result<()> {
        let dim = dim.as_array_index();
        ffi::pbsession_SetStringArrayItem(
            self.session.as_ptr(),
            self.ptr,
            dim.as_ptr(),
            value.as_pbstr().as_ptr()
        )
        .into()
    }

    /// 设置对象类型元素的值
    ///
    /// # Panics
    ///
    /// 索引越界或类型不匹配时会触发Panic
    pub fn set_item_object(&mut self, dim: impl AsArrayIndex, value: &Object) -> Result<()> {
        let dim = dim.as_array_index();
        assert!(self.is_item_object(dim) || self.info.value_type() == ValueType::Any);
        self.check_set_index(dim);
        unsafe { self.set_item_object_unchecked(dim, value) }
    }

    /// 设置对象类型元素的值,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_object_unchecked(&mut self, dim: impl AsArrayIndex, value: &Object) -> Result<()> {
        let dim = dim.as_array_index();
        ffi::pbsession_SetObjectArrayItem(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.as_ptr())
            .into()
    }

    /// 设置`any`类型元素的值,`value`参数将被消耗
    ///
    /// # Panics
    ///
    /// 索引越界或类型不兼容时会触发Panic
    pub fn set_item_any(&mut self, dim: impl AsArrayIndex, value: Value) {
        let dim = dim.as_array_index();
        self.check_set(dim, value.get_type());
        unsafe {
            ffi::pbsession_SetArrayItemValue(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.as_ptr())
        }
    }

    /// 设置`any`类型元素的值,`value`参数将被消耗
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_any_unchecked(&mut self, dim: impl AsArrayIndex, value: Value) {
        let dim = dim.as_array_index();
        ffi::pbsession_SetArrayItemValue(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.as_ptr())
    }

    /// 设置元素为NULL
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn set_item_to_null(&mut self, dim: impl AsArrayIndex) {
        let dim = dim.as_array_index();
        self.check_set_index(dim);
        unsafe {
            ffi::pbsession_SetArrayItemToNull(self.session.as_ptr(), self.ptr, dim.as_ptr());
        }
    }

    /// 拷贝元素的值
    pub fn acquire_item_value(&mut self, dim: impl AsArrayIndex) -> OwnedValue {
        let dim = dim.as_array_index();
        unsafe {
            let new_value =
                ffi::pbsession_AcquireArrayItemValue(self.session.as_ptr(), self.ptr, dim.as_ptr());
            OwnedValue::from_ptr(new_value, self.session.clone())
        }
    }
}

macro_rules! impl_array {
    (@simple
        $type: ty, $type_check: expr, $type_doc: literal,
        $getter: ident, $getter_unchecked: ident, $getter_ffi: ident,
        $setter: ident, $setter_unchecked: ident, $setter_ffi: ident
    ) => {
        impl_array!(
            @simple_getter
            $type, $type_check, $type_doc,
            $getter, $getter_unchecked, $getter_ffi
        );
        impl_array!(
            @simple_setter
            $type, $type_check, $type_doc,
            $setter, $setter_unchecked, $setter_ffi
        );
    };
    (@simple_getter
        $type: ty, $type_check: expr, $type_doc: literal,
        $getter: ident, $getter_unchecked: ident, $getter_ffi: ident
    ) => {
        #[doc = "获取"]
        #[doc = $type_doc]
        /// 类型元素值
        ///
        /// # Panics
        ///
        /// 索引越界或类型不匹配时会触发Panic
        pub fn $getter(&self, dim: impl AsArrayIndex) -> Option<$type> {
            let dim = dim.as_array_index();
            self.check_get(dim, $type_check);
            unsafe { self.$getter_unchecked(dim) }
        }

        #[doc = "获取"]
        #[doc = $type_doc]
        /// 类型元素值,不检查类型
        ///
        /// # Safety
        ///
        /// 索引越界或类型不兼容时可能会出现未定义行为
        pub unsafe fn $getter_unchecked(&self, dim: impl AsArrayIndex) -> Option<$type> {
            let dim = dim.as_array_index();
            let mut is_null = Default::default();
            let v = ffi::$getter_ffi(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
            if is_null == true {
                None
            } else {
                Some(v.into())
            }
        }
    };
    (@simple_setter
        $type: ty, $type_check: expr, $type_doc: literal,
        $setter: ident, $setter_unchecked: ident, $setter_ffi: ident
    ) => {
        #[doc = "设置"]
        #[doc = $type_doc]
        /// 类型元素的值
        ///
        /// # Panics
        ///
        /// 索引越界或类型不匹配时会触发Panic
        pub fn $setter(&mut self, dim: impl AsArrayIndex, value: $type) -> Result<()> {
            let dim = dim.as_array_index();
            self.check_set(dim, $type_check);
            unsafe { self.$setter_unchecked(dim, value) }
        }

        #[doc = "设置"]
        #[doc = $type_doc]
        /// 类型元素的值,不检查类型
        ///
        /// # Safety
        ///
        /// 索引越界或类型不兼容时可能会出现未定义行为
        pub unsafe fn $setter_unchecked(&mut self, dim: impl AsArrayIndex, value: $type) -> Result<()> {
            let dim = dim.as_array_index();
            ffi::$setter_ffi(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.into()).into()
        }
    };

    (@complex
        $type: ty, $type_check: expr, $type_doc: literal,
        $getter: ident, $getter_unchecked: ident, $getter_ffi: ident, $getter_acc: ident,
        $setter: ident, $setter_unchecked: ident, $setter_ffi: ident, $setter_new: ident
    ) => {
        impl_array!(
            @complex_getter
            $type, $type_check, $type_doc,
            $getter, $getter_unchecked, $getter_ffi, $getter_acc
        );
        impl_array!(
            @complex_setter
            $type, $type_check, $type_doc,
            $setter, $setter_unchecked, $setter_ffi, $setter_new
        );
    };
    (@complex_getter
        $type: ty, $type_check: expr, $type_doc: literal,
        $getter: ident, $getter_unchecked: ident, $getter_ffi: ident, $getter_acc: ident
    ) => {
        #[doc = "获取"]
        #[doc = $type_doc]
        /// 类型元素值
        ///
        /// # Panics
        ///
        /// 索引越界或类型不匹配时会触发Panic
        pub fn $getter(&self, dim: impl AsArrayIndex) -> Option<$type> {
            let dim = dim.as_array_index();
            self.check_get(dim, $type_check);
            unsafe { self.$getter_unchecked(dim) }
        }

        #[doc = "获取"]
        #[doc = $type_doc]
        /// 类型元素值,不检查类型
        ///
        /// # Safety
        ///
        /// 索引越界或类型不兼容时可能会出现未定义行为
        pub unsafe fn $getter_unchecked(&self, dim: impl AsArrayIndex) -> Option<$type> {
            let dim = dim.as_array_index();
            let mut is_null = Default::default();
            let v = ffi::$getter_ffi(self.session.as_ptr(), self.ptr, dim.as_ptr(), &mut is_null);
            if is_null == true {
                None
            } else {
                Some(self.session.$getter_acc(v))
            }
        }
    };
    (@complex_setter
        $type: ty, $type_check: expr, $type_doc: literal,
        $setter: ident, $setter_unchecked: ident, $setter_ffi: ident, $setter_new: ident
    ) => {
        #[doc = "设置"]
        #[doc = $type_doc]
        /// 类型元素的值
        ///
        /// # Panics
        ///
        /// 索引越界或类型不匹配时会触发Panic
        pub fn $setter(&mut self, dim: impl AsArrayIndex, value: $type) -> Result<()> {
            let dim = dim.as_array_index();
            self.check_set(dim, $type_check);
            unsafe { self.$setter_unchecked(dim, value) }
        }

        #[doc = "设置"]
        #[doc = $type_doc]
        /// 类型元素的值,不检查类型
        ///
        /// # Safety
        ///
        /// 索引越界或类型不兼容时可能会出现未定义行为
        pub unsafe fn $setter_unchecked(&mut self, dim: impl AsArrayIndex, value: $type) -> Result<()> {
            let dim = dim.as_array_index();
            ffi::$setter_ffi(self.session.as_ptr(), self.ptr, dim.as_ptr(), self.session.$setter_new(value))
                .into()
        }
    };
}

impl<'arr> Array<'arr> {
    impl_array!(
        @simple
        pbint, ValueType::Int, "int",
        get_item_int, get_item_int_unchecked, pbsession_GetIntArrayItem,
        set_item_int, set_item_int_unchecked, pbsession_SetIntArrayItem
    );
    impl_array!(
        @simple
        pbuint, ValueType::Uint, "uint",
        get_item_uint, get_item_uint_unchecked, pbsession_GetUintArrayItem,
        set_item_uint, set_item_uint_unchecked, pbsession_SetUintArrayItem
    );
    impl_array!(
        @simple
        pblong, ValueType::Long, "long",
        get_item_long, get_item_long_unchecked, pbsession_GetLongArrayItem,
        set_item_long, set_item_long_unchecked, pbsession_SetLongArrayItem
    );
    impl_array!(
        @simple
        pbulong, ValueType::Ulong, "ulong",
        get_item_ulong, get_item_ulong_unchecked, pbsession_GetUlongArrayItem,
        set_item_ulong, set_item_ulong_unchecked, pbsession_SetUlongArrayItem
    );
    impl_array!(
        @simple
        pblonglong, ValueType::LongLong, "longlong",
        get_item_longlong, get_item_longlong_unchecked, pbsession_GetLongLongArrayItem,
        set_item_longlong, set_item_longlong_unchecked, pbsession_SetLongLongArrayItem
    );
    impl_array!(
        @simple
        pbreal, ValueType::Real, "real",
        get_item_real, get_item_real_unchecked, pbsession_GetRealArrayItem,
        set_item_real, set_item_real_unchecked, pbsession_SetRealArrayItem
    );
    impl_array!(
        @simple
        pbdouble, ValueType::Double, "double",
        get_item_double, get_item_double_unchecked, pbsession_GetDoubleArrayItem,
        set_item_double, set_item_double_unchecked, pbsession_SetDoubleArrayItem
    );
    impl_array!(
        @simple
        pbbyte, ValueType::Byte, "byte",
        get_item_byte, get_item_byte_unchecked, pbsession_GetByteArrayItem,
        set_item_byte, set_item_byte_unchecked, pbsession_SetByteArrayItem
    );
    impl_array!(
        @simple
        bool, ValueType::Boolean, "boolean",
        get_item_bool, get_item_bool_unchecked, pbsession_GetBoolArrayItem,
        set_item_bool, set_item_bool_unchecked, pbsession_SetBoolArrayItem
    );
    impl_array!(
        @simple
        PBChar, ValueType::Char, "char",
        get_item_char, get_item_char_unchecked, pbsession_GetCharArrayItem,
        set_item_char, set_item_char_unchecked, pbsession_SetCharArrayItem
    );

    impl_array!(
        @complex
        Decimal, ValueType::Decimal, "decimal",
        get_item_dec, get_item_dec_unchecked, pbsession_GetDecArrayItem, get_dec_unchecked,
        set_item_dec, set_item_dec_unchecked, pbsession_SetDecArrayItem, new_pbdec
    );
    impl_array!(
        @complex
        NaiveDate, ValueType::Date, "date",
        get_item_date, get_item_date_unchecked, pbsession_GetDateArrayItem, get_date_unchecked,
        set_item_date, set_item_date_unchecked, pbsession_SetDateArrayItem, new_pbdate
    );
    impl_array!(
        @complex
        NaiveTime, ValueType::Time, "time",
        get_item_time, get_item_time_unchecked, pbsession_GetTimeArrayItem, get_time_unchecked,
        set_item_time, set_item_time_unchecked, pbsession_SetTimeArrayItem, new_pbtime
    );
    impl_array!(
        @complex
        NaiveDateTime, ValueType::DateTime, "datetime",
        get_item_datetime, get_item_datetime_unchecked, pbsession_GetDateTimeArrayItem, get_datetime_unchecked,
        set_item_datetime, set_item_datetime_unchecked, pbsession_SetDateTimeArrayItem, new_pbdatetime
    );
    impl_array!(
        @complex_getter
        &'arr [u8], ValueType::Blob, "blob",
        get_item_blob, get_item_blob_unchecked, pbsession_GetBlobArrayItem, get_blob_unchecked
    );
    impl_array!(
        @complex_setter
        &[u8], ValueType::Blob, "blob",
        set_item_blob, set_item_blob_unchecked, pbsession_SetBlobArrayItem, new_pbblob
    );
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
            assert!(dim > 0 && dim <= self.ptr.as_ref().numDimensions);
            let bound = self.ptr.as_ref().bounds.get_unchecked((dim - 1) as usize);
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

macro_rules! impl_iter_item {
    ($type: ty, $pbtype: expr, $fn: ident) => {
        impl<'arr> ArrayIterItem<'arr> for $type {
            fn check_type(arr: &Array) -> bool { arr.info.value_type() == $pbtype }
            fn get_value(arr: &Array<'arr>, index: pblong) -> Option<Self> { unsafe { arr.$fn(index) } }
        }
    };
    ($type: ty, @object, $fn: ident) => {
        impl<'arr> ArrayIterItem<'arr> for $type {
            fn check_type(arr: &Array) -> bool { arr.is_object }
            fn get_value(arr: &Array<'arr>, index: pblong) -> Option<Self> { unsafe { arr.$fn(index) } }
        }
    };
}

impl_iter_item!(pbint, ValueType::Int, get_item_int_unchecked);
impl_iter_item!(pbuint, ValueType::Uint, get_item_uint_unchecked);
impl_iter_item!(pblong, ValueType::Long, get_item_long_unchecked);
impl_iter_item!(pbulong, ValueType::Ulong, get_item_ulong_unchecked);
impl_iter_item!(pblonglong, ValueType::LongLong, get_item_longlong_unchecked);
impl_iter_item!(pbreal, ValueType::Real, get_item_real_unchecked);
impl_iter_item!(pbdouble, ValueType::Double, get_item_double_unchecked);
impl_iter_item!(Decimal, ValueType::Decimal, get_item_dec_unchecked);
impl_iter_item!(NaiveDate, ValueType::Date, get_item_date_unchecked);
impl_iter_item!(NaiveTime, ValueType::Time, get_item_time_unchecked);
impl_iter_item!(NaiveDateTime, ValueType::DateTime, get_item_datetime_unchecked);
impl_iter_item!(bool, ValueType::Boolean, get_item_bool_unchecked);
impl_iter_item!(pbbyte, ValueType::Byte, get_item_byte_unchecked);
impl_iter_item!(&'arr PBStr, ValueType::String, get_item_str_unchecked);
impl_iter_item!(PBString, ValueType::String, get_item_string_unchecked);
impl_iter_item!(&'arr [u8], ValueType::Blob, get_item_blob_unchecked);
impl_iter_item!(Object<'arr>, @object, get_item_object_unchecked);
impl_iter_item!(Value<'arr>, ValueType::Any, get_item_any_unchecked);

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
