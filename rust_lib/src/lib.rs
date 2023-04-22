#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings/bindings.rs"));

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
    unsafe {
        hello_from_c();
    }
}

