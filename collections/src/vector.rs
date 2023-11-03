pub(crate) fn run() {
    trait_object_array();
    num_vec_sort();
    struct_vec_sort();
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn struct_vec_sort() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];
    // 定义一个按照年龄倒序排序的对比函数
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
    println!("{:?}", people);

    people.sort_unstable();
    println!("{:?}", people);
}

fn num_vec_sort() {
    // sort sort_by 相等元素不进行重新排序
    // sort_unstable sort_unstable_by 相等元素不保证顺序
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}

trait IpAddr {
    fn display(&self);
}

struct V4(String);

impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String);

impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn trait_object_array() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
