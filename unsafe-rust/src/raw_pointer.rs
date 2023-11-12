use std::{slice, slice::from_raw_parts, str::from_utf8_unchecked};

// 解引用裸指针，就如上例所示
// 调用一个 unsafe 或外部的函数
// 访问或修改一个可变的静态变量
// 实现一个 unsafe 特征
// 访问 union 中的字段
pub fn run() {
    raw_pointer_deref();
    raw_pointer_deref_from_ref();
    raw_pointer_deref_from_smart_pointer();
    call_unsafe_fn();
    safe_fn_wrap_unsafe_code();
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn safe_fn_wrap_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

unsafe fn dangerous() {}

fn call_unsafe_fn() {
    unsafe {
        dangerous();
    }
}

fn raw_pointer_deref_from_smart_pointer() {
    let a: Box<i32> = Box::new(10);
    // 需要先解引用a
    let b: *const i32 = &*a;
    // 使用 into_raw 来创建
    let c: *const i32 = Box::into_raw(a); // a moved
    unsafe {
        println!("raw_p1: {}, raw_p2: {}", *b, *c);
    }
}

fn raw_pointer_deref() {
    let mut num = 5;
    let raw_p1 = &num as *const i32;
    let raw_p2: &mut i32 = &mut num;
    // 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为
    unsafe {
        println!("raw_p1: {}, raw_p2: {}", *raw_p1, *raw_p2);
    }
}

// 获取字符串的内存地址和长度
fn get_memory_location() -> (usize, usize) {
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn raw_pointer_deref_from_ref() {
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:x} stored: {}", // 二进制:b 八进制:o 十六进制:x
        length, pointer, message
    );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);
}
