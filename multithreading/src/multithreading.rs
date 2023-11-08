use std::cell::{Cell, RefCell};
use std::sync::{Arc, Barrier, Condvar, Mutex, Once};
use std::thread;
use std::thread::LocalKey;

use thread_local::ThreadLocal;

pub(crate) fn run() {
    sync_barrier();
    thread_local_macro();
    thread_local_crate();
    condition_variables();
    sync_once();
}

fn sync_once() {
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}

fn condition_variables() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap(); // wait 方法挂起等待子线程的通知，并释放了锁 started
    }

    println!("started changed");
}

fn thread_local_crate() {
    let tls = Arc::new(ThreadLocal::new());

    // 创建多个线程
    for _ in 0..5 {
        let tls2 = tls.clone();
        thread::spawn(move || {
            // 将计数器加1
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        })
        .join()
        .unwrap();
    }

    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| x + y.get());

    // 和为5
    assert_eq!(total, 5);
}

fn thread_local_macro() {
    // 线程局部变量 FOO
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    println!("{:p}", &FOO);
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
        println!("{:p}", f);
    });

    // 每个线程开始时都会拿到线程局部变量的FOO的初始值
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
            println!("{:p}", f);
        });
    });

    // 等待线程完成
    t.join().unwrap();

    // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
        println!("{:p}", f);
    });

    Foo::FOO.with(|x| println!("{:?}", x));

    Bar::constructor().foo.with(|x| println!("{:?}", x));
}

struct Foo;

impl Foo {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }
}

thread_local! {
    static FOO: RefCell<usize> = RefCell::new(0);
}
struct Bar {
    foo: &'static LocalKey<RefCell<usize>>,
}

impl Bar {
    fn constructor() -> Self {
        Self { foo: &FOO }
    }
}

fn sync_barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone(); // Arc::clone(&barrier)
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
