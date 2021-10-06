// &xと&yと戻り値のライフタイムが同じであることを示す
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // ライフタイムが短い方に合わせるのでresultに入る参照のライフタイムが内側のスコープなので、コンパイルエラー
    // let string1 = String::from("abcd");
    // let string2;
    // {
    //     string2 = "xyz";

    //     let result = longest(string1.as_str(), string2);
    //     // 最長の文字列は、{}です
    // }
    // println!("The longest string is {}", result);
}
