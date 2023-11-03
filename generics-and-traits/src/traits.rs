use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Add;

pub(crate) fn run() {
    tty_into_trait();
    impl_add_trait();
    impl_display_trait();
}

fn tty_into_trait() {
    let a: u8 = 10;
    let b: u16 = 100;
    // let b: u16 = 1000;

    let b_: u8 = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T>> {
    //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, p: Self) -> Self::Output {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn impl_add_trait() {
    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "Open"),
            FileState::Closed => write!(f, "Closed"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn impl_display_trait() {
    let mut f6 = File::new("f6.txt");
    f6.state = FileState::Open;

    println!("{:?}", f6);
    println!("{:?}", f6.state);
    println!("{:?}", f6.data);
    println!("Customized format output: {}", f6);
}
