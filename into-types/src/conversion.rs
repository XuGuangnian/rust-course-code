use std::sync::Arc;

pub(crate) fn run() {
    // as operator 转换不具有传递性
    conversion_as();
    conversion_try_into();
    conversion_common();
    transmutes();
}

fn transmutes() {
    // transmutes raw pointer to function pointer
    transmutes_raw_ptr_to_func_ptr();
    transmutes_lifetime();
}

#[allow(dead_code)]
fn transmutes_lifetime() {
    struct R<'a>(&'a i32);

    // 将 'b 生命周期延长至 'static 生命周期
    unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
        std::mem::transmute::<R<'b>, R<'static>>(r)
    }

    // 将 'static 生命周期缩短至 'c 生命周期
    unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
        std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
    }
}

fn transmutes_raw_ptr_to_func_ptr() {
    fn foo() -> i32 {
        0
    }

    let pointer = foo as *const ();
    let function = unsafe {
        // 将裸指针转换为函数指针
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(noop_method_call)]
fn conversion_common() {
    // 在匹配特征时，不会做任何强制转换(除了方法)
    // `.` operator: value.foo() 方法调用步骤
    // 1. 值方法调用，T::foo(value)
    // 2. 引用方法调用，<&T>::foo(value) 和 <&mut T>::foo(value)
    // 3. 如果T: Deref<Target = U>，则解引用为U，U::foo(value)， 也可以自定义 Deref trait 的实现
    // 4. 如果T不能被解引用，且 T 是一个定长类型，那么编译器也会尝试将 T 从定长类型转为不定长类型，例如将 [i32; 2] 转为 [i32]
    fn do_stuff<T: Clone>(value: &T) {
        let cloned = value.clone(); // T
    }

    fn do_stuff_2<T>(value: &T) {
        let cloned = value.clone(); // &T
    }

    #[derive(Clone)]
    struct Container<T>(Arc<T>);

    fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
        let foo_cloned = foo.clone(); // Container<i32>
        let bar_cloned = bar.clone(); // &Container<T>
    }
}

fn conversion_try_into() {
    let a: u8 = 10;
    let b: u16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

fn conversion_as() {
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8; // 将字符'a'转换为整数，97

    println!("{},{},{}", a, b, c);

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
}
