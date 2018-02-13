fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let h = "hello";
    let h1 = "Здравствуйте";
    // let ch = &h[0];     // 编译错误

    // for c in h1.chars() {
    //     println!("{}", c);
    // }

    let mut split = "some string 123 ffd".split("123");

    println!("{}", &h1[0..2]);
}
