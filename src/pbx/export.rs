use crate::{
    pbx::{bindings::*, *}, primitive::*
};
use std::collections::HashMap;

#[cfg(feature = "global_function")]
use crate::pbx::codegen::global_function::GlobalFunction;

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
use crate::pbx::userobject::UserObjectWrap;

#[cfg(feature = "global_function")]
#[static_init::dynamic]
static mut GLOBAL_FUNCTION_HANDLERS: HashMap<&'static PBStr, fn(ci: CallInfoRef<'_>) -> Result<()>> =
    HashMap::new();

#[cfg(feature = "nonvisualobject")]
#[static_init::dynamic]
static mut NVO_HANDLERS: HashMap<
    &'static PBStr,
    fn(session: Session, object: Object) -> Result<pbuserobject>
> = HashMap::new();

#[cfg(feature = "visualobject")]
#[static_init::dynamic]
static mut VO_HANDLERS: HashMap<
    &'static PBStr,
    fn(session: Session, object: Object) -> Result<bindings::pbuserobject>
> = HashMap::new();

/// 注册全局函数回调接口
#[cfg(feature = "global_function")]
pub fn register_global_function<T: GlobalFunction>() {
    let mut map = GLOBAL_FUNCTION_HANDLERS.write();
    map.insert(T::NAME, T::invoke);
}

#[cfg(feature = "nonvisualobject")]
pub fn register_nonvisualobject<T: NonVisualObject>() {
    fn new<T: NonVisualObject>(session: Session, object: Object) -> Result<bindings::pbuserobject> {
        let pbobject = object.as_ptr();
        let obj = Box::new(UserObjectWrap::new(T::new(session, object)?, session.as_ptr(), pbobject));
        unsafe {
            let om = NVOM::<UserObjectWrap<T>> {
                ctx: NonNull::new_unchecked(Box::into_raw(obj)),
                type_id: type_id::<T>(),
                destory: handler::destroy,
                invoke: handler::invoke,
                get_inherit_ptr: handler::get_inherit_ptr
            };
            Ok(ffi::NewNonVisualObject(&om as *const NVOM<UserObjectWrap<T>> as _))
        }
    }

    let mut map = NVO_HANDLERS.write();
    map.insert(T::CLASS_NAME, new::<T>);
}

#[cfg(feature = "visualobject")]
pub fn register_visualobject<T: VisualObject>() {
    fn new<T: VisualObject>(session: Session, object: Object) -> Result<bindings::pbuserobject> {
        let pbobject = object.as_ptr();
        let obj = Box::new(UserObjectWrap::new(T::new(session, object)?, session.as_ptr(), pbobject));
        unsafe {
            let om = VOM::<UserObjectWrap<T>> {
                ctx: NonNull::new_unchecked(Box::into_raw(obj)),
                type_id: type_id::<T>(),
                cls_name: T::WINDOW_CLASS_NAME.as_ptr(),
                destory: handler::destroy,
                invoke: handler::invoke,
                get_inherit_ptr: handler::get_inherit_ptr,
                create_control: handler::create_control,
                get_event_id: handler::get_event_id
            };
            Ok(ffi::NewVisualObject(&om as *const VOM<UserObjectWrap<T>> as _))
        }
    }

    let mut map = VO_HANDLERS.write();
    map.insert(T::CLASS_NAME, new::<T>);
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
#[doc(hidden)]
mod handler {
    use super::*;

    pub unsafe extern "C" fn destroy<T: UserObject>(ctx: NonNull<UserObjectWrap<T>>) {
        drop(Box::from_raw(ctx.as_ptr()));
    }

    pub unsafe extern "C" fn invoke<T: UserObject>(
        mut ctx: NonNull<UserObjectWrap<T>>,
        session: Session,
        _obj: pbobject,
        mid: MethodId,
        ci: pbcallinfo
    ) -> PBXRESULT {
        let ci = CallInfoRef::from_ptr(ci, session);
        <T as UserObject>::invoke(ctx.as_mut(), mid, &ci).into()
    }

    pub unsafe extern "C" fn get_inherit_ptr<T: UserObject>(
        ctx: NonNull<UserObjectWrap<T>>,
        type_id: u64
    ) -> *const () {
        <T as UserObject>::get_inherit_ptr(ctx.as_ref(), type_id)
    }

    #[cfg(feature = "visualobject")]
    pub unsafe extern "C" fn create_control<T: VisualObject>(
        mut ctx: NonNull<UserObjectWrap<T>>,
        dwExStyle: u32,
        window_name: LPCTSTR,
        dwStyle: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: HWND,
        instance: HINSTANCE
    ) -> HWND {
        let window_name = PBStr::from_ptr_str(window_name);
        <T as VisualObject>::create_control(
            ctx.as_mut(),
            dwExStyle,
            window_name,
            dwStyle,
            x,
            y,
            width,
            height,
            parent,
            instance
        )
    }

    #[cfg(feature = "visualobject")]
    pub unsafe extern "C" fn get_event_id<T: VisualObject>(
        ctx: NonNull<UserObjectWrap<T>>,
        hwnd: HWND,
        msg: u16,
        wparam: u32,
        lparam: u32
    ) -> i32 {
        const PB_NULL: i32 = -1;
        <T as VisualObject>::get_event_id(ctx.as_ref(), hwnd, msg, wparam, lparam).unwrap_or(PB_NULL)
    }
}

#[doc(hidden)]
mod export {
    use super::*;

    #[cfg(feature = "global_function")]
    #[no_mangle]
    unsafe extern "C" fn RS_InvokeGlobalFunction(
        session: Session,
        functionName: LPCTSTR,
        ci: pbcallinfo
    ) -> PBXRESULT {
        let functionName = PBStr::from_ptr_str(functionName);
        let map = GLOBAL_FUNCTION_HANDLERS.read();
        if let Some(handler) = map.get(functionName) {
            let ci = CallInfoRef::from_ptr(ci, session);
            handler(ci).into()
        } else {
            PBXRESULT::E_NO_REGISTER_FUNCTION
        }
    }

    #[cfg(feature = "nonvisualobject")]
    #[no_mangle]
    unsafe extern "C" fn RS_CreateNonVisualObject(
        session: Session,
        pbobj: pbobject,
        className: LPCTSTR,
        mut obj: NonNull<pbuserobject>
    ) -> PBXRESULT {
        let className = PBStr::from_ptr_str(className);
        let map = NVO_HANDLERS.read();
        if let Some(handler) = map.get(className) {
            let object = Object::from_ptr(pbobj, session);
            match handler(session, object) {
                Ok(ptr) => {
                    *obj.as_mut() = ptr;
                    PBXRESULT::OK
                },
                Err(e) => e
            }
        } else {
            PBXRESULT::E_NO_SUCH_CLASS
        }
    }

    #[cfg(feature = "visualobject")]
    #[no_mangle]
    unsafe extern "C" fn RS_CreateVisualObject(
        session: Session,
        pbobj: pbobject,
        className: LPCTSTR,
        mut obj: NonNull<pbuserobject>
    ) -> PBXRESULT {
        let className = PBStr::from_ptr_str(className);
        let map = VO_HANDLERS.read();
        if let Some(handler) = map.get(className) {
            let object = Object::from_ptr(pbobj, session);
            match handler(session, object) {
                Ok(ptr) => {
                    *obj.as_mut() = ptr;
                    PBXRESULT::OK
                },
                Err(e) => e
            }
        } else {
            PBXRESULT::E_NO_SUCH_CLASS
        }
    }
}
