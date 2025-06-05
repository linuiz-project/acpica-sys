# acpica-sys
System bindings for the ACPICA libray.

## Methodology
This means of implementation of modifying the compilation is taken primarily from: https://github.com/MarkRoss470/acpica-rust-bindings/

However, the method has been adapted for usage with ACPICA as a submodule that will be copied, compiled, and `rust-bindgen`'d to create the requisite bindings automatically. Each ACPICA version update will correlate with a release in this repository.
