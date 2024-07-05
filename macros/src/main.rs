
macro_rules! say_hello {
    ($name:expr) => (
        println!("Hello {}", $name);
    )
}

macro_rules! print_type_path {
    ($path:path) => {
        println!("Type path: {}", stringify!($path));
    };
}

// fn main() {
//     say_hello!("Gorge");
//     print_type_path!(fmt::Display);
// }


macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // 强制类型为整型
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2 // 看到了吧，`eval` 可并不是 Rust 的关键字！
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
