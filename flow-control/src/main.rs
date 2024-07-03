fn main() {
    
    let n = 6;
    let big_N = {
        if n > 5 {
            n * 100
        }else {
            n * 10
        }
    };

    println!("big N {}", big_N);

    let mut counter = 0;
    let counter2 = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter * 100;
        }
    };

    println!("counter {}", counter2);

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

}
