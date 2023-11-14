use std::mem;

pub fn run() {
    pin_test();
}

fn pin_test() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!(
        "a: {}, &a:{:p} b: {}, {:?}",
        test1.a(),
        &test1.a,
        test1.b(),
        test1.b
    );
    println!(
        "a: {}, &b:{:p},b: {}, {:?}",
        test2.a(),
        &test2.a,
        test2.b(),
        test2.b
    );
    println!("test1: {:p}", &test1);
    println!("a: {}, b: {}", test1.a(), test1.b());
    mem::swap(&mut test1, &mut test2); // 交换后test1和test2指向的内存地址没有变，值变为对方的
    println!("test1: {:p}", &test1);
    test1.a = "I've totally changed now!".to_string();
    println!("a: {}, b: {}", test2.a(), test2.b());
}

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}
