use std::io;

fn primeiroprograma() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Informe um numero:");

    io::stdin().read_line(&mut num1)
        .expect("Falha ao ler entrada.");

    let num1: f32 = num1.trim().parse()
        .expect("Por favor, digite um número válido.");

    println!("Informe um segundo numero:");
    
    io::stdin().read_line(&mut num2)
        .expect("Falha ao ler entrada.");

    let num2: f32 = num2.trim().parse()
        .expect("Por favor, digite um número válido.");

    println!("O resultado foi: {:.2}", num1 + num2);
}
