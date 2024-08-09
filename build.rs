use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let eigen3_include_path =
        env::var("EIGEN3_INCLUDE_DIRS").unwrap_or("/opt/homebrew/include/eigen3".to_string());

    let opencv_include_path =
        env::var("OpenCV_INCLUDE_DIRS").unwrap_or("/opt/homebrew/include/opencv4".to_string());

    let includes = vec![
        ".",
        "./lib/ByteTrack/include",
        &eigen3_include_path,
        &opencv_include_path,
    ];

    bytetrack(&out_dir, "./lib/ByteTrack");
    bytetrack_sdk(&out_dir, &includes);
}

fn bytetrack(out: &PathBuf, source: &str) {
    Command::new("cmake")
        .current_dir(source)
        .arg(".")
        .arg("-B")
        .arg(out)
        .status()
        .unwrap();

    Command::new("cmake")
        .current_dir(out)
        .arg("--build")
        .arg(".")
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=bytetrack");
    println!("cargo::rerun-if-changed=bytetrack.cpp");
}

fn bytetrack_sdk(out: &PathBuf, includes: &[&str]) {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .cpp_set_stdlib("c++")
        .includes(includes)
        .file("bytetrack.cpp")
        .compile("bytetrack_sdk");

    let c_args = includes
        .iter()
        .map(|v| format!("-I{v}"))
        .collect::<Vec<_>>();

    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg("-stdlib=libc++")
        .clang_args(c_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .header("bytetrack.h")
        .allowlist_function("create_object")
        .allowlist_function("create_bt_tracker")
        .allowlist_function("update_bt_tracker")
        .allowlist_function("destroy_bt_tracker")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=static=bytetrack_sdk");
}
