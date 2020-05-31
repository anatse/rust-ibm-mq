extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let library_path = option_env!("DYLD_LIBRARY_PATH")
        .unwrap_or("/Users/anatolysementsov/projects/9.1.5.0-IBM-MQ-Toolkit-MacX64/lib64/");

    println!("cargo:rustc-flags=-l mqic_r -L {}", library_path);
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=mqic_r");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
