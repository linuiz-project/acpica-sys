# acpica-sys
System bindings for the ACPICA libray.

## Methodology
This means of implementation of modifying the compilation is taken primarily from: https://github.com/MarkRoss470/acpica-rust-bindings/

However, the method has been adapted for usage in a continuously integrated pipeline, allowing the `acpica` repository to make changes and updates that will be pulled via GitHub Actions, recompiling the repository and opening a pull request with the updated binaries.
