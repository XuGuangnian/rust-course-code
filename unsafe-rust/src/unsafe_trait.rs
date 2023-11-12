pub fn run() {
    unsafe trait Foo {
        // 方法列表
        unsafe fn foo_test();
    }

    unsafe impl Foo for i32 {
        // 实现相应的方法
        unsafe fn foo_test() {
            println!("foo_test");
        }
    }

    unsafe {
        i32::foo_test();
    }
}
