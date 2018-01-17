fn main() {
    // 块级作用域
    {
        let s = 20;
        println!("{}", s);
    }
    let s = 50;
    println!("{}", s);

    // String
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    // 移动
    // let s1 = String::from("hello");
    // let s2 = s1;
    // 不能使用s1,此时s1已经无效,栈上数据已经被清理(数据移动到了s2)
    // println!("{}", s1);

    // 克隆
    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    takes_owership(&mut s1);    // s1被移入了函数中,所以此时s1是一个无效变量

    println!("{}, {}", s1, s2);

    // 返回值可以转移作用域
    let s3 = gives_ownership();
    println!("{}", s3);

    let (sr, len) = return_tuples(s3);
    println!("{}, {}", sr, len);

    println!("{}", firsr_word_length(&s1));

    s1.clear();         // reset to ''
    println!("{}", s1);
}
fn takes_owership(s: &mut String) {
    s.push_str("zp1996");
    println!("{}", s);
} // 离开作用域时,堆上内存被释放

fn gives_ownership() -> String {
    let s = String::from("hello");
    return s;
}

fn return_tuples(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}

// rust不允许垂直指针的出现
// s 在离开作用域之后被释放了, &s的指向变得不确定
// fn no_dangle() -> &String {
//     let s = "Hello World";
//     return &s;
// }

// 输出第一个单词的长度
fn firsr_word_length(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}
