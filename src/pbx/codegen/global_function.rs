use crate::pbx::{bindings::*, callinfo::FromCallInfo, value::ToValue, *};

/// 全局函数抽象
#[cfg(feature = "global_function")]
pub trait GlobalFunction: Sized {
    /// 函数名(小写)
    const NAME: &'static PBStr;

    /// 接口调用
    fn invoke(ci: CallInfoRef) -> Result<()>;

    /// 注册
    fn register() { export::register_global_function::<Self>() }
}

/// 通用函数调用的抽象工厂
pub trait Factory<T, R> {
    fn call(&self, param: T) -> R;
}

/// 通过抽象工厂接口反射调用函数
#[doc(hidden)]
pub fn factory_call<'ci, F, T, R>(f: F, ci: &CallInfoRef<'ci>) -> Result<()>
where
    F: Factory<T, R>,
    T: FromCallInfo<'ci>,
    R: ToValue
{
    match T::from_callinfo(ci) {
        Ok(param) => f.call(param).to_value(&mut ci.return_value()),
        Err(e) => {
            //发生NULL错误说明参数接收者不支持传NULL值,此时自动转义为返回NULL
            if e == PBXRESULT::E_VALUE_IS_NULL {
                ci.return_value().set_to_null()
            } else {
                Err(e)
            }
        }
    }
}

#[rustfmt::skip]
mod m {
    use super::*;

    /// 将函数参数列表反射为tuple类型
    macro_rules! factory_tuple {
        ($(($n:tt, $T:ident)),+) => {
            #[doc(hidden)]
            impl<FUNC,$($T),+,R> Factory<($($T,)+),R> for FUNC
            where
                FUNC: Fn($($T),+) -> R
            {
                fn call(&self, param: ($($T,)+)) -> R { (self)($(param.$n),+) }
            }
        }
    }

    #[doc(hidden)]
    impl<F,R> Factory<(),R> for F
    where
        F: Fn() -> R
    {
        fn call(&self, _: ()) -> R { (self)() }
    }

    factory_tuple!((0, A));
    factory_tuple!((0, A), (1, B));
    factory_tuple!((0, A), (1, B), (2, C));
    factory_tuple!((0, A), (1, B), (2, C), (3, D));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N), (14, O));

}
