use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    send_for_raw_pointer();
    sync_for_raw_pointer();
}

fn send_for_raw_pointer() {
    let p = MyBox(5 as *const u8);
    let t = thread::spawn(move || {
        println!("{:?}", p);
    });

    t.join().unwrap();
}

#[derive(Debug)]
struct MyBox(*const u8);

unsafe impl Send for MyBox {}

fn sync_for_raw_pointer() {
    let b = &MyBox(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let handle = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });
    handle.join().unwrap();
}

unsafe impl Sync for MyBox {}
