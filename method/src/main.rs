fn main() {
    method_getter();
    enum_method();
}

// 一般来说，方法跟字段同名，往往适用于实现 getter 访问器
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // （关联）函数不能用`.`来调用，需要用 `::` 来调用
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }
}

fn method_getter() {
    let rect1 = Rectangle::new(30, 50);

    // automatic referencing and dereferencing: `.`
    println!("{}", rect1.width());
    println!("{}", (&rect1).width());
    println!("{}", (&&rect1).width());
    // 也可以通过 T::method 调用，但需要手动匹配类型（自动引用与解引用失效）
    println!("{}", Rectangle::height(&rect1));
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
        println!("{:?}", self);
    }
}

fn enum_method() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
