use crate::pbstr::*;

mod library;
pub mod ffi;

pub fn foo() {
    unsafe {
        ffi::Api::load().unwrap();
    }
}
