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

    /*
    TODO
    /// 获取对象类型值的引用
    ///
    /// # Panics
    ///
    /// 类型不匹配时会触发Panic
    pub fn get_object(&self) -> Option<Object<'val>> { self.try_get_object().unwrap() }

    /// 尝试获取对象类型值的引用
    pub fn try_get_object(&self) -> Result<Option<Object<'val>>> {
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
    /// 类型不兼容时可能会出现未定义行为
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
    pub fn get_array(&self) -> Option<Array<'val>> { self.try_get_array().unwrap() }

    /// 尝试获取数组类型值的引用
    pub fn try_get_array(&self) -> Result<Option<Array<'val>>> {
        if self.is_array() {
            unsafe { Ok(self.get_array_unchecked()) }
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
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
            Some(Array::from_ptr(self.inner.val.ptr as _, self.is_object(), self.session.clone()))
        }
    }

    /// 设置值为NULL
    pub fn set_to_null(&mut self) { self.set_info_flag(1, DATA_NULLVAL_SHIFT, DATA_NULLVAL_MASK); }

    /*
    TODO
    /// 设置对象类型的值
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
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 设置数组类型的值,不检查类型
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_array_unchecked(&mut self, value: &Array) {
        self.set_ptr(
            API.ot_copy_array(self.session.as_ptr(), value.as_ptr() as _) as _,
            value.info().value_type() as OB_CLASS_ID
        );
        self.set_info_group(OB_GROUPTYPE::OB_ARRAY);
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
            self.get_type_kind() == value.get_type_kind() &&
            self.get_info_group() == value.get_info_group() &&
            self.get_info_style() == value.get_info_style()
        {
            unsafe {
                self.set_value_unchecked(value);
            }
            Ok(())
        } else {
            Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
        }
    }

    /// 从参数拷贝并覆盖现有值
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_value_unchecked(&mut self, value: &Value) {
        API.rtDataCopy(self.session.as_ptr(), self.as_ptr(), value.as_ptr(), true as BOOL);
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

macro_rules! impl_value {
    /*
        简单类型
    */
    (
        @simple
        $type_name: ty, $type: ty, $type_check: pat,
        $field: ident, $field_style: expr, $field_type: expr
    ) => {
        impl_value!(
            @simple_getter
            $type_name, $type, $type_check,
            $field
        );
        impl_value!(
            @simple_setter
            $type_name, $type, $type_check,
            $field, $field_style, $field_type
        );
    };
    (
        @simple_getter
        $type_name: ty, $type: ty, $type_check: pat,
        $field: ident
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
            /// 类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_ $type_name _unchecked>](&self) -> Option<$type> {
                if self.is_null() {
                    None
                } else {
                    Some(impl_value!(@simple_get_val self.inner.val.$field, $type_name) as _)
                }
            }
        }
    };
    (
        @simple_setter
        $type_name: ty, $type: ty, $type_check: pat,
        $field: ident, $field_style: expr, $field_type: expr
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
            /// 类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_ $type_name _unchecked>](&mut self, value: $type) {
                self.free_val_ptr();
                self.inner.val.$field = impl_value!(@simple_set_val value, $type_name) as _;
                self.set_info($field_style, $field_type, OB_GROUPTYPE::OB_SIMPLE);
            }
        }
    };
    (@simple_get_val $value: expr, bool) => {
        if $value == 1 {
            true
        } else {
            false
        }
    };
    (@simple_get_val $value: expr, $type_name: ty) => {
        $value
    };
    (@simple_set_val $value: expr, bool) => {
        if $value {
            1
        } else {
            0
        }
    };
    (@simple_set_val $value: expr, $type_name: ty) => {
        $value
    };

    /*
        复杂类型
    */
    (
        @complex
        $type_name: ty, $type: ty, $type_check: pat,
        $field_type: expr
    ) => {
        impl_value!(
            @complex_getter
            $type_name, $type, $type_check
        );
        impl_value!(
            @complex_setter
            $type_name, $type, $type_check,
            $field_type
        );
    };
    (
        @complex_getter
        $type_name: ty, $type: ty, $type_check: pat
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
            /// 类型不兼容时可能会出现未定义行为
            pub unsafe fn [<get_ $type_name _unchecked>](&self) -> Option<$type> {
                if self.is_null() {
                    None
                } else {
                    Some(impl_value!(@complex_get_val self.session, self.inner.val.ptr, $type_name))
                }
            }
        }
    };
    (
        @complex_setter
        $type_name: ty, $type: ty, $type_check: pat,
        $field_type: expr
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
            /// 类型不兼容时可能会出现未定义行为
            pub unsafe fn [<set_ $type_name _unchecked>](&mut self, value: $type) {
                self.set_ptr(impl_value!(@complex_set_val self.session, value, $type_name) as _, $field_type);
            }
        }
    };
    (@complex_get_val $session: expr, $value: expr, longlong) => {
        *($value as *const pblonglong)
    };
    (@complex_get_val $session: expr, $value: expr, double) => {
        *($value as *const pbdouble)
    };
    (@complex_get_val $session: expr, $value: expr, str) => {
        PBStr::from_ptr_str($value as _)
    };
    (@complex_get_val $session: expr, $value: expr, string) => {
        PBString::from_ptr_str($value as _)
    };
    (@complex_get_val $session: expr, $value: expr, $type_name: ty) => {
        ::paste::paste! {
            $session.[<get_ $type_name _unchecked>]($value as _)
        }
    };
    (@complex_set_val $session: expr, $value: expr, longlong) => {
        API.ob_dup_longlong($session.as_ptr(), &$value as *const pblonglong as _)
    };
    (@complex_set_val $session: expr, $value: expr, double) => {
        API.ob_dup_double($session.as_ptr(), &$value as *const pbdouble as _)
    };
    (@complex_set_val $session: expr, $value: expr, str) => {
        API.ob_dup_string($session.as_ptr(), $value.as_pbstr().as_ptr() as _)
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
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        ::paste::paste! {
            #[doc = "获取`" $type_name "`类型值"]
            ///
            /// # Panics
            ///
            /// 类型不匹配时会触发Panic
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
                    Err(PBRESULT::E_MISMATCHED_DATA_TYPE)
                }
            }
        }
    };
    (
        @checked_setter
        $type_name: ty, $type: ty, $type_check: pat
    ) => {
        ::paste::paste! {
            #[doc = "设置`" $type_name "`类型值"]
            ///
            /// # Panics
            ///
            /// 类型不匹配时会触发Panic
            pub fn [<set_ $type_name>](&mut self, value: $type) {
                self.[<try_set_ $type_name>](value).unwrap();
            }

            #[doc = "设置`" $type_name "`类型值"]
            pub fn [<try_set_ $type_name>](&mut self, value: $type) -> Result<()> {
                if matches!(self.get_type(), $type_check | ValueType::NoType) {
                    unsafe {
                        self.[<set_ $type_name _unchecked>](value);
                    }
                    Ok(())
                } else {
                    return Err(PBRESULT::E_MISMATCHED_DATA_TYPE);
                }
            }
        }
    };
}

impl<'val> Value<'val> {
    impl_value!(
        @simple
        int, pbint, ValueType::Int,
        int_val, OB_DATASTYLE::INT_STYLE, INT_TYPE
    );
    impl_value!(
        @simple
        uint, pbuint, ValueType::Uint,
        uint_val, OB_DATASTYLE::INT_STYLE, UINT_TYPE
    );
    impl_value!(
        @simple
        long, pblong, ValueType::Long,
        long_val, OB_DATASTYLE::LONG_STYLE, LONG_TYPE
    );
    impl_value!(
        @simple
        ulong, pbulong, ValueType::Ulong,
        ulong_val, OB_DATASTYLE::LONG_STYLE, ULONG_TYPE
    );
    impl_value!(
        @simple
        real, pbreal, ValueType::Real,
        fl_val, OB_DATASTYLE::FLOAT_STYLE, FLOAT_TYPE
    );
    impl_value!(
        @simple
        byte, pbbyte, ValueType::Byte,
        uint_val, OB_DATASTYLE::INT_STYLE, BYTE_TYPE
    );
    impl_value!(
        @simple
        bool, bool, ValueType::Boolean,
        int_val, OB_DATASTYLE::INT_STYLE, BOOL_TYPE
    );
    impl_value!(
        @simple
        char, PBChar, ValueType::Char,
        uint_val, OB_DATASTYLE::INT_STYLE, CHAR_TYPE
    );

    impl_value!(
        @complex
        longlong, pblonglong, ValueType::LongLong,
        LONGLONG_TYPE
    );
    impl_value!(
        @complex
        double, pbdouble, ValueType::Double,
        DOUBLE_TYPE
    );
    impl_value!(
        @complex
        dec, Decimal, ValueType::Decimal,
        DEC_TYPE
    );
    impl_value!(
        @complex
        date, NaiveDate, ValueType::Date,
        DATE_TYPE
    );
    impl_value!(
        @complex
        time, NaiveTime, ValueType::Time,
        TIME_TYPE
    );
    impl_value!(
        @complex
        datetime, NaiveDateTime, ValueType::DateTime,
        DATETIME_TYPE
    );
    impl_value!(
        @complex_getter
        blob, &'val [u8], ValueType::Blob
    );
    impl_value!(
        @complex_setter
        blob, &[u8], ValueType::Blob,
        BINARY_TYPE
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
        str, impl AsPBStr, ValueType::String,
        STRING_TYPE
    );
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
            //解引用参数
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
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl<'val> FromValue<'val> for &'val PBStr {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_str()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for PBString {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_string()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for String {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_string()?.map(|v| v.to_string_lossy()).ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbint {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_int()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbuint {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_uint()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pblong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_long()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbulong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_ulong()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pblonglong {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_longlong()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbdouble {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_double()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbreal {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_real()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for Decimal {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_dec()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveDate {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_date()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveTime {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_time()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for NaiveDateTime {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_datetime()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for pbbyte {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_byte()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for bool {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_bool()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}

impl<'val> FromValue<'val> for &'val [u8] {
    fn from_value(val: Option<Value<'val>>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_blob()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
        }
    }
}
impl FromValue<'_> for Vec<u8> {
    fn from_value(val: Option<Value>) -> Result<Self> {
        if let Some(val) = val {
            val.try_get_blob()?.ok_or(PBRESULT::E_VALUE_IS_NULL).map(Vec::from)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
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
            val.try_get_array()?.ok_or(PBRESULT::E_VALUE_IS_NULL)
        } else {
            Err(PBRESULT::E_WRONG_NUM_ARGS)
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
            Err(PBRESULT::E_WRONG_NUM_ARGS)
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
            if e == PBRESULT::E_WRONG_NUM_ARGS || e == PBRESULT::E_VALUE_IS_NULL {
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
/*
TODO
impl ToValue for Object<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_object(&self) }
}
*/
impl ToValue for Array<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_array(&self) }
}
impl ToValue for Value<'_> {
    fn to_value(self, val: &mut Value) -> Result<()> { val.try_set_value(&self) }
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
