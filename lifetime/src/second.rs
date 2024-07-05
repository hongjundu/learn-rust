pub fn main() {
    let x = 1;
    let y = 2;

    let p: Point<i32> = Point::new(&x,&y);
    println!("{:?}", p);
}


#[derive(Debug)]
struct Point<'a,T> {
    x: &'a T,
    y: &'a T,
}

impl <'a,T> Point<'a,T> {
    fn new(x: &'a T, y: &'a T) -> Point<'a, T> {
        Point {
            x, y
        }
    }
}