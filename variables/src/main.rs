fn main() {
    let oka: char = '岡';
    println!("{}", oka);

    let tup = (500, 6.5, 1);

    let (x, _, z) = tup;

    println!("{}, {}", x, z);

    let a = [1,2,3,4,5];
    let i = 1;

    let element = a[i];
    println!("The value of element is: {}", element);
}
