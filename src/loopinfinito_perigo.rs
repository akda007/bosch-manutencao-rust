use std::io::{self, Write};

fn loopinfinito_perigo() {
    let mut num: i32 = 0;
    let mut cont: i32 = 0;

    loop {
        print!("fala ai um numero cacildis: \n");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

    
        num = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, insira um número válido.");
                continue;
            }
        };

        cont += 1;

        if num == 0 {
            break;
        }
    }

    println!("foram informados {} numeros.", cont - 1);
}
