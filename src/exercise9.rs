use std::io;

pub fn exercise9() {
    let mut code = 0;
    let mut age = 0;
    let mut height = 0.0;
    let mut weight = 0.0;

    let mut buff = String::new();

    println!("Qual o código:");
    io::stdin().read_line(&mut buff.to_string())
        .expect("Falha ao ler a entrada");
    let code: i32 = buff.trim().parse()
        .expect("Entrada inválida");

    println!("Qual a idade:");
    io::stdin().read_line(&mut buff.to_string())
        .expect("Falha ao ler a entrada");
    let age: i32 = buff.trim().parse()
        .expect("Entrada inválida");

    println!("Qual a altura:");
    io::stdin().read_line(&mut buff.to_string())
        .expect("Falha ao ler a entrada");
    let height: f32 = buff.trim().parse()
        .expect("Entrada inválida");

    println!("Qual o peso:");
    io::stdin().read_line(&mut buff.to_string())
        .expect("Falha ao ler a entrada");
    let weight: f32 = buff.trim().parse()
        .expect("Entrada inválida");

    println!("\nO CÓDIGO: {}\nA IDADE: {}\nA ALTURA: {:.2}\nO PESO: {:.2}", code, age, height, weight);
}