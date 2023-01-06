#![allow(unused_imports)]
#![allow(dead_code)]

use proc_macro::TokenStream;

#[cfg(feature = "pbx")]
mod pbx;

#[cfg(feature = "syslib")]
mod syslib;

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
#[cfg(all(feature = "pbx", feature = "global_function"))]
#[proc_macro_attribute]
pub fn pbx_global_function(args: TokenStream, input: TokenStream) -> TokenStream {
    pbx::global_function(args, input)
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
/// - Rust(pbx-rs)
///
/// ```no_run
/// struct RustObject { }
///
/// #[nonvisualobject(name = "n_pbx")]
/// impl RustObject {
///     #[constructor]
///     fn new(session: Session, pbobject: Object) -> RustObject {
///         RustObject { }
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
/// #[nonvisualobject(name = "n_pbx_child", inherit = "parent")]
/// impl RustChildObject {
///     #[constructor]
///     fn new(session: Session, pbobject: Object) -> RustChildObject {
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
#[cfg(all(feature = "pbx", feature = "nonvisualobject"))]
#[proc_macro_attribute]
pub fn pbx_nonvisualobject(args: TokenStream, input: TokenStream) -> TokenStream {
    pbx::nonvisualobject(args, input)
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
/// struct RustObject { }
///
/// #[visualobject(name = "u_canvas")]
/// impl RustObject {
///     #[constructor]
///     fn new(session: Session, pbobject: Object) -> RustObject {
///         RustObject { }
///     }
///     #[method(name="of_Hello")]
///     fn hello(&self, world: String) -> String {
///         format!("hello {}!",world)
///     }
/// }
/// ```
#[cfg(all(feature = "pbx", feature = "visualobject"))]
#[proc_macro_attribute]
pub fn pbx_visualobject(args: TokenStream, input: TokenStream) -> TokenStream {
    pbx::visualobject(args, input)
}

/// 标记对象的构造函数
///
/// # Notice
///
/// 未指定构造函数时默认使用`Default::default`函数构造对象（要求对象实现`Default trait`）
///
/// # Examples
///
/// ```no_run
/// #[constructor]
/// fn new(session: Session, pbobject: Object) -> RustObject {
///     RustObject {
///         session,
///         ctx
///     }
/// }
/// ```
#[cfg(all(feature = "pbx", any(feature = "nonvisualobject", feature = "visualobject")))]
#[proc_macro_attribute]
pub fn pbx_constructor(args: TokenStream, input: TokenStream) -> TokenStream { pbx::constructor(args, input) }

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
#[cfg(all(feature = "pbx", any(feature = "nonvisualobject", feature = "visualobject")))]
#[proc_macro_attribute]
pub fn pbx_method(args: TokenStream, input: TokenStream) -> TokenStream { pbx::method(args, input) }

/// 标记对象事件,如果方法体没有代码,则自动生成对应的调用代码
///
/// # Parameters
///
/// - `name`: 映射的PB事件名 (默认为Rust函数名)
///
/// # Examples
///
/// ```no_run
/// struct RustObject { }
///
/// #[nonvisualobject(name = "n_pbx")]
/// impl RustObject {
///     #[constructor]
///     fn new(session: Session, pbobject: Object) -> RustObject {
///         RustObject { }
///     }
///     #[event(name="onFire")]
///     fn on_fire(&mut self) {}
/// }
/// ```
#[cfg(all(feature = "pbx", any(feature = "nonvisualobject", feature = "visualobject")))]
#[proc_macro_attribute]
pub fn pbx_event(args: TokenStream, input: TokenStream) -> TokenStream { pbx::event(args, input) }
