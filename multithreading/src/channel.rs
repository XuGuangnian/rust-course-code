use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub(crate) fn run() {
    single_prod_single_cons();
    single_prod_single_cons_for();
    multi_prod_single_cons();
    mpsc_sync_channel();
    send_multi_types();
    channel_attention();
    // mpmc crate: crossbeam-channel flume
}

fn channel_attention() {
    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 在这里drop send...
    drop(send); // drop main线程中的 send

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
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
    // 同步通道 bound: 消息缓存条数
    let (tx, rx) = mpsc::sync_channel(1);

    let handle = thread::spawn(move || {
        println!("首次发送之前");
        tx.send(1).unwrap(); // bound = 0 时，接收者未准备好（等待接收），发送者会阻塞
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
        thread::sleep(Duration::from_secs(3));
    });

    for received in rx {
        // 需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
        println!("Got: {}", received);
    }
}

fn single_prod_single_cons_for() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // std 内部实现
    // impl<T> Iterator for IntoIter<T> {
    //     type Item = T;
    //     fn next(&mut self) -> Option<T> {
    //         self.rx.recv().ok() // 接收者在此处等待接收
    //     }
    // }
    for received in rx {
        // 默认调用 rx.into_iter() -> IntoIter
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
    println!("receive {}", rx.recv().unwrap()); // rx.recv()会阻塞当前线程，直到读取到值，或者通道被关闭
}
