fn main() {
    let e1 = Event::Paste(String::from("hello"));

    inspect(e1);

    let e2 = Event::KeyPress('c');

    inspect(e2);

    let e3 = Event::MouseMove{x: 10, y: 20};

    inspect(e3);
}


#[derive(Debug)]
enum Event 
{
    Paste(String),
    KeyPress(char),
    MouseMove{x: u16,y: u16}
}

fn inspect(event : Event) {
    println!("{:?}", event);

    match event {
        Event::Paste(s) => println!("Paste String: {}", s),
        Event::KeyPress(c) => println!("KeyPress Char: {}", c),
        Event::MouseMove { x, y } => println!("MouseMove x: {} y: {}", x, y),
    }
}