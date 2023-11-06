use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

// num_enum = "0.7.1" https://crates.io/crates/num_enum
pub(crate) fn run() {
    // num 转换为 enum
    use_crate_num_traits(); // num-traits = "0.2.17" num-derive = "0.4.1"
    impl_try_from_trait();
    impl_try_from_with_macro();
    use_transmute();
}

#[allow(dead_code)]
fn use_transmute() {
    #[repr(i32)]
    enum MyEnum {
        A = 1,
        B,
        C,
    }

    let x = MyEnum::C;
    let y = x as i32;
    let z: MyEnum = unsafe { std::mem::transmute(y) };

    // match the enum that came from an int
    match z {
        MyEnum::A => {
            println!("Found A");
        }
        MyEnum::B => {
            println!("Found B");
        }
        MyEnum::C => {
            println!("Found C");
        }
    }
}

fn impl_try_from_with_macro() {
    #[macro_export]
    macro_rules! back_to_enum {
    // (语法定义) => {模版文本}
    // 使用“$”号声明变量，$变量名:类型标记， “*”号表示会出现0-n个，“?”号表示会出现0-1个
    // meta标记是宏相关的，比如: #[allow(unused)]...
    // $name 枚举名、$vname 枚举属性名、$val 枚举属性值
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
            $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
        }) => {
            $(#[$meta])*
            $vis enum $name {
                $($(#[$vmeta])* $vname $(= $val)?,)*
            }

            impl std::convert::TryFrom<i32> for $name {
                type Error = ();

                fn try_from(v: i32) -> Result<Self, Self::Error> {
                    match v {
                        $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                        _ => Err(()),
                    }
                }
            }
        }
    }

    back_to_enum! {
        enum MyEnum {
            A = 1,
            B,
            C,
        }
    }
}

fn impl_try_from_trait() {
    enum MyEnum {
        A = 1,
        B,
        C,
    }
    impl TryFrom<i32> for MyEnum {
        type Error = ();

        fn try_from(v: i32) -> Result<Self, Self::Error> {
            match v {
                x if x == MyEnum::A as i32 => Ok(MyEnum::A),
                x if x == MyEnum::B as i32 => Ok(MyEnum::B),
                x if x == MyEnum::C as i32 => Ok(MyEnum::C),
                _ => Err(()),
            }
        }
    }

    let x = MyEnum::C as i32;

    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    }
}

fn use_crate_num_traits() {
    #[derive(FromPrimitive)]
    enum MyEnum {
        A = 1,
        B,
        C,
    }

    // 将枚举转换成整数，顺利通过
    let x = MyEnum::C as i32;
    println!("{}", x);

    // 将整数转换为枚举
    match FromPrimitive::from_i32(x) {
        Some(MyEnum::A) => println!("Got A"),
        Some(MyEnum::B) => println!("Got B"),
        Some(MyEnum::C) => println!("Got C"),
        None => println!("Couldn't convert {}", x),
    }
}
