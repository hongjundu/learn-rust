pub fn main() {
    let pig = Pig::new("Gorge");
    println!("{:?}", pig);

    pig.fly(32);
    pig.fly(40.3);
    pig.fly("hello");
}

trait Fly<T> {
    fn fly(&self, _: T);
}

#[derive(Debug)]
struct Pig {
    name : String,
}

impl Pig {
    fn new (name : &str) -> Self {
        Pig { name : name.to_string(), }
    }
}

impl <T> Fly<T> for Pig 
    where T : std::fmt::Display {
    fn fly(&self, t : T) {
        println!("Pig {} fly {}", self.name, t);
    }
}