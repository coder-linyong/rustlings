// quiz2.rs
//
// 这是以下部分的测验：
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// 让我们以函数的形式构建一台小机器。
// 作为输入，我们将给出字符串和命令的列表。
// 这些命令确定将应用于字符串的操作。
// 它可以是：
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// 其确切形式为：
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// 这次没有提示！

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

impl Command {
    fn execute(self: &Self, string: &String) -> String {
        match self {
            Command::Uppercase => string.to_uppercase(),
            Command::Trim => (&string).trim().to_string(),
            Command::Append(n) => {
                let mut string = string.clone();
                string.push_str(&"bar".repeat(*n));
                string
            }
        }
    }
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            output.push(command.execute(string));
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    // TODO: 我们需要导入什么才能将“变压器”纳入范围？
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}

fn main() {}