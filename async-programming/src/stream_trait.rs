use futures::channel::mpsc;
use futures::executor::block_on;
use futures::{SinkExt, StreamExt};

pub fn run() {
    stream_test();
}

fn stream_test() {
    block_on(send_recv());
}
// trait Stream {
//     // Stream生成的值的类型
//     type Item;
//
//     // 尝试去解析Stream中的下一个值,
//     // 若无数据，返回`Poll::Pending`, 若有数据，返回 `Poll::Ready(Some(x))`, `Stream`完成则返回 `Poll::Ready(None)`
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
//         -> Poll<Option<Self::Item>>;
// }

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
    // 因此还需要使用`.await`来获取具体的值
    // assert_eq!(Some(1), rx.next().await);
    // assert_eq!(Some(2), rx.next().await);
    // assert_eq!(None, rx.next().await);
    println!("{:?}", rx.next().await);
    println!("{:?}", rx.next().await);
    println!("{:?}", rx.next().await);
}
