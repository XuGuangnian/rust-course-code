fn main() {
    // 闭包本身的使用方式
    closure_fn_once(); // 闭包移动(未实现Copy)
    closure_fn_mut(); // 可变引用
    closure_fn(); // 不可变引用
}

fn closure_fn() {
    let s = String::new();

    let update_string = |str| println!("{},{}", s, str);

    fn_ref(update_string);

    println!("{:?}", s);
}

// 需要不可变引用的闭包类型
fn fn_ref<'a, F: Fn(&'a str)>(f: F) {
    f("hello")
}

// 一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量
fn closure_fn_mut() {
    // 闭包获取不可变引用
    closure_ref();
    // 闭包获取可变引用
    closure_mut_ref();
    // 闭包捕获所有权
    closure_ownership();
}

fn closure_ref() {
    let s = String::new();

    let update_string = |_| println!("{}", s); // 闭包获取 s 的不可变引用
                                               // rust analyzer:update_string: impl FnMut(&str)

    fn_mut(update_string); // exec 获取 update_string 的所有权

    println!("{:?}", s); // s 还可用
}

fn closure_ownership() {
    let mut s = String::new();

    let update_string = move |str| s.push_str(str); // 闭包获取 s 的所有权
                                                    // rust analyzer:update_string: impl FnMut(&str)

    fn_mut(update_string); // exec 获取 update_string 的所有权

    // println!("{:?}", s); // s 不可用
}

fn closure_mut_ref() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str); // 闭包获取 s 的可变引用
                                               // rust analyzer:update_string: impl FnMut(&str)

    fn_mut(update_string); // exec 获取 update_string 的所有权

    println!("{:?}", s); // s 还可用
}

fn fn_mut<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

fn closure_fn_once() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
    println!("{:?}", x);
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}
