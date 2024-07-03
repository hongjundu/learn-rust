use core::fmt;
use std::{num::ParseIntError, str::FromStr};

fn main() {
    let num = Number {value: 10};
    println!("{}", num);

    let num2 = Number::from(20);
    println!("{}", num2);

    let num3 : Number = 30.into();
    println!("{}", num3);

    let num4 = Number::from(40.4);
    println!("{}", num4);

    let num5 : Number = 50.5.into();
    println!("{}", num5);

    let str_num5 = num5.to_string();
    println!("string from num5: {}", str_num5);

    let num6 = "100".parse::<Number>();
    if let Ok(n) = num6 {
        println!("parsed from string OK: {}", n)
    }else {
        eprintln!("{:?}", num6.err());
    }

    let num6: Result<Number, ParseIntError> = "abc".parse::<Number>();
    if let Ok(n) = num6 {
        println!("parsed from string OK: {}", n)
    }else {
        eprintln!("{:#?}", num6.err());
    }

}



#[derive(Debug)]
struct Number {
    value : i32,
}

impl fmt::Display for  Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I am a number: {}", self.value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number {value}
    }
}

impl  From<f32> for Number {
    fn from(value: f32) -> Self {
        let value_f = value as i32;
        Number {value : value_f}
    }
}

// impl ToString for Number {
//     fn to_string(&self) -> String {
//         format!("{}", self.value)
//     }
// }


impl FromStr for Number {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(value) => Ok(Number { value }),
            Err(e) => Err(e)
        }
    }
}