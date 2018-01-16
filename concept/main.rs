fn main() {
    // // number
    // let num = 1_000;
    // println!("{}", num);
    //
    // // bool
    // let flag: bool = false;
    // println!("{}", flag);
    //
    // // char
    // let c = 'a';
    // println!("{}", c);
    //
    // // tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // // destructuring
    // let (x, y, z) = tup;
    // println!("{}, {}, {}", x, y, z);
    //
    // // array
    // let arr = [1, 2, 3];
    //
    // for x in arr.iter() {
    //     println!("{}", x);
    // }
    // // index is over, rust will break out
    // // println!("{}", arr[arr.len()]);
    //
    // if num < 5 {
    //     println!("{}", true);
    // } else {
    //     println!("{}", false);
    // }
    //
    // println!("{}", add(10, 20));
    //
    // // equal type
    // let mut n = if flag {
    //     10
    // } else {
    //     20
    // };
    // println!("{}", n);

    while_demo();
    loop_demo();

    // 遍历 1 <= number < 4
    for number in 1..4 {
        println!("number is: {}", number);
    }
}

fn loop_demo() {
    let mut i = 0;
    loop {
        if i == 5 {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}

fn while_demo() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }
}
//
// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }
