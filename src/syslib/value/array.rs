use crate::{
    prelude::*, syslib::{bindings::*, *}
};

/// 数组的引用
///
/// # Parameters
///
/// 数组函数的`dims`参数可指定不同维度的索引: `&[dim_1_index,dim_2_index,...]`
pub struct Array<'arr> {
    ptr: OB_ARRAY_ID,
    info: ArrayInfo,
    is_object: bool,
    session: Session,
    _marker: PhantomData<&'arr OB_ARRAY_ID>
}

impl<'arr> Array<'arr> {
    pub(crate) unsafe fn from_ptr(ptr: OB_ARRAY_ID, is_object: bool, session: Session) -> Array<'arr> {
        let info = ArrayInfo::new(ptr, session.clone());
        Array {
            ptr,
            info,
            is_object,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn as_ptr(&self) -> OB_ARRAY_ID { self.ptr }

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
            item_type == get_type || get_type == ValueType::Any,
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
    pub fn len(&self) -> pblong {
        unsafe { API.ot_array_num_items(self.session.as_ptr(), self.ptr) as pblong }
    }

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
        //FIXME
        let item = self.get_item_value(dim);
        item.get_type()
    }

    /// 判断元素是否为对象类型
    pub fn is_item_object(&self, _dim: impl AsArrayIndex) -> bool { self.is_object }

    /// 判断元素是否为NULL
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn is_item_null(&self, dim: impl AsArrayIndex) -> bool {
        let item = self.get_item_value(dim);
        item.is_null()
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
        let item = self.get_item_value_unchecked(dim);
        item.get_str_unchecked()
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
        self.get_item_str_unchecked(dim).map(|v| v.to_ucstring())
    }

    /*
    TODO
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
        let val = self.get_item_value_unchecked(dim);
        val.get_object_unchecked()
    }
    */

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
    pub fn get_item_value(&self, dim: impl AsArrayIndex) -> Value<'arr> {
        let dim = dim.as_array_index();
        self.check_get(dim, ValueType::Any);
        unsafe { self.get_item_value_unchecked(dim) }
    }

    /// 获取`any`类型元素值的引用,不检查类型
    ///
    /// # Safety
    ///
    /// - 索引越界时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_item_value_unchecked(&self, dim: impl AsArrayIndex) -> Value<'arr> {
        let dim = dim.as_array_index();
        let idx = API.ob_array_get_index_from_subs(self.session.as_ptr(), self.ptr, dim.as_ptr() as _);
        let v = API.ot_array_index(self.session.as_ptr(), self.as_ptr(), idx);
        if v.is_null() {
            panic!("invalid item");
        }
        Value::from_ptr(v, self.session.clone())
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
        let mut item = self.get_item_value_unchecked(dim);
        item.set_str_unchecked(value)
    }

    /*
    TODO
    /// 设置对象类型元素的值
    ///
    /// # Panics
    ///
    /// 索引越界或类型不匹配时会触发Panic
    pub fn set_item_object(&mut self, dim: impl AsArrayIndex, value: &Object) -> Result<()> {
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
        ffi::set_Object_unchecked(self.session.as_ptr(), self.ptr, dim.as_ptr(), value.as_ptr())
            .into()
    }
    */
    /// 设置元素的值,`value`参数将被消耗
    ///
    /// # Panics
    ///
    /// 索引越界或类型不兼容时会触发Panic
    pub fn set_item_value(&mut self, dim: impl AsArrayIndex, value: Value) {
        let dim = dim.as_array_index();
        self.check_set(dim, value.get_type());
        unsafe { self.set_item_value_unchecked(dim, value) }
    }

    /// 设置元素的值,`value`参数将被消耗
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_value_unchecked(&mut self, dim: impl AsArrayIndex, value: Value) {
        let mut item = self.get_item_value_unchecked(dim);
        item.set_value_unchecked(&value)
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
            let mut item = self.get_item_value_unchecked(dim);
            item.set_to_null().unwrap();
        }
    }

    /*
    TODO
    /// 拷贝元素的值
    pub fn acquire_item_value(&mut self, dim: impl AsArrayIndex) -> OwnedValue {
        unsafe {
            let item = self.get_item_value_unchecked(dim);
            item.acquire()
        }
    }
    */
}

macro_rules! impl_array {
    (@all
        $type: ty, $type_check: expr, $type_doc: literal,
        $getter: ident, $getter_unchecked: ident, $getter_ffi: ident,
        $setter: ident, $setter_unchecked: ident, $setter_ffi: ident
    ) => {
        impl_array!(
            @getter
            $type, $type_check, $type_doc,
            $getter, $getter_unchecked, $getter_ffi
        );
        impl_array!(
            @setter
            $type, $type_check, $type_doc,
            $setter, $setter_unchecked, $setter_ffi
        );
    };
    (@getter
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
            let item = self.get_item_value_unchecked(dim);
            item.$getter_ffi()
        }
    };
    (@setter
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
            let mut item = self.get_item_value_unchecked(dim);
            item.$setter_ffi(value)
        }
    };
}

impl<'arr> Array<'arr> {
    impl_array!(
        @all
        pbint, ValueType::Int, "int",
        get_item_int, get_item_int_unchecked, get_int_unchecked,
        set_item_int, set_item_int_unchecked, set_int_unchecked
    );
    impl_array!(
        @all
        pbuint, ValueType::Uint, "uint",
        get_item_uint, get_item_uint_unchecked, get_uint_unchecked,
        set_item_uint, set_item_uint_unchecked, set_uint_unchecked
    );
    impl_array!(
        @all
        pblong, ValueType::Long, "long",
        get_item_long, get_item_long_unchecked, get_long_unchecked,
        set_item_long, set_item_long_unchecked, set_long_unchecked
    );
    impl_array!(
        @all
        pbulong, ValueType::Ulong, "ulong",
        get_item_ulong, get_item_ulong_unchecked, get_ulong_unchecked,
        set_item_ulong, set_item_ulong_unchecked, set_ulong_unchecked
    );
    impl_array!(
        @all
        pblonglong, ValueType::LongLong, "longlong",
        get_item_longlong, get_item_longlong_unchecked, get_longlong_unchecked,
        set_item_longlong, set_item_longlong_unchecked, set_longlong_unchecked
    );
    impl_array!(
        @all
        pbreal, ValueType::Real, "real",
        get_item_real, get_item_real_unchecked, get_real_unchecked,
        set_item_real, set_item_real_unchecked, set_real_unchecked
    );
    impl_array!(
        @all
        pbdouble, ValueType::Double, "double",
        get_item_double, get_item_double_unchecked, get_double_unchecked,
        set_item_double, set_item_double_unchecked, set_double_unchecked
    );
    impl_array!(
        @all
        pbbyte, ValueType::Byte, "byte",
        get_item_byte, get_item_byte_unchecked, get_byte_unchecked,
        set_item_byte, set_item_byte_unchecked, set_byte_unchecked
    );
    impl_array!(
        @all
        bool, ValueType::Boolean, "boolean",
        get_item_bool, get_item_bool_unchecked, get_bool_unchecked,
        set_item_bool, set_item_bool_unchecked, set_bool_unchecked
    );
    impl_array!(
        @all
        PBChar, ValueType::Char, "char",
        get_item_char, get_item_char_unchecked, get_char_unchecked,
        set_item_char, set_item_char_unchecked, set_char_unchecked
    );

    impl_array!(
        @all
        Decimal, ValueType::Decimal, "decimal",
        get_item_dec, get_item_dec_unchecked, get_dec_unchecked,
        set_item_dec, set_item_dec_unchecked, set_dec_unchecked
    );
    impl_array!(
        @all
        NaiveDate, ValueType::Date, "date",
        get_item_date, get_item_date_unchecked, get_date_unchecked,
        set_item_date, set_item_date_unchecked, set_date_unchecked
    );
    impl_array!(
        @all
        NaiveTime, ValueType::Time, "time",
        get_item_time, get_item_time_unchecked, get_time_unchecked,
        set_item_time, set_item_time_unchecked, set_time_unchecked
    );
    impl_array!(
        @all
        NaiveDateTime, ValueType::DateTime, "datetime",
        get_item_datetime, get_item_datetime_unchecked, get_datetime_unchecked,
        set_item_datetime, set_item_datetime_unchecked, set_datetime_unchecked
    );
    impl_array!(
        @getter
        &'arr [u8], ValueType::Blob, "blob",
        get_item_blob, get_item_blob_unchecked, get_blob_unchecked
    );
    impl_array!(
        @setter
        &[u8], ValueType::Blob, "blob",
        set_item_blob, set_item_blob_unchecked, set_blob_unchecked
    );
}

/// 数组信息
pub struct ArrayInfo {
    ptr: POB_ARRAY_INST,
    session: Session
}

impl ArrayInfo {
    pub(crate) unsafe fn new(ptr: OB_ARRAY_ID, session: Session) -> ArrayInfo {
        ArrayInfo {
            ptr: ptr as POB_ARRAY_INST,
            session
        }
    }

    pub(crate) fn item_group(&self) -> OB_GROUP_ID {
        unsafe {
            let arr = &*self.ptr;
            arr.elementType.group_hndl
        }
    }

    /// 是否固定长度
    pub fn bounded(&self) -> bool {
        unsafe {
            let arr = &*self.ptr;
            arr.type_ == OB_ARRAY_TYPE::OB_ARRAY_STATIC as _
        }
    }

    /// 元素类型
    pub fn value_type(&self) -> ValueType {
        unsafe {
            let arr = &*self.ptr;
            arr.elementType.class_id.into()
        }
    }

    /// 维度数量
    pub fn dimensions(&self) -> pbuint {
        unsafe {
            let arr = &*self.ptr;
            arr.nDims
        }
    }

    /// 获取指定维度的边界
    pub fn bound(&self, dim: pbuint) -> (pblong, pblong) {
        unsafe {
            let arr = &*self.ptr;
            assert!(dim > 0 && dim <= arr.nDims);
            let bounds = ptr::addr_of!(arr.bounds).add(dim as usize - 1);
            let lower_bound = (*bounds)[0];
            let upper_bound = (*bounds)[1];
            (lower_bound, upper_bound)
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
    ($type: ty, @value, $fn: ident) => {
        impl<'arr> ArrayIterItem<'arr> for $type {
            fn check_type(_arr: &Array) -> bool { true }
            fn get_value(arr: &Array<'arr>, index: pblong) -> Option<Self> { unsafe { Some(arr.$fn(index)) } }
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
// arr_iter_item!(Object<'arr>, @object, get_item_object_unchecked);
impl_iter_item!(Value<'arr>, @value, get_item_value_unchecked);

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
