use std::io;
use std::f64::consts::{self, PI};

pub fn code() {
    let raio: f64;

    println!("Informe o raio do circulo");
    let mut str: String = String::new();

    let _ = io::stdin().read_line(&mut str);

    raio = str.trim().parse::<f64>().expect("Unable to parse data");

    println!("Area do circulo: {}", f64::powf(raio, 2.0) * PI)
    
}