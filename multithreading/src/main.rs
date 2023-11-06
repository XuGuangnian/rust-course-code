mod atomic;
mod channel;
mod multithreading;
mod shared_memory;

fn main() {
    multithreading::run();
    channel::run();
    shared_memory::run();
    atomic::run();
}
