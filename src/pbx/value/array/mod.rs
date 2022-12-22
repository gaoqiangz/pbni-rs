use crate::{
    pbx::{bindings::*, *}, prelude::*
};

mod impls;

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

    /// 获取数组信息
    pub fn info(&self) -> &ArrayInfo { &self.info }

    /// 获取数组长度(仅一维数组有效)
    pub fn len(&self) -> pblong { unsafe { ffi::pbsession_GetArrayLength(self.session.as_ptr(), self.ptr) } }

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
        unsafe { Ok(ffi::pbsession_GetArrayItemType(self.session.as_ptr(), self.ptr, dim.as_ptr())) }
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
        unsafe { Ok(ffi::pbsession_IsArrayItemNull(self.session.as_ptr(), self.ptr, dim.as_ptr()).into()) }
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
        ffi::pbsession_SetArrayItemToNull(self.session.as_ptr(), self.ptr, dim.as_ptr());
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
                let (lower, upper) = self.info.bound(dim as pbuint);
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
    ///
    /// # Panics
    ///
    /// 维度超出有效范围会触发Panic
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
