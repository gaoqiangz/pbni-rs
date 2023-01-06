use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse, parse_quote, Attribute, AttributeArgs, Error, FnArg, Ident, ImplItem, ImplItemMethod, Index, ItemImpl, Lit, Meta, NestedMeta, Pat, Result, ReturnType, Type
};

/// 生成不可视对象代码
pub fn gen_nvo(args: AttributeArgs, input: TokenStream) -> Result<TokenStream> {
    let args = AttrArgs::parse(args)?;
    let (ident, mut output) = gen_object(args.name, args.inherit, input)?;
    let ident_ctor = format_ident!("__ctor_{}", ident);
    let ident_ctor_mod = format_ident!("__priv_{}", ident);

    let nvo_impl = quote! {
        impl ::pbni::pbx::NonVisualObject for #ident { }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        mod #ident_ctor_mod {
            #[::pbni::pbx::__private::codegen::constructor]
            extern "C" fn #ident_ctor() {
                <super::#ident as ::pbni::pbx::NonVisualObject>::register();
            }
        }
    };

    output.extend(nvo_impl);

    Ok(output.into())
}

/// 生成可视对象代码
pub fn gen_vo(args: AttributeArgs, input: TokenStream) -> Result<TokenStream> {
    let args = AttrArgs::parse(args)?;
    let (ident, mut output) = gen_object(args.name, args.inherit, input)?;
    let ident_ctor = format_ident!("__ctor_{}", ident);
    let ident_ctor_mod = format_ident!("__priv_{}", ident);

    let vo_impl = quote! {
        impl ::pbni::pbx::VisualObject for #ident { }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        mod #ident_ctor_mod {
            #[::pbni::pbx::__private::codegen::constructor]
            extern "C" fn #ident_ctor() {
                <super::#ident as ::pbni::pbx::VisualObject>::register();
            }
        }
    };

    output.extend(vo_impl);

    Ok(output.into())
}

/// 生成对象事件映射
pub fn gen_event(args: AttributeArgs, input: TokenStream) -> Result<TokenStream> {
    let mut ast = parse::<ImplItemMethod>(input)?;

    //仅当函数体为空时才自动生成代码
    if ast.block.stmts.is_empty() {
        let args = AttrArgs::parse(args)?;
        let name = args.name.unwrap_or_else(|| ast.sig.ident.to_string());

        //解析函数定义的错误列表
        let mut fn_arg = Vec::new();
        let mut fn_arg_index = Vec::<Index>::new();
        for arg in &ast.sig.inputs {
            if let FnArg::Typed(arg) = arg {
                match arg.pat.as_ref() {
                    Pat::Ident(ident) => {
                        fn_arg.push(ident.ident.clone());
                        fn_arg_index.push(fn_arg_index.len().into());
                    },
                    _ => {
                        return Err(Error::new_spanned(arg, "Not supported argument token"));
                    }
                }
            }
        }

        //检查返回值类型是否为Result<T>
        let mut rty_is_result = false;
        if let ReturnType::Type(_, ty) = &ast.sig.output {
            if let Type::Path(ty) = ty.as_ref() {
                if let Some(ty) = ty.path.segments.last() {
                    rty_is_result = ty.ident.to_string() == "Result";
                }
            }
        }

        if rty_is_result {
            //返回值为Result<T>,传播错误值
            ast.block = parse_quote! {
                {
                    use ::pbni::pbx::__private::codegen::{FromValue,ToValue};
                    let mut object = self.get_object();
                    let invoker = object.begin_invoke_event(::pbni::pbstr!(#name))?;
                    #(
                        ToValue::to_value(#fn_arg,&mut invoker.arg(#fn_arg_index))?;
                    )*
                    let rv = invoker.trigger()?;
                    FromValue::from_value(Some(rv))
                }
            };
        } else {
            //返回值T,不传播错误值,如果发生错误则Panic
            ast.block = parse_quote! {
                {
                    use ::pbni::pbx::__private::codegen::{FromValue,ToValue};
                    let mut object = self.get_object();
                    let invoker = object.begin_invoke_event(::pbni::pbstr!(#name)).expect(concat!("begin invoke ",#name));
                    #(
                        ToValue::to_value(#fn_arg,&mut invoker.arg(#fn_arg_index)).expect(concat!("pass argument ",stringify!(#fn_arg)));
                    )*
                    let rv = invoker.trigger().expect(concat!("invoke ",#name));
                    FromValue::from_value(Some(rv)).expect(concat!("mismatched return type ",#name))
                }
            };
        }
    }

    Ok(ast.into_token_stream().into())
}

/// 通用的生成对象实现代码
fn gen_object(
    cls_name: Option<String>,
    inherit: Option<String>,
    input: TokenStream
) -> Result<(Ident, TokenStream2)> {
    let body = parse::<ItemImpl>(input)?;
    let block = ImplBlock::parse(&body)?;
    let ident = block.ident;
    let cls_name = cls_name.unwrap_or_else(|| ident.to_string()).to_ascii_lowercase();
    let inherit = inherit.map(|name| Ident::new(&name, ident.span()));

    let mut method_ident = Vec::new();
    let mut method_name = Vec::new();
    let mut method_idx = Vec::<Index>::new();
    let mut method_idx_offset = Vec::<Index>::new();
    let mut method_overload = Vec::<Index>::new();
    let mut event_ident = Vec::new();
    let mut event_idx = Vec::<Index>::new();
    let mut event_idx_offset = Vec::<Index>::new();
    let mut overload_cc = 0;
    let mut last_method_idx: Index = 0.into();

    for (idx, item) in block.items.into_iter().enumerate() {
        let mut overload = 0;
        match item {
            Item::Method(method) => {
                method_name.push(method.attr_args.name.unwrap_or(method.ident.to_string()));
                method_ident.push(method.ident);
                method_idx.push(idx.into());
                method_idx_offset.push(overload_cc.into());
                method_overload.push(method.attr_args.overload.into());
                overload = method.attr_args.overload;
            },
            Item::Event(event) => {
                event_ident.push(event.ident);
                event_idx.push(idx.into());
                event_idx_offset.push(overload_cc.into());
            }
        }
        overload_cc += overload;
        last_method_idx = (idx + overload_cc + overload + 1).into();
    }
    let new_impl = if let Some(ctor) = block.ctor {
        quote! {
            ::pbni::pbx::__private::codegen::safe_invoke_ctor(session,stringify!(#ctor),::std::any::type_name::<#ident>(),file!(),line!(),column!(),||#ident::#ctor(session, ctx))
        }
    } else {
        quote! {
            ::pbni::pbx::__private::codegen::safe_invoke_ctor(session,concat!(stringify!(#ident),"::default"),::std::any::type_name::<#ident>(),file!(),line!(),column!(),||#ident::default())
        }
    };
    let invoke_impl = if let Some(inherit) = &inherit {
        quote! {
            let method_idx_base = if let Some(method_idx_base) = self.#inherit.invoke(mid,ci)? {
                method_idx_base.value()
            } else {
                return Ok(None);
            };
            #(
                if mid >= method_idx_base + #method_idx + #method_idx_offset && mid < method_idx_base + #method_idx + #method_idx_offset + #method_overload + 1 {
                    return ::pbni::pbx::__private::codegen::safe_invoke(
                        ci.session(),
                        #method_name,
                        concat!(module_path!(),"::",stringify!(#ident),"::",stringify!(#method_ident)),
                        file!(),line!(),column!(),
                        ::std::panic::AssertUnwindSafe(||::pbni::pbx::__private::codegen::method_factory_call(#ident::#method_ident, self, &ci))
                    ).map(|_|None);
                }
            )*
            #(
                if mid >= method_idx_base + #event_idx + #event_idx_offset && mid < method_idx_base + #event_idx + #event_idx_offset + 1 {
                    return Ok(None);
                }
            )*
            //::pbni::pbx::__private::codegen::safe_invoke(ci.session(),&format!("{:?}",mid),::std::any::type_name::<#ident>(),file!(),line!(),column!(),||Err(::pbni::pbx::PBXRESULT::E_NO_REGISTER_FUNCTION))
            Ok(Some(unsafe { ::pbni::primitive::MethodId::new(method_idx_base + #last_method_idx) }))
        }
    } else {
        quote! {
            #(
                if mid >= #method_idx + #method_idx_offset && mid < #method_idx + #method_idx_offset + #method_overload + 1 {
                    return ::pbni::pbx::__private::codegen::safe_invoke(
                        ci.session(),
                        #method_name,
                        concat!(module_path!(),"::",stringify!(#ident),"::",stringify!(#method_ident)),
                        file!(),line!(),column!(),
                        ::std::panic::AssertUnwindSafe(||::pbni::pbx::__private::codegen::method_factory_call(#ident::#method_ident, self, ci))
                    ).map(|_|None);
                }
            )*
            #(
                if mid >= #event_idx + #event_idx_offset && mid < #event_idx + #event_idx_offset + 1 {
                    return Ok(None);
                }
            )*
            //::pbni::pbx::__private::codegen::safe_invoke(ci.session(),&format!("{:?}",mid),::std::any::type_name::<#ident>(),file!(),line!(),column!(),||Err(::pbni::pbx::PBXRESULT::E_NO_REGISTER_FUNCTION))
            Ok(Some(unsafe { ::pbni::primitive::MethodId::new(#last_method_idx) }))
        }
    };
    let get_inherit_ptr_impl = if let Some(inherit) = &inherit {
        quote! {
            if ::pbni::pbx::__private::codegen::type_id::<Self>() == type_id {
                self as *const Self as _
            } else {
                self.#inherit.get_inherit_ptr(type_id)
            }
        }
    } else {
        quote! {
            if ::pbni::pbx::__private::codegen::type_id::<Self>() == type_id {
                self as *const Self as _
            } else {
                ::std::ptr::null()
            }
        }
    };

    let output = quote! {
        #body
        impl ::pbni::pbx::UserObject for #ident {
            const CLASS_NAME: &'static ::pbni::primitive::PBStr = ::pbni::pbstr!(#cls_name);
            fn new(session: ::pbni::pbx::Session, ctx: ::pbni::pbx::Object) -> Result<Self> {
                #new_impl
            }
            fn invoke(&mut self, mid: ::pbni::primitive::MethodId, ci: &::pbni::pbx::CallInfoRef) -> ::pbni::pbx::Result<Option<::pbni::primitive::MethodId>> {
                #invoke_impl
            }
            fn get_inherit_ptr(&self, type_id: u64) -> *const () {
                #get_inherit_ptr_impl
            }
        }
    };

    Ok((ident, output))
}

/// 属性参数
struct AttrArgs {
    name: Option<String>,
    inherit: Option<String>
}

impl AttrArgs {
    fn parse(args: AttributeArgs) -> Result<Self> {
        let mut name = None;
        let mut inherit = None;

        for arg in args {
            match arg {
                NestedMeta::Meta(Meta::NameValue(nv)) => {
                    if nv.path.is_ident("name") {
                        if let Lit::Str(lit) = nv.lit {
                            name = Some(lit.value());
                        } else {
                            return Err(Error::new_spanned(nv.lit, "Attribute name expects literal string!"));
                        }
                    } else if nv.path.is_ident("inherit") {
                        if let Lit::Str(lit) = nv.lit {
                            inherit = Some(lit.value());
                        } else {
                            return Err(Error::new_spanned(
                                nv.lit,
                                "Attribute inherit expects literal string!"
                            ));
                        }
                    } else {
                        return Err(Error::new_spanned(
                            nv.path,
                            "Unknown attribute key is specified. Allowed: name, inherit"
                        ));
                    }
                },
                _ => return Err(Error::new_spanned(arg, "Unknown attribute"))
            }
        }

        Ok(AttrArgs {
            name,
            inherit
        })
    }
}

/// `impl`块定义
struct ImplBlock {
    ident: Ident,
    ctor: Option<Ident>,
    items: Vec<Item>
}

/// `impl`块定义的项
enum Item {
    Method(Method),
    Event(Event)
}

impl ImplBlock {
    fn parse(ast: &ItemImpl) -> Result<Self> {
        //获取实现目标对象的类型 (impl <TYPE> {})
        let ident = match ast.self_ty.as_ref() {
            Type::Path(tp) => {
                tp.path.get_ident().ok_or_else(|| Error::new_spanned(ast, "Not supported tokens"))?.clone()
            },
            _ => return Err(Error::new_spanned(ast, "Not supported tokens"))
        };
        let mut ctor = None;
        //解析函数列表
        let mut items = Vec::new();
        for item in &ast.items {
            if let ImplItem::Method(m) = item {
                for attr in &m.attrs {
                    if attr.path.is_ident("constructor") {
                        if ctor.is_none() {
                            ctor = Some(m.sig.ident.clone());
                        } else {
                            return Err(Error::new_spanned(item, "More then one constructor"));
                        }
                    } else if attr.path.is_ident("method") {
                        items.push(Item::Method(Method {
                            ident: m.sig.ident.clone(),
                            attr_args: MethodAttrArgs::parse(attr)?
                        }))
                    } else if attr.path.is_ident("event") {
                        items.push(Item::Event(Event {
                            ident: m.sig.ident.clone(),
                            attr_args: EventAttrArgs::parse(attr)?
                        }))
                    }
                }
            }
        }
        Ok(ImplBlock {
            ident,
            ctor,
            items
        })
    }
}

/// 方法项
struct Method {
    ident: Ident,
    attr_args: MethodAttrArgs
}

/// 方法属性参数
struct MethodAttrArgs {
    name: Option<String>,
    overload: usize
}

impl MethodAttrArgs {
    fn parse(attr: &Attribute) -> Result<Self> {
        let mut name = None;
        let mut overload = 0;

        match attr.parse_meta()? {
            Meta::List(list) => {
                for attr in list.nested {
                    match attr {
                        NestedMeta::Meta(Meta::NameValue(nv)) => {
                            if nv.path.is_ident("name") {
                                if let Lit::Str(lit) = nv.lit {
                                    name = Some(lit.value());
                                } else {
                                    return Err(Error::new_spanned(
                                        nv.lit,
                                        "Attribute name expects literal string!"
                                    ));
                                }
                            } else if nv.path.is_ident("overload") {
                                if let Lit::Int(lit) = nv.lit {
                                    overload = lit.base10_parse()?;
                                } else {
                                    return Err(Error::new_spanned(
                                        nv.lit,
                                        "Attribute overload expects int!"
                                    ));
                                }
                            } else {
                                return Err(Error::new_spanned(
                                    nv.path,
                                    "Unknown attribute key is specified. Allowed: name, overload"
                                ));
                            }
                        },
                        _ => return Err(Error::new_spanned(attr, "Unexpected attribute item type"))
                    }
                }
            },
            Meta::Path(_) => {},
            _ => return Err(Error::new_spanned(attr, "Unexpected attribute type"))
        }

        Ok(MethodAttrArgs {
            name,
            overload
        })
    }
}

/// 事件项
#[allow(dead_code)]
struct Event {
    ident: Ident,
    attr_args: EventAttrArgs
}

/// 事件属性参数
#[allow(dead_code)]
struct EventAttrArgs {
    name: Option<String>
}

impl EventAttrArgs {
    fn parse(attr: &Attribute) -> Result<Self> {
        let mut name = None;

        match attr.parse_meta()? {
            Meta::List(list) => {
                for attr in list.nested {
                    match attr {
                        NestedMeta::Meta(Meta::NameValue(nv)) => {
                            if nv.path.is_ident("name") {
                                if let Lit::Str(lit) = nv.lit {
                                    name = Some(lit.value());
                                } else {
                                    return Err(Error::new_spanned(
                                        nv.lit,
                                        "Attribute name expects literal string!"
                                    ));
                                }
                            } else {
                                return Err(Error::new_spanned(
                                    nv.path,
                                    "Unknown attribute key is specified. Allowed: name"
                                ));
                            }
                        },
                        _ => return Err(Error::new_spanned(attr, "Unexpected attribute item type"))
                    }
                }
            },
            Meta::Path(_) => {},
            _ => return Err(Error::new_spanned(attr, "Unexpected attribute type"))
        }

        Ok(EventAttrArgs {
            name
        })
    }
}
