pub(crate) fn run() {
    // for item in collection	    for item in IntoIterator::into_iter(collection)	转移所有权
    // for item in &collection	    for item in collection.iter()	                不可变借用
    // for item in &mut collection	for item in collection.iter_mut()	            可变借用
    array_for_index();
    for_comparison();
}

fn array_for_index() {
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    // enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}

fn for_comparison() {
    // 第一种
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        print!("{} ", item);
    }
    println!();

    // 第二种：更快（无索引越界检查），更安全（迭代，所有权）
    for item in collection.iter() {
        print!("{} ", item);
    }
    println!();
}
