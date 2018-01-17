// rust的结构体的语法与js的对象类似

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(u8, u8, u8);

fn build_user(name: String, email: String) -> User {
    return User {
        name,
        email,
        active: true,
        sign_in_count: 1
    };
}
fn main() {
    let black = Color(0, 0, 0);

    let user = User {
        name: String::from("zp1996"),
        email: String::from("zp1996@qq.com"),
        active: true,
        sign_in_count: 2
    };
    let new_person = build_user(
        String::from("zpy"),
        String::from("zpy@qq.com")
    );
    let user2 = User {
        name: String::from("demo"),
        email: String::from("demo@qq.com"),
        ..user
    };

    println!("{}", user.name);
    println!("{}", user2.name);
    println!("{}", new_person.name);
}
