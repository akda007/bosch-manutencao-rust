use std::io::{self, Write};

fn if01() {
    let mut sal: f32 = 0.0;

    print!("QUAL O SALÁRIO: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");


    let sal_input: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada inválida. Por favor, insira um número válido.");
            return;
        }
    };

    sal = sal_input;

    if sal > 5000.0 {
        println!("MAIOR QUE R$5.000,00");
    }
}

