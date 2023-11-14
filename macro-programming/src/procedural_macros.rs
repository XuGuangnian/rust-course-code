use hello_macro::{HelloMacro, MyDefault};
use hello_macro_derive::{HelloMacro, MyDefault};
use route_macro_attribute::route;

pub fn run() {
    // derive 宏
    derive_macro();
    // attribute-like macros
    attribute_like_macro();
    // function-like macros
    function_like_macro();
}

fn function_like_macro() {
    // todo
    // let sql = sql!(SELECT * FROM posts WHERE id=1);
}

fn attribute_like_macro() {
    // todo
    #[route(GET, "/")]
    fn index() {}
}

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

fn derive_macro() {
    Sunfei::hello_macro();
    Sunface::hello_macro();

    let options: SomeOptions = Default::default();
    println!("{:?}", options);

    let options: SomeData = MyDefault::default();
    println!("{:?}", options);

    let options: User = MyDefault::default();
    println!("{:?}", options);
}
