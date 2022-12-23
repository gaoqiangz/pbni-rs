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
        self.check_get_index(dim)?;
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
        if item_type == get_type {
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

/*
    Getter/Setter
*/

macro_rules! impl_item {
    (
        @all
        $type_name: ty, $type: ty, $type_check: expr
    ) => {
        impl_item!(
            @getter
            $type_name, $type, $type_check
        );
        impl_item!(
            @setter
            $type_name, $type, $type_check
        );
    };
    (
        @getter
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
                self.check_get(dim, $type_check)?;
                unsafe {
                    Ok(self.[<get_item_ $type_name _unchecked>](dim))
                }
            }

            #[doc = "获取`" $type_name "`类型元素值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_item_ $type_name _unchecked>](&self, dim: impl AsArrayIndex) -> Option<$type> {
                let item = self.get_item_value_unchecked(dim);
                item.[<get_ $type_name _unchecked>]()
            }
        }
    };
    (
        @setter
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

            #[doc = "设置`" $type_name "`类型元素值,不检查类型"]
            ///
            /// # Safety
            ///
            /// 索引越界或类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_item_ $type_name _unchecked>](&mut self, dim: impl AsArrayIndex, value: $type) {
                let dim = dim.as_array_index();
                let mut item = self.get_item_value_unchecked(dim);
                item.[<set_ $type_name _unchecked>](impl_item!(@complex_set_val self, dim, value, $type_name));
            }
        }
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, value) => {
        &$value
    };
    (@complex_set_val $self: expr, $dim: expr, $value: expr, $type_name: ty) => {
        $value
    };
    (@check_type_set $self: expr, $dim: expr, $value: expr, $type_check: expr, value) => {
        $self.check_set($dim, $value.get_type())?;
    };
    (@check_type_set $self: expr, $dim: expr, $value: expr, $type_check: expr, $type_name: ty) => {
        $self.check_set($dim, $type_check)?;
    };
}

impl<'arr> Array<'arr> {
    impl_item!(
        @all
        int, pbint, ValueType::Int
    );
    impl_item!(
        @all
        uint, pbuint, ValueType::Uint
    );
    impl_item!(
        @all
        long, pblong, ValueType::Long
    );
    impl_item!(
        @all
        ulong, pbulong, ValueType::Ulong
    );
    impl_item!(
        @all
        longlong, pblonglong, ValueType::LongLong
    );
    impl_item!(
        @all
        real, pbreal, ValueType::Real
    );
    impl_item!(
        @all
        double, pbdouble, ValueType::Double
    );
    impl_item!(
        @all
        byte, pbbyte, ValueType::Byte
    );
    impl_item!(
        @all
        bool, bool, ValueType::Boolean
    );
    impl_item!(
        @all
        char, PBChar, ValueType::Char
    );

    impl_item!(
        @all
        dec, Decimal, ValueType::Decimal
    );
    impl_item!(
        @all
        date, NaiveDate, ValueType::Date
    );
    impl_item!(
        @all
        time, NaiveTime, ValueType::Time
    );
    impl_item!(
        @all
        datetime, NaiveDateTime, ValueType::DateTime
    );
    impl_item!(
        @getter
        blob, &'arr [u8], ValueType::Blob
    );
    impl_item!(
        @setter
        blob, &[u8], ValueType::Blob
    );
    impl_item!(
        @getter
        str, &'arr PBStr, ValueType::String
    );
    impl_item!(
        @getter
        string, PBString, ValueType::String
    );
    impl_item!(
        @setter
        str, impl AsPBStr, ValueType::String
    );
    impl_item!(
        @setter
        value, Value, ValueType::NoType
    );
}
