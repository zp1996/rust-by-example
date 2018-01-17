#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

// 定义结构体方法
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.length;
    }
    fn cmp(&self, other: &Rectangle) -> bool {
        return self.length > other.length && self.width > other.width;
    }
}


fn area(length: u32, width: u32) -> u32 {
    return length * width;
}

fn area_tuple(info: (u32, u32)) -> u32 {
    return info.0 * info.1;
}

fn area_struct(rec: &Rectangle) -> u32 {
    return rec.width * rec.length;
}

fn main() {
    let width = 50;
    let length = 30;

    let rec = Rectangle { width, length };
    let rec1 = Rectangle { width: 100, height: 60 };

    println!("{}", area(width, length));
    println!("{}", area_tuple((width,length)));
    println!("{}", area_struct(&rec));
    println!("{:#?}", rec);
    println!("{}", rec.area());
    println!("{}", rec.cmp(&rec1));
}
