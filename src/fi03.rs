use std::io::{stdin, stdout};

pub fn fi03() {
    let num: i32;

    println!("Informe um numero");
    
    let mut buff = String::new();
    stdin().read_line(&mut buff);
    num = buff.trim().parse().expect("Unable to parse number");

    let res:String = if num % 2 == 0 { "Numero é par".to_string() } else { "Numero é impar".to_string() };
    println!("{}", res);
}