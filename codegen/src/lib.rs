use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

#[cfg(feature = "global_function")]
mod global_function;

#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
mod object;

/// 生成全局函数
///
/// # Parameters
/// - `name`: 映射的PB函数名 (默认为Rust函数名)
///
/// # Examples
/// ```no_run
/// #[global_function(name = "gf_bitor")]
/// fn bit_or(session: Session, a: pblong, b: pblong) -> pblong { a | b }
/// ```
#[cfg(feature = "global_function")]
#[proc_macro_attribute]
pub fn global_function(args: TokenStream, input: TokenStream) -> TokenStream {
    match global_function::gen(parse_macro_input!(args as AttributeArgs), input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into()
    }
}

/// 生成不可视对象
///
/// # Parameters
///
/// - `name`: 映射的PB对象类名 (默认为Rust对象名)
/// - `inherit`: 继承的对象 (此对象的字段)
///
/// # Notice
///
/// 继承模式不支持覆盖(override)方法实现,并且在PB端需要将父类的方法声明在子类中重新声明
///
/// # Examples
///
/// - PowerBuilder
///
/// ```vbscript
/// /* 父类声明 */
/// forward
/// global type n_parent from nonvisualobject
/// end type
/// end forward
///
/// global type n_parent from nonvisualobject native "pbrs.dll"
/// public function string of_hello (string world)
/// end type
/// global n_parent n_parent
///
/// type variables
/// end variables
///
/// on n_parent.create
/// call super::create
/// TriggerEvent( this, "constructor" )
/// end on
///
/// on n_parent.destroy
/// TriggerEvent( this, "destructor" )
/// call super::destroy
/// end on
///
/// /* 子类声明 */
/// forward
/// global type n_child from n_parent
/// end type
/// end forward
///
/// global type n_child from n_parent native "pbrs.dll"
/// // 重新声明父类方法
/// public function string of_hello (string world)
/// // 声明子类私有的方法
/// public function string of_foo (string bar)
/// end type
/// global n_child n_child
///
/// type variables
/// end variables
///
/// on n_child.create
/// call super::create
/// TriggerEvent( this, "constructor" )
/// end on
///
/// on n_child.destroy
/// TriggerEvent( this, "destructor" )
/// call super::destroy
/// end on
/// ```
///
/// - Rust(pbni-rs)
///
/// ```no_run
/// struct RustObject {
///     session: Session,
///     ctx: ContextObject
/// }
///
/// #[nonvisualobject(name = "n_pbni")]
/// impl RustObject {
///     #[constructor]
///     fn new(session: Session, ctx: ContextObject) -> RustObject {
///         RustObject {
///             session,
///             ctx
///         }
///     }
///     #[method(name="of_Hello")]
///     fn hello(&self, world: String) -> String {
///         format!("hello {}!",world)
///     }
/// }
///
/// struct RustChildObject {
///     parent: RustObject
/// }
///
/// #[nonvisualobject(name = "n_pbni_child", inherit = "parent")]
/// impl RustChildObject {
///     #[constructor]
///     fn new(session: Session, ctx: ContextObject) -> RustChildObject {
///         RustChildObject {
///             parent : RustObject {
///                 session,
///                 ctx
///             }
///         }
///     }
///     #[method(name="of_Foo")]
///     fn foo(&self, bar: String) -> String {
///         format!("foo {}!",bar)
///     }
/// }
/// ```
#[cfg(feature = "nonvisualobject")]
#[proc_macro_attribute]
pub fn nonvisualobject(args: TokenStream, input: TokenStream) -> TokenStream {
    match object::gen_nvo(parse_macro_input!(args as AttributeArgs), input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into()
    }
}

/// 生成可视对象
///
/// # Parameters
///
/// - `name`: 映射的PB对象类名 (默认为Rust对象名)
/// - `inherit`: 继承的对象 (此对象的字段)
///
/// # Notice
///
/// 继承模式不支持覆盖(override)方法实现,并且在PB端需要将父类的方法声明在子类中重新声明,参见[`nonvisualobject`]说明
///
/// [`nonvisualobject`]: macro@nonvisualobject
///
/// # Examples
///
/// ```no_run
/// struct RustObject {
///     session: Session,
///     ctx: ContextObject
/// }
///
/// #[visualobject(name = "u_canvas")]
/// impl RustObject {
///     #[constructor]
///     fn new(session: Session, ctx: ContextObject) -> RustObject {
///         RustObject {
///             session,
///             ctx
///         }
///     }
///     #[method(name="of_Hello")]
///     fn hello(&self, world: String) -> String {
///         format!("hello {}!",world)
///     }
/// }
/// ```
#[cfg(feature = "visualobject")]
#[proc_macro_attribute]
pub fn visualobject(args: TokenStream, input: TokenStream) -> TokenStream {
    match object::gen_vo(parse_macro_input!(args as AttributeArgs), input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into()
    }
}

/// 标记对象的构造函数
///
/// # Examples
///
/// ```no_run
/// #[constructor]
/// fn new(session: Session, ctx: ContextObject) -> RustObject {
///     RustObject {
///         session,
///         ctx
///     }
/// }
/// ```
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
#[proc_macro_attribute]
pub fn constructor(_: TokenStream, input: TokenStream) -> TokenStream { input }

/// 标记对象函数
///
/// # Parameters
///
/// - `name`: 映射的PB函数类名 (默认为Rust函数名)
/// - `overload`: 重载次数 (默认0,意为没有重载)
///
/// # Examples
///
/// ```no_run
/// #[method(name="of_Hello")]
/// fn hello(&self, world: String) -> String {
///     format!("hello {}!",world)
/// }
/// ```
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
#[proc_macro_attribute]
pub fn method(_: TokenStream, input: TokenStream) -> TokenStream { input }

/// 标记对象事件,如果方法体没有代码,则自动生成对应的调用代码
///
/// # Parameters
///
/// - `name`: 映射的PB事件名 (默认为Rust函数名)
///
/// # Required
///
/// 自动生成事件代码,需要对象实现`context_mut`方法:
///
/// # Examples
///
/// ```no_run
/// struct RustObject {
///     session: Session,
///     ctx: ContextObject
/// }
///
/// impl RustObject {
///     fn context_mut(&mut self) -> &mut ContextObject { &mut self.ctx }
/// }
///
/// #[nonvisualobject(name = "n_pbni")]
/// impl RustObject {
///     #[constructor]
///     fn new(session: Session, ctx: ContextObject) -> RustObject {
///         RustObject {
///             session,
///             ctx
///         }
///     }
///     #[event(name="onFire")]
///     fn on_fire(&mut self) {}
/// }
/// ```
#[cfg(any(feature = "nonvisualobject", feature = "visualobject"))]
#[proc_macro_attribute]
pub fn event(args: TokenStream, input: TokenStream) -> TokenStream {
    match object::gen_event(parse_macro_input!(args as AttributeArgs), input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into()
    }
}
