use std::fmt;
use std::hash::Hash;
use std::ops::Deref;

pub(crate) fn run() {
    ui_component();
    trait_object_safety();
    associated_type();
    new_type();
}

// 方法的返回类型不能是 Self
// 方法没有任何泛型参数
// https://doc.rust-lang.org/reference/items/traits.html#object-safety
fn trait_object_safety() {
    // pub trait Clone {
    //     fn clone(&self) -> Self;
    // }

    // pub struct Screen {
    //     pub components: Vec<Box<dyn Clone>>, // `Clone` cannot be made into an object
    // }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
    }
}

#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn ui_component() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

fn associated_type() {
    pub trait CacheableItem: Clone + Default + fmt::Debug {
        type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
        fn is_null(&self) -> bool;
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.join(", ")) // 实现解引用 self->self.0
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn new_type() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
