#[cfg(feature = "global_function")]
pub(crate) mod global_function;

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
mod method;

/// 构造PB过程调用的参数列表
///
/// # Examples
///
/// ```
/// let rv: pbint = obj.invoke_method("of_Test", pbx_args!["abcd", 123]).unwrap();
/// let rv: pbint = obj.trigger_event("onTest", pbx_args!["abcd", 123]).unwrap();
/// let rv: pbint = session.invoke_function("gf_Test", pbx_args!["abcd", 123]).unwrap();
/// ```
#[macro_export]
macro_rules! pbx_args {
    [$($arg:expr),*] => {{
        |args| -> Result<()> {
            let mut idx = 0;
            $(
                $crate::pbx::__private::codegen::ToValue::to_value($arg, &mut args.get(idx))?;
                idx += 1;
            )*
            Ok(())
        }
    }};
}
pub use pbx_args;

/// 抛出PB异常,包含调用处的位置信息,用法与[`format!`]相同
///
/// [`format!`]: std::format
///
/// # Examples
///
/// ```
/// pbx_throw!(session,"this is a {}!","exception");
/// ```
///
/// Exception:
///
/// ```text
/// this is a exception!
/// at <module_path> (<file>:<line>:<column>)
/// ```
#[macro_export]
macro_rules! pbx_throw {
    ($session:ident,$($arg:tt)*) => {{ $session.throw_exception(format!("{}\r\nat {} ({}:{}:{})",format_args!($($arg)*),module_path!(),file!(),line!(),column!())) }};
}
pub use pbx_throw;

#[doc(hidden)]
pub mod __private {
    use crate::pbx::{bindings::*, *};
    use std::panic::{self, UnwindSafe};

    pub use crate::pbx::bindings::type_id;
    pub use value::{FromValue, FromValueOwned, ToValue};

    #[cfg(feature = "global_function")]
    pub use super::global_function::{factory_call as global_function_factory_call, GlobalFunction};

    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    pub use super::method::factory_call as method_factory_call;

    #[cfg(feature = "static_init")]
    pub use static_init::constructor;

    /// 函数调用,捕获Panic和返回值错误,自动转换为PB异常
    pub fn safe_invoke<F>(
        session: &Session,
        source: &str,
        module: &str,
        file: &str,
        line: u32,
        column: u32,
        f: F
    ) -> Result<()>
    where
        F: FnOnce() -> Result<()> + UnwindSafe
    {
        match panic::catch_unwind(move || f()) {
            Ok(rv) => {
                if let Err(e) = rv {
                    //不覆盖异常
                    if e.is_err() && !session.has_exception() {
                        session.throw_exception(format!(
                            "[{}] cause exception: {:?}\r\nat {} ({}:{}:{})",
                            source, e, module, file, line, column
                        ))
                    } else {
                        //使异常可以正常被PB捕获
                        Err(PBXRESULT::OK)
                    }
                } else {
                    Ok(())
                }
            },
            Err(e) => {
                let panic_info = match e.downcast_ref::<String>() {
                    Some(e) => &e,
                    None => {
                        match e.downcast_ref::<&'static str>() {
                            Some(e) => e,
                            None => ""
                        }
                    },
                };
                if !panic_info.is_empty() {
                    session.throw_exception(format!(
                        "[{}] cause panic: {:?}\r\nat {} ({}:{}:{})",
                        source, panic_info, module, file, line, column
                    ))
                } else {
                    session.throw_exception(format!(
                        "[{}] cause unknown panic\r\nat {} ({}:{}:{})",
                        source, module, file, line, column
                    ))
                }
            }
        }
    }

    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    pub trait ConstructorValue: Sized {
        type Target: UserObject;
        fn value(self) -> Result<Self::Target>;
    }

    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    impl<T: UserObject> ConstructorValue for T {
        type Target = T;
        fn value(self) -> Result<T> { Ok(self) }
    }

    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    impl<T: UserObject> ConstructorValue for Result<T> {
        type Target = T;
        fn value(self) -> Result<T> { self }
    }

    /// 构造函数调用,捕获Panic和返回值错误,自动转换为PB异常
    #[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
    pub fn safe_invoke_ctor<F, R>(
        session: &Session,
        source: &str,
        module: &str,
        file: &str,
        line: u32,
        column: u32,
        f: F
    ) -> Result<R::Target>
    where
        F: FnOnce() -> R + UnwindSafe,
        R: ConstructorValue
    {
        match panic::catch_unwind(move || R::value(f())) {
            Ok(rv) => {
                match rv {
                    Ok(rv) => Ok(rv),
                    Err(e) => {
                        //不覆盖异常
                        if e.is_err() && !session.has_exception() {
                            session
                                .throw_exception(format!(
                                    "[{}] cause exception: {:?}\r\nat {} ({}:{}:{})",
                                    source, e, module, file, line, column
                                ))
                                .and_then(|_| Err(PBXRESULT::OK))
                        } else {
                            //使异常可以正常被PB捕获
                            Err(PBXRESULT::OK)
                        }
                    }
                }
            },
            Err(e) => {
                let panic_info = match e.downcast_ref::<String>() {
                    Some(e) => &e,
                    None => {
                        match e.downcast_ref::<&'static str>() {
                            Some(e) => e,
                            None => ""
                        }
                    },
                };
                if !panic_info.is_empty() {
                    session
                        .throw_exception(format!(
                            "[{}] cause panic: {:?}\r\nat {} ({}:{}:{})",
                            source, panic_info, module, file, line, column
                        ))
                        .and_then(|_| Err(PBXRESULT::OK))
                } else {
                    session
                        .throw_exception(format!(
                            "[{}] cause unknown panic\r\nat {} ({}:{}:{})",
                            source, module, file, line, column
                        ))
                        .and_then(|_| Err(PBXRESULT::OK))
                }
            }
        }
    }
}
