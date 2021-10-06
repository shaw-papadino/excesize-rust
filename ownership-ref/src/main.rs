fn main() {
    let mut s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &mut s1;
    change(&mut s1);

    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
}
// 関数の引数に参照を取ることを借用と呼ぶ
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}