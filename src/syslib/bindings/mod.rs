use crate::pbstr::*;

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
