use crate::pbstr::*;

use self::ffi::POB_THIS;

mod library;
mod ffi;

lazy_static::lazy_static! {
static ref API: ffi::Api = unsafe { ffi::Api::load().unwrap() };
}

pub fn foo() {
    unsafe {
        let this = API.rt_get_current_this();
        println!("{this:p}");
    }
}

#[no_mangle]
unsafe extern "stdcall" fn test_syslib(this: POB_THIS, arg_cnt: i32) -> ffi::DWORD {
    use std::mem::MaybeUninit;
    let mut rv: MaybeUninit<ffi::OB_DATA> = MaybeUninit::uninit();
    //rv.assume_init()
    //API.set_data
    0
}
