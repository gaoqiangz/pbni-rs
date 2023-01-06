use crate::{
    prelude::*, syslib::{
        bindings::*, value::{FromValueOwned, Value}, *
    }
};
use std::{borrow::Cow, ops::Deref};

/// Session对象
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Session {
    ptr: POB_THIS
}

impl Session {
    pub(crate) unsafe fn from_ptr(ptr: POB_THIS) -> Session {
        Session {
            ptr
        }
    }
    pub(crate) fn as_ptr(&self) -> POB_THIS { self.ptr }

    //TODO
    /// 判断是否有重启Session的请求 (在PowerScript中调用了`Restart`函数)
    //pub fn restart_requested(&self) -> bool { unsafe { ffi::pbsession_RestartRequested(self.ptr).into() } }

    //TODO
    /// 是否创建了可视化对象 (打开了顶层窗口)
    //pub fn has_visual_object(&self) -> bool { unsafe { ffi::pbsession_HasPBVisualObject(self.ptr).into() } }

    /// 重启`Session`
    pub fn restart(&self) {
        unsafe {
            API.ob_mgr_restart(self.ptr);
        }
    }

    /// 终止`Session`
    pub fn halt(&self, close_event: bool) {
        unsafe {
            API.ot_halt(self.ptr, close_event as BOOL);
        }
    }

    //TODO
    /// 处理PB消息
    ///
    /// # Notice
    ///
    /// 开启了消息循环后,需要处理PB的消息以执行PowerScript中的`Post`调用
    //pub fn process_message(&self) -> bool { unsafe { ffi::pbsession_ProcessPBMessage(self.ptr).into() } }

    /*
    TODO
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
    */

    /*
        Exception
    */

    /// 检查当前是否有异常未处理
    pub fn has_exception(&self) -> bool { unsafe { !(&*self.ptr).thrown_exception.is_null() } }

    /*
    TODO
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
    */
    /*
        Prop
    */

    /*
        Enum
    */

    /*
    TODO
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
    */

    /*
        Value
    */

    /// 创建一个新值
    pub fn new_value(&self) -> Value<'static> { unsafe { Value::new(self.clone()) } }

    /*
        Object
    */

    /*
    TODO

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
    */

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
            let ty = OB_CLASS_HNDL {
                group_hndl: (&*self.ptr).appclshndl.group_hndl,
                class_id: item_type as OB_CLASS_ID
            };
            let var_info = array_var_info(item_type);
            let ptr = API.ot_array_create_unbounded(self.ptr, ty, var_info);
            if ptr.is_null() {
                return Err(PBRESULT::E_INVALID_ARGUMENT);
            }
            Ok(Array::take_ptr(ptr, false, self.clone()))
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
            let ty = OB_CLASS_HNDL {
                group_hndl: (&*self.ptr).appclshndl.group_hndl,
                class_id: item_type as OB_CLASS_ID
            };
            let var_info = array_var_info(item_type);
            let mut num_items = 0;
            let mut bounds = Vec::with_capacity(dims.len() * 2);
            for (lower, upper) in dims {
                if upper < lower {
                    return Err(PBRESULT::E_OUT_OF_BOUNDS);
                }
                bounds.push(*lower);
                bounds.push(*upper);
                num_items += (upper - lower) as pbulong + 1;
            }
            let ptr = API.ot_array_create_bounded(
                self.ptr,
                num_items,
                ty,
                var_info,
                dims.len() as USHORT,
                bounds.as_mut_ptr()
            );
            if ptr.is_null() {
                return Err(PBRESULT::E_INVALID_ARGUMENT);
            }
            Ok(Array::take_ptr(ptr, false, self.clone()))
        }
    }

    /*
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
    */
    /*
        Blob
    */

    pub(crate) fn new_pbblob(&self, bin: impl AsRef<[u8]>) -> PSH_BINARY {
        let bin = bin.as_ref();
        assert!(!bin.is_empty());
        unsafe {
            let blb = API.ob_alloc_blob(self.ptr, bin.len() as ULONG);
            ptr::copy_nonoverlapping(bin.as_ptr(), blb as *mut u8, bin.len());
            blb
        }
    }
    pub(crate) unsafe fn get_blob_unchecked<'a>(&self, pbbin: PSH_BINARY) -> &'a [u8] {
        let pbbin = &*pbbin;
        std::slice::from_raw_parts(pbbin.data.as_ptr() as *const u8, pbbin.len as usize)
    }

    /*
        Date, Time and DateTime
    */

    pub(crate) fn new_pbdate(&self, dt: NaiveDate) -> PSH_TIME {
        unsafe {
            let pbtm = &mut *API.ob_alloc_time(self.ptr);
            pbtm.tm_year = dt.year() as i16;
            pbtm.tm_mon = dt.month() as u8;
            pbtm.tm_mday = dt.day() as u8;
            pbtm
        }
    }
    pub(crate) fn new_pbtime(&self, tm: NaiveTime) -> PSH_TIME {
        unsafe {
            let pbtm = &mut *API.ob_alloc_time(self.ptr);
            pbtm.tm_hour = tm.hour() as u8;
            pbtm.tm_min = tm.minute() as u8;
            pbtm.tm_sec = tm.second() as u8;
            pbtm.tm_msec = (tm.nanosecond() / 1000) as i32;
            pbtm
        }
    }
    pub(crate) fn new_pbdatetime(&self, dtt: NaiveDateTime) -> PSH_TIME {
        unsafe {
            let pbtm = &mut *API.ob_alloc_time(self.ptr);
            pbtm.tm_year = dtt.year() as i16;
            pbtm.tm_mon = dtt.month() as u8;
            pbtm.tm_mday = dtt.day() as u8;
            pbtm.tm_hour = dtt.hour() as u8;
            pbtm.tm_min = dtt.minute() as u8;
            pbtm.tm_sec = dtt.second() as u8;
            pbtm.tm_msec = (dtt.nanosecond() / 1000) as i32;
            pbtm
        }
    }
    pub(crate) unsafe fn get_date_unchecked(&self, pbtm: PSH_TIME) -> NaiveDate {
        let pbtm = &*pbtm;
        NaiveDate::from_ymd_opt(pbtm.tm_year as i32, pbtm.tm_mon as u32, pbtm.tm_mday as u32).unwrap()
    }
    pub(crate) unsafe fn get_time_unchecked(&self, pbtm: PSH_TIME) -> NaiveTime {
        let pbtm = &*pbtm;
        NaiveTime::from_hms_micro_opt(
            pbtm.tm_hour as u32,
            pbtm.tm_min as u32,
            pbtm.tm_sec as u32,
            pbtm.tm_msec as u32
        )
        .unwrap()
    }
    pub(crate) unsafe fn get_datetime_unchecked(&self, pbtm: PSH_TIME) -> NaiveDateTime {
        let pbtm = &*pbtm;
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(pbtm.tm_year as i32, pbtm.tm_mon as u32, pbtm.tm_mday as u32).unwrap(),
            NaiveTime::from_hms_micro_opt(
                pbtm.tm_hour as u32,
                pbtm.tm_min as u32,
                pbtm.tm_sec as u32,
                pbtm.tm_msec as u32
            )
            .unwrap()
        )
    }

    /*
        Decimal number
    */

    // fields description of SH_DEC structure
    // flags:
    // bit offset   desc
    // 0            sign, 1 is negative
    // 1            overflow
    // 2            underflow
    // 3            divide by zero
    // 4            undetermined such as 0/0
    // 5-7          unused
    // 8-12         precision 0-30, the max digits after zero point is 30. 31 is a special number
    // 13-15        unused
    //
    // v:
    // v[0] is lowest
    // v[6] is highest
    // Supported value
    //  0.000,000,000,000,000,000,000,000,000,001 to  999,999,999,999,999,999,999,999,999,999
    //  0
    // -0.000,000,000,000,000,000,000,000,000,001 to -999,999,999,999,999,999,999,999,999,999
    //
    // v: 按原码表示法存储14个字节(112位),最高字节存储符号位

    pub(crate) fn new_pbdec(&self, dec: Decimal) -> PSH_DEC {
        unsafe {
            let pbdec = API.ob_alloc_dec(self.ptr);
            if API.version() >= 110 {
                let nag = dec.is_sign_negative();
                let num = if nag {
                    !dec.mantissa() + 1
                } else {
                    dec.mantissa()
                } as u128;
                let num = num.to_le_bytes();
                let pbdec = &mut *pbdec;
                ptr::copy_nonoverlapping(
                    num.as_ptr(),
                    ptr::addr_of_mut!(pbdec.v) as *mut u8,
                    DEC_ARRAY_LEN as usize * mem::size_of::<USHORT>()
                );
                pbdec.flags =
                    (nag as USHORT) << DEC_SIGN_SHIFT | (dec.scale() as USHORT) << DEC_PRECISION_SHIFT;
                pbdec
            } else {
                //FIXME
                //PB11以下版本存储格式不一样,使用字符串转换
                let decstr = dec.to_string();
                let decstr = decstr.as_pbstr();
                API.shAsciiToDec(pbdec, decstr.as_ptr() as LPTSTR)
            }
        }
    }
    pub(crate) unsafe fn get_dec_unchecked(&self, pbdec: PSH_DEC) -> Decimal {
        if API.version() >= 110 {
            let pbdec = &*pbdec;
            let neg = (pbdec.flags & (1 << DEC_SIGN_SHIFT)) != 0;
            let scale = ((pbdec.flags & 0x1f00) >> DEC_PRECISION_SHIFT) as u32;
            let mut buf: [u8; mem::size_of::<u128>()] = mem::zeroed();
            ptr::copy_nonoverlapping(
                ptr::addr_of!(pbdec.v) as *const u8,
                buf.as_mut_ptr(),
                DEC_ARRAY_LEN as usize * mem::size_of::<USHORT>()
            );
            let num = u128::from_le_bytes(buf);
            let num = if neg {
                !num + 1
            } else {
                num
            } as i128;
            Decimal::from_i128_with_scale(num, scale)
        } else {
            //FIXME
            //PB11以下版本存储格式不一样,使用字符串转换
            let mut buf: [PBChar; 64] = mem::zeroed();
            API.shDecToAscii(buf.as_mut_ptr(), pbdec);
            let decstr = PBString::from_vec_with_nul(buf).unwrap().to_string_lossy();
            decstr.parse().unwrap()
        }
    }
}

/*
TODO
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
*/
/// 栈帧
///
/// 当**直接**从Rust端调用PB代码时一般需要创建栈帧来释放调用阶段产生的临时资源,比如创建的对象
///
/// # Examples
///
/// ```
/// //创建栈帧
/// let frame = LocalFrame::new(session);
/// //手动退出栈帧
/// //pop调用不是必须的,变量drop时会自动退出
/// frame.pop();
/// ```
#[must_use]
pub struct LocalFrame {
    session: Session
}

impl LocalFrame {
    /// 创建栈帧
    pub fn new(session: Session) -> LocalFrame {
        unsafe {
            (&mut *session.ptr).routine_level += 1;
        }
        LocalFrame {
            session
        }
    }
    /// 手动退出栈帧
    pub fn pop(self) {}
}

impl Drop for LocalFrame {
    fn drop(&mut self) {
        unsafe {
            let obthis = &mut *self.session.ptr;
            obthis.routine_level -= 1;
            if obthis.routine_level == 0 {
                API.rt_hit_level_0(obthis);
            } else if obthis.response_window_stack.count > 0 {
                let stack = &*obthis
                    .response_window_stack
                    .stack
                    .offset((obthis.response_window_stack.count - 1) as isize);
                if stack.routine_level == obthis.routine_level as UINT {
                    API.rt_hit_level_0(obthis);
                }
            }
        }
    }
}
