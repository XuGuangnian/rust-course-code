#![allow(unused)]

use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::{Rc, Weak};

fn main() {
    weak_example();
    weak_circle_ref();
    self_ref_unsafe();
    self_ref_pin();
}

fn self_ref_pin() {
    // 下面是一个自引用数据结构体，因为 slice 字段是一个指针，指向了 data 字段
    // 我们无法使用普通引用来实现，因为违背了 Rust 的编译规则
    // 因此，这里我们使用了一个裸指针，通过 NonNull 来确保它不会为 null
    struct Unmovable {
        data: String,
        slice: NonNull<String>,
        _pin: PhantomPinned,
    }

    impl Unmovable {
        // 为了确保函数返回时数据的所有权不会被转移，我们将它放在堆上，唯一的访问方式就是通过指针
        fn new(data: String) -> Pin<Box<Self>> {
            let res = Unmovable {
                data,
                // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
                slice: NonNull::dangling(),
                _pin: PhantomPinned,
            };
            let mut boxed = Box::pin(res);

            let slice = NonNull::from(&boxed.data);
            // 这里其实安全的，因为修改一个字段不会转移整个结构体的所有权
            unsafe {
                let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).slice = slice;
            }
            boxed
        }
    }

    let unmoved = Unmovable::new("hello".to_string());
    // 只要结构体没有被转移，那指针就应该指向正确的位置，而且我们可以随意移动指针
    let mut still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

    // 因为我们的类型没有实现 `Unpin` 特征，下面这段代码将无法编译
    // let mut new_unmoved = Unmovable::new("world".to_string());
    // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
}

fn self_ref_unsafe() {
    struct SelfRef {
        value: String,

        // 该引用指向上面的value
        pointer_to_value: *mut String,
    }

    impl SelfRef {
        fn new(txt: &str) -> Self {
            SelfRef {
                value: String::from(txt),
                pointer_to_value: std::ptr::null_mut(),
            }
        }

        fn init(&mut self) {
            self.pointer_to_value = &mut self.value;
        }

        fn value(&self) -> &str {
            &self.value
        }

        fn pointer_to_value(&self) -> &String {
            assert!(
                !self.pointer_to_value.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*self.pointer_to_value }
        }
    }

    let mut t = SelfRef::new("hello");
    t.init();
    println!("{}, {:p}", t.value(), t.pointer_to_value());

    t.value.push_str(", world");
    unsafe {
        (&mut *t.pointer_to_value).push_str("!");
    }

    println!("{}, {:p}", t.value(), t.pointer_to_value());
}

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn weak_circle_ref() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn weak_example() {
    // 创建Rc，持有一个值5
    let five = Rc::new(5);

    // 通过Rc，创建一个Weak指针
    let weak_five = Rc::downgrade(&five);

    // Weak引用的资源依然存在，取到值5
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);

    // 手动释放资源`five`
    drop(five);

    // Weak引用的资源已不存在，因此返回None
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(strong_five, None);
}
