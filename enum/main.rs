// #[derive(Debug)]
// enum IpAddrKind {
//     V4(String),
//     V6(String)
// }

// struct IPAddr {
//     kind: IpAddrKind,
//     address: String
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }
//
// impl Message {
//     fn call(&self) {
//
//     }
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn get_coin_value(coin: Coin) -> i32 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{}", state);
            return 25;
        }
    }
}

fn match_option(x: Option<i32>) -> bool {
    match x {
        None => false,
        Some(i) => {
            println!("{}", i);
            return true;
        }
    }
}

fn main() {
    // let home = IPAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    //
    // let loopback = IPAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    // let loopback = IpAddrKind::V6(String::from("::1"));

    // 语法糖 if let for match
    let x = true;
    if let true = x {
        println!("demo");
    }

    println!("{}", get_coin_value(Coin::Penny));
    println!("{}",
        get_coin_value(
            Coin::Quarter(String::from("zp1996"))
        )
    );

    let five = Some(5);
    println!("{}", match_option(five));
    println!("{}", match_option(None));
    // println!("{:?}", home);
}
