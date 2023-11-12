// https://doc.rust-lang.org/reference/items/unions.html
pub fn run() {
    union_init();
}

fn union_init() {
    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 };
    println!("f: {}", f);
}

// 联合体的关键特性是联合体的所有字段共享同一段存储。因此，对联合体的一个字段的写操作会覆盖其他字段，
// 而联合体的尺寸由其尺寸最大的字段的尺寸所决定。
#[repr(C)] // 用另一种指定的表示法来替换 rust 默认的
union MyUnion {
    f1: u32,
    f2: f32,
}
