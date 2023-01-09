use crate::{
    pbx::{bindings::*, *}, prelude::*
};
use std::ptr;

/// 过程调用参数列表对象的引用,此对象可以添加参数
///
/// # Examples
///
/// ```no_run
/// ci.args.add_long(123);
/// ci.args.add_long(None); //添加NULL值
/// ```
pub struct Arguments<'args> {
    ci: pbcallinfo,
    count: pbint,
    session: Session,
    _marker: PhantomData<&'args pbcallinfo>
}

impl<'args> Arguments<'args> {
    pub(crate) unsafe fn from_raw(ci: pbcallinfo, session: Session) -> Arguments<'args> {
        let count = ffi::pbargs_GetCount(ci.as_ref().pArgs);
        Arguments {
            ci,
            count,
            session,
            _marker: PhantomData
        }
    }

    /// 获取引用对象
    pub fn as_ref(&self) -> ArgumentsRef {
        unsafe { ArgumentsRef::from_raw(self.ci.as_ref().pArgs, self.session.clone()) }
    }

    /// 获取引用元素迭代器
    pub fn iter(&self) -> ArgumentsIter {
        let args = unsafe { ArgumentsRef::from_raw(self.ci.as_ref().pArgs, self.session.clone()) };
        ArgumentsIter {
            args,
            idx: 0
        }
    }

    /// 参数数量
    pub fn count(&self) -> pbint { self.count }

    /// 获取参数值
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn get(&self, index: pbint) -> Value<'args> {
        if let Ok(val) = self.try_get(index) {
            val
        } else {
            panic!("arg index {} is out of bound {}", index, self.count)
        }
    }

    /// 尝试获取参数值
    pub fn try_get(&self, index: pbint) -> Result<Value<'args>> {
        if index < 0 || index >= self.count {
            return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
        }
        unsafe { Ok(Value::from_raw(ffi::pbargs_GetAt(self.ci.as_ref().pArgs, index), self.session.clone())) }
    }

    /// 添加`int`类型参数
    pub fn add_int(&mut self, value: impl Into<Option<pbint>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddIntArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`uint`类型参数
    pub fn add_uint(&mut self, value: impl Into<Option<pbuint>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddUintArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`long`类型参数
    pub fn add_long(&mut self, value: impl Into<Option<pblong>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddLongArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`ulong`类型参数
    pub fn add_ulong(&mut self, value: impl Into<Option<pbulong>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddUlongArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`longlong`类型参数
    pub fn add_longlong(&mut self, value: impl Into<Option<pblonglong>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddLongLongArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`real`类型参数
    pub fn add_real(&mut self, value: impl Into<Option<pbreal>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddRealArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`double`类型参数
    pub fn add_double(&mut self, value: impl Into<Option<pbdouble>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddDoubleArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`decimal`类型参数
    pub fn add_dec(&mut self, value: impl Into<Option<Decimal>>) -> Result<()> {
        unsafe {
            let value = value.into().map(|v| self.session.new_pbdec(v));
            let pbxr = ffi::pbsession_AddDecArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or(NonNull::new_unchecked(0 as _)),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`string`类型参数
    pub fn add_string<T, D>(&mut self, value: T) -> Result<()>
    where
        T: Into<Option<D>>,
        D: AsPBStr
    {
        let value = value.into();
        unsafe {
            let value = value.map(|v| v.as_pbstr().as_ptr());
            let pbxr = ffi::pbsession_AddStringArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or(ptr::null()),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`boolean`类型参数
    pub fn add_bool(&mut self, value: impl Into<Option<bool>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddBoolArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default().into(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`blob`类型参数
    pub fn add_blob<'a>(&mut self, value: impl Into<Option<&'a [u8]>>) -> Result<()> {
        let value = value.into();
        if let Some(ref value) = value {
            if value.is_empty() {
                return Err(PBXRESULT::E_OUTOF_MEMORY);
            }
        }
        unsafe {
            let value = value.map(|v| self.session.new_pbblob(v));
            let pbxr = ffi::pbsession_AddBlobArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or(NonNull::new_unchecked(0 as _)),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`date`类型参数
    pub fn add_date(&mut self, value: impl Into<Option<NaiveDate>>) -> Result<()> {
        unsafe {
            let value = value.into().map(|v| self.session.new_pbdate(v));
            let pbxr = ffi::pbsession_AddDateArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or(NonNull::new_unchecked(0 as _)),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`time`类型参数
    pub fn add_time(&mut self, value: impl Into<Option<NaiveTime>>) -> Result<()> {
        unsafe {
            let value = value.into().map(|v| self.session.new_pbtime(v));
            let pbxr = ffi::pbsession_AddTimeArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or(NonNull::new_unchecked(0 as _)),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`datetime`类型参数
    pub fn add_datetime(&mut self, value: impl Into<Option<NaiveDateTime>>) -> Result<()> {
        unsafe {
            let value = value.into().map(|v| self.session.new_pbdatetime(v));
            let pbxr = ffi::pbsession_AddDateTimeArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or(NonNull::new_unchecked(0 as _)),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加`char`类型参数
    pub fn add_char(&mut self, value: impl Into<Option<PBChar>>) -> Result<()> {
        let value = value.into();
        unsafe {
            let pbxr = ffi::pbsession_AddCharArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or_default(),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加对象类型参数
    pub fn add_object<'a, 'b: 'a>(&mut self, value: impl Into<Option<&'a Object<'b>>>) -> Result<()> {
        unsafe {
            let value = value.into().map(|v| v.as_raw());
            let pbxr = ffi::pbsession_AddObjectArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or(NonNull::new_unchecked(0 as _)),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }

    /// 添加数组类型参数
    pub fn add_array<'a, 'b: 'a>(&mut self, value: impl Into<Option<&'a Array<'b>>>) -> Result<()> {
        unsafe {
            let value = value.into().map(|v| v.as_raw());
            let pbxr = ffi::pbsession_AddArrayArgument(
                self.session.as_raw(),
                self.ci,
                value.unwrap_or(NonNull::new_unchecked(0 as _)),
                value.is_none().into()
            );
            if pbxr.is_ok() {
                self.count += 1;
            }
            pbxr.into()
        }
    }
}

/// 过程调用参数列表对象的引用,此对象不可增加参数
pub struct ArgumentsRef<'args> {
    ptr: pbarguments,
    count: pbint,
    session: Session,
    _marker: PhantomData<&'args pbarguments>
}

impl<'args> ArgumentsRef<'args> {
    pub(crate) unsafe fn from_raw(ptr: pbarguments, session: Session) -> ArgumentsRef<'args> {
        let count = ffi::pbargs_GetCount(ptr);
        ArgumentsRef {
            ptr,
            count,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn clone(&self) -> ArgumentsRef<'args> {
        ArgumentsRef {
            ptr: self.ptr,
            count: self.count,
            session: self.session.clone(),
            _marker: PhantomData
        }
    }

    /// 获取引用元素迭代器
    pub fn iter(&self) -> ArgumentsIter {
        let args = unsafe { ArgumentsRef::from_raw(self.ptr, self.session.clone()) };
        ArgumentsIter {
            args,
            idx: 0
        }
    }

    /// 参数数量
    pub fn count(&self) -> pbint { self.count }

    /// 获取参数值
    ///
    /// # Panics
    ///
    /// 索引越界时会触发Panic
    pub fn get(&self, index: pbint) -> Value<'args> {
        if let Ok(val) = self.try_get(index) {
            val
        } else {
            panic!("arg index {} is out of bound {}", index, self.count)
        }
    }

    /// 尝试获取参数值
    pub fn try_get(&self, index: pbint) -> Result<Value<'args>> {
        if index < 0 || index >= self.count {
            return Err(PBXRESULT::E_ARRAY_INDEX_OUTOF_BOUNDS);
        }
        unsafe { Ok(Value::from_raw(ffi::pbargs_GetAt(self.ptr, index), self.session.clone())) }
    }
}

///参数迭代器
pub struct ArgumentsIter<'args> {
    args: ArgumentsRef<'args>,
    idx: pbint
}

impl<'args> Iterator for ArgumentsIter<'args> {
    type Item = Value<'args>;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        if idx < self.args.count() {
            Some(self.args.get(idx))
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, Some(self.args.count() as usize)) }
}

impl<'args> IntoIterator for Arguments<'args> {
    type Item = Value<'args>;
    type IntoIter = ArgumentsIter<'args>;

    fn into_iter(self) -> Self::IntoIter {
        let args = unsafe { ArgumentsRef::from_raw(self.ci.as_ref().pArgs, self.session.clone()) };
        ArgumentsIter {
            args,
            idx: 0
        }
    }
}

impl<'args> IntoIterator for ArgumentsRef<'args> {
    type Item = Value<'args>;
    type IntoIter = ArgumentsIter<'args>;

    fn into_iter(self) -> Self::IntoIter {
        ArgumentsIter {
            args: self,
            idx: 0
        }
    }
}
