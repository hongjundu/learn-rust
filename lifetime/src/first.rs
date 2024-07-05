pub fn main() {
    let x = longest("hello", "hello1");
    println!("{}", x);

    let x = String::from("hello123");
    let y = String::from("hello1");
    println!("{}", longest(&x, &y));
}

fn longest<'a>(lhs : &'a str, rhs : &'a str) -> &'a str {
    if lhs.len() > rhs.len() {
        return lhs
    }
    return rhs
}