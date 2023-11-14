#![allow(dead_code)]

mod async_await;
mod async_intro;
mod futures;
mod pin_unpin;

fn main() {
    async_intro::run();
    futures::run();
    pin_unpin::run();
    async_await::run();
}
