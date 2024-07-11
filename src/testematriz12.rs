use std::io::{self, Write};

const DIML: usize = 3; 
const DIMC: usize = 3;

pub fn testematriz() {
    let mut matriz = [[0; DIMC]; DIML];

    
    for l in 0..DIML {
        for c in 0..DIMC {
            println!("Informe a posição {}, {} da matriz:", l, c);
            let _ = io::stdout().flush(); 
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Falha ao ler a entrada");
            matriz[l][c] = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Entrada inválida. Insira um número válido.");
                    continue;
                }
            };
        }
    }

    
    for l in 0..DIML {
        for c in 0..DIMC {
            print!("{:4}", matriz[l][c]);
        }
        println!();
    }
}
