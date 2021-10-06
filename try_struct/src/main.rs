#[derive(Debug)]
struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
impl User {
    fn from(email: String, user_name: String) -> User {
        User {
            email,
            user_name,
            sign_in_count : 1,
            active : true,
        }
    }
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User::from(
        String::from("user1@gmail.com"),
        String::from("hoge"),
    );
    println!("{:?}", user1);
}
