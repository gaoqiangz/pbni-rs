use crate::{
    pbni::{bindings::*, invoker::GlobalFunction, value::FromValueOwned, *}, prelude::*
};
use std::{borrow::Cow, ops::Deref};

/// Session对象
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

    /// 克隆Session对象
    ///
    /// # Safety
    ///
    /// 此方法不能延长Session对象的生命周期,因此不能保证克隆后的Session对象始终有效,生命周期将始终与此对象一样
    pub unsafe fn clone(&self) -> Session {
        Session {
            ptr: self.ptr
        }
    }

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
        unsafe { ex.set_var_str_unchecked(pbstr!("text"), exstr)? };
        unsafe {
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
    pub fn get_enum_item_name<'a>(
        &'a self,
        enum_name: impl AsPBStr,
        item_value: pblong
    ) -> Option<&'a PBStr> {
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
    pub fn new_user_object<'a>(&'a self, cls_name: impl AsPBStr) -> Result<Object<'a>> {
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
    pub fn new_system_object<'a>(&'a self, cls_name: impl AsPBStr) -> Result<Object<'a>> {
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
    pub fn new_object<'a>(&'a self, cls_name: impl AsPBStr) -> Result<Object<'a>> {
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
    pub fn new_array<'a>(&'a self, item_type: ValueType) -> Result<Array<'a>> {
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
        &'a self,
        item_type: ValueType,
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
            let ptr =
                ffi::pbsession_NewBoundedSimpleArray(self.ptr, item_type, dims.len() as u16, dims.as_ptr())
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
    pub fn new_user_object_array<'a>(&'a self, cls_name: impl AsPBStr) -> Result<Array<'a>> {
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
    pub fn new_system_object_array<'a>(&'a self, cls_name: impl AsPBStr) -> Result<Array<'a>> {
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
    pub fn new_object_array<'a>(&'a self, cls_name: impl AsPBStr) -> Result<Array<'a>> {
        let cls_name = cls_name.as_pbstr();
        self.new_user_object_array(cls_name.as_ref()).or_else(|_| self.new_system_object_array(cls_name))
    }

    fn new_object_array_impl<'a>(&'a self, cls: pbclass) -> Result<Array<'a>> {
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
        &'a self,
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
        &'a self,
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
        &'a self,
        cls_name: impl AsPBStr,
        dims: &[(pblong, pblong)]
    ) -> Result<Array<'a>> {
        let cls_name = cls_name.as_pbstr();
        self.new_bounded_user_object_array(cls_name.as_ref(), dims)
            .or_else(|_| self.new_bounded_system_object_array(cls_name, dims))
    }

    fn new_bounded_object_array_impl<'a>(
        &'a self,
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

    #[cfg(feature = "datetime")]
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
    #[cfg(feature = "datetime")]
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
    #[cfg(feature = "datetime")]
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
    #[cfg(feature = "datetime")]
    pub(crate) unsafe fn get_date_unchecked(&self, pbdt: pbdate) -> NaiveDate {
        let mut year = 0;
        let mut month = 0;
        let mut day = 0;
        let pbxr = ffi::pbsession_SplitDate(self.ptr, pbdt, &mut year, &mut month, &mut day);
        assert!(pbxr == PBXRESULT::OK);
        NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap()
    }
    #[cfg(feature = "datetime")]
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
    #[cfg(feature = "datetime")]
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
    #[cfg(feature = "decimal")]
    pub(crate) fn new_pbdec(&self, dec: Decimal) -> pbdec {
        unsafe {
            let dec = PBString::from_str_unchecked(dec.to_string());
            let pbdec = ffi::pbsession_NewDecimal(self.ptr);
            let pbxr = ffi::pbsession_SetDecimal(self.ptr, pbdec, dec.as_ptr());
            assert!(pbxr == PBXRESULT::OK);
            pbdec
        }
    }
    #[cfg(feature = "decimal")]
    pub(crate) unsafe fn get_dec_unchecked(&self, pbdec: pbdec) -> Decimal {
        let cstr = ffi::pbsession_GetDecimalString(self.ptr, pbdec);
        let str = String::from_pbstr_unchecked(cstr);
        ffi::pbsession_ReleaseDecimalString(self.ptr, cstr);
        str.parse().unwrap()
    }
}

/*
    Global variable
*/

/// 全局变量ID抽象
pub trait GlobalVarId {
    fn var_id(&self, session: &Session) -> FieldId;
}

impl<T: AsPBStr> GlobalVarId for T {
    #[inline]
    fn var_id(&self, session: &Session) -> FieldId {
        let pbstr = self.as_pbstr();
        session
            .get_var_id(pbstr.as_ref())
            .ok_or_else(|| format!("invalid global var {}", pbstr.to_string_lossy()))
            .unwrap()
    }
}
impl GlobalVarId for FieldId {
    #[inline]
    fn var_id(&self, _session: &Session) -> FieldId { *self }
}
impl GlobalVarId for Option<FieldId> {
    #[inline]
    fn var_id(&self, _session: &Session) -> FieldId { self.unwrap() }
}

impl Session {
    /// 获取全局变量ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = session.get_var_id("gs_var").unwrap();
    /// session.set_var_str(fid,"rust");
    /// ```
    pub fn get_var_id(&self, name: impl AsPBStr) -> Option<FieldId> {
        unsafe {
            let fid = ffi::pbsession_GetGlobalVarID(self.ptr, name.as_pbstr().as_ptr());
            if fid.is_undefined() {
                None
            } else {
                Some(fid)
            }
        }
    }

    /// 判断是否存在指定全局变量
    ///
    /// # Examples
    ///
    /// ```
    /// if session.has_var("gs_var") {
    ///     session.set_var_str("gs_var","rust");
    /// }
    /// ```
    pub fn has_var(&self, name: impl AsPBStr) -> bool { self.get_var_id(name).is_some() }

    /// 获取指定全局变量类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// if session.get_var_type("gs_var") == ValueType::String {
    ///     session.set_var_str("gs_var","rust");
    /// }
    /// ```
    pub fn get_var_type(&self, fid: impl GlobalVarId) -> ValueType {
        unsafe { ffi::pbsession_GetGlobalVarType(self.ptr, fid.var_id(self)) }
    }

    /// 判断指定全局变量是否为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_null(&self, fid: impl GlobalVarId) -> bool {
        unsafe { ffi::pbsession_IsGlobalVarNull(self.ptr, fid.var_id(self)).into() }
    }

    /// 判断指定全局变量是否为数组类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_array(&self, fid: impl GlobalVarId) -> bool {
        unsafe { ffi::pbsession_IsGlobalVarArray(self.ptr, fid.var_id(self)).into() }
    }

    /// 判断指定全局变量是否为对象类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_object(&self, fid: impl GlobalVarId) -> bool {
        unsafe { ffi::pbsession_IsGlobalVarObject(self.ptr, fid.var_id(self)).into() }
    }

    /// 获取`int`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_int(&self, fid: impl GlobalVarId) -> Option<pbint> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Int);
        unsafe { self.get_var_int_unchecked(fid) }
    }

    /// 获取`int`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_int_unchecked(&self, fid: impl GlobalVarId) -> Option<pbint> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetIntGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`uint`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_uint(&self, fid: impl GlobalVarId) -> Option<pbuint> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Uint);
        unsafe { self.get_var_uint_unchecked(fid) }
    }

    /// 获取`uint`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_uint_unchecked(&self, fid: impl GlobalVarId) -> Option<pbuint> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetUintGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`long`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_long(&self, fid: impl GlobalVarId) -> Option<pblong> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Long);
        unsafe { self.get_var_long_unchecked(fid) }
    }

    /// 获取`long`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_long_unchecked(&self, fid: impl GlobalVarId) -> Option<pblong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetLongGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`ulong`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_ulong(&self, fid: impl GlobalVarId) -> Option<pbulong> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Ulong);
        unsafe { self.get_var_ulong_unchecked(fid) }
    }

    /// 获取`ulong`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_ulong_unchecked(&self, fid: impl GlobalVarId) -> Option<pbulong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetUlongGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`longlong`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_longlong(&self, fid: impl GlobalVarId) -> Option<pblonglong> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::LongLong);
        unsafe { self.get_var_longlong_unchecked(fid) }
    }

    /// 获取`longlong`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_longlong_unchecked(&self, fid: impl GlobalVarId) -> Option<pblonglong> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetLongLongGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`real`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_real(&self, fid: impl GlobalVarId) -> Option<pbreal> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Real);
        unsafe { self.get_var_real_unchecked(fid) }
    }

    /// 获取`real`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_real_unchecked(&self, fid: impl GlobalVarId) -> Option<pbreal> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetRealGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`double`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_double(&self, fid: impl GlobalVarId) -> Option<pbdouble> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Double);
        unsafe { self.get_var_double_unchecked(fid) }
    }

    /// 获取`double`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_double_unchecked(&self, fid: impl GlobalVarId) -> Option<pbdouble> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDoubleGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`decimal`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn get_var_dec(&self, fid: impl GlobalVarId) -> Option<Decimal> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Decimal);
        unsafe { self.get_var_dec_unchecked(fid) }
    }

    /// 获取`decimal`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn get_var_dec_unchecked(&self, fid: impl GlobalVarId) -> Option<Decimal> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDecGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.get_dec_unchecked(v))
        }
    }

    /// 获取`string`类型全局变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_str<'a>(&'a self, fid: impl GlobalVarId) -> Option<&'a PBStr> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::String);
        self.get_var_str_unchecked(fid)
    }

    /// 获取`string`类型全局变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_str_unchecked<'a>(&'a self, fid: impl GlobalVarId) -> Option<&'a PBStr> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetStringGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            self.get_string_unchecked(v)
        }
    }

    /// 获取`string`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_string(&self, fid: impl GlobalVarId) -> Option<PBString> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::String);
        unsafe { self.get_var_string_unchecked(fid) }
    }

    /// 获取`string`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_string_unchecked(&self, fid: impl GlobalVarId) -> Option<PBString> {
        self.get_var_str_unchecked(fid).map(|v| v.to_ucstring())
    }

    /// 获取`boolean`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_bool(&self, fid: impl GlobalVarId) -> Option<bool> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Boolean);
        unsafe { self.get_var_bool_unchecked(fid) }
    }

    /// 获取`boolean`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_bool_unchecked(&self, fid: impl GlobalVarId) -> Option<bool> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetBoolGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v.into())
        }
    }

    /// 获取`blob`类型全局变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_blob<'a>(&'a self, fid: impl GlobalVarId) -> Option<&'a [u8]> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Blob);
        self.get_var_blob_unchecked(fid)
    }

    /// 获取`blob`类型全局变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_blob_unchecked<'a>(&'a self, fid: impl GlobalVarId) -> Option<&'a [u8]> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetBlobGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.get_blob_unchecked(v))
        }
    }

    /// 获取`date`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_var_date(&self, fid: impl GlobalVarId) -> Option<NaiveDate> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Date);
        unsafe { self.get_var_date_unchecked(fid) }
    }

    /// 获取`date`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_var_date_unchecked(&self, fid: impl GlobalVarId) -> Option<NaiveDate> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDateGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.get_date_unchecked(v))
        }
    }

    /// 获取`time`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_var_time(&self, fid: impl GlobalVarId) -> Option<NaiveTime> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Time);
        unsafe { self.get_var_time_unchecked(fid) }
    }

    /// 获取`time`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_var_time_unchecked(&self, fid: impl GlobalVarId) -> Option<NaiveTime> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetTimeGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.get_time_unchecked(v))
        }
    }

    /// 获取`datetime`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn get_var_datetime(&self, fid: impl GlobalVarId) -> Option<NaiveDateTime> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::DateTime);
        unsafe { self.get_var_datetime_unchecked(fid) }
    }

    /// 获取`datetime`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn get_var_datetime_unchecked(&self, fid: impl GlobalVarId) -> Option<NaiveDateTime> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetDateTimeGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(self.get_datetime_unchecked(v))
        }
    }

    /// 获取`char`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_char(&self, fid: impl GlobalVarId) -> Option<PBChar> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Char);
        unsafe { self.get_var_char_unchecked(fid) }
    }

    /// 获取`char`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_char_unchecked(&self, fid: impl GlobalVarId) -> Option<PBChar> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetCharGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取`byte`类型全局变量值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn get_var_byte(&self, fid: impl GlobalVarId) -> Option<pbbyte> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Byte);
        unsafe { self.get_var_byte_unchecked(fid) }
    }

    /// 获取`byte`类型全局变量值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn get_var_byte_unchecked(&self, fid: impl GlobalVarId) -> Option<pbbyte> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetByteGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(v)
        }
    }

    /// 获取对象类型全局变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_object<'a>(&'a self, fid: impl GlobalVarId) -> Option<Object<'a>> {
        let fid = fid.var_id(self);
        assert!(self.is_var_object(fid));
        self.get_var_object_unchecked(fid)
    }

    /// 获取对象类型全局变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_object_unchecked<'a>(&'a self, fid: impl GlobalVarId) -> Option<Object<'a>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetObjectGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Object::from_ptr(v, self.clone()))
        }
    }

    /// 获取数组类型全局变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_array<'a>(&'a self, fid: impl GlobalVarId) -> Option<Array<'a>> {
        let fid = fid.var_id(self);
        assert!(self.is_var_array(fid));
        self.get_var_array_unchecked(fid)
    }

    /// 获取数组类型全局变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_array_unchecked<'a>(&'a self, fid: impl GlobalVarId) -> Option<Array<'a>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetArrayGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Array::from_ptr(v, self.is_var_object(fid), self.clone()))
        }
    }

    /// 获取`any`类型全局变量的引用
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_any<'a>(&'a self, fid: impl GlobalVarId) -> Option<Value<'a>> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Any);
        self.get_var_any_unchecked(fid)
    }

    /// 获取`any`类型全局变量的引用,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// - 类型不兼容时可能会出现未定义行为
    /// - 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    pub unsafe fn get_var_any_unchecked<'a>(&'a self, fid: impl GlobalVarId) -> Option<Value<'a>> {
        let mut is_null = Default::default();
        let v = ffi::pbsession_GetPBAnyGlobalVar(self.ptr, fid.var_id(self), &mut is_null);
        if is_null == true {
            None
        } else {
            Some(Value::from_ptr(v, self.clone()))
        }
    }

    /// 设置全局变量的值为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn set_var_to_null(&self, fid: impl GlobalVarId) {
        unsafe { ffi::pbsession_SetGlobalVarToNull(self.ptr, fid.var_id(self)) }
    }

    /// 设置`int`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_int(&mut self, fid: impl GlobalVarId, value: pbint) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Int);
        unsafe { ffi::pbsession_SetIntGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`int`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_int_unchecked(&mut self, fid: impl GlobalVarId, value: pbint) -> Result<()> {
        ffi::pbsession_SetIntGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`uint`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_uint(&mut self, fid: impl GlobalVarId, value: pbuint) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Uint);
        unsafe { ffi::pbsession_SetUintGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`uint`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_uint_unchecked(&mut self, fid: impl GlobalVarId, value: pbuint) -> Result<()> {
        ffi::pbsession_SetUintGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`long`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_long(&mut self, fid: impl GlobalVarId, value: pblong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Long);
        unsafe { ffi::pbsession_SetLongGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`long`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_long_unchecked(&mut self, fid: impl GlobalVarId, value: pblong) -> Result<()> {
        ffi::pbsession_SetLongGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`ulong`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_ulong(&mut self, fid: impl GlobalVarId, value: pbulong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Ulong);
        unsafe { ffi::pbsession_SetUlongGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`ulong`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_ulong_unchecked(&mut self, fid: impl GlobalVarId, value: pbulong) -> Result<()> {
        ffi::pbsession_SetUlongGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`longlong`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_longlong(&mut self, fid: impl GlobalVarId, value: pblonglong) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::LongLong);
        unsafe { ffi::pbsession_SetLongLongGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`longlong`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_longlong_unchecked(
        &mut self,
        fid: impl GlobalVarId,
        value: pblonglong
    ) -> Result<()> {
        ffi::pbsession_SetLongLongGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`real`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_real(&mut self, fid: impl GlobalVarId, value: pbreal) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Real);
        unsafe { ffi::pbsession_SetRealGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`real`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_real_unchecked(&mut self, fid: impl GlobalVarId, value: pbreal) -> Result<()> {
        ffi::pbsession_SetRealGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`double`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_double(&mut self, fid: impl GlobalVarId, value: pbdouble) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Double);
        unsafe { ffi::pbsession_SetDoubleGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`double`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_double_unchecked(&mut self, fid: impl GlobalVarId, value: pbdouble) -> Result<()> {
        ffi::pbsession_SetDoubleGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`decimal`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "decimal")]
    pub fn set_var_dec(&mut self, fid: impl GlobalVarId, value: Decimal) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Decimal);
        unsafe { ffi::pbsession_SetDecGlobalVar(self.ptr, fid, self.new_pbdec(value)).into() }
    }

    /// 设置`decimal`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "decimal")]
    pub unsafe fn set_var_dec_unchecked(&mut self, fid: impl GlobalVarId, value: Decimal) -> Result<()> {
        ffi::pbsession_SetDecGlobalVar(self.ptr, fid.var_id(self), self.new_pbdec(value)).into()
    }

    /// 设置`string`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_str(&mut self, fid: impl GlobalVarId, value: impl AsPBStr) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::String);
        unsafe { ffi::pbsession_SetStringGlobalVar(self.ptr, fid, value.as_pbstr().as_ptr()).into() }
    }

    /// 设置`string`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_str_unchecked(&mut self, fid: impl GlobalVarId, value: impl AsPBStr) -> Result<()> {
        ffi::pbsession_SetStringGlobalVar(self.ptr, fid.var_id(self), value.as_pbstr().as_ptr()).into()
    }

    /// 设置`boolean`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_bool(&mut self, fid: impl GlobalVarId, value: bool) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Boolean);
        unsafe { ffi::pbsession_SetBoolGlobalVar(self.ptr, fid, value.into()).into() }
    }

    /// 设置`boolean`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_bool_unchecked(&mut self, fid: impl GlobalVarId, value: bool) -> Result<()> {
        ffi::pbsession_SetBoolGlobalVar(self.ptr, fid.var_id(self), value.into()).into()
    }

    /// 设置`blob`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_blob(&mut self, fid: impl GlobalVarId, value: impl AsRef<[u8]>) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Blob);
        unsafe { ffi::pbsession_SetBlobGlobalVar(self.ptr, fid, self.new_pbblob(value)).into() }
    }

    /// 设置`blob`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_blob_unchecked(
        &mut self,
        fid: impl GlobalVarId,
        value: impl AsRef<[u8]>
    ) -> Result<()> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(PBXRESULT::E_OUTOF_MEMORY);
        }
        ffi::pbsession_SetBlobGlobalVar(self.ptr, fid.var_id(self), self.new_pbblob(value)).into()
    }

    /// 设置`date`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_var_date(&mut self, fid: impl GlobalVarId, value: NaiveDate) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Date);
        unsafe { ffi::pbsession_SetDateGlobalVar(self.ptr, fid, self.new_pbdate(value)).into() }
    }

    /// 设置`date`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_var_date_unchecked(&mut self, fid: impl GlobalVarId, value: NaiveDate) -> Result<()> {
        ffi::pbsession_SetDateGlobalVar(self.ptr, fid.var_id(self), self.new_pbdate(value)).into()
    }

    /// 设置`time`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_var_time(&mut self, fid: impl GlobalVarId, value: NaiveTime) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Time);
        unsafe { ffi::pbsession_SetTimeGlobalVar(self.ptr, fid, self.new_pbtime(value)).into() }
    }

    /// 设置`time`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_var_time_unchecked(&mut self, fid: impl GlobalVarId, value: NaiveTime) -> Result<()> {
        ffi::pbsession_SetTimeGlobalVar(self.ptr, fid.var_id(self), self.new_pbtime(value)).into()
    }

    /// 设置`datetime`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    #[cfg(feature = "datetime")]
    pub fn set_var_datetime(&mut self, fid: impl GlobalVarId, value: NaiveDateTime) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::DateTime);
        unsafe { ffi::pbsession_SetDateTimeGlobalVar(self.ptr, fid, self.new_pbdatetime(value)).into() }
    }

    /// 设置`datetime`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    #[cfg(feature = "datetime")]
    pub unsafe fn set_var_datetime_unchecked(
        &mut self,
        fid: impl GlobalVarId,
        value: NaiveDateTime
    ) -> Result<()> {
        ffi::pbsession_SetDateTimeGlobalVar(self.ptr, fid.var_id(self), self.new_pbdatetime(value)).into()
    }

    /// 设置`char`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_char(&mut self, fid: impl GlobalVarId, value: PBChar) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Char);
        unsafe { ffi::pbsession_SetCharGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`char`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_char_unchecked(&mut self, fid: impl GlobalVarId, value: PBChar) -> Result<()> {
        ffi::pbsession_SetCharGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置`byte`类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_byte(&mut self, fid: impl GlobalVarId, value: pbbyte) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.get_var_type(fid) == ValueType::Byte);
        unsafe { ffi::pbsession_SetByteGlobalVar(self.ptr, fid, value).into() }
    }

    /// 设置`byte`类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_byte_unchecked(&mut self, fid: impl GlobalVarId, value: pbbyte) -> Result<()> {
        ffi::pbsession_SetByteGlobalVar(self.ptr, fid.var_id(self), value).into()
    }

    /// 设置对象类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_object(&mut self, fid: impl GlobalVarId, value: &Object) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.is_var_object(fid));
        unsafe { ffi::pbsession_SetObjectGlobalVar(self.ptr, fid, value.as_ptr()).into() }
    }

    /// 设置对象类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_object_unchecked(&mut self, fid: impl GlobalVarId, value: &Object) -> Result<()> {
        ffi::pbsession_SetObjectGlobalVar(self.ptr, fid.var_id(self), value.as_ptr()).into()
    }

    /// 设置数组类型全局变量的值
    ///
    /// # Panics
    ///
    /// 访问不存在的变量或类型不匹配时会触发Panic
    pub fn set_var_array(&mut self, fid: impl GlobalVarId, value: &Array) -> Result<()> {
        let fid = fid.var_id(self);
        assert!(self.is_var_array(fid));
        unsafe { ffi::pbsession_SetArrayGlobalVar(self.ptr, fid, value.as_ptr()).into() }
    }

    /// 设置数组类型全局变量的值,不检查类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Safety
    ///
    /// 类型不兼容时可能会出现未定义行为
    pub unsafe fn set_var_array_unchecked(&mut self, fid: impl GlobalVarId, value: &Array) -> Result<()> {
        ffi::pbsession_SetArrayGlobalVar(self.ptr, fid.var_id(self), value.as_ptr()).into()
    }
}

/*
    Method calling
*/

/// 函数名抽象
pub trait AsMethodName {
    fn as_method_name(&self) -> (Cow<'_, PBStr>, Cow<'_, PBStr>);
}

impl<T: AsPBStr> AsMethodName for T {
    #[inline]
    fn as_method_name(&self) -> (Cow<'_, PBStr>, Cow<'_, PBStr>) { (self.as_pbstr(), "".as_pbstr()) }
}

impl<T: AsPBStr, S: AsPBStr> AsMethodName for (T, S) {
    #[inline]
    fn as_method_name(&self) -> (Cow<'_, PBStr>, Cow<'_, PBStr>) {
        let (name, sign) = self;
        (name.as_pbstr(), sign.as_pbstr())
    }
}

/// 全局函数ID
#[derive(Clone, Copy)]
pub struct GlobalFunctionId {
    pub(crate) cls: pbclass,
    pub(crate) mid: MethodId
}

/// 全局函数ID抽象
pub trait AsGlobalFunctionId {
    fn as_mid(&self, session: &Session) -> Result<GlobalFunctionId>;
}

impl<T: AsMethodName> AsGlobalFunctionId for T {
    #[inline]
    fn as_mid(&self, session: &Session) -> Result<GlobalFunctionId> {
        session.get_function_id(self.as_method_name()).ok_or(PBXRESULT::E_INVALID_METHOD_ID)
    }
}
impl AsGlobalFunctionId for GlobalFunctionId {
    #[inline]
    fn as_mid(&self, _session: &Session) -> Result<GlobalFunctionId> { Ok(*self) }
}
impl AsGlobalFunctionId for Option<GlobalFunctionId> {
    #[inline]
    fn as_mid(&self, _session: &Session) -> Result<GlobalFunctionId> {
        self.ok_or(PBXRESULT::E_INVALID_METHOD_ID)
    }
}

impl Session {
    /// 获取用户定义全局函数ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = session.get_user_function_id("gf_test").unwrap();
    /// let invoker = session.begin_invoke_function(fid).unwrap();
    /// invoker.invoke();
    /// ```
    pub fn get_user_function_id(&self, name: impl AsMethodName) -> Option<GlobalFunctionId> {
        let (name, sign) = name.as_method_name();
        if let Some(group) = self.find_group(name.as_ref(), GroupType::Function) {
            self.find_class(group, name.as_ref()).and_then(|cls| {
                self.get_method_id(cls, name.as_ref(), RoutineType::Function, sign, true).map(|mid| {
                    GlobalFunctionId {
                        cls,
                        mid
                    }
                })
            })
        } else {
            None
        }
    }

    /// 获取系统全局函数ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = session.get_system_function_id(("MessageBox","ISS")).unwrap();
    /// let invoker = session.begin_invoke_function(fid).unwrap();
    /// invoker.arg(0).set_str("title");
    /// invoker.arg(1).set_str("content");
    /// invoker.invoke();
    /// ```
    pub fn get_system_function_id(&self, name: impl AsMethodName) -> Option<GlobalFunctionId> {
        let (name, sign) = name.as_method_name();
        self.find_class(self.get_system_group(), "SystemFunctions").and_then(|cls| {
            self.get_method_id(cls, name, RoutineType::Function, sign, true).map(|mid| {
                GlobalFunctionId {
                    cls,
                    mid
                }
            })
        })
    }

    /// 获取任意全局函数ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = session.get_function_id(("MessageBox","ISS")).unwrap();
    /// let invoker = session.begin_invoke_function(fid).unwrap();
    /// invoker.arg(0).set_str("title");
    /// invoker.arg(1).set_str("content");
    /// invoker.invoke();
    /// ```
    pub fn get_function_id(&self, name: impl AsMethodName) -> Option<GlobalFunctionId> {
        let (name, sign) = name.as_method_name();
        self.get_user_function_id((name.as_ref(), sign.as_ref()))
            .or_else(|| self.get_system_function_id((name, sign)))
    }

    /// 调用全局函数
    ///
    /// # Examples
    ///
    /// ```
    /// let rv: pbint = session.invoke_function(("MessageBox","ISS"),pbargs!["title","content"]).unwrap();
    /// ```
    pub fn invoke_function<F, R>(&self, mid: impl AsGlobalFunctionId, arg_cb: F) -> Result<R>
    where
        F: FnOnce(Arguments) -> Result<()>,
        R: FromValueOwned
    {
        let invoker = self.begin_invoke_function(mid)?;
        arg_cb(invoker.args())?;
        let rv = invoker.invoke()?;
        R::from_value(Some(rv))
    }

    /// 初始化全局函数调用上下文
    ///
    /// # Examples
    ///
    /// ```
    /// let invoker = session.begin_invoke_function(("MessageBox","ISS")).unwrap();
    /// invoker.arg(0).set_str("title");
    /// invoker.arg(1).set_str("content");
    /// invoker.invoke();
    /// ```
    pub fn begin_invoke_function<'a>(
        &'a self,
        mid: impl AsGlobalFunctionId
    ) -> Result<Invoker<GlobalFunction<'a>>> {
        let mid = mid.as_mid(self)?;
        let ci = unsafe { CallInfo::new(mid.cls, mid.mid, self.clone())? };
        Ok(Invoker::<GlobalFunction>::new(GlobalFunction::new(mid.cls), ci))
    }

    pub(crate) fn get_method_id(
        &self,
        cls: pbclass,
        methodName: impl AsPBStr,
        rt: RoutineType,
        signature: impl AsPBStr,
        publicOnly: bool
    ) -> Option<MethodId> {
        unsafe {
            let mid = ffi::pbsession_GetMethodID(
                self.ptr,
                cls,
                methodName.as_pbstr().as_ptr(),
                rt,
                signature.as_pbstr().as_ptr(),
                publicOnly.into()
            );
            if mid.is_undefined() {
                None
            } else {
                Some(mid)
            }
        }
    }
    pub(crate) fn find_matching_function(
        &self,
        cls: pbclass,
        methodName: impl AsPBStr,
        rt: RoutineType,
        readableSignature: impl AsPBStr
    ) -> Option<MethodId> {
        unsafe {
            let mid = ffi::pbsession_FindMatchingFunction(
                self.ptr,
                cls,
                methodName.as_pbstr().as_ptr(),
                rt,
                readableSignature.as_pbstr().as_ptr()
            );
            if mid.is_undefined() {
                None
            } else {
                Some(mid)
            }
        }
    }
    pub(crate) fn get_method_id_by_event_id(&self, cls: pbclass, eventID: impl AsPBStr) -> Option<MethodId> {
        unsafe {
            let mid = ffi::pbsession_GetMethodIDByEventID(self.ptr, cls, eventID.as_pbstr().as_ptr());
            if mid.is_undefined() {
                None
            } else {
                Some(mid)
            }
        }
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
pub struct LocalFrame<'session> {
    session: &'session Session
}

impl<'session> LocalFrame<'session> {
    /// 创建栈帧
    pub fn new(session: &Session) -> LocalFrame {
        unsafe { ffi::pbsession_PushLocalFrame(session.ptr) }
        LocalFrame {
            session
        }
    }
    /// 手动退出栈帧
    pub fn pop(self) {}
}

impl<'session> Drop for LocalFrame<'session> {
    fn drop(&mut self) { unsafe { ffi::pbsession_PopLocalFrame(self.session.ptr) } }
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
