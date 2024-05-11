// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// 不应该拥有所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应该拥有所有权
fn string_uppercase(data: String) {
    let data = data.to_uppercase();

    println!("{}", data);
}