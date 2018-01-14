/*
 * rust句尾有;
 * 调用fn! 调用的是rust的宏而非普通函数
 * 预编译静态类型语言,需要先编译生成二进制的可执行文件
 */
fn main() {
    println!("Hello {}", "World");
}
