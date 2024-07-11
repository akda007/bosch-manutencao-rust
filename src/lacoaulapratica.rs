use std::io::{self, Write};

pub fn lacoaulapratica() {
    let mut idade = 0;
    let mut cont = 0;
    let mut totalidade = 0;

    for _ in 1..=2 {
        println!("Informe a idade da criança:");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Falha ao ler a entrada");
        idade = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Insira uma idade válida.");
                continue;
            }
        };
        totalidade += idade;
    }

    let media = totalidade as f32 / 2.0;

    println!("\nO total das idades é {} anos", totalidade);
    println!("A média de idade é {:.1} anos", media);
}