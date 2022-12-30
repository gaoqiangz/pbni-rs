#![allow(unused_imports)]
use std::{env, fs::OpenOptions, io::Write, path::PathBuf};

fn main() {
    if std::env::var_os("DOCS_RS").is_some() {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");

    //生成map文件
    #[cfg(feature = "symbol_map")]
    {
        println!("cargo:rustc-cdylib-link-arg=/MAP:symbol.map");
        //map文件生成导出符号表
        println!("cargo:rustc-cdylib-link-arg=/MAPINFO:EXPORTS");
    }

    #[cfg(feature = "pbx")]
    build_pbx();

    #[cfg(feature = "syslib")]
    build_syslib();
}

/// 编译`PBNI`
#[cfg(feature = "pbx")]
fn build_pbx() {
    println!("cargo:rerun-if-changed=cpp/bindings.cpp");

    let mut cfg = cc::Build::new();
    cfg.cpp(true).include("cpp").file("sdk/pbni/bindings.cpp");

    //PB10及以上版本
    cfg.define("UNICODE", None);
    cfg.define("_UNICODE", None);
    #[cfg(feature = "global_function")]
    cfg.define("GLOBALFUNCTION", None);
    #[cfg(feature = "nonvisualobject")]
    cfg.define("NONVISUALOBEJCT", None);
    #[cfg(feature = "visualobject")]
    cfg.define("VISUALOBEJCT", None);

    //cfg.warnings_into_errors(true);

    cfg.compile("pbx");
}

/// 编译`SystemLibrary`
#[cfg(feature = "syslib")]
fn build_syslib() {
    if std::env::var_os("GEN_SYSLIB") != Some("1".into()) {
        return;
    }

    let bindings = bindgen::builder()
        .header("sdk/syslib/api.h")
        .clang_args(["-x", "c++", "-std=c++14"])
        .disable_header_comment()
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true})
        .default_macro_constant_type(bindgen::MacroTypeVariation::Unsigned)
        .override_abi(bindgen::Abi::Stdcall, "_PB_.*")
        .derive_debug(true)
        .layout_tests(false)
        .dynamic_library_name("Api")
        .dynamic_link_require_all(true)
        .allowlist_var(".*_TYPE|.*_MASK|.*_SHIFT|.*_LEN|.*DATE.*|.*TIME.*")
        .allowlist_type("(OB_|POB_|SH|PSH).*")
        .allowlist_function("_PB_(pbstg_|ob_|ot_|rt|shformat|sh.*Dec|shdt).*")
        .blocklist_function("_PB_(pbstg_new_pool_with_size|pbstg_shrink|pbstg_dde_strdup|ob_set_main_obthis|rt_set_current_this|ob_narray_dynamic_item_init_callback|ob_group_is_normalized_window|ob_group_set_normalized_window|ob_add_liblist|ob_reload_group_source|ob_get_runtime_group_hndl|ob_insert_local_inst_ref_dbg|ob_load_pspp_dlls|ot_clear_array_data|ob_get_local_session|ob_lookup_routine_by_signature|ob_type_proto_add|ob_create_proto_throws_list|ob_create_proto_args|ob_proto_overload_search_src|ob_find_type_ancestor_assign|ob_get_pspp_class_name|ob_find_method|rt_build_exception_using_error|rt_handle_uncaught_exception|rt_populate_error_struct|rt_populate_error_from_stack|rt_call_error_callback|rtRoutineProtoInfoFree|rt_StartJaguarDebug|rt_StopJaguarDebug|rt_JagBreakpointHit|rt_JaguarGetCurrentContext|ot_strict_type_check|ot_generateVarInfo|ot_build_flditemupdate_refpak|ob_create_interface_in_library|rtdb_.*)")
        .blocklist_type("(rtdb_|PRTDB_|PBI|PPBI|PBObject|PBValue|PBVariable|PBArg|PBScript|PBType|PBCHAR).*")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("syslib.rs")).unwrap();
}
