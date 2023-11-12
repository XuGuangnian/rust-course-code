#![allow(dead_code)]

mod custom_error;
mod option_result;

fn main() {
    option_result::run();
    custom_error::run();
}
