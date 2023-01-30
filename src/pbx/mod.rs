mod bindings;
#[cfg(feature = "vm")]
mod vm;
mod session;
mod callinfo;
mod value;
mod invoker;
mod arguments;
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
mod userobject;
#[cfg(any(feature = "global_function", feature = "nonvisualobject", feature = "visualobject"))]
mod export;
mod codegen;

pub use arguments::{Arguments, ArgumentsRef};
pub use bindings::{pbobject, pbsession, PBXRESULT};
pub use callinfo::{CallInfo, CallInfoRef};
pub use codegen::{pbx_args, pbx_throw};
pub use session::{LocalFrame, OwnedSession, Session};
pub use value::{
    array::Array, object::{Object, SharedObject}, FromValue, OwnedValue, ToValue, Value
};

#[cfg(feature = "vm")]
pub use vm::VM;

#[cfg(feature = "nonvisualobject")]
pub use userobject::NonVisualObject;
#[cfg(feature = "visualobject")]
pub use userobject::VisualObject;
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub use userobject::{AliveState, UserObject};

#[cfg(feature = "global_function")]
pub use pbni_codegen::pbx_global_function as global_function;
#[cfg(feature = "nonvisualobject")]
pub use pbni_codegen::pbx_nonvisualobject as nonvisualobject;
#[cfg(feature = "visualobject")]
pub use pbni_codegen::pbx_visualobject as visualobject;
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub use pbni_codegen::{pbx_constructor as constructor, pbx_event as event, pbx_method as method};

pub type Result<T> = ::std::result::Result<T, PBXRESULT>;

#[doc(hidden)]
pub mod __private {
    pub use super::codegen::__private as codegen;
}
