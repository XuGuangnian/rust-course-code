// const 泛型表达式目前只能在nightly版本下使用
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod generics;
mod trait_object;
mod traits;

fn main() {
    generics::run();
    traits::run();
    trait_object::run();
}
