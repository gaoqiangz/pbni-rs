mod bindings;
#[cfg(feature = "vm")]
mod vm;
mod session;
mod callinfo;
mod value;
mod object;
mod invoker;
mod arguments;
#[cfg(any(feature = "global_function", feature = "nonvisualobject", feature = "visualobject"))]
mod export;
#[doc(hidden)]
mod codegen;

pub use arguments::{Arguments, ArgumentsRef};
pub use bindings::{
    pbbyte, pbdouble, pbint, pblong, pblonglong, pbreal, pbuint, pbulong, FieldId, MethodId, ValueType, PBXRESULT
};
pub use callinfo::{CallInfo, CallInfoRef};
pub use codegen::{pbargs, throw};
pub use invoker::Invoker;
pub use object::{ContextObject, Object, SharedObject};
pub use session::{LocalFrame, OwnedSession, Session};
pub use value::{Array, OwnedValue, Value};

#[cfg(feature = "vm")]
pub use vm::VM;

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
pub use object::UserObject;

#[cfg(feature = "nonvisualobject")]
pub use object::NonVisualObject;

#[cfg(feature = "visualobject")]
pub use object::VisualObject;

#[cfg(any(feature = "global_function", feature = "nonvisualobject", feature = "visualobject"))]
pub use pbni_codegen::{
    pbni_constructor as constructor, pbni_event as event, pbni_global_function as global_function, pbni_method as method, pbni_nonvisualobject as nonvisualobject, pbni_visualobject as visualobject
};

pub type Result<T> = ::std::result::Result<T, PBXRESULT>;

#[doc(hidden)]
pub mod __private {
    pub use super::codegen::__private as codegen;
}
