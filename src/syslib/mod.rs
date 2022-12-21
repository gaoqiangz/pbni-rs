mod bindings;
mod value;
mod session;
mod arguments;

pub use arguments::ArgumentsRef;
pub use bindings::PBRESULT;
pub use session::{LocalFrame, Session};
pub use value::Value;

pub type Result<T> = ::std::result::Result<T, PBRESULT>;

use bindings::*;
use rust_decimal::{prelude::FromPrimitive, Decimal};

#[no_mangle]
pub unsafe extern "stdcall" fn test_syslib(obthis: POB_THIS, arg_cnt: i32) -> DWORD {
    let session = Session::from_ptr(obthis);
    let args = ArgumentsRef::from_ptr(session.clone(), arg_cnt);

    let mut str_val = args.get(0);
    let mut dec_val = args.get(1);
    let mut long_val = args.get(2);
    let mut arr_val = args.get(3).get_array().unwrap();

    let mut rv = session.new_value();
    {
        let str_val = str_val.get_string().unwrap();
        let arr_val =
            arr_val.iter::<&PBStr>().map(|item| item.unwrap().to_string_lossy()).collect::<String>();
        let str_rv = format!(
            "{}, {}, {}, {}",
            str_val.to_string_lossy(),
            dec_val.get_dec().unwrap(),
            long_val.get_long().unwrap(),
            arr_val
        );
        rv.set_str(str_rv);
    }

    str_val.set_str("Rust");
    dec_val.set_dec(Decimal::from_f64(3223.13232f64).unwrap());
    long_val.set_long(1010313);
    arr_val.set_item_str(1, "d");
    arr_val.set_item_str(2, "e");
    arr_val.set_item_str(3, "f");
    arr_val.set_item_str(4, "g");

    {
        let mut tmp = session.new_value();
        tmp.set_str("new_value");
        arr_val.set_item_value(5, tmp);
    }

    API.ot_set_return_val(obthis, rv.as_ptr());
    rv.forget();

    1
}
