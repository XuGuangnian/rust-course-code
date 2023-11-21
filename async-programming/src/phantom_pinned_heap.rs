use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;

use pin_utils::pin_mut;

pub fn run() {
    heap_test();
    unpin_future();
}

fn unpin_future() {
    // 函数的参数是一个`Future`，但是要求该`Future`实现`Unpin`
    fn execute_unpin_future(x: impl Future<Output = ()> + Unpin) {
        /* ... */
    }

    let fut = async { /* ... */ };
    // 下面代码报错: 默认情况下，`fut` 实现的是`!Unpin`，并没有实现`Unpin`
    // execute_unpin_future(fut);

    // 使用`Box`进行固定
    let fut = async { /* ... */ };
    let fut = Box::pin(fut);
    execute_unpin_future(fut); // OK

    // 使用`pin_mut!`进行固定
    let fut = async { /* ... */ };
    pin_mut!(fut);
    execute_unpin_future(fut); // OK
}

fn heap_test() {
    let test1 = Test::new("test1");
    let test2 = Test::new("test2");

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}

struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe {
            boxed.as_mut().get_unchecked_mut().b = self_ptr;
        }

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}
