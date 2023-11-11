#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;

use lazy_static::lazy_static;

fn main() {
    const_variable();
    static_variable();
    atomic_variable();
    // lazy_static 运行期初始化静态变量
    lazy_static_variable();
    box_leak();
    once_variable(); // OnceCell OnceLock
}

fn once_variable() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = Logger::global();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = Logger::global();
    logger.log("some message".to_string());

    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

// Rust 1.70版本以上
// static LOG: OnceCell<Logger> = OnceCell::new();
static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn global() -> &'static Logger {
        // 获取或初始化 Logger
        LOGGER.get_or_init(|| {
            println!("Logger is being created..."); // 初始化打印
            Logger
        })
    }

    fn log(&self, message: String) {
        println!("{}", message)
    }
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c))
}

fn box_leak() {
    // 不允许局部生命周期的变量赋值给全局声明周期的 CONFIG
    // unsafe {
    //     CONFIG = Some(&mut Config {
    //         a: "A".to_string(),
    //         b: "B".to_string(),
    //     });
    //
    //     println!("{:?}", CONFIG)
    // }

    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        CONFIG = Some(Box::leak(c));
        println!("{:?}", CONFIG);

        CONFIG = init();
        println!("{:?}", CONFIG);
    }
}

// let ref r = s 等价于 let r = &s
lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
}

fn lazy_static_variable() {
    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());

    let mut v = NAMES.lock().unwrap();
    v.push_str(", Myth");
    println!("{}", v);
}

fn atomic_variable() {
    struct Factory {
        factory_id: usize,
    }

    impl Factory {
        fn new() -> Self {
            Self {
                factory_id: generate_id(),
            }
        }
    }

    static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    const MAX_ID: usize = usize::MAX / 2;

    fn generate_id() -> usize {
        // 检查两次溢出，否则直接加一可能导致溢出
        let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        if current_val > MAX_ID {
            panic!("Factory ids overflowed");
        }
        GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        if next_id > MAX_ID {
            panic!("Factory ids overflowed");
        }
        next_id
    }
}

static mut REQUEST_RECV: usize = 0;

// 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
// 存储在静态变量中的值必须要实现 Sync trait
fn static_variable() {
    // 对静态变量的访问和修改需要 unsafe
    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    }
}

const MAX_ID: usize = usize::MAX / 2;

fn const_variable() {
    println!("用户ID允许的最大值是{}", MAX_ID);
}
