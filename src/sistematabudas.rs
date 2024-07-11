use std::io::stdin;


pub fn tabuada() {
    println!("Numero de tabuadas: ");

    let num: i32;
    let mut buff = String::new();

    stdin().read_line(&mut buff);

    num = buff.trim().parse().expect("Unable to parse");

    for i in 1..11 {
        println!("{} * {} = {}", num, i, num*i);
    }
}