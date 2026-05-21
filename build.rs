use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=OPENFX_DIR");

    let openfx_dir = env::var("OPENFX_DIR")
        .expect("Please set the OPENFX_DIR environment variable pointing to your OpenFX folder");

    let include_paths = [
        format!("-I{openfx_dir}/include"),
        format!("-I{openfx_dir}/HostSupport/include"),
    ];

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(["-x", "c++"])
        .clang_args(include_paths)
        .enable_cxx_namespaces()
        .allowlist_type("Ofx.*")
        .allowlist_var("kOfx.*")
        .allowlist_type("OFX::.*")
        .allowlist_function("OFX::.*")
        .opaque_type("std::.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
