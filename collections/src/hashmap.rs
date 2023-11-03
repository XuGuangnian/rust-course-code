use std::collections::HashMap;
use std::hash::BuildHasherDefault;

// 引入第三方的哈希函数
use twox_hash::XxHash64;

pub(crate) fn run() {
    hashmap_from_iter_collect();
    hashmap_get_value();
    hash_func_in_hashmap();
}

fn hash_func_in_hashmap() {
    // 指定HashMap使用第三方的哈希函数XxHash64
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}

fn hashmap_get_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    assert_eq!(score, Some(&10));

    let score = scores.get(&team_name).copied().unwrap_or(0);
    assert_eq!(score, 10)
}

fn hashmap_from_iter_collect() {
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

    println!("{:?}", teams_map)
}
