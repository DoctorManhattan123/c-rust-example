use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=extern.h");

    let bindings = bindgen::Builder::default()
        .header("../extern.h")
        .generate()
        .expect("Unable to generate bindings");

    // Create a bindings directory if it doesn't exist
    let bindings_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("bindings");
    fs::create_dir_all(&bindings_dir).expect("Failed to create bindings directory");

    // Write the bindings to the bindings/bindings.rs file.
    bindings
        .write_to_file(bindings_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

