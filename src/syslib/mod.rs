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

    let mut rv = session.new_value();
    {
        let str_val = str_val.get_string().unwrap();
        let str_rv = format!(
            "PBArgs: {}, {}, {}",
            str_val.to_string_lossy(),
            dec_val.get_dec().unwrap(),
            long_val.get_long().unwrap()
        );
        rv.set_str(str_rv).unwrap();
    }

    str_val.set_str("Rust").unwrap();
    dec_val.set_dec(Decimal::from_f64(3223.13232f64).unwrap()).unwrap();
    long_val.set_long(1010313).unwrap();

    API.ot_set_return_val(obthis, rv.as_ptr());
    rv.forget();

    1
}
