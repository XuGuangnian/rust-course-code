//! crate 或 module 行注释放在文件开头
//!
/*!
crate 或 module 块注释放在文件开头
 */
/// [`add_one`] 文档行注释 链接调转到add_one
/// # Markdown语法
/// 文档测试
/// # Examples
///
/// ```
/// use comments::add_one;
/// let arg = 5;
/// # let arg = 6; // 文档中隐藏，但仍会运行
/// let answer = add_one(arg);
///
/// assert_eq!(7, answer);
/// ```
/// ## 生成 html doc
/// `cargo doc --open`
/**
文档快注释
 */
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// 跳转到结构体  [`Foo`](struct@Foo)
pub struct Bar;

/// 跳转到同名函数 [`Foo`](fn@Foo)
pub struct Foo {}

/// 跳转到同名宏 [`foo!`]
#[allow(non_snake_case)]
pub fn Foo() {}

#[macro_export]
macro_rules! foo {
    () => {};
}

/// 文档搜索别名
#[doc(alias = "x")]
#[doc(alias = "big")]
pub struct BigX;

#[doc(alias("y", "big"))]
pub struct BigY;
