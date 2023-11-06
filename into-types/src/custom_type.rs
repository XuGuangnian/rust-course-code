use std::fmt;

pub(crate) fn run() {
    // 为外部类型实现外部特征
    impl_trait_for_newtype();
    type_alias();
    never_type();
}

fn never_type() {
    let i = 2;
    let v = match i {
        0..=3 => i,
        _ => panic!("不合规定的值:{}", i), // panic!的返回值是`!`，代表它不会返回任何值
    };
    println!("{}", v);
}

fn type_alias() {
    type Meters = u32;

    let x: u32 = 5;
    let y: Meters = 5;

    println!("x + y = {}", x + y);
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn impl_trait_for_newtype() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
