pub fn main() {
    let p = Point{x: 10, y: 10};

    println!("i32 point: {:?}", p);

    print_point(p);

    let p = Point{x: 10.2, y: 10.9};

    println!("f64 point: {:?}", p);

    print_point(p);

}

fn print_point<T> (p : Point<T>) 
    where T : std::fmt::Display {
    let Point {x, y} = p;

    println!("x = {}, y = {}", x, y);
}

#[derive(Debug)]
struct Point<T>{
    x : T,
    y : T,
}