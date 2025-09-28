fn main() {
    let block_dir = std::path::Path::new("tree-sitter-markdown").join("src");
    let inline_dir = std::path::Path::new("tree-sitter-markdown-inline").join("src");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(&block_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    if std::env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_headers = std::path::Path::new(&manifest_dir).join("wasm/include");
        let wasm_src = std::path::Path::new(&manifest_dir).join("wasm/src");

        println!("cargo:rerun-if-changed={}", wasm_headers.display());
        println!("cargo:rerun-if-changed={}", wasm_src.display());

        c_config.include(&wasm_headers);
        c_config.files([
            wasm_src.join("stdio.c"),
            wasm_src.join("stdlib.c"),
            wasm_src.join("string.c"),
        ]);
    }

    for path in &[
        block_dir.join("parser.c"),
        block_dir.join("scanner.c"),
        inline_dir.join("parser.c"),
        inline_dir.join("scanner.c"),
    ] {
        c_config.file(path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    c_config.compile("tree-sitter-markdown");
}
