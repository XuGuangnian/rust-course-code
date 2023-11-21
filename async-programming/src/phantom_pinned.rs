use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;

pub fn run() {
    phantom_pinned_test();
    phantom_pinned_shadow_test();
}

fn phantom_pinned_shadow_test() {
    let mut test1 = Test::new("test1");
    // 如果不用同名变量shadow，那么test1在pin的声明周期之外（drop之后）还是可移动
    let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1_pin.as_mut()); // Gets a pinned mutable reference from this pinned pointer.

    drop(test1_pin); // 将此行移动到最后一行，会导致编译错误，因为在pin的声明周期内不允许改变值
    println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

    let mut test2 = Test::new("test2");
    mem::swap(&mut test1, &mut test2);
    println!("... and now it points nowhere: {:?}", test1.b);
}

fn phantom_pinned_test() {
    // 此时的`test1`可以被安全的移动
    let mut test1 = Test::new("test1");
    // 新的`test1`由于使用了`Pin`，因此无法再被移动，这里的声明会将之前的`test1`遮蔽掉(shadow)
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    // `PhantomPinned` cannot be unpinned
    // mem::swap(test1.get_mut(), test2.get_mut());
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        // self.b = self_ptr; //对于Pin中的值不能直接修改，需要先获取Pin中数据的可变引用
        let this = unsafe {
            self.get_unchecked_mut() // Gets a mutable reference to the data inside of this Pin.
        };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a // Gets a shared reference out of a pin.
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}
