use std::sync::{Arc, Condvar, Mutex, MutexGuard, RwLock};
use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;
use tokio::sync::Semaphore;

pub(crate) fn run() {
    mutex_in_single_thread();
    arc_mutex();
    // dead_lock();
    rwlock();
    // lock crate: parking_lot spin
    condition_variables();
    semaphore();
}

fn mutex_in_single_thread() {
    // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
    let m = Mutex::new(5);

    {
        // 获取锁，然后deref为`m`的引用
        // lock返回的是Result
        let mut num = m.lock().unwrap();
        // T = MutexGuard
        // fn deref(&self) -> &T {
        //     unsafe { &*self.lock.data.get() }
        // }
        *num = 6;
        // 锁自动被drop
    }

    println!("m = {:?}", m);
}

#[tokio::main]
async fn semaphore() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            // 在这里执行任务...
            println!("semaphore");
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}

fn condition_variables() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let handle = thread::spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
                // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
                lock = ccond.wait(lock).unwrap();
            }

            *lock = false;

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    handle.join().unwrap();
    println!("{:?}", flag);
}

fn rwlock() {
    let lock = RwLock::new(5);

    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被drop

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
    } // 写锁在此处被drop
}

lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

#[allow(unused)]
fn dead_lock() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    thread::sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX2.try_lock().unwrap();
                    // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                    let _guard = MUTEX1.try_lock().unwrap();
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}

fn arc_mutex() {
    // 通过`Rc`实现`Mutex`的多所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有子线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出最终的计数结果
    println!("Result: {}", *counter.lock().unwrap());
}
