use crate::{
    pbx::{
        bindings::*, invoker::{GlobalFunction, Invoker}, value::FromValueOwned, *
    }, prelude::*
};
use std::ops::Deref;

mod global_var;
mod global_func;

/// Session对象
#[derive(Clone)]
#[repr(transparent)]
pub struct Session {
    ptr: pbsession
}

impl Session {
    pub(crate) unsafe fn from_ptr(ptr: pbsession) -> Session {
        Session {
            ptr
        }
    }
    pub(crate) fn as_ptr(&self) -> pbsession { self.ptr }

    /// 判断是否有重启Session的请求 (在PowerScript中调用了`Restart`函数)
    pub fn restart_requested(&self) -> bool { unsafe { ffi::pbsession_RestartRequested(self.ptr).into() } }

    /// 是否创建了可视化对象 (打开了顶层窗口)
    pub fn has_visual_object(&self) -> bool { unsafe { ffi::pbsession_HasPBVisualObject(self.ptr).into() } }

    /// 处理PB消息
    ///
    /// # Notice
    ///
    /// 开启了消息循环后,需要处理PB的消息以执行PowerScript中的`Post`调用
    pub fn process_message(&self) -> bool { unsafe { ffi::pbsession_ProcessPBMessage(self.ptr).into() } }

    /// 获取系统类组
    pub(crate) fn get_system_group(&self) -> pbgroup { unsafe { ffi::pbsession_GetSystemGroup(self.ptr) } }

    /// 查找指定名称的类组
    pub(crate) fn find_group(&self, name: impl AsPBStr, r#type: GroupType) -> Option<pbgroup> {
        unsafe { ffi::pbsession_FindGroup(self.ptr, name.as_pbstr().as_ptr(), r#type) }
    }

    /// 在指定组下查找类定义
    pub(crate) fn find_class(&self, group: pbgroup, name: impl AsPBStr) -> Option<pbclass> {
        unsafe { ffi::pbsession_FindClass(self.ptr, group, name.as_pbstr().as_ptr()) }
    }

    /// 获取类定义的组
    ///
    /// FIXME: 此方法通过类名反查类所在的组,目前PBNI没有提供直接的获取方法
    pub(crate) fn get_group(&self, mut cls: pbclass) -> Option<pbgroup> {
        unsafe {
            'outer: loop {
                let cls_name = ffi::pbsession_GetClassName(self.ptr, cls);
                for group_type in (GroupType::Application as i32)..(GroupType::Unknown as i32) {
                    let group = ffi::pbsession_FindGroup(self.ptr, cls_name, group_type.into());
                    if group.is_some() {
                        break 'outer group;
                    }
                }
                match ffi::pbsession_GetSuperClass(self.ptr, cls) {
                    Some(v) => cls = v,
                    None => break None
                }
            }
        }
    }

    /*
        Exception
    */

    /// 检查当前是否有异常未处理
    pub fn has_exception(&self) -> bool { unsafe { ffi::pbsession_HasExceptionThrown(self.ptr).into() } }

    /// 清除异常
    pub fn clear_exception(&self) {
        unsafe {
            ffi::pbsession_ClearException(self.ptr);
        }
    }

    /// 抛出`PBXRuntimeError`异常
    ///
    /// # Exmaples
    ///
    /// ```
    /// session.throw_exception("test");
    /// // 使用宏
    /// throw!(session,"this is a {}!","exception");
    /// ```
    pub fn throw_exception(&self, exstr: impl AsPBStr) -> Result<()> {
        let mut ex = self.new_system_object(pbstr!("PBXRuntimeError"))?;
        unsafe {
            ex.set_field_str_unchecked(pbstr!("text"), exstr);
            ffi::pbsession_ThrowException(self.ptr, ex.as_ptr());
        }
        Ok(())
    }

    /*
        Prop
    */

    /// 与Session绑定`static`引用参数
    pub fn set_prop<T, D>(&self, name: T, data: &'static D)
    where
        T: AsPBStr,
        D: Sized
    {
        self.set_prop_ptr(name, data as *const D)
    }

    /// 与Session绑定指针参数
    pub fn set_prop_ptr<T, D>(&self, name: T, data: *const D)
    where
        T: AsPBStr,
        D: Sized
    {
        unsafe { ffi::pbsession_SetProp(self.ptr, name.as_pbstr().as_ptr(), data as _) }
    }

    /// 获取Session绑定的指针参数
    pub fn get_prop_ptr<T, D>(&self, name: T) -> *const D
    where
        T: AsPBStr,
        D: Sized
    {
        unsafe { ffi::pbsession_GetProp(self.ptr, name.as_pbstr().as_ptr()) as *const D }
    }
    /// 获取Session绑定的引用参数
    ///
    ///
    /// # Safety
    ///
    /// 引用的生命周期由调用处提供,需要开发者自行保证期有效性
    pub unsafe fn get_prop_ref<'a, T, D>(&self, name: T) -> Option<&'a D>
    where
        T: AsPBStr,
        D: Sized
    {
        let ptr: *const D = self.get_prop_ptr(name);
        if ptr.is_null() {
            None
        } else {
            Some(&*ptr)
        }
    }

    /// 获取Session绑定的可变引用参数
    ///
    ///
    /// # Safety
    ///
    /// 引用的生命周期由调用处提供,需要开发者自行保证期有效性
    pub unsafe fn get_prop_mut<'a, T, D>(&self, name: T) -> Option<&'a mut D>
    where
        T: AsPBStr,
        D: Sized
    {
        let ptr: *const D = self.get_prop_ptr(name);
        if ptr.is_null() {
            None
        } else {
            Some(&mut *(ptr as *mut D))
        }
    }

    /// 解绑Session参数
    ///
    /// # Notice
    ///
    /// 如果绑定的参数内存是由`Box`分配的,那么需要在解绑前正确释放内存
    pub fn remove_prop(&self, name: impl AsPBStr) {
        unsafe { ffi::pbsession_RemoveProp(self.ptr, name.as_pbstr().as_ptr()) }
    }

    /*
        Enum
    */

    /// 获取指定名称枚举的值,不区分大小写
    ///
    /// # Examples
    ///
    /// ```
    /// // `Center!`枚举的值
    /// session.get_enum_item_value("Aligment","Center");
    /// ```
    pub fn get_enum_item_value(&self, enum_name: impl AsPBStr, item_name: impl AsPBStr) -> pblong {
        unsafe {
            ffi::pbsession_GetEnumItemValue(
                self.ptr,
                enum_name.as_pbstr().as_ptr(),
                item_name.as_pbstr().as_ptr()
            )
        }
    }

    /// 获取枚举值的枚举名称,不区分大小写
    ///
    /// # Examples
    ///
    /// ```
    /// let val = session.get_enum_item_value("Aligment","Center");
    /// let name = session.get_enum_item_name("Aligment",val); //center
    /// ```
    pub fn get_enum_item_name<'a>(&self, enum_name: impl AsPBStr, item_value: pblong) -> Option<&'a PBStr> {
        unsafe {
            let cstr = ffi::pbsession_GetEnumItemName(self.ptr, enum_name.as_pbstr().as_ptr(), item_value);
            if cstr.is_null() {
                None
            } else {
                Some(PBStr::from_ptr_str(cstr))
            }
        }
    }

    /*
        String
    */

    #[inline]
    pub(crate) unsafe fn get_string_unchecked<'a>(&self, pbstr: pbstring) -> Option<&'a PBStr> {
        let cstr = ffi::pbsession_GetString(self.ptr, pbstr);
        if !cstr.is_null() {
            Some(PBStr::from_ptr_str(cstr))
        } else {
            None
        }
    }

    /*
        Object
    */

    /// 实例化用户自定义对象
    ///
    /// # Examples
    ///
    /// ```
    /// let obj = session.new_user_object("n_cst_object");
    /// ```
    pub fn new_user_object<'a>(&self, cls_name: impl AsPBStr) -> Result<Object<'a>> {
        unsafe {
            let cls_name = cls_name.as_pbstr();
            let group = self
                .find_group(cls_name.as_ref(), GroupType::UserObject)
                .ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
            let cls = self.find_class(group, cls_name).ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
            let ptr = ffi::pbsession_NewObject(self.ptr, cls);
            Ok(Object::from_ptr(ptr, self.clone()))
        }
    }

    /// 实例化系统对象
    ///
    /// # Examples
    ///
    /// ```
    /// let obj = session.new_system_object("datastore");
    /// ```
    pub fn new_system_object<'a>(&self, cls_name: impl AsPBStr) -> Result<Object<'a>> {
        unsafe {
            let group = self.get_system_group();
            let cls = self.find_class(group, cls_name).ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
            let ptr = ffi::pbsession_NewObject(self.ptr, cls);
            Ok(Object::from_ptr(ptr, self.clone()))
        }
    }

    /// 实例化任意PB对象 (包括用户自定义对象和系统对象)
    ///
    /// # Examples
    ///
    /// ```
    /// let obj = session.new_object("n_cst_object");
    /// let obj = session.new_object("datastore");
    /// ```
    pub fn new_object<'a>(&self, cls_name: impl AsPBStr) -> Result<Object<'a>> {
        let cls_name = cls_name.as_pbstr();
        self.new_user_object(cls_name.as_ref()).or_else(|_| self.new_system_object(cls_name))
    }

    /*
        Array
    */

    /// 实例化变长标量一维数组
    ///
    /// # Parameters
    ///
    /// - **item_type** 元素标量类型
    ///
    /// # Examples
    ///
    /// ```
    /// //int arr[]
    /// let mut arr = session.new_array(ValueType::Int).unwrap();
    /// //arr[1] = 123
    /// arr.set_item_int(&[1],123);
    /// ```
    pub fn new_array<'a>(&self, item_type: ValueType) -> Result<Array<'a>> {
        unsafe {
            let ptr = ffi::pbsession_NewUnboundedSimpleArray(self.ptr, item_type)
                .ok_or(PBXRESULT::E_INVALID_ARGUMENT)?;
            Ok(Array::from_ptr(ptr, false, self.clone()))
        }
    }

    /// 实例化定长标量多维数组
    ///
    /// # Parameters
    ///
    /// - **item_type** 元素标量类型
    /// - **dims** `&[(下标,上标)]`
    ///
    /// # Examples
    ///
    /// ```
    /// //int arr[10]
    /// let mut arr = session.new_bounded_array(ValueType::Int,&[(1,10)]).unwrap();
    /// //arr[1] = 123
    /// arr.set_item_int(&[1],123);
    ///
    /// //int arr[2,4]
    /// let mut arr = session.new_bounded_array(ValueType::Int,&[(1,2),(1,4)]).unwrap();
    /// //arr[1,2] = 123
    /// arr.set_item_int(&[1,2],123);
    ///
    /// //int arr[1 to 5,2 to 10]
    /// let mut arr = session.new_bounded_array(ValueType::Int,&[(1,5),(2,10)]).unwrap();
    /// //arr[1,2] = 123
    /// arr.set_item_int(&[1,2],123);
    /// ```
    pub fn new_bounded_array<'a>(
        &self,
        item_type: ValueType,
        dims: &[(pblong, pblong)]
    ) -> Result<Array<'a>> {
        unsafe {
            let mut bounds = Vec::with_capacity(dims.len());
            for (lower, upper) in dims {
                if upper < lower {
                    return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
                }
                bounds.push(ArrayBound {
                    lowerBound: *lower,
                    upperBound: *upper
                });
            }
            let ptr =
                ffi::pbsession_NewBoundedSimpleArray(self.ptr, item_type, dims.len() as u16, bounds.as_ptr())
                    .ok_or(PBXRESULT::E_INVALID_ARGUMENT)?;
            Ok(Array::from_ptr(ptr, false, self.clone()))
        }
    }

    /// 实例化变长用户自定义对象一维数组
    ///
    /// # Parameters
    ///
    /// - **cls_name** 元素对象类名
    ///
    /// # Examples
    ///
    /// ```
    /// //n_cst_test arr[]
    /// let mut arr = session.new_user_object_array("n_cst_test").unwrap();
    /// //arr[1] = Create n_cst_test
    /// let obj = session.new_user_object("n_cst_test").unwrap();
    /// arr.set_item_object(&[1],&obj);
    /// ```
    pub fn new_user_object_array<'a>(&self, cls_name: impl AsPBStr) -> Result<Array<'a>> {
        let cls_name = cls_name.as_pbstr();
        let group =
            self.find_group(cls_name.as_ref(), GroupType::UserObject).ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
        let cls = self.find_class(group, cls_name).ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
        self.new_object_array_impl(cls)
    }

    /// 实例化变长系统对象一维数组
    ///
    /// # Parameters
    ///
    /// - **cls_name** 元素对象类名
    ///
    /// # Examples
    ///
    /// ```
    /// //datastore arr[]
    /// let mut arr = session.new_system_object_array("datastore").unwrap();
    /// //arr[1] = Create datastore
    /// let obj = session.new_user_object("datastore").unwrap();
    /// arr.set_item_object(&[1],&obj);
    /// ```
    pub fn new_system_object_array<'a>(&self, cls_name: impl AsPBStr) -> Result<Array<'a>> {
        let group = self.get_system_group();
        let cls = self.find_class(group, cls_name).ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
        self.new_object_array_impl(cls)
    }

    /// 实例化变长任意对象一维数组 (包括用户自定义对象和系统对象)
    ///
    /// # Parameters
    ///
    /// - **cls_name** 元素对象类名
    ///
    /// # Examples
    ///
    /// ```
    /// //datastore arr[]
    /// let mut arr = session.new_object_array("datastore").unwrap();
    /// //arr[1] = Create datastore
    /// let obj = session.new_user_object("datastore").unwrap();
    /// arr.set_item_object(&[1],&obj);
    /// ```
    pub fn new_object_array<'a>(&self, cls_name: impl AsPBStr) -> Result<Array<'a>> {
        let cls_name = cls_name.as_pbstr();
        self.new_user_object_array(cls_name.as_ref()).or_else(|_| self.new_system_object_array(cls_name))
    }

    fn new_object_array_impl<'a>(&self, cls: pbclass) -> Result<Array<'a>> {
        unsafe {
            let ptr =
                ffi::pbsession_NewUnboundedObjectArray(self.ptr, cls).ok_or(PBXRESULT::E_INVALID_ARGUMENT)?;
            Ok(Array::from_ptr(ptr, true, self.clone()))
        }
    }

    /// 实例化定长用户自定义对象多维数组
    ///
    /// # Parameters
    ///
    /// - **cls_name** 元素对象类名
    /// - **dims** `&[(下标,上标)]`
    ///
    /// # Examples
    ///
    /// ```
    /// //n_cst_test arr[10]
    /// let mut arr = session.new_bounded_user_object_array("n_cst_test",&[(1,10)]).unwrap();
    /// //arr[1] = Create n_cst_test
    /// let obj = session.new_user_object("n_cst_test").unwrap();
    /// arr.set_item_object(&[1],&obj);
    ///
    /// //n_cst_test arr[2,4]
    /// let mut arr = session.new_bounded_user_object_array("n_cst_test",&[(1,2),(1,4)]).unwrap();
    /// //arr[1,2] = Create n_cst_test
    /// let obj = session.new_user_object("n_cst_test").unwrap();
    /// arr.set_item_object(&[1,2],&obj);
    ///
    /// //n_cst_test arr[1 to 5,2 to 10]
    /// let mut arr = session.new_bounded_user_object_array("n_cst_test",&[(1,5),(2,10)]).unwrap();
    /// //arr[1,2] = Create n_cst_test
    /// arr.set_item_object(&[1,2],&obj);
    /// ```
    pub fn new_bounded_user_object_array<'a>(
        &self,
        cls_name: impl AsPBStr,
        dims: &[(pblong, pblong)]
    ) -> Result<Array<'a>> {
        let cls_name = cls_name.as_pbstr();
        let group =
            self.find_group(cls_name.as_ref(), GroupType::UserObject).ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
        let cls = self.find_class(group, cls_name).ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
        self.new_bounded_object_array_impl(cls, dims)
    }

    /// 实例化定长用户自定义对象多维数组
    ///
    /// # Parameters
    ///
    /// - **cls_name** 元素对象类名
    /// - **dims** `&[(下标,上标)]`
    ///
    /// # Examples
    ///
    /// ```
    /// //datastore arr[10]
    /// let mut arr = session.new_bounded_system_object_array("datastore",&[(1,10)]).unwrap();
    /// //arr[1] = Create datastore
    /// let obj = session.new_user_object("datastore").unwrap();
    /// arr.set_item_object(&[1],&obj);
    ///
    /// //datastore arr[2,4]
    /// let mut arr = session.new_bounded_system_object_array("datastore",&[(1,2),(1,4)]).unwrap();
    /// //arr[1,2] = Create datastore
    /// let obj = session.new_user_object("datastore").unwrap();
    /// arr.set_item_object(&[1,2],&obj);
    ///
    /// //datastore arr[1 to 5,2 to 10]
    /// let mut arr = session.new_bounded_system_object_array("datastore",&[(1,5),(2,10)]).unwrap();
    /// //arr[1,2] = Create datastore
    /// arr.set_item_object(&[1,2],&obj);
    /// ```
    pub fn new_bounded_system_object_array<'a>(
        &self,
        cls_name: impl AsPBStr,
        dims: &[(pblong, pblong)]
    ) -> Result<Array<'a>> {
        let group = self.get_system_group();
        let cls = self.find_class(group, cls_name).ok_or(PBXRESULT::E_NO_SUCH_CLASS)?;
        self.new_bounded_object_array_impl(cls, dims)
    }

    /// 实例化定长任意对象多维数组
    ///
    /// # Parameters
    ///
    /// - **cls_name** 元素对象类名
    /// - **dims** `&[(下标,上标)]`
    ///
    /// # Examples
    ///
    /// ```
    /// //datastore arr[10]
    /// let mut arr = session.new_bounded_object_array("datastore",&[(1,10)]).unwrap();
    /// //arr[1] = Create datastore
    /// let obj = session.new_user_object("datastore").unwrap();
    /// arr.set_item_object(&[1],&obj);
    ///
    /// //datastore arr[2,4]
    /// let mut arr = session.new_bounded_object_array("datastore",&[(1,2),(1,4)]).unwrap();
    /// //arr[1,2] = Create datastore
    /// let obj = session.new_user_object("datastore").unwrap();
    /// arr.set_item_object(&[1,2],&obj);
    ///
    /// //datastore arr[1 to 5,2 to 10]
    /// let mut arr = session.new_bounded_object_array("datastore",&[(1,5),(2,10)]).unwrap();
    /// //arr[1,2] = Create datastore
    /// arr.set_item_object(&[1,2],&obj);
    /// ```
    pub fn new_bounded_object_array<'a>(
        &self,
        cls_name: impl AsPBStr,
        dims: &[(pblong, pblong)]
    ) -> Result<Array<'a>> {
        let cls_name = cls_name.as_pbstr();
        self.new_bounded_user_object_array(cls_name.as_ref(), dims)
            .or_else(|_| self.new_bounded_system_object_array(cls_name, dims))
    }

    fn new_bounded_object_array_impl<'a>(
        &self,
        cls: pbclass,
        dims: &[(pblong, pblong)]
    ) -> Result<Array<'a>> {
        unsafe {
            let dims: Vec<ArrayBound> = dims
                .iter()
                .map(|&(lowerBound, upperBound)| {
                    ArrayBound {
                        lowerBound,
                        upperBound
                    }
                })
                .collect();
            let ptr = ffi::pbsession_NewBoundedObjectArray(self.ptr, cls, dims.len() as u16, dims.as_ptr())
                .ok_or(PBXRESULT::E_INVALID_ARGUMENT)?;
            Ok(Array::from_ptr(ptr, true, self.clone()))
        }
    }

    /*
        Blob
    */

    pub(crate) fn new_pbblob(&self, bin: impl AsRef<[u8]>) -> pbblob {
        let bin = bin.as_ref();
        assert!(!bin.is_empty());
        unsafe { ffi::pbsession_NewBlob(self.ptr, bin.as_ptr() as _, bin.len() as pblong) }
    }
    pub(crate) unsafe fn get_blob_unchecked<'a>(&self, pbbin: pbblob) -> &'a [u8] {
        let ptr = ffi::pbsession_GetBlob(self.ptr, pbbin);
        let len = if !ptr.is_null() {
            ffi::pbsession_GetBlobLength(self.ptr, pbbin)
        } else {
            0
        };
        std::slice::from_raw_parts(ptr as *const u8, len as usize)
    }

    /*
        Date, Time and DateTime
    */

    pub(crate) fn new_pbdate(&self, dt: NaiveDate) -> pbdate {
        unsafe {
            let pbdt = ffi::pbsession_NewDate(self.ptr);
            let pbxr = ffi::pbsession_SetDate(
                self.ptr,
                pbdt,
                dt.year() as pbint,
                dt.month() as pbint,
                dt.day() as pbint
            );
            assert!(pbxr == PBXRESULT::OK);
            pbdt
        }
    }
    pub(crate) fn new_pbtime(&self, tm: NaiveTime) -> pbtime {
        unsafe {
            let pbtm = ffi::pbsession_NewTime(self.ptr);
            let pbxr = ffi::pbsession_SetTime(
                self.ptr,
                pbtm,
                tm.hour() as pbint,
                tm.minute() as pbint,
                tm.second() as pbdouble + tm.nanosecond() as pbdouble / 1000_000_000.0
            );
            assert!(pbxr == PBXRESULT::OK);
            pbtm
        }
    }
    pub(crate) fn new_pbdatetime(&self, dtt: NaiveDateTime) -> pbdatetime {
        unsafe {
            let pbdtt = ffi::pbsession_NewDateTime(self.ptr);
            let pbxr = ffi::pbsession_SetDateTime(
                self.ptr,
                pbdtt,
                dtt.year() as pbint,
                dtt.month() as pbint,
                dtt.day() as pbint,
                dtt.hour() as pbint,
                dtt.minute() as pbint,
                dtt.second() as pbdouble + dtt.nanosecond() as pbdouble / 1000_000_000.0
            );
            assert!(pbxr == PBXRESULT::OK);
            pbdtt
        }
    }
    pub(crate) unsafe fn get_date_unchecked(&self, pbdt: pbdate) -> NaiveDate {
        let mut year = 0;
        let mut month = 0;
        let mut day = 0;
        let pbxr = ffi::pbsession_SplitDate(self.ptr, pbdt, &mut year, &mut month, &mut day);
        assert!(pbxr == PBXRESULT::OK);
        NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap()
    }
    pub(crate) unsafe fn get_time_unchecked(&self, pbtm: pbtime) -> NaiveTime {
        let mut hour = 0;
        let mut minute = 0;
        let mut second = 0.0;
        let pbxr = ffi::pbsession_SplitTime(self.ptr, pbtm, &mut hour, &mut minute, &mut second);
        assert!(pbxr == PBXRESULT::OK);
        NaiveTime::from_hms_nano_opt(
            hour as u32,
            minute as u32,
            second as u32,
            ((second - second.trunc()) * 1000_000_000.0) as u32
        )
        .unwrap()
    }
    pub(crate) unsafe fn get_datetime_unchecked(&self, pbdtt: pbdatetime) -> NaiveDateTime {
        let mut year = 0;
        let mut month = 0;
        let mut day = 0;
        let mut hour = 0;
        let mut minute = 0;
        let mut second = 0.0;
        let pbxr = ffi::pbsession_SplitDateTime(
            self.ptr,
            pbdtt,
            &mut year,
            &mut month,
            &mut day,
            &mut hour,
            &mut minute,
            &mut second
        );
        assert!(pbxr == PBXRESULT::OK);
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap(),
            NaiveTime::from_hms_nano_opt(
                hour as u32,
                minute as u32,
                second as u32,
                ((second - second.trunc()) * 1000_000_000.0) as u32
            )
            .unwrap()
        )
    }

    /*
        Decimal number
    */
    pub(crate) fn new_pbdec(&self, dec: Decimal) -> pbdec {
        unsafe {
            let dec = PBString::from_str_unchecked(dec.to_string());
            let pbdec = ffi::pbsession_NewDecimal(self.ptr);
            let pbxr = ffi::pbsession_SetDecimal(self.ptr, pbdec, dec.as_ptr());
            assert!(pbxr == PBXRESULT::OK);
            pbdec
        }
    }
    pub(crate) unsafe fn get_dec_unchecked(&self, pbdec: pbdec) -> Decimal {
        let cstr = ffi::pbsession_GetDecimalString(self.ptr, pbdec);
        let str = String::from_pbstr_unchecked(cstr);
        ffi::pbsession_ReleaseDecimalString(self.ptr, cstr);
        str.parse().unwrap()
    }
}

/// 拥有所有权的Session对象
///
/// 此对象由[`VM`]创建,`drop`时会释放会话
pub struct OwnedSession<'vm> {
    session: Session,
    _marker: PhantomData<&'vm ()>
}

impl<'vm> OwnedSession<'vm> {
    pub(crate) unsafe fn from_ptr(ptr: pbsession) -> OwnedSession<'vm> {
        OwnedSession {
            session: Session::from_ptr(ptr),
            _marker: PhantomData
        }
    }

    /// 设置`DebugTrace`文件
    pub fn set_debug_trace(&self, traceFile: impl AsPBStr) {
        unsafe { ffi::pbsession_SetDebugTrace(self.session.ptr, traceFile.as_pbstr().as_ptr()) }
    }

    /// 释放会话
    pub fn release(self) {}
}

impl Drop for OwnedSession<'_> {
    fn drop(&mut self) { unsafe { ffi::pbsession_Release(self.session.ptr) } }
}

impl Deref for OwnedSession<'_> {
    type Target = Session;
    fn deref(&self) -> &Self::Target { &self.session }
}

/// 栈帧
///
/// 当**直接**从Rust端调用PB代码时一般需要创建栈帧来释放调用阶段产生的临时资源,比如创建的对象
///
/// # Examples
///
/// ```
/// //创建栈帧
/// let frame = LocalFrame::new(&session);
/// //手动退出栈帧
/// //pop调用不是必须的,变量drop时会自动退出
/// frame.pop();
/// ```
#[must_use]
pub struct LocalFrame {
    session: pbsession
}

impl LocalFrame {
    /// 创建栈帧
    pub fn new(session: &Session) -> LocalFrame {
        unsafe { ffi::pbsession_PushLocalFrame(session.ptr) }
        LocalFrame {
            session: session.ptr
        }
    }
    /// 手动退出栈帧
    pub fn pop(self) {}
}

impl Drop for LocalFrame {
    fn drop(&mut self) { unsafe { ffi::pbsession_PopLocalFrame(self.session) } }
}

/*
thread_local! {
    static SESSION: Cell<Option<Session>> = Cell::new(None)
}

/// 设置线程当前SESSION对象
pub(crate) fn set_current_session(session: Session) {
    SESSION.with(|s| {
        s.set(Some(session));
    });
}

/// 线程当前SESSION对象
pub fn current_session() -> Option<Session> { SESSION.with(|s| s.get()) }
*/
