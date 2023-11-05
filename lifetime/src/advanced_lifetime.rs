pub(crate) fn run() {
    closure_lifttime();
    reborrow();
    impl_lifetime_elision();
    lifetime_examples();
}

#[allow(dead_code)]
fn lifetime_examples() {
    {
        let mut list = List {
            manager: Manager { text: "hello" },
        };

        list.get_interface().noop();

        println!("Interface should be dropped here and the borrow released");

        // 下面的调用会失败，因为同时有不可变/可变借用
        // 但是Interface在之前调用完成后就应该被释放了
        use_list(&list);
    }

    struct Interface<'a, 'b> {
        manager: &'b mut Manager<'a>,
    }

    impl<'a, 'b> Interface<'a, 'b> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }

    struct Manager<'a> {
        text: &'a str,
    }

    struct List<'a> {
        manager: Manager<'a>,
    }

    impl<'a> List<'a> {
        pub fn get_interface<'b>(&'b mut self) -> Interface<'a, 'b> {
            Interface {
                manager: &mut self.manager,
            }
        }
    }

    fn use_list(list: &List) {
        println!("{}", list.manager.text);
    }
}

#[allow(dead_code)]
fn impl_lifetime_elision() {
    trait Reader {
        fn version(&self) -> &str;
    }
    struct Book<'a> {
        content: &'a str,
    }
    impl Reader for Book<'_> {
        // impl内部实际上没有用到'a, 使用 '_ ：有一个不使用的生命周期，我们可以忽略它，无需为它创建一个名称。
        fn version(&self) -> &str {
            "Reader 1.0"
        }
    }

    // Rust 2015
    struct Ref<'a, T: 'a> {
        field: &'a T,
    }

    // Rust 2018
    struct Ref2<'a, T> {
        field: &'a T,
    }
}

fn reborrow() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn move_to(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
    }

    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr = &*r;

    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
}

fn closure_lifttime() {
    fn fn_elision(x: &i32) -> &i32 {
        x
    }
    // returning this value requires that `'1`(input x) must outlive `'2`(return x)
    // let closure_slision = |x: &i32| -> &i32 { x };
    // 通过 `Fn` Trait 解决闭包声明周期
    let closure_elision = fun(|x: &i32| -> &i32 { x });
    assert_eq!(*fn_elision(&45), 45);
    assert_eq!(*closure_elision(&45), 45);
}

fn fun<T, F: Fn(&T) -> &T>(f: F) -> F {
    f
}
