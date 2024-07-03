fn main() {
 
    let a : Option<i32> = None;
    let b : Option<i32> = Some(10);

    if let Some(x)= b    {
        println!("{}", x);
    }

    if let None = a {
        println!("None");
    }

    let mut p = Some(0);
    while let Some(i) = p {
        if i < 10 {
            p = Some( i + 1);
            println!("{:#?}", p);
        }else {
            p = None;
        }
    }

    println!("{:?}", p);
 
}
