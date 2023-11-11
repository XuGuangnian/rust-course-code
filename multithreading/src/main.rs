#![allow(dead_code, unused_variables)]

mod atomic;
mod channel;
mod multithreading;
mod send_sync;
mod shared_memory;

fn main() {
    multithreading::run();
    channel::run();
    shared_memory::run();
    atomic::run();
    send_sync::run();
}
