pub(crate) fn run() {
    match_expression();
    matches_macro();
    while_let();
    deconstruct_array_slice();
    at_operator();
    at_operator_new();
    at_operator_new_2();
}

fn deconstruct_array_slice() {
    // array
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;
    assert_eq!(x, 114);
    assert_eq!(y, 514);

    // array slice
    let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }

    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }

    let arr: &[u16] = &[];
    assert!(matches!(arr, [])); // 第二个参数是模式
    assert!(matches!(arr, []));
    assert!(matches!(arr, &[..]));
    assert!(matches!(arr, &[..]));
    assert!(!matches!(arr, [_x, ..])); // 注意 `!`
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn match_expression() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

fn matches_macro() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let v_foo: Vec<&MyEnum> = v.iter().filter(|x| matches!(x, MyEnum::Foo)).collect();
    println!("{:?}", v_foo);

    // matches! 第二个参数就是match模式的写法 (match guard)
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}

fn while_let() {
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn at_operator() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7, // id_variable @
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 1.56新增
fn at_operator_new() {
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    }
}

// 1.53新增
fn at_operator_new_2() {
    let x = 1;
    match x {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
