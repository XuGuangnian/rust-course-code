#![allow(dead_code, unused_variables)]

mod declarative_macros;
mod procedural_macros;

fn main() {
    declarative_macros::run();
    // 学习过程宏的使用：https://github.com/dtolnay/proc-macro-workshop
    procedural_macros::run();
}
