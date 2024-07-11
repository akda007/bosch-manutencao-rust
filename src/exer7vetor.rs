use std::io::{self, Write};

fn exer7vetor() {
    let mut num: [i32; 50] = [0; 50];
    let mut soma: i32 = 0;

    for cont in 0..50 {
        print!("Informe o {}º número: ", cont + 1);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

       
        let num_input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        num[cont] = num_input; 
        soma += num[cont]; 
    }

    println!("A soma dos números: {}", soma); 
}
