use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse, AttributeArgs, Error, ItemFn, Lit, Meta, NestedMeta, Result};

/// 生成全局函数代码
pub fn gen(args: AttributeArgs, input: TokenStream) -> Result<TokenStream> {
    let args = AttrArgs::parse(args)?;
    let ast = parse::<ItemFn>(input)?;
    let ident = ast.sig.ident.clone();
    let ident_st = format_ident!("{}_st", ident);
    let ident_reg = format_ident!("{}_reg", ident);
    let name = args.name.unwrap_or_else(|| ident.to_string());
    let cls_name = name.to_ascii_lowercase();

    let output = quote! {
        #ast
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        struct #ident_st;
        impl ::pbni::pbx::__private::codegen::GlobalFunction for #ident_st {
            const NAME: &'static ::pbni::primitive::PBStr = ::pbni::pbstr!(#cls_name);
            fn invoke(ci: ::pbni::pbx::CallInfoRef) -> ::pbni::pbx::Result<()> {
                ::pbni::pbx::__private::codegen::safe_invoke(
                    ci.session(),
                    stringify!(#name),
                    concat!(module_path!(),"::",stringify!(#ident)),
                    file!(),line!(),column!(),
                    ||::pbni::pbx::__private::codegen::global_function_factory_call(#ident, &ci)
                )
            }
        }
        #[::pbni::pbx::__private::codegen::constructor]
        extern "C" fn #ident_reg() {
            use ::pbni::pbx::__private::codegen::GlobalFunction;
            <#ident_st as GlobalFunction>::register();
        }
    };

    Ok(output.into())
}

/// 属性参数
struct AttrArgs {
    name: Option<String>
}

impl AttrArgs {
    fn parse(args: AttributeArgs) -> Result<Self> {
        let mut name = None;

        for arg in args {
            match arg {
                NestedMeta::Meta(Meta::NameValue(nv)) => {
                    if nv.path.is_ident("name") {
                        if let Lit::Str(lit) = nv.lit {
                            name = Some(lit.value());
                        } else {
                            return Err(Error::new_spanned(nv.lit, "Attribute name expects literal string!"));
                        }
                    } else {
                        return Err(Error::new_spanned(
                            nv.path,
                            "Unknown attribute key is specified. Allowed: name"
                        ));
                    }
                },
                _ => return Err(Error::new_spanned(arg, "Unexpected attribute type"))
            }
        }

        Ok(AttrArgs {
            name
        })
    }
}
