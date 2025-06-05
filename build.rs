use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=TODO");
    println!("cargo:rustc-link-lib=acpica");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");
}
