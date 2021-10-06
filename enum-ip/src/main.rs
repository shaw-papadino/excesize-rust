#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}
fn main() {
    // let m = Message::Write(String::from("hello"));

    // m.call()
    // let x: i8 = 3;
    // let y: Option<i8> = Some(10);

    // let res: i8 = match y {
    //     Some(num) => x + num,
    //     // None => x,
    // };

    // matchケースで漏れがあればコンパイル時にエラー出力してくれる
    let res: String = match IpAddr::V4(String::from("127.0.0.1")) {
        IpAddr::V4(s) => s,
        _ => String::from("None"),
    };

    // if letは戻り値は()?
    // if let IpAddr::V4(s) =IpAddr::V4(String::from("127.0.0.1")) {
    //     s;
    // } else {
    //     String::from("None");
    // }

    println!("{}", res);
}
