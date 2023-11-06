pub(crate) fn run() {
    drop_example();
    mem_drop();
    // 互斥的 Copy 和 Drop
    copy_drop();
}

fn copy_drop() {
    // #[derive(Copy)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!")
        }
    }
}

fn mem_drop() {
    let foo = Foo;
    drop(foo);
    println!("Running!");
}

fn drop_example() {
    let _x = HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    };
    let _foo = Foo;
    println!("Running!");
}

struct HasDrop1;

struct HasDrop2;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}

#[allow(dead_code)]
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}
