pub(crate) fn run() {
    // Box 背后是调用 jemalloc 来做内存管理
    smart_ptr_box();
    smart_ptr_box_dyn();
    smart_ptr_box_leak();
}

fn smart_ptr_box_leak() {
    let s = gen_static_str();
    println!("{}", s);
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}

fn smart_ptr_box_dyn() {
    trait Draw {
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("这是屏幕上第{}号按钮", self.id)
        }
    }

    struct Select {
        id: u32,
    }

    impl Draw for Select {
        fn draw(&self) {
            println!("这个选择框贼难用{}", self.id)
        }
    }

    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }
}

fn smart_ptr_box() {
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    let b = *a + 1;
    println!("b = {}", b);
}
