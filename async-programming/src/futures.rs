use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::time::Duration;

use async_std::task::sleep;

pub fn run() {
    // join_test();
}

fn join_test() {
    let future_a = future_a();
    let future_b = future_b();
    // todo 实现一个可异步运行的Join 和 AndThenFut，进行测试
    // let join = Join {
    //     a: Some(future_a),
    //     b: Some(future_b),
    // };
}

async fn future_a() {
    for i in 0..10 {
        println!("FutureA running {i}");
        sleep(Duration::from_millis(1)).await;
    }
}

async fn future_b() {
    for i in 0..10 {
        println!("FutureB running {i}");
        sleep(Duration::from_millis(1)).await;
    }
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

/// 一个SimpleFuture，它会并发地运行两个Future直到它们完成
///
/// 之所以可以并发，是因为两个Future的轮询可以交替进行，一个阻塞，另一个就可以立刻执行，反之亦然
pub struct Join<FutureA, FutureB> {
    // 结构体的每个字段都包含一个Future，可以运行直到完成.
    // 等到Future完成后，字段会被设置为 `None`. 这样Future完成后，就不会再被轮询
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> Future for Join<FutureA, FutureB> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // 尝试去完成一个 Future `a`
        if let Some(a) = &mut self.a {
            // 此处调用poll，传入函数指针，在poll方法中执行wake函数
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take(); // Takes the value out of the option, leaving a None in its place.
            }
        }

        // 尝试去完成一个 Future `b`
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            // 两个 Future都已完成 - 我们可以成功地返回了
            Poll::Ready(())
        } else {
            // 至少还有一个 Future 没有完成任务，因此返回 `Poll::Pending`.
            // 当该 Future 再次准备好时，通过调用`wake()`函数来继续执行
            Poll::Pending
        }
    }
}

/// 一个SimpleFuture, 它使用顺序的方式，一个接一个地运行两个Future
//
// 注意: 由于本例子用于演示，因此功能简单，`AndThenFut` 会假设两个 Future 在创建时就可用了.
// 而真实的`Andthen`允许根据第一个`Future`的输出来创建第二个`Future`，因此复杂的多。
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> Future for AndThenFut<FutureA, FutureB> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // 我们已经完成了第一个 Future， 可以将它移除， 然后准备开始运行第二个
                Poll::Ready(()) => self.first.take(), // Takes the value out of the option, leaving a None in its place.
                // 第一个 Future 还不能完成
                Poll::Pending => return Poll::Pending,
            };
        }

        // 运行到这里，说明第一个Future已经完成，尝试去完成第二个
        self.second.poll(wake)
    }
}
