use regex::Regex;
use std::{env, path::Path};
use std::{ffi::OsString, path::PathBuf};
use tempdir::TempDir;

fn main() {
    let temp_dir = TempDir::new("acpica-sys")
        .expect("failed to create temporary directory for ACPICA compilation");

    prepare_temp_dir(&temp_dir);
    patch_acrust_include(&temp_dir);
    compile_acpica(&temp_dir);
}

fn prepare_temp_dir(temp_dir: &TempDir) {
    fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
        std::fs::create_dir_all(dst.as_ref())?;

        src.as_ref().read_dir()?.try_for_each(|entry| {
            let entry = entry?;
            let entry_ty = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.as_ref().join(entry.file_name());

            if entry_ty.is_dir() {
                copy_dir_all(src_path, dst_path)?;
            } else {
                std::fs::copy(src_path, dst_path)?;
            }

            Ok(())
        })
    }

    // copy all of the APCPICA source to the temp dir
    copy_dir_all("acpica/source/", temp_dir.path().join("source/"))
        .expect("failed to copy ACPICA source files to temporary directory for compilation");

    // copy the custom platform header we've premade
    std::fs::copy(
        "c_headers/acrust.h",
        temp_dir.path().join("source/include/platform/acrust.h"),
    )
    .expect("failed to copy `acrust.h` platform headers");

    std::fs::create_dir(temp_dir.path().join("artifacts/"))
        .expect("failed to create temporary artifacts dir");
}

fn patch_acrust_include(temp_dir: &TempDir) {
    let acenv_h_path = temp_dir.path().join("source/include/platform/acenv.h");

    let acenv_h = std::fs::read_to_string(acenv_h_path.as_path())
        .expect("could not find or read `source/include/platform/acenv.h`");

    let search_regex =
        Regex::new(r"(?s)#if defined\(_LINUX\).+?#endif").expect("failed to compile search regex");

    let acenv_h_patched = search_regex.replace(&acenv_h, r#"#include "acrust.h""#);

    if acenv_h_patched == acenv_h {
        panic!(
            "acenv.h should have contained a section of platform-specific includes (or detection failed)"
        );
    }

    std::fs::write(acenv_h_path.as_path(), acenv_h_patched.as_bytes())
        .expect("failed to write patched `acenv.h`");
}

fn compile_acpica(temp_dir: &TempDir) {
    cc::Build::new()
        .warnings(false)
        .include(temp_dir.path().join("source/include/"))
        .out_dir(temp_dir.path().join("artifacts/"))
        .define("ACPI_DEBUG_OUTPUT", None)
        .flag("-fno-stack-protector")
        .opt_level(1)
        .files({
            std::fs::read_dir(temp_dir.path().join("source/components/"))
                .expect("source directory should contains a `components` sub-directory")
                .map(|component_dir| component_dir.expect("could not read component directory"))
                .flat_map(|component_dir| {
                    std::fs::read_dir(component_dir.path())
                        .expect("failed to read the files within the component directory")
                        .map(|c_file| {
                            c_file.expect("failed to read C file from component directory")
                        })
                        // Exclude the debugger and disassembler dirs because they give 'undefined type' errors
                        .filter(|c_file| {
                            ![OsString::from("debugger"), OsString::from("disassembler")]
                                .contains(&c_file.file_name())
                        })
                        .map(|c_file| {
                            println!("cargo:warning=adding component: {:?}", c_file.path());
                            c_file.path()
                        })
                })
        })
        .compile("acpica");
}

fn generate_bindings() {
    println!("cargo:rustc-link-search=TODO");
    println!("cargo:rustc-link-lib=acpica");

    let bindings = bindgen::Builder::default()
        .header("c_headers/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");
}
