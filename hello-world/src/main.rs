use core::fmt;


#[derive(Debug)]
struct Struct(i32);

#[derive(Debug)]
struct Point
{
    x: i32,
    y: i32
}

impl Point {
    fn foo(&self) {
        println!("{:?}", self.x * self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "override fmt::Display x: {}, y: {}", self.x, self.y)
    }
}


fn main() {
    println!("{:?}",10);
    println!("{:?}", Struct(10));

    let p = Point{x:10, y:10};
    println!("{:?}", p);
    println!("{:#?}", p);
    println!("{}",p);
    p.foo();
}
