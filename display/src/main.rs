use core::fmt;


#[derive(Debug)]
struct MyVec(Vec<i32>);

impl fmt::Display for MyVec{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let v = &self.0;

        for i in v.iter() {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}

fn main() {
    let my_vec = MyVec(vec![1,2,3]);

    println!("{}", my_vec);

    let t = (1,"hello",3.5);

    println!("{:?}", t);
}
