use std::fmt::{Debug, Display};
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn main() {
    lifetime_bound();
    static_lifetime();
    static_lifetime_bound();
    static_lifetime_bound_2();

    // rust-by-example 15.4.8 static
    rust_by_example_static();
}

// 产生一个拥有 `'static` 生命周期的常量。
static NUM: i32 = 18;

// 返回一个指向 `NUM` 的引用，该引用不取 `NUM` 的 `'static` 生命周期，
// 而是被强制转换成和输入参数的一样。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn rust_by_example_static() {
    {
        // 产生一个 `string` 字面量并打印它：
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 离开作用域时，该引用不能再使用，不过
        // 数据仍然存在于二进制文件里面。
    }

    {
        // 产生一个整型给 `coerce_static` 使用：
        let lifetime_num = 9;

        // 将对 `NUM` 的引用强制转换成 `lifetime_num` 的生命周期：
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

fn static_lifetime_bound_2() {
    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 42;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
        // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
    }

    println!("&'static i32: {}", r1); // -> 42
    println!("&'static str: {}", r2); // -> &'static str

    let r3: &str;

    {
        let s1 = "String".to_string();

        // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
        // 充分说明这个约束是多么的弱。。
        static_bound(&s1);

        // s1 是 String 类型，没有 'static 的生命周期，因此下面代码(println!("{}", r3);)会报错
        r3 = &s1;
        println!("{}", r3);

        // s1 在这里被 drop
    }
    // println!("{}", r3);
}

// 原因在于我们约束的是 T，但是使用的却是它的引用 &T，换而言之，我们根本没有直接使用 T，因此编译器就没有去检查 T 的生命周期约束！
fn static_bound<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn static_lifetime_bound() {
    let i = 5;
    print_it(i);
    // print_it(&i); // &i的生命周期不是 ‘static
    print_it_2(&i); // i 的生命周期是 ’static

    let i = "hello";
    print_it(i); // i 为 &str 声明周期满足 'static
    print_it_2(&i); // i 的生命周期是 ’static
}

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it_2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn static_lifetime() {
    // 'static vs T: 'static
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);
}

fn get_memory_location() -> (usize, usize) {
    // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
    // `string` 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn lifetime_bound() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    i.announce_and_return_part("announcement");
    i.announce_and_return_part_2("announcement_2");
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime bound
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part // 'b < 'a -> 'a: 'b
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part_2<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}
