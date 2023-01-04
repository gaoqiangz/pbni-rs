use crate::{
    pbx::{bindings::*, *}, primitive::*
};

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
    fn get_inherit_ptr(&self, type_id: u64) -> *const ();
}

/// PB不可视对象
#[cfg(feature = "nonvisualobject")]
pub trait NonVisualObject: UserObject {
    /// 注册
    fn register() { crate::pbx::export::register_nonvisualobject::<Self>() }

    /// 创建PB对象
    fn new_object(session: &Session) -> Result<Object> { session.new_user_object(Self::CLASS_NAME) }

    /// 创建PB对象并在`modify`回调中修改
    fn new_object_modify<F>(session: &Session, modify: F) -> Result<Object>
    where
        F: FnOnce(&mut Self)
    {
        let mut obj = session.new_user_object(Self::CLASS_NAME)?;
        {
            let obj = unsafe { obj.get_native_mut::<Self>()? };
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
    fn get_event_id(&self, hwnd: HWND, msg: u16, wparam: u32, lparam: u32) -> Option<i32> { None }

    /// 注册
    fn register() { crate::pbx::export::register_visualobject::<Self>() }
}
