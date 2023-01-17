use crate::{
    pbx::{bindings::*, *}, primitive::*
};
use std::{
    ops::{Deref, DerefMut}, sync::{Arc, Weak}
};

/// PB用户对象抽象
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub trait UserObject: Sized + 'static {
    /// 类名(小写)
    const CLASS_NAME: &'static PBStr;

    /// 创建对象
    fn new(session: Session, ctx: Object) -> Result<Self>;

    /// 接口调用
    ///
    /// # Returns
    ///
    /// - 调用的方法ID被处理后返回`Ok(None)`
    /// - 调用的方法ID未处理则返回`Ok(Some(mid))`,`mid`为最后一个方法ID的偏移,此设计用于实现继承
    fn invoke(this: &mut Self, mid: MethodId, ci: &CallInfoRef) -> Result<Option<MethodId>>;

    /// 获取父类指针
    ///
    /// # Design
    ///
    /// 实现运行时实例指针协变 `&Child -> &Parent`
    fn get_inherit_ptr(this: &Self, type_id: u64) -> *const ();

    /// 获取`Session`
    #[inline]
    fn get_session(&self) -> Session {
        unsafe {
            let wrap = &*(self as *const Self as *const UserObjectWrap<Self>);
            Session::from_raw(wrap.pbsession)
        }
    }

    /// 获取关联的PB对象
    #[inline]
    fn get_object(&mut self) -> Object {
        unsafe {
            let wrap = &*(self as *const Self as *const UserObjectWrap<Self>);
            Object::from_raw(wrap.pbobject, Session::from_raw(wrap.pbsession))
        }
    }

    /// 存活状态
    #[inline]
    fn get_alive_state(&self) -> AliveState {
        unsafe {
            let wrap = &*(self as *const Self as *const UserObjectWrap<Self>);
            AliveState(Arc::downgrade(&wrap.alive))
        }
    }
}

/// PB不可视对象
#[cfg(feature = "nonvisualobject")]
pub trait NonVisualObject: UserObject {
    /// 注册
    fn register() { crate::pbx::export::register_nonvisualobject::<Self>() }

    /// 创建PB对象
    fn new_object<'a>(session: Session) -> Result<Object<'a>> { session.new_user_object(Self::CLASS_NAME) }

    /// 创建PB对象并在`modify`回调中修改
    fn new_object_modify<'a, F>(session: Session, modify: F) -> Result<Object<'a>>
    where
        F: FnOnce(&mut Self)
    {
        let mut obj = session.new_user_object(Self::CLASS_NAME)?;
        {
            let obj = obj.get_native_mut()?;
            modify(obj);
        }
        Ok(obj)
    }
}

/// PB可视对象
#[cfg(feature = "visualobject")]
#[allow(unused_variables)]
pub trait VisualObject: UserObject {
    /// 窗口类名
    const WINDOW_CLASS_NAME: &'static PBStr;

    /// 创建窗口
    fn create_control(
        this: &mut Self,
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
    fn get_event_id(this: &Self, hwnd: HWND, msg: u16, wparam: u32, lparam: u32) -> Option<i32> { None }

    /// 注册
    fn register() { crate::pbx::export::register_visualobject::<Self>() }
}

/// PB用户对象封装
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
#[repr(C)]
pub struct UserObjectWrap<T: UserObject> {
    inner: T, //NOTE 必须在第一个字段
    pbsession: pbsession,
    pbobject: pbobject,
    alive: Arc<()>
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
impl<T: UserObject> UserObjectWrap<T> {
    pub fn new(inner: T, pbsession: pbsession, pbobject: pbobject) -> Self {
        UserObjectWrap {
            inner,
            pbsession,
            pbobject,
            alive: Arc::new(())
        }
    }
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
impl<T: UserObject> Deref for UserObjectWrap<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.inner }
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
impl<T: UserObject> DerefMut for UserObjectWrap<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

/// PB用户对象存活状态
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
#[derive(Clone)]
pub struct AliveState(Weak<()>);

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
impl AliveState {
    /// 是否存活
    pub fn is_alive(&self) -> bool { self.0.strong_count() != 0 }

    /// 是否死亡
    pub fn is_dead(&self) -> bool { self.0.strong_count() == 0 }
}
