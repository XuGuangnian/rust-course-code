pub(crate) fn run() {
    // 常见的 DST 类型有: str、[T]、dyn Trait，它们都无法单独被使用，必须要通过引用或者 Box 来间接使用 。
    // 例如：&dyn Trait 或者 Box<dyn Trait> (还有 Rc<dyn Trait>)
    // 在泛型中使用DST: <T: ?Ssize>
    let s1: Box<str> = "Hello there!".into();
    println!("{}", s1);
}
