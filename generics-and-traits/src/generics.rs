pub(crate) fn run() {
    // 为具体的泛型类型实现方法
    method_for_specific_generic_type();
    // const 泛型：针对值的泛型，1.51 引入重要特性
    const_generics();
    // const 泛型表达式（nightly）,目前stable最新版本为1.73.0
    generic_const_expr();
    // const fn
    // todo!()
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn method_for_specific_generic_type() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p = Point {
        x: 5.0_f32,
        y: 10.4_f32,
    };
    println!("{}", p.distance_from_origin());
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn const_generics() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

fn something<T>(_val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
{
    println!("something");
}

fn generic_const_expr() {
    something([0u8; 0]); // ok
    something([0u8; 512]); // ok
                           // something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制
}

// ---

pub enum Assert<const CHECK: bool> {
    //
}

pub trait IsTrue {
    //
}

impl IsTrue for Assert<true> {
    //
}
