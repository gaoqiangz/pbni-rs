use crate::{
    pbx::{
        bindings::*, invoker::{ObjectEvent, ObjectMethod}, session::AsMethodName, value::FromValueOwned, *
    }, prelude::*
};
use std::{
    cell::{Cell, RefCell}, ops::{Deref, DerefMut}, rc::Rc
};

mod impls;

/// 对象实例的引用
pub struct Object<'obj> {
    ptr: pbobject,
    group: Cell<Option<pbgroup>>,
    cls: pbclass,
    session: Session,
    _marker: PhantomData<&'obj pbobject>
}

impl<'obj> Object<'obj> {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: Session) -> Object<'obj> {
        let group = Cell::new(None);
        let cls = ffi::pbsession_GetClass(session.as_ptr(), ptr).unwrap();
        Object {
            ptr,
            group,
            cls,
            session,
            _marker: PhantomData
        }
    }
    pub(crate) fn as_ptr(&self) -> pbobject { self.ptr }
    pub(crate) fn get_group(&self) -> pbgroup {
        match self.group.get() {
            Some(group) => group,
            None => {
                let group = self.session.get_group(self.cls);
                self.group.set(group);
                group.unwrap()
            }
        }
    }
    pub(crate) fn get_class(&self) -> pbclass { self.cls }

    /// 是否为原生对象 (由pbni-rs导出的对象)
    /// FIXME: 始终返回false?
    pub fn is_native(&self) -> bool {
        unsafe { ffi::pbsession_IsNativeObject(self.session.as_ptr(), self.ptr).into() }
    }

    /// 获取原生对象的引用
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    ///
    /// # Examples
    ///
    /// ```
    /// let obj = obj.get_native_ref::<RustObject>().uwnrap();
    /// ```
    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    pub unsafe fn get_native_ref<T: UserObject>(&self) -> Result<&'obj T> {
        /*if !self.is_native() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }*/
        let obj = ffi::pbsession_GetNativeInterface(self.session.as_ptr(), self.ptr);
        if obj.is_none() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }
        let ctx = ffi::GetSafeContext(obj.unwrap(), type_id::<T>());
        if ctx.is_null() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        } else {
            Ok(&*(ctx as *const T))
        }
    }

    /// 获取原生对象的可变引用
    ///
    /// # Safety
    ///
    /// 引用类型不能保证始终有效,详情请阅读[内存安全]说明
    ///
    /// [内存安全]: ./index.html#内存安全
    ///
    /// # Examples
    ///
    /// ```
    /// let mut obj = obj.get_native_mut::<RustObject>().uwnrap();
    /// obj.set_var("is_test","rust").unwrap();
    /// ```
    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    pub unsafe fn get_native_mut<T: UserObject>(&mut self) -> Result<&'obj mut T> {
        /*if !self.is_native() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }*/
        let obj = ffi::pbsession_GetNativeInterface(self.session.as_ptr(), self.ptr);
        if obj.is_none() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        }
        let ctx = ffi::GetSafeContext(obj.unwrap(), type_id::<T>());
        if ctx.is_null() {
            return Err(PBXRESULT::E_MISMATCHED_DATA_TYPE);
        } else {
            Ok(&mut *(ctx as *mut T))
        }
    }

    /// 共享对象
    pub fn share(&self) -> SharedObject { unsafe { SharedObject::from_ptr(self.ptr, self.session.clone()) } }

    /// 转换为共享对象
    pub fn into_shared(self) -> SharedObject { self.into() }
}

/*
    Instance variable
*/

/// 实例变量抽象
pub trait VarId {
    fn var_id(&self, obj: &Object) -> FieldId;
    fn try_var_id(&self, obj: &Object) -> Result<FieldId>;
}

impl<T: AsPBStr> VarId for T {
    #[inline]
    fn var_id(&self, obj: &Object) -> FieldId {
        VarId::try_var_id(self, obj)
            .map_err(|_| format!("invalid var {}", self.as_pbstr().to_string_lossy()))
            .unwrap()
    }
    #[inline]
    fn try_var_id(&self, obj: &Object) -> Result<FieldId> {
        let pbstr = self.as_pbstr();
        obj.get_var_id(pbstr.as_ref()).ok_or(PBXRESULT::E_INVALID_FIELD_ID)
    }
}
impl VarId for FieldId {
    #[inline]
    fn var_id(&self, _obj: &Object) -> FieldId { *self }
    #[inline]
    fn try_var_id(&self, _obj: &Object) -> Result<FieldId> { Ok(*self) }
}
impl VarId for Option<FieldId> {
    #[inline]
    fn var_id(&self, _obj: &Object) -> FieldId { self.unwrap() }
    #[inline]
    fn try_var_id(&self, _obj: &Object) -> Result<FieldId> { self.ok_or(PBXRESULT::E_INVALID_FIELD_ID) }
}

impl<'obj> Object<'obj> {
    /// 获取实例变量数量
    ///
    /// # Examples
    ///
    /// ```
    /// let cnt = obj.get_var_count("is_var");
    /// //遍历所有变量并输出变量名
    /// for id in 0..cnt {
    ///     let fid = unsafe { FieldId::new(id) };
    ///     println!("field id: {}, name: {}", id, obj.get_var_name(fid));
    /// }
    /// ```
    pub fn get_var_count(&self) -> pbulong {
        unsafe { ffi::pbsession_GetNumOfFields(self.session.as_ptr(), self.cls) }
    }

    /// 查找实例变量ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = obj.get_var_id("is_var").unwrap();
    /// obj.set_var_str(fid,"rust");
    /// ```
    pub fn get_var_id(&self, name: impl AsPBStr) -> Option<FieldId> {
        unsafe {
            let fid = ffi::pbsession_GetFieldID(self.session.as_ptr(), self.cls, name.as_pbstr().as_ptr());
            if fid.is_undefined() {
                None
            } else {
                Some(fid)
            }
        }
    }

    /// 通过实例变量ID获取变量名
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = obj.get_var_id("is_var").unwrap();
    /// assert_eq!(pbstr!("is_var"),obj.get_var_name(fid));
    /// ```
    pub fn get_var_name(&self, fid: impl VarId) -> &PBStr {
        unsafe {
            PBStr::from_ptr_str(ffi::pbsession_GetFieldName(
                self.session.as_ptr(),
                self.cls,
                fid.var_id(self)
            ))
        }
    }

    /// 判断是否存在指定实例变量
    ///
    /// # Examples
    ///
    /// ```
    /// if object.has_var("is_var") {
    ///     object.set_var_str("is_var","rust");
    /// }
    /// ```
    pub fn has_var(self, name: impl AsPBStr) -> bool { self.get_var_id(name).is_some() }

    /// 获取变量类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// if object.get_var_type("is_var") == ValueType::String {
    ///     object.set_var_str("is_var","rust");
    /// }
    /// ```
    pub fn get_var_type(&self, fid: impl VarId) -> ValueType {
        unsafe { ffi::pbsession_GetFieldType(self.session.as_ptr(), self.cls, fid.var_id(self)) }
    }

    /// 判断实例变量是否为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_null(&self, fid: impl VarId) -> bool {
        unsafe { ffi::pbsession_IsFieldNull(self.session.as_ptr(), self.ptr, fid.var_id(self)).into() }
    }

    /// 判断实例变量类型是否为数组
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_array(&self, fid: impl VarId) -> bool {
        unsafe { ffi::pbsession_IsFieldArray(self.session.as_ptr(), self.cls, fid.var_id(self)).into() }
    }

    /// 判断实例变量类型是否为对象
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_var_object(&self, fid: impl VarId) -> bool {
        unsafe { ffi::pbsession_IsFieldObject(self.session.as_ptr(), self.cls, fid.var_id(self)).into() }
    }

    /// 刷新实例变量关联的UI状态,如窗口的`title`变量修改后需要调用此函数以刷新UI
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// object.set_var_str("title","rust");
    /// object.update_field("title");
    /// ```
    pub fn update_field(&self, fid: impl VarId) -> Result<()> {
        unsafe { ffi::pbsession_UpdateField(self.session.as_ptr(), self.ptr, fid.var_id(self)).into() }
    }

    /// 设置实例变量的值为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn set_var_to_null(&self, fid: impl VarId) {
        unsafe { ffi::pbsession_SetFieldToNull(self.session.as_ptr(), self.ptr, fid.var_id(self)) }
    }
}

/*
    Shared variable
*/

/// 共享(静态)变量ID抽象
pub trait SharedVarId {
    fn var_id(&self, obj: &Object) -> FieldId;
    fn try_var_id(&self, obj: &Object) -> Result<FieldId>;
}

impl<T: AsPBStr> SharedVarId for T {
    #[inline]
    fn var_id(&self, obj: &Object) -> FieldId {
        SharedVarId::try_var_id(self, obj)
            .map_err(|_| format!("invalid shared var {}", self.as_pbstr().to_string_lossy()))
            .unwrap()
    }
    #[inline]
    fn try_var_id(&self, obj: &Object) -> Result<FieldId> {
        let pbstr = self.as_pbstr();
        obj.get_shared_var_id(pbstr.as_ref()).ok_or(PBXRESULT::E_INVALID_FIELD_ID)
    }
}
impl SharedVarId for FieldId {
    #[inline]
    fn var_id(&self, _obj: &Object) -> FieldId { *self }
    #[inline]
    fn try_var_id(&self, _obj: &Object) -> Result<FieldId> { Ok(*self) }
}
impl SharedVarId for Option<FieldId> {
    #[inline]
    fn var_id(&self, _obj: &Object) -> FieldId { self.unwrap() }
    #[inline]
    fn try_var_id(&self, _obj: &Object) -> Result<FieldId> { self.ok_or(PBXRESULT::E_INVALID_FIELD_ID) }
}

impl<'obj> Object<'obj> {
    /// 获取共享(静态)变量ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = object.get_shared_var_id("ss_var").unwrap();
    /// object.set_shared_var_str(fid,"rust");
    /// ```
    pub fn get_shared_var_id(&self, name: impl AsPBStr) -> Option<FieldId> {
        unsafe {
            let fid = ffi::pbsession_GetSharedVarID(
                self.session.as_ptr(),
                self.get_group(),
                name.as_pbstr().as_ptr()
            );
            if fid.is_undefined() {
                None
            } else {
                Some(fid)
            }
        }
    }

    /// 判断是否存在指定共享(静态)变量
    ///
    /// # Examples
    ///
    /// ```
    /// if object.has_shared_var("ss_var") {
    ///     object.set_shared_var_str("ss_var","rust");
    /// }
    /// ```
    pub fn has_shared_var(self, name: impl AsPBStr) -> bool { self.get_shared_var_id(name).is_some() }

    /// 获取指定共享(静态)变量类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    ///
    /// # Examples
    ///
    /// ```
    /// if object.get_shared_var_type("ss_var") == ValueType::String {
    ///     object.set_shared_var_str("ss_var","rust");
    /// }
    /// ```
    pub fn get_shared_var_type(&self, fid: impl SharedVarId) -> ValueType {
        unsafe { ffi::pbsession_GetSharedVarType(self.session.as_ptr(), self.get_group(), fid.var_id(self)) }
    }

    /// 判断指定共享(静态)变量是否为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_null(&self, fid: impl SharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarNull(self.session.as_ptr(), self.get_group(), fid.var_id(self)).into()
        }
    }

    /// 判断指定共享(静态)变量是否为数组类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_array(&self, fid: impl SharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarArray(self.session.as_ptr(), self.get_group(), fid.var_id(self)).into()
        }
    }

    /// 判断指定共享(静态)变量是否为对象类型
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn is_shared_var_object(&self, fid: impl SharedVarId) -> bool {
        unsafe {
            ffi::pbsession_IsSharedVarObject(self.session.as_ptr(), self.get_group(), fid.var_id(self)).into()
        }
    }

    /// 设置共享(静态)变量的值为NULL
    ///
    /// # Panics
    ///
    /// 访问不存在的变量时会触发Panic
    pub fn set_shared_var_to_null(&self, fid: impl VarId) {
        unsafe {
            ffi::pbsession_SetSharedVarToNull(self.session.as_ptr(), self.get_group(), fid.var_id(self))
        }
    }
}

/*
    Method calling
*/

/// 函数ID抽象
pub trait AsMethodId {
    fn as_mid(&self, obj: &Object) -> Result<MethodId>;
    fn as_eid(&self, obj: &Object) -> Result<MethodId>;
}

impl<T: AsMethodName> AsMethodId for T {
    #[inline]
    fn as_mid(&self, obj: &Object) -> Result<MethodId> {
        obj.get_method_id(self.as_method_name()).ok_or(PBXRESULT::E_INVALID_METHOD_ID)
    }
    #[inline]
    fn as_eid(&self, obj: &Object) -> Result<MethodId> {
        obj.get_event_id(self.as_method_name()).ok_or(PBXRESULT::E_INVALID_METHOD_ID)
    }
}
impl AsMethodId for MethodId {
    #[inline]
    fn as_mid(&self, _obj: &Object) -> Result<MethodId> { Ok(*self) }
    #[inline]
    fn as_eid(&self, _obj: &Object) -> Result<MethodId> { Ok(*self) }
}
impl AsMethodId for Option<MethodId> {
    #[inline]
    fn as_mid(&self, _obj: &Object) -> Result<MethodId> { self.ok_or(PBXRESULT::E_INVALID_METHOD_ID) }
    #[inline]
    fn as_eid(&self, _obj: &Object) -> Result<MethodId> { self.ok_or(PBXRESULT::E_INVALID_METHOD_ID) }
}

impl<'obj> Object<'obj> {
    /// 查找函数ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = object.get_method_id("of_test").unwrap();
    /// let invoker = object.begin_invoke_method(fid).unwrap();
    /// invoker.invoke();
    /// ```
    pub fn get_method_id(&self, name: impl AsMethodName) -> Option<MethodId> {
        let (name, sign) = name.as_method_name();
        self.session.get_method_id(self.cls, name, RoutineType::Function, sign, false)
    }

    /// 查找事件ID
    ///
    /// # Examples
    ///
    /// ```
    /// let fid = object.get_event_id("on_test").unwrap();
    /// let invoker = object.begin_invoke_event(fid).unwrap();
    /// invoker.trigger();
    /// ```
    pub fn get_event_id(&self, name: impl AsMethodName) -> Option<MethodId> {
        let (name, sign) = name.as_method_name();
        self.session.get_method_id(self.cls, name, RoutineType::Event, sign, false)
    }

    /// 调用函数
    ///
    /// # Examples
    ///
    /// ```
    /// let rv: pbint = obj.invoke_method("of_Test", pbargs!["abcd", 123]).unwrap();
    /// ```
    pub fn invoke_method<F, R>(&mut self, mid: impl AsMethodId, arg_cb: F) -> Result<R>
    where
        F: FnOnce(Arguments) -> Result<()>,
        R: FromValueOwned
    {
        let invoker = self.begin_invoke_method(mid)?;
        arg_cb(invoker.args())?;
        let rv = invoker.invoke()?;
        R::from_value(Some(rv))
    }

    /// 调用事件
    ///
    /// # Examples
    ///
    /// ```
    /// let rv: pbint = obj.trigger_event("onTest", pbargs!["abcd", 123]).unwrap();
    /// ```
    pub fn trigger_event<F, R>(&mut self, mid: impl AsMethodId, arg_cb: F) -> Result<R>
    where
        F: FnOnce(Arguments) -> Result<()>,
        R: FromValueOwned
    {
        let invoker = self.begin_invoke_event(mid)?;
        arg_cb(invoker.args())?;
        let rv = invoker.trigger()?;
        R::from_value(Some(rv))
    }

    /// 初始化函数调用上下文
    ///
    /// # Examples
    ///
    /// ```
    /// let invoker = object.begin_invoke_method("of_test").unwrap();
    /// invoker.invoke();
    /// ```
    pub fn begin_invoke_method<'a>(
        &'a mut self,
        mid: impl AsMethodId
    ) -> Result<Invoker<ObjectMethod<'a, 'obj>>> {
        let mid = mid.as_mid(self)?;
        let ci = unsafe { CallInfo::new(self.cls, mid, self.session.clone())? };
        Ok(Invoker::<ObjectMethod>::new(ObjectMethod::new(self), ci))
    }

    /// 初始化事件调用上下文
    ///
    /// # Examples
    ///
    /// ```
    /// let invoker = object.begin_invoke_event("on_test").unwrap();
    /// invoker.trigger();
    /// ```
    pub fn begin_invoke_event<'a>(
        &'a mut self,
        mid: impl AsMethodId
    ) -> Result<Invoker<ObjectEvent<'a, 'obj>>> {
        let mid = mid.as_eid(self)?;
        let ci = unsafe { CallInfo::new(self.cls, mid, self.session.clone())? };
        Ok(Invoker::<ObjectEvent>::new(ObjectEvent::new(self), ci))
    }
}

/// 共享的对象
/// 增加了全局引用计数
#[derive(Clone)]
pub struct SharedObject {
    inner: Rc<RefCell<SharedObjectInner>>
}

impl SharedObject {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: Session) -> SharedObject {
        SharedObject {
            inner: Rc::new(RefCell::new(SharedObjectInner::from_ptr(ptr, session)))
        }
    }
}

impl Deref for SharedObject {
    type Target = RefCell<SharedObjectInner>;
    fn deref(&self) -> &Self::Target { &self.inner }
}

impl From<Object<'_>> for SharedObject {
    fn from(obj: Object) -> Self { unsafe { Self::from_ptr(obj.ptr, obj.session) } }
}

pub struct SharedObjectInner {
    obj: Object<'static>
}

impl SharedObjectInner {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: Session) -> SharedObjectInner {
        ffi::pbsession_AddGlobalRef(session.as_ptr(), ptr);
        SharedObjectInner {
            obj: Object::from_ptr(ptr, session)
        }
    }
}

impl Drop for SharedObjectInner {
    fn drop(&mut self) { unsafe { ffi::pbsession_RemoveGlobalRef(self.obj.session.as_ptr(), self.obj.ptr) } }
}

impl Deref for SharedObjectInner {
    type Target = Object<'static>;
    fn deref(&self) -> &Self::Target { &self.obj }
}

impl DerefMut for SharedObjectInner {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.obj }
}

/// Rust对象绑定的PB对象
pub struct ContextObject {
    obj: Object<'static>
}

impl ContextObject {
    pub(crate) unsafe fn from_ptr(ptr: pbobject, session: &Session) -> ContextObject {
        ContextObject {
            obj: Object::from_ptr(ptr, session.clone())
        }
    }
}

impl Deref for ContextObject {
    type Target = Object<'static>;
    fn deref(&self) -> &Self::Target { &self.obj }
}

impl DerefMut for ContextObject {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.obj }
}

/// PB用户对象抽象
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub trait UserObject: Sized + 'static {
    /// 类名(小写)
    const CLASS_NAME: &'static PBStr;

    /// 创建对象
    fn new(session: Session, ctx: ContextObject) -> Result<Self>;

    /// 接口调用
    ///
    /// # Returns
    ///
    /// - 调用的方法ID被处理后返回`Ok(None)`
    /// - 调用的方法ID未处理则返回`Ok(Some(mid))`,`mid`为最后一个方法ID的偏移,此设计用于实现继承
    fn invoke(&mut self, mid: MethodId, ci: &CallInfoRef) -> Result<Option<MethodId>>;

    /// 获取父类指针
    ///
    /// # Design
    ///
    /// 实现运行时实例指针协变 `&Child -> &Parent`
    unsafe fn get_inherit_ptr(&self, type_id: u64) -> *const ();
}

/// PB不可视对象
#[cfg(feature = "nonvisualobject")]
pub trait NonVisualObject: UserObject {
    /// 注册
    fn register() { export::register_nonvisualobject::<Self>() }
}

/// PB可视对象
#[cfg(feature = "visualobject")]
pub trait VisualObject: UserObject {
    /// 窗口类名
    const WINDOW_CLASS_NAME: &'static PBStr;

    /// 创建窗口
    fn create_control(
        &mut self,
        dwExStyle: u32,
        window_name: &PBStr,
        dwStyle: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: HWND,
        instance: HINSTANCE
    ) -> HWND;

    /// 窗口消息与PB事件ID映射
    #[allow(unused_variables)]
    fn get_event_id(&self, hwnd: HWND, msg: u16, wparam: u32, lparam: u32) -> Option<i32> { None }

    /// 注册
    fn register() { export::register_visualobject::<Self>() }
}
