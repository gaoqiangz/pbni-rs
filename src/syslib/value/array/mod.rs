use crate::{
    prelude::*, syslib::{bindings::*, *}
};

mod impls;

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
    /// 类型不匹配或不是一维数组时会触发Panic
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
    pub fn get_item_type(&self, dim: impl AsArrayIndex) -> ValueType { self.try_get_item_type(dim).unwrap() }

    /// 获取元素类型
    pub fn try_get_item_type(&self, dim: impl AsArrayIndex) -> Result<ValueType> {
        let dim = dim.as_array_index();
        self.check_get_index(dim)?;
        unsafe {
            let idx = API.ob_array_get_index_from_subs(self.session.as_ptr(), self.ptr, dim.as_ptr() as _);
            let v = API.ot_array_index(self.session.as_ptr(), self.as_ptr(), idx);
            if v.is_null() {
                return Err(PBRESULT::E_OUT_OF_BOUNDS);
            }
            Ok((&*v).type_.into())
        }
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
        let item = self.try_get_item_value(dim)?;
        Ok(item.is_null())
    }

    /*
    TODO
    /// 获取对象类型元素值的引用
    ///
    /// # Panics
    ///
    /// 索引越界或类型不匹配时会触发Panic
    pub fn get_item_object(&self, dim: impl AsArrayIndex) -> Option<Object<'arr>> {
        assert!(self.is_item_object(dim));
        unsafe { self.get_item_object_unchecked(dim) }
    }

    /// 获取对象类型元素值的引用,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn get_item_object_unchecked(&self, dim: impl AsArrayIndex) -> Option<Object<'arr>> {
        let val = self.get_item_value_unchecked(dim);
        val.get_object_unchecked()
    }
    */

    /// 获取`value`类型元素值的引用
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn get_item_value(&self, dim: impl AsArrayIndex) -> Value<'arr> {
        self.try_get_item_value(dim).unwrap()
    }

    /// 获取`value`类型元素值的引用
    pub fn try_get_item_value(&self, dim: impl AsArrayIndex) -> Result<Value<'arr>> {
        let dim = dim.as_array_index();
        self.check_get(dim, ValueType::Any)?;
        unsafe { Ok(self.get_item_value_unchecked(dim)) }
    }

    /// 获取`value`类型元素值的引用,不检查类型
    ///
    /// # Safety
    ///
    /// 索引越界时可能会出现未定义行为
    pub unsafe fn get_item_value_unchecked(&self, dim: impl AsArrayIndex) -> Value<'arr> {
        let dim = dim.as_array_index();
        let idx = API.ob_array_get_index_from_subs(self.session.as_ptr(), self.ptr, dim.as_ptr() as _);
        let v = API.ot_array_index(self.session.as_ptr(), self.as_ptr(), idx);
        if v.is_null() {
            panic!("invalid item: {:?}", dim);
        }
        Value::from_ptr(v, self.session.clone())
    }

    /*
    TODO
    /// 设置对象类型元素的值
    ///
    /// # Panics
    ///
    /// 索引越界或类型不匹配时会触发Panic
    pub fn set_item_object(&mut self, dim: impl AsArrayIndex, value: &Object) -> Result<()> {
        self.check_set_index(dim);
        assert!(self.is_item_object(dim) || self.info.value_type() == ValueType::Any);
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
        self.try_set_item_value(dim, value).unwrap();
    }

    /// 设置元素的值,`value`参数将被消耗
    pub fn try_set_item_value(&mut self, dim: impl AsArrayIndex, value: Value) -> Result<()> {
        let dim = dim.as_array_index();
        self.check_set(dim, value.get_type())?;
        unsafe {
            self.set_item_value_unchecked(dim, value);
        }
        Ok(())
    }

    /// 设置元素的值,`value`参数将被消耗
    ///
    /// # Safety
    ///
    /// 索引越界或类型不兼容时可能会出现未定义行为
    pub unsafe fn set_item_value_unchecked(&mut self, dim: impl AsArrayIndex, value: Value) {
        let mut item = self.get_item_value_unchecked(dim);
        item.set_value_unchecked(&value);
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
        let mut item = self.get_item_value_unchecked(dim);
        item.set_to_null();
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

    #[inline]
    fn check_get_index(&self, dim: &[pblong]) -> Result<()> {
        if dim.is_empty() {
            return Err(PBRESULT::E_OUT_OF_MEMORY);
        }
        if !self.info.bounded() {
            if dim.len() > 1 {
                return Err(PBRESULT::E_OUT_OF_BOUNDS);
            }
            if dim[0] < 1 || dim[0] > self.len() {
                return Err(PBRESULT::E_OUT_OF_BOUNDS);
            }
        } else {
            if dim.len() != self.info.dimensions() as usize {
                return Err(PBRESULT::E_OUT_OF_BOUNDS);
            }
            for (dim, &idx) in dim.iter().enumerate() {
                let (lower, upper) = self.info.bound(dim as pbuint);
                if idx < lower || idx > upper {
                    return Err(PBRESULT::E_OUT_OF_BOUNDS);
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn check_get(&self, dim: &[pblong], get_type: ValueType) -> Result<()> {
        //get_item_type包含check_get_index
        let item_type = self.get_item_type(dim);
        if item_type == get_type || get_type == ValueType::Any {
            Ok(())
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    #[inline]
    fn check_set_index(&self, dim: &[pblong]) -> Result<()> {
        if dim.is_empty() {
            return Err(PBRESULT::E_OUT_OF_MEMORY);
        }
        if !self.info.bounded() {
            if dim.len() > 1 {
                return Err(PBRESULT::E_OUT_OF_BOUNDS);
            }
            if dim[0] < 1 {
                return Err(PBRESULT::E_OUT_OF_BOUNDS);
            }
        } else {
            if dim.len() != self.info.dimensions() as usize {
                return Err(PBRESULT::E_OUT_OF_BOUNDS);
            }
            for (dim, &idx) in dim.iter().enumerate() {
                let (lower, upper) = self.info.bound(dim as pbuint);
                if idx < lower || idx > upper {
                    return Err(PBRESULT::E_OUT_OF_BOUNDS);
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
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }
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
    ///
    /// # Panics
    ///
    /// 维度超出有效范围会触发Panic
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
