// lifetimes1.rs
//
// Rust 编译器需要知道如何检查提供的引用是否有效，
// 这样它就可以在使用引用之前让程序员知道它是否有超出范围的风险。
//请记住，参考文献是借用的，并不拥有自己的数据。
// 如果它们的主人超出了范围怎么办？
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}