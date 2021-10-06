fn main() {
    let s1 = String::from("hello");

    takes_ownership(s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_ownership(s2);
    println!("{}", s3);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}