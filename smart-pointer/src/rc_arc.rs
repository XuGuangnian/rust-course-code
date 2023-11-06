use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

pub(crate) fn run() {
    rc_example();
    arc_example();
    refcell();
    rc_refcell();
    cell();
}

fn refcell() {
    // 创建一个新的银行
    let bank = Bank::new();
    // 存款100元
    bank.deposit(100);
    // 取款50元，余额应该是50元
    assert!(bank.withdraw(50));
    assert_eq!(*bank.balance.borrow(), 50);
}

// 定义银行结构体
struct Bank {
    // 使用RefCell存储余额，因为余额是内部可变的
    balance: RefCell<i32>,
}

impl Bank {
    // 创建一个新的银行对象
    fn new() -> Bank {
        // 初始化余额为0
        Bank {
            balance: RefCell::new(0),
        }
    }

    // 存款
    fn deposit(&self, amount: i32) {
        // 获取内部可变引用
        let mut balance = self.balance.borrow_mut();
        // 修改余额
        *balance += amount;
    }

    // 取款
    fn withdraw(&self, amount: i32) -> bool {
        // 获取内部可变引用
        let mut balance = self.balance.borrow_mut();
        // 如果余额充足，则修改余额并返回true
        if *balance >= amount {
            *balance -= amount;
            true
            // 否则返回false
        } else {
            false
        }
    }
}

fn cell() {
    let mut vec = vec![10, 20, 30];
    retain_even(&mut vec);
    fn is_even(i: i32) -> bool {
        i % 2 == 0
    }

    fn retain_even(nums: &mut Vec<i32>) {
        let slice = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
        let mut i = 0;
        for num in slice.iter().filter(|num| is_even(num.get())) {
            slice[i].set(num.get());
            i += 1;
        }
        nums.truncate(i);
    }
}

fn rc_refcell() {
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", oh yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

fn arc_example() {
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || println!("{}", s));
        handle.join().unwrap();
    }
}

fn rc_example() {
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&b);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
