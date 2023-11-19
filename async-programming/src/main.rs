#![allow(dead_code, unused_variables)]

mod async_await;
mod async_intro;
mod futures;
mod phantom_pinned;
mod pin_unpin;

fn main() {
    async_intro::run();
    futures::run();
    pin_unpin::run();
    phantom_pinned::run();
    async_await::run();
}
