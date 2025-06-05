use std::{
    ffi::OsString,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};

static TEMP_DIR: LazyLock<tempdir::TempDir> = LazyLock::new(|| {
    tempdir::TempDir::new("acpica-sys")
        .expect("failed to create temporary directory for ACPICA compilation")
});
static SOURCE_DIR: LazyLock<PathBuf> = LazyLock::new(|| TEMP_DIR.path().join("source/"));
static SOURCE_INCLUDE_DIR: LazyLock<PathBuf> = LazyLock::new(|| SOURCE_DIR.join("include/"));
static SOURCE_INCLUDE_PLATFORM_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| SOURCE_INCLUDE_DIR.join("platform/"));
static SOURCE_COMPONENTS_DIR: LazyLock<PathBuf> = LazyLock::new(|| SOURCE_DIR.join("components/"));

fn main() {
    prepare_temp_dir();
    patch_acrust_include();
    compile_acpica();
    generate_bindings();
    cleanup();
}

fn prepare_temp_dir() {
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
    copy_dir_all("acpica/source/", SOURCE_DIR.as_path())
        .expect("failed to copy ACPICA source files to temporary directory for compilation");

    // copy the custom platform header we've premade
    std::fs::copy(
        "c_headers/acrust.h",
        SOURCE_INCLUDE_PLATFORM_DIR.join("acrust.h"),
    )
    .expect("failed to copy `acrust.h` platform headers");
}

fn patch_acrust_include() {
    let acenv_h_path = SOURCE_INCLUDE_PLATFORM_DIR.join("acenv.h");

    let acenv_h = std::fs::read_to_string(acenv_h_path.as_path())
        .expect("could not find or read `source/include/platform/acenv.h`");

    let search_regex = regex::Regex::new(r"(?s)#if defined\(_LINUX\).+?#endif")
        .expect("failed to compile search regex");

    let acenv_h_patched = search_regex.replace(&acenv_h, r#"#include "acrust.h""#);

    if acenv_h_patched == acenv_h {
        panic!(
            "acenv.h should have contained a section of platform-specific includes (or detection failed)"
        );
    }

    std::fs::write(acenv_h_path.as_path(), acenv_h_patched.as_bytes())
        .expect("failed to write patched `acenv.h`");
}

fn compile_acpica() {
    cc::Build::new()
        .warnings(false)
        .include(SOURCE_INCLUDE_DIR.as_path())
        .define("ACPI_DEBUG_OUTPUT", None)
        .flag("-fno-stack-protector")
        .flag("-Wno-format-truncation") // Get rid of annoying warning when compiling ACPICA.
        .opt_level(1)
        .files({
            std::fs::read_dir(SOURCE_COMPONENTS_DIR.as_path())
                .expect("source directory should contain a `components` sub-directory")
                .map(|component_dir| component_dir.expect("could not read component directory"))
                .filter(|component_dir| {
                    // Exclude the debugger and disassembler dirs because they give 'undefined type' errors.
                    // TODO consider fixing this if the needs arises on the OS side.
                    ![OsString::from("debugger"), OsString::from("disassembler")]
                        .contains(&component_dir.file_name())
                })
                .flat_map(|component_dir| {
                    std::fs::read_dir(component_dir.path())
                        .expect("failed to read the files within the component directory")
                        .map(|c_file| {
                            c_file.expect("failed to read C file from component directory")
                        })
                        .map(|c_file| c_file.path())
                })
        })
        .compile("acpica");
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .use_core()
        .header("acpica/source/include/acpi.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindings");

    let mut lib_file = OpenOptions::new()
        .create(false)
        .write(true)
        .truncate(true)
        .open("src/lib.rs")
        .expect("could not open `lib.rs`");
    lib_file
        .write_fmt(format_args!(
            r#"#![no_std]
#![allow(
    dead_code,
    unused_imports,
    improper_ctypes,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc
)]

"#
        ))
        .expect("failed to write attributes to `lib.rs`");

    bindings
        .write(Box::new(&lib_file))
        .expect("failed to write bindings");
}

fn cleanup() {
    Command::new("cargo")
        .arg("fmt")
        .output()
        .expect("failed to format crate");
}
