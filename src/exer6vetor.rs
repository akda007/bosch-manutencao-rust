use std::io::{self, Write};

fn exer6vetor() {
    let mut num = [0; 50];

    // Input phase
    let mut count = 0;
    while (count < 50) {
        println!("Informe o {}º número:", count + 1);
        
        let _ = io::stdout().flush(); 
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Falha ao ler a entrada");

        num[count] = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Insira um número válido.");
                count -= 1;
                continue;
            }
        };

        count +=1;
    }

    for count in 0..50 {
        println!("{}º número: {}", count + 1, num[count]);
    }
}
