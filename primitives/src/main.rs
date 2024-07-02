fn main() {
    
    let pair = (true, 20);

    println!("{:?}", pair);

    println!("{:?}", reverse_tuple(pair));

}

fn reverse_tuple(pair : (bool, i32)) -> (i32, bool) {
    let (b,i) = pair;
    (i,b)
}