use std::future::Future;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::thread;
use std::time::Duration;

use futures::future::BoxFuture;
use futures::task::{waker_ref, ArcWake};
use futures::FutureExt;

use timer_future::TimerFuture;

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
struct Task {
    /// 进行中的Future，在未来的某个时间点会被完成
    ///
    /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
    /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
    /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
    ///
    /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
    /// pub type BoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>>;
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以将该任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>, // 自引用
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数(任务队列的最大长度)
    // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        println!("发送任务task");
        self.task_sender.send(task).expect("任务队列已满");
    }
}

impl ArcWake for Task {
    // fn wake(self: Arc<Self>) {
    //     Self::wake_by_ref(&self)
    // }

    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
        println!("wake by ref");
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            println!("获取任务task");
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`, Creates a reference to a Waker from a reference to Arc<impl ArcWake>.
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // BoxFuture<T>是Pin<Box<dyn Future<Output = T> + Send + 'static>>的类型别名
                // 通过调用as_mut方法，可以将上面的类型转换成Pin<&mut dyn Future + Send + 'static>，此时Pin里面的就是传过来
                // 的future即那个async块，执行这个poll方法，那就会执行块中的代码，在遇到.await后这个外层future（async块）会阻塞，
                // 因为执行到第二句，里面的.await会对子future即TimerFuture也进行poll，发现不能完成(返回Pending)，则会阻塞
                // 当线程睡醒之后，改变共享状态，并调用wake方法，那就会调用wake_by_ref方法将任务再次放回通道，因为run方法一直在执行着
                // 当再次进入到while循环，执行到这里，即再次对async块进行poll，会从当时阻塞的地方开始，因为不会再次打印println!("howdy!")，
                // 那么.await再次对子future即TimeFuture也进行poll，发现其状态已改变，因此子future完成，开始往下执行，那么执行println!("done!")，
                // 到此外部future（async块）也完成，整个流程结束
                thread::sleep(Duration::from_secs(2));
                println!("before executor poll task-future, {:?}", context);
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll，因为共享的同一任务，这边不放回 ，Task将不能将该任务自身放回到任务通道中，缺少future
                    // 因为前面使用了take方法 Task里的Option已经变为None
                    *future_slot = Some(future);
                    println!("放回任务中，等待下次被poll");
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("future: howdy!");
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await; // 遇到.await会暂停当前函数执行，直到Future执行完成，
                                                     // 当返回Ready(())，代表Future执行完成，
                                                     // 而当completed = true(定时完成)时，才会返回Ready(())
                                                     // 如果不加.await，因为TimerFuture本身实现了Future trait，则运行不会阻塞，直接执行下一行
        println!("future: done!");
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}
