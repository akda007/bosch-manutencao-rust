use std::io::{self, Write};


#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}


struct FichaAluno {
    codigo: i32,
    name: String,
    telefone: i32,
    mail: String
}

impl FichaAluno {
    pub fn new(codigo: i32, name: String, telefone: i32, mail: String) -> Self {
        Self {codigo, name, telefone, mail}
    }

    pub fn new_read() -> Self {
        print!("Nome do aluno: ");
        io::stdout().flush().expect("Flush Failed");
        read_str!(nome);

        print!("Codigo do aluno: ");
        io::stdout().flush().expect("Flush Failed");
        read!(codigo as i32);

        print!("Telefone do aluno: ");
        io::stdout().flush().expect("Flush Failed");
        read!(tel as i32);

        print!("Mail do aluno: ");
        io::stdout().flush().expect("Flush Failed");
        read_str!(mail);

        Self::new(codigo, nome.to_string(), tel, mail.to_string())
    }

    pub fn show_data(&self) {
        println!("=============================");
        println!("Codigo: {}", self.codigo);
        println!("Nome: {}", self.name);
        println!("Telefone: {}", self.telefone);
        println!("Mail: {}", self.mail);
    }
}

pub fn struct_exercise() {
    
    let ficha = FichaAluno::new_read();

    ficha.show_data();

    
}