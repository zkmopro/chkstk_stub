use std::{env, fs, path::Path};

fn main() {
    let target = env::var("TARGET").expect("TARGET not set");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    // Create the stub source file
    if target.contains("apple-ios") {
        stub_for_ios(&out_dir);
    }
}

fn stub_for_ios(out_dir: &str) {
    let stub_path = Path::new(&out_dir).join("chkstk_stub.c");
    fs::write(
        &stub_path,
        r#"
        void __chkstk_darwin(void) {}
        "#,
    )
    .unwrap();

    // Compile the stub into a static lib
    cc::Build::new()
        .file(&stub_path)
        .out_dir(&out_dir)
        .compile("chkstk_stub");

    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=static=chkstk_stub");
}
