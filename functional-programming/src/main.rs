fn main() {
    closure_fn_once();
    closure_fn_mut();
}

fn closure_fn_mut() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);
    // rust analyzer:update_string: impl FnMut(&str)

    exec(update_string);

    println!("{:?}", s);

    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }
}

fn closure_fn_once() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}
