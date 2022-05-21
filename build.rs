fn main() {
    let library_path = option_env!("DYLD_LIBRARY_PATH")
        .unwrap_or("/opt/mqm/lib64/");

    println!("cargo:rustc-flags=-l mqic_r -L {}", library_path);
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=mqic_r");
}
