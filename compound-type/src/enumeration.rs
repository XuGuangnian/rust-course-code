pub(crate) fn run() {
    option_enum();
}

fn option_enum() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // None值需要手动声明类型

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
}
