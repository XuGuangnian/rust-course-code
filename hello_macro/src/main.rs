#![allow(dead_code)]

//
use hello_macro::HelloMacro;
use hello_macro::MyDefault;
use hello_macro_derive::{HelloMacro, MyDefault};

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

#[derive(Default, Debug)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

// MyDefault 宏展开
// impl MyDefault for SomeData {
//     fn default() -> Self {
//         Self {
//             0: u32::default(),
//             1: String::default(),
//         }
//     }
// }
#[derive(MyDefault, Debug)]
struct SomeData(u32, String);

// MyDefault 宏展开
// impl MyDefault for User {
//     fn default() -> Self {
//         Self {
//             name: String::default(),
//             data: SomeData::default(),
//         }
//     }
// }
#[derive(MyDefault, Debug)]
struct User {
    name: String,
    data: SomeData,
}

fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();

    let options: SomeOptions = Default::default();
    println!("{:?}", options);

    let options: SomeData = MyDefault::default();
    println!("{:?}", options);

    let options: User = MyDefault::default();
    println!("{:?}", options);
}
