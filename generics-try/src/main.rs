fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for n in list.iter() {
        if n > largest {
            largest = n;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 特定の型を持つ(?)構造体のインスタンのインスタンスにメソッドを追加できる
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &n in list.iter() {
        if n > largest {
            largest = n;
        }
    }

    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &n in list.iter() {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 100, 28, 12];
    let result = largest(&number_list);
    println!("{}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let result = largest(&char_list);
    println!("{}", result);

    let integer = Point { x: 0, y: 1 };
    let float = Point { x: 0.3, y: 1.5 };
}
