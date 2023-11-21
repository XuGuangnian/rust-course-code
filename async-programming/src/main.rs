#![allow(dead_code, unused_variables)]

mod async_await;
mod async_intro;
mod futures;
mod phantom_pinned;
mod phantom_pinned_heap;
mod pin_unpin;
mod stream_trait;

fn main() {
    async_intro::run();
    futures::run();
    pin_unpin::run();
    phantom_pinned::run();
    phantom_pinned_heap::run();
    async_await::run();
    stream_trait::run()
}
