use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub(crate) fn run() {
    single_prod_single_cons();
    multi_prod_single_cons();
    mpsc_sync_channel();
    send_multi_types();
    // mpmc crate: crossbeam-channel flume
}

enum Fruit {
    Apple(u8),
    Orange(String),
}

fn send_multi_types() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}

fn mpsc_sync_channel() {
    let (tx, rx) = mpsc::sync_channel(1);

    let handle = thread::spawn(move || {
        println!("首次发送之前");
        tx.send(1).unwrap();
        println!("首次发送之后");
        tx.send(1).unwrap();
        println!("再次发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

fn multi_prod_single_cons() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn single_prod_single_cons() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 在主线程中接收子线程发送的消息并输出
    // println!("receive {}", rx.try_recv().unwrap()); // 不会阻塞线程，当通道中没有消息时，它会立刻返回一个错误
    println!("receive {}", rx.recv().unwrap());
}
