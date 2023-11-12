mod ffi;
mod raw_pointer;
mod union;
mod unsafe_trait;

fn main() {
    raw_pointer::run();
    ffi::run();
    unsafe_trait::run();
    union::run();
    // bindgen => build.rs
}
