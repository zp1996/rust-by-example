fn main() {
    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);

    let first: &i32 = &v[0];
    println!("{}", first);

    // 返回一个Option<T>类型
    let value = v.get(100);
    match value {
        None => println!("index error"),
        Some(i) => println!("{}", i)
    }

    for item in v.iter() {
        println!("{}", item);
    }
}
