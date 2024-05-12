// traits1.rs
//
//是时候实现一些特性了！您的任务是为“String”类型实现“AppendBar”特征。
// 特征 AppendBar 只有一个函数，它将“Bar”附加到实现此特征的任何对象。
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) -> Self {
        format!("{self}Bar")
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}