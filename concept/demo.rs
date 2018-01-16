// 斐波那契数列
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
// 温度转换
fn get_temperature(c: f64) -> f64 {
    return 1.8 * c + 32.0;
}
fn main() {
    println!("{}", fibonacci(3));
    println!("{}", get_temperature(36.5));
}
