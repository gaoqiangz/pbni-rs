#![allow(unused_imports)]
use std::{env, fs::OpenOptions, io::Write, path::PathBuf};

fn main() {
    if std::env::var_os("DOCS_RS").is_some() {
        return;
    }

    //生成map文件
    #[cfg(feature = "symbol_map")]
    {
        println!("cargo:rustc-cdylib-link-arg=/MAP:symbol.map");
        //map文件生成导出符号表
        println!("cargo:rustc-cdylib-link-arg=/MAPINFO:EXPORTS");
    }

    //#[cfg(feature = "syslib")]
    //build_syslib();

    #[cfg(feature = "pbni")]
    build_pbni();
}

/// 编译`PBNI`
#[cfg(feature = "pbni")]
fn build_pbni() {
    println!("cargo:rerun-if-changed=cpp/bindings.cpp");

    let mut cfg = cc::Build::new();
    cfg.cpp(true).include("cpp").file("sdk/pbni/bindings.cpp");

    //PB10及以上版本
    cfg.define("UNICODE", None);
    cfg.define("_UNICODE", None);

    //cfg.warnings_into_errors(true);

    cfg.compile("pbx");

    //生成PBNI导出符号定义文件
    //NOTE 解决rust msvc 32bit dll编译符号被截断的问题
    let mut file_path = std::env::var_os("OUT_DIR").map(PathBuf::from).unwrap();
    file_path.push("export.def");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)
        .expect("fail to create export.def");
    file.write_all(b"LIBRARY\r\n").unwrap();
    file.write_all(b"EXPORTS\r\n").unwrap();
    file.write_all(b"\t_PBX_GetVersion@0\r\n").unwrap();

    #[cfg(feature = "global_function")]
    file.write_all(b"\t_PBX_InvokeGlobalFunction@12\r\n").unwrap();
    #[cfg(feature = "nonvisualobject")]
    file.write_all(b"\t_PBX_CreateNonVisualObject@16\r\n").unwrap();
    #[cfg(feature = "visualobject")]
    file.write_all(b"\t_PBX_CreateVisualObject@16\r\n").unwrap();

    //覆盖DEF文件链接
    println!("cargo:rustc-cdylib-link-arg=/DEF:{}", file_path.display());
}

/*
/// 编译`SystemLibrary`
#[cfg(feature = "syslib")]
fn build_syslib() {
    let bindings = bindgen::builder()
        .header("sdk/syslib/pbvm.h")
        .clang_args(["-x", "c++", "-std=c++14"])
        .derive_debug(false)
        .layout_tests(false)
        .dynamic_library_name("Api")
        .dynamic_link_require_all(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .allowlist_function("_PB_.*")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("pbvm.rs")).unwrap();
}
*/
