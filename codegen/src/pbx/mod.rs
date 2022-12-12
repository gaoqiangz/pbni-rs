use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

#[cfg(feature = "global_function")]
mod global_function;

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
mod object;

#[cfg(feature = "global_function")]
pub fn global_function(args: TokenStream, input: TokenStream) -> TokenStream {
    match global_function::gen(parse_macro_input!(args as AttributeArgs), input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into()
    }
}

#[cfg(feature = "nonvisualobject")]
pub fn nonvisualobject(args: TokenStream, input: TokenStream) -> TokenStream {
    match object::gen_nvo(parse_macro_input!(args as AttributeArgs), input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into()
    }
}

#[cfg(feature = "visualobject")]
pub fn visualobject(args: TokenStream, input: TokenStream) -> TokenStream {
    match object::gen_vo(parse_macro_input!(args as AttributeArgs), input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into()
    }
}

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub fn constructor(_: TokenStream, input: TokenStream) -> TokenStream { input }

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub fn method(_: TokenStream, input: TokenStream) -> TokenStream { input }

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub fn event(args: TokenStream, input: TokenStream) -> TokenStream {
    match object::gen_event(parse_macro_input!(args as AttributeArgs), input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into()
    }
}
