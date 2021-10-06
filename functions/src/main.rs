fn main() {
    let x = {
        let x = 3;
        x * 2
    };
    another_function(x);
    let double = double(x);
    println!("Double, {}!", double);
}

fn another_function(x: i32) {
    println!("Another function, {}!", x);
}

fn double(x: i32) -> i32 {
    x * x
}
