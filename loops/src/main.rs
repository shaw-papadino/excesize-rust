fn main() {
    let cels = 25;
    let fah = celsius_to_fahrenheit(cels);
    println!("cels:{} → fah:{}", cels, fah); 
}

fn celsius_to_fahrenheit(cels: i32) -> i32 {
    (cels * 9 / 5) + 32
}
