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
            b: std::ptr::null(), // Creates a null raw pointer.
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

// let fut_one = /* ... */; // Future 1
// let fut_two = /* ... */; // Future 2
// async move {
//     fut_one.await;
//     fut_two.await;
// }

// `async { ... }`语句块创建的 `Future` 类型(编译成以下形式)
// struct AsyncFuture {
//     fut_one: FutOne,
//     fut_two: FutTwo,
//     state: State,
// }
//
// // `async` 语句块可能处于的状态
// enum State {
//     AwaitingFutOne,
//     AwaitingFutTwo,
//     Done,
// }
//
// impl Future for AsyncFuture {
//     type Output = ();
//
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
//         loop {
//             match self.state {
//                 State::AwaitingFutOne => match self.fut_one.poll(..) {
//                     Poll::Ready(()) => self.state = State::AwaitingFutTwo,
//                     Poll::Pending => return Poll::Pending,
//                 }
//                 State::AwaitingFutTwo => match self.fut_two.poll(..) {
//                     Poll::Ready(()) => self.state = State::Done,
//                     Poll::Pending => return Poll::Pending,
//                 }
//                 State::Done => return Poll::Ready(()),
//             }
//         }
//     }
// }

// async {
//     let mut x = [0; 128];
//     let read_into_buf_fut = read_into_buf(&mut x);
//     read_into_buf_fut.await;
//     println!("{:?}", x);
// }
//
// 这段代码会编译成下面的形式：
// struct ReadIntoBuf<'a> {
//     buf: &'a mut [u8], // 指向下面的`x`字段
// }
//
// struct AsyncFuture {
//     x: [u8; 128],
//     read_into_buf_fut: ReadIntoBuf<'what_lifetime?>,
// }
