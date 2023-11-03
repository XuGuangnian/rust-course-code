#![allow(unused)]

use std::fmt::{Display, Formatter};

pub(crate) fn run() {
    struct_mut();
    struct_memory_structure();
    tuple_struct();
    unit_like_struct();
    reference_in_struct();
    dbg_macro();
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn struct_mut() {
    // Rust 不支持将某个结构体某个字段标记为可变。
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // .. 语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // user1的username移动到user2, 其他 实现`Copy`的字段还可以继续使用
    };

    println!("{}", user1.active);
    // 下面这行会报错
    // println!("{:?}", user1);
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn struct_memory_structure() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn unit_like_struct() {
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    impl ToString for AlwaysEqual {
        fn to_string(&self) -> String {
            format!("{}", "AlwaysEqual")
        }
    }

    println!("{}", subject.to_string());
}

struct UserReference<'a, 'b> {
    username: &'a str,
    email: &'b str,
    sign_in_count: u64,
    active: bool,
}

fn reference_in_struct() {
    let user1 = UserReference {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn dbg_macro() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
