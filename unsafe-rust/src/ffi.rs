pub fn run() {
    call_extern_c();
    call_rust_from_c();
    // rust-bindgen: rust calls c
    // cbindgen: c calls rust
}

#[no_mangle] // 它用于告诉 Rust 编译器：不要乱改函数的名称
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn call_rust_from_c() {
    // 假装是c调用
    call_from_c();
}

// "C" 定义了外部函数所使用的应用二进制接口ABI (Application Binary Interface)
// ABI 定义了如何在汇编层面来调用该函数。在所有 ABI 中，C 语言的是最常见的。
extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_extern_c() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
