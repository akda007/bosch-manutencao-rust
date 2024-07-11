use std::io::stdin;


pub fn idade() {
    let age: i32;

    let mut str = String::new();

    stdin().read_line(&mut str);

    age = str.trim().parse::<i32>().expect("Unable to parse!");
    println!("A idade do aluno é {}", age);
}