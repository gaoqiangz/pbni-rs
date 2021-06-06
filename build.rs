fn main() {
    println!("cargo:rerun-if-changed=cpp/bindings.cpp");

    let mut cfg = cc::Build::new();
    cfg.cpp(true).include("cpp").file("cpp/bindings.cpp");

    //PB10及以上版本
    cfg.define("UNICODE", None);
    cfg.define("_UNICODE", None);

    //cfg.warnings_into_errors(true);

    cfg.compile("pbx");

    build_def();
}

//生成导出符号定义文件
//解决rust msvc 32bit dll编译符号被截断的问题
fn build_def() {
    use std::{fs::OpenOptions, io::Write, path::PathBuf};

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
    //生成map文件
    println!("cargo:rustc-cdylib-link-arg=/MAP:symbol.map");
    //map文件生成导出符号表
    println!("cargo:rustc-cdylib-link-arg=/MAPINFO:EXPORTS");
}
