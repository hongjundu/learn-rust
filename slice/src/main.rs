use std::any::Any;
use std::fmt::Debug;


#[derive(Debug)]
struct Point 
{
    x : i32,
    y : i32,

}


fn main() {
    let mut v = Vec::<Box<dyn Any>>::new();

    v.push(Box::new(1));
    v.push(Box::new(3.4));
    v.push(Box::new(Point{x: 1, y: 2}));
    v.push(Box::new(String::from("hello")));

    println!("{:?}", v);

    for item in &v {
        if let Some(int) = item.downcast_ref::<i32>() {
            println!("Int: {}", int);
        } else if let Some(float) = item.downcast_ref::<f64>() {
            println!("Float: {}", float);
        } else if let Some(point) = item.downcast_ref::<Point>() {
            println!("Point: ({}, {})", point.x, point.y);
        } else if let Some(s) = item.downcast_ref::<String>() {
            println!("String: {}", s);
        } 
        else {
            println!("Unknown type");
        }
    }
}
