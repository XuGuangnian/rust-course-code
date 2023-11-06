use std::ops::Deref;
use std::ops::DerefMut;

pub(crate) fn run() {
    custom_smart_ptr();
    //三种 Deref 转换
    // 当 T: Deref<Target=U>，可以将 &T 转换成 &U，也就是我们之前看到的例子
    // 当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
    // 当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
    deref_mut_example();
}

fn deref_mut_example() {
    let mut s = MyBox::new(String::from("hello, "));
    display(&mut s);
}

struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn display(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}

fn custom_smart_ptr() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let y = MyBox::new(5);

    assert_eq!(5, *y);
}
