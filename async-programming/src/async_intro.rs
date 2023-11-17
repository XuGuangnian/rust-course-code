use std::thread;
use std::time::Duration;

use async_std::task::sleep;
use futures::executor;

pub fn run() {
    get_two_sites();
    let future = get_two_sites_async(); // 对于异步函数需要执行器Executor
                                        // 异步函数的返回值是一个 Future，若直接调用该函数，不会输出任何结果，因为 Future 还未被执行
                                        // futures do nothing unless you `.await` or poll them
    executor::block_on(future); // `block_on`会阻塞当前线程直到指定的`Future`执行完成，这种阻塞当前线程以等待任务完成的方式较为简单、粗暴，
                                // 好在其它运行时的执行器(executor)会提供更加复杂的行为，例如将多个`future`调度到同一个线程上执行。

    let future = hello_world();
    executor::block_on(future);

    executor::block_on(async_await());
}

fn get_two_sites() {
    // 创建两个新线程执行任务
    let thread_one = thread::spawn(|| download("https://course.rs"));
    let thread_two = thread::spawn(|| download("https://fancy.rs"));

    // 等待两个线程的完成
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

fn download(url: &str) {
    println!("{}", url);
}

async fn get_two_sites_async() {
    // 创建两个不同的`future`，你可以把`future`理解为未来某个时刻会被执行的计划任务
    // 当两个`future`被同时执行后，它们将并发的去下载目标页面
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // 同时运行两个`future`，直至完成
    futures::join!(future_one, future_two);
}

async fn download_async(url: &str) {
    println!("{}", url);
}

async fn async_await() {
    let f1 = learn_and_sing();
    let f2 = dance();
    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}

async fn hello_world() {
    // 在一个async fn函数中去调用另一个async fn并等待其完成后再执行后续的代码
    hello_cat().await; // futures do nothing unless you `.await` or poll them
                       // 与 block_on 不同，.await 不会阻塞当前线程，而是异步等待 future 完成，如果 future 当前无法取得进展，则允许其他任务运行。
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!");
}

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "周杰伦".to_string(),
        name: String::from("《菊花台》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "菊花残，满地伤~ ~"
    );
}

async fn learn_and_sing() {
    // 此为async-std 库中的异步函数
    sleep(Duration::from_millis(1000)).await;
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;

    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}
