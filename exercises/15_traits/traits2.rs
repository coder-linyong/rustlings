// traits2.rs
//
// 您的任务是为字符串向量实现“AppendBar”特征。
// 要实现此特征，请考虑一下将“附加“Bar””到字符串向量的含义。
//
// 这次没有样板代码，你可以做到！
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(&mut self) -> Self;
}

// TODO:为字符串向量实现特征“AppendBar”。
impl AppendBar for Vec<String> {
    fn append_bar(&mut self) -> Self {
        self.push("Bar".to_string());
        self.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

fn main() {}