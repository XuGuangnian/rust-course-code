pub fn run() {
    or_and_xor(); // 分别对应 逻辑或|| 逻辑与&& 逻辑异或^
    or_else();
    and_then();
    filter();
    map();
    map_err();
}

fn map_err() {
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2
}

fn map() {
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);

    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");

    let fn_character_count = |s: &str| s.chars().count();

    assert_eq!(s1.map(fn_character_count), s2); // Some1 map = Some2
    assert_eq!(n1.map(fn_character_count), n2); // None1 map = None2

    assert_eq!(o1.map(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map(fn_character_count), e2); // Err1 map = Err2
}

fn filter() {
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;

    let fn_is_even = |x: &i8| x % 2 == 0;

    assert_eq!(s1.filter(fn_is_even), n); // Some(3) -> 3 is not even -> None
    assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
    assert_eq!(n.filter(fn_is_even), n); // None -> no value -> None
}

fn or_else() {
    // or_else with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2");

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1); // Some1 or_else Some2 = Some1
    assert_eq!(s1.or_else(fn_none), s1); // Some or_else None = Some
    assert_eq!(n.or_else(fn_some), s2); // None or_else Some = Some
    assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

    // or_else with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2");

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
    assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
    assert_eq!(e1.or_else(fn_ok), o2); // Err or_else Ok = Ok
    assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2
}

fn and_then() {
    // and_then with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = |_| Some("some2");

    let n: Option<&str> = None;
    let fn_none = |_| None;

    assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
    assert_eq!(s1.and_then(fn_none), n); // Some and_then None = None
    assert_eq!(n.and_then(fn_some), n); // None and_then Some = None
    assert_eq!(n.and_then(fn_none), n); // None1 and_then None2 = None1

    // and_then with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2");

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.and_then(fn_ok), o2); // Ok1 and_then Ok2 = Ok2
    assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
    assert_eq!(e1.and_then(fn_ok), e1); // Err and_then Ok = Err
    assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
}

fn or_and_xor() {
    let some1 = Some("some1");
    let some2 = Some("some2");
    let none: Option<&str> = None;

    let ok1: Result<&str, &str> = Ok("ok1");
    let ok2: Result<&str, &str> = Ok("ok2");
    let err1: Result<&str, &str> = Err("error1");
    let err2: Result<&str, &str> = Err("error2");

    assert_eq!(some1.or(some2), some1); // Some1 or Some2 = Some1
    assert_eq!(some1.or(none), some1); // Some or None = Some
    assert_eq!(none.or(some1), some1); // None or Some = Some
    assert_eq!(none.or(none), none); // None1 or None2 = None2

    assert_eq!(ok1.or(ok2), ok1); // Ok1 or Ok2 = Ok1
    assert_eq!(ok1.or(err1), ok1); // Ok or Err = Ok
    assert_eq!(err1.or(ok1), ok1); // Err or Ok = Ok
    assert_eq!(err1.or(err2), err2); // Err1 or Err2 = Err2

    assert_eq!(some1.and(some2), some2); // Some1 and Some2 = Some2
    assert_eq!(some1.and(none), none); // Some and None = None
    assert_eq!(none.and(some1), none); // None and Some = None
    assert_eq!(none.and(none), none); // None1 and None2 = None1

    assert_eq!(ok1.and(ok2), ok2); // Ok1 and Ok2 = Ok2
    assert_eq!(ok1.and(err1), err1); // Ok and Err = Err
    assert_eq!(err1.and(ok1), err1); // Err and Ok = Err
    assert_eq!(err1.and(err2), err1); // Err1 and Err2 = Err1

    // xor 异或只应用于Option类型
    assert_eq!(some1.xor(none), some1);
    assert_eq!(none.xor(some2), some2);
    assert_eq!(some1.xor(some2), none);
    assert_eq!(none.xor(none), none);
}
