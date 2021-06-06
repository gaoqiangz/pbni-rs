use crate::{bindings::*, *};
use std::{collections::HashMap, intrinsics::type_id};

#[cfg(feature = "global_function")]
use crate::codegen::global_function::GlobalFunction;

#[cfg(feature = "global_function")]
#[static_init::dynamic]
static mut GLOBAL_FUNCTION_HANDLERS: HashMap<&'static PBStr, fn(CallInfoRef<'_>) -> Result<()>> =
    HashMap::new();

/// 注册全局函数回调接口
#[cfg(feature = "global_function")]
pub fn register_global_function<T: GlobalFunction>() {
    let mut map = GLOBAL_FUNCTION_HANDLERS.write();
    map.insert(T::NAME, T::invoke);
}

#[cfg(feature = "nonvisualobject")]
#[static_init::dynamic]
static mut NVO_HANDLERS: HashMap<
    &'static PBStr,
    fn(session: Session, obj: ContextObject) -> Result<pbuserobject>
> = HashMap::new();

#[cfg(feature = "nonvisualobject")]
pub fn register_nonvisualobject<T: NonVisualObject>() {
    fn new<T: NonVisualObject>(session: Session, ctx: ContextObject) -> Result<bindings::pbuserobject> {
        let obj = Box::new(T::new(session, ctx)?);
        unsafe {
            let om = NVOM::<T> {
                ctx: NonNull::new_unchecked(Box::into_raw(obj)),
                type_id: type_id::<T>(),
                destory: handler::destroy,
                invoke: handler::invoke
            };
            Ok(ffi::NewNonVisualObject(&om as *const NVOM<T> as _))
        }
    }

    let mut map = NVO_HANDLERS.write();
    map.insert(T::CLASS_NAME, new::<T>);
}

#[cfg(feature = "visualobject")]
#[static_init::dynamic]
static mut VO_HANDLERS: HashMap<
    &'static PBStr,
    fn(session: Session, obj: ContextObject) -> Result<bindings::pbuserobject>
> = HashMap::new();

#[cfg(feature = "visualobject")]
pub fn register_visualobject<T: VisualObject>() {
    fn new<T: VisualObject>(session: Session, ctx: ContextObject) -> Result<bindings::pbuserobject> {
        let obj = Box::new(T::new(session, ctx)?);
        unsafe {
            let om = VOM::<T> {
                ctx: NonNull::new_unchecked(Box::into_raw(obj)),
                type_id: type_id::<T>(),
                cls_name: T::WINDOW_CLASS_NAME.as_ptr(),
                destory: handler::destroy,
                invoke: handler::invoke,
                create_control: handler::create_control,
                get_event_id: handler::get_event_id
            };
            Ok(ffi::NewVisualObject(&om as *const VOM<T> as _))
        }
    }

    let mut map = VO_HANDLERS.write();
    map.insert(T::CLASS_NAME, new::<T>);
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
#[doc(hidden)]
mod handler {
    use super::*;

    pub unsafe extern "C" fn destroy<T: UserObject>(ctx: NonNull<T>) { Box::from_raw(ctx.as_ptr()); }
    pub unsafe extern "C" fn invoke<T: UserObject>(
        mut ctx: NonNull<T>,
        session: Session,
        _obj: pbobject,
        mid: MethodId,
        ci: pbcallinfo
    ) -> PBXRESULT {
        let ci = CallInfoRef::from_ptr(ci, session);
        ctx.as_mut().invoke(mid, &ci).into()
    }
    #[cfg(feature = "visualobject")]
    pub unsafe extern "C" fn create_control<T: VisualObject>(
        mut ctx: NonNull<T>,
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
        ctx.as_mut().create_control(dwExStyle, window_name, dwStyle, x, y, width, height, parent, instance)
    }
    #[cfg(feature = "visualobject")]
    pub unsafe extern "C" fn get_event_id<T: VisualObject>(
        ctx: NonNull<T>,
        hwnd: HWND,
        msg: u16,
        wparam: u32,
        lparam: u32
    ) -> i32 {
        const PB_NULL: i32 = -1;
        ctx.as_ref().get_event_id(hwnd, msg, wparam, lparam).unwrap_or(PB_NULL)
    }
}

#[doc(hidden)]
mod export {
    use super::*;

    #[cfg(feature = "global_function")]
    #[no_mangle]
    unsafe extern "stdcall" fn PBX_InvokeGlobalFunction(
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
    unsafe extern "stdcall" fn PBX_CreateNonVisualObject(
        session: Session,
        pbobj: pbobject,
        className: LPCTSTR,
        mut obj: NonNull<pbuserobject>
    ) -> PBXRESULT {
        let className = PBStr::from_ptr_str(className);
        let map = NVO_HANDLERS.read();
        if let Some(handler) = map.get(className) {
            let pbobj = ContextObject::from_ptr(pbobj, &session);
            match handler(session, pbobj) {
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
    unsafe extern "stdcall" fn PBX_CreateVisualObject(
        session: Session,
        pbobj: pbobject,
        className: LPCTSTR,
        mut obj: NonNull<pbuserobject>
    ) -> PBXRESULT {
        let className = PBStr::from_ptr_str(className);
        let map = VO_HANDLERS.read();
        if let Some(handler) = map.get(className) {
            let pbobj = ContextObject::from_ptr(pbobj, &session);
            match handler(session, pbobj) {
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
