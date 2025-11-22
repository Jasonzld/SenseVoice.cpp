use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=../src/");
    println!("cargo:rerun-if-changed=../include/");

    // 链接 SenseVoice.cpp 库
    // 注意：实际编译时需要先构建 SenseVoice.cpp
    let build_dir = "../build";
    println!("cargo:rustc-link-search={}", build_dir);

    // 静态链接 SenseVoice 库
    // 这是一个占位符，实际库名可能不同
    // println!("cargo:rustc-link-lib=static=sensevoice");

    // 链接系统库
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=Foundation");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=ole32");
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=pthread");
    }

    // 生成 Rust FFI 绑定
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // 允许的类型
        .allowlist_function("sensevoice_.*")
        .allowlist_type("SenseVoice.*")
        // 生成文档注释
        .generate_comments(true)
        // 使用核心库而不是标准库
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
