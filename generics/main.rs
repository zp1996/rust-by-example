use std::cmp::PartialOrd;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

fn largest<T>(list: &[T]) -> T where T: PartialOrd + Copy {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];

    println!("largest number is: {:?}", largest(&numbers));
    println!("largest char is: {:?}", largest(&chars));

    let p1 = Point { x: 5, y: 5 };
    let p2 = Point { x: 1.0, y: 1.0 };

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
}
