// 寻找第一个单词
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn get_slice(arr: &[i32]) -> &[i32] {
    return &arr[0..2];
}
// 字符串切片
fn main() {
    let s = String::from("hello");
    let arr = [1, 2, 3, 4, 5];

    println!("{}", get_slice(&arr).len());
    println!("{}", first_word(&s));
}
