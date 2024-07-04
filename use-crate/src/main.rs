
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();

    let random_number: u32 = rng.gen();
    println!("Random u32 (0-99): {}", random_number % 100);

    let random_float: f64 = rng.gen();
    println!("Random float: {}", random_float);

    let random_range = rng.gen_range(1..101);
    println!("Random number in range 1 to 100: {}", random_range);

    let random_bool: bool = rng.gen();
    println!("Random bool: {}", random_bool);
}


#[allow(dead_code)]
fn foo() {
    println!("foo");
}