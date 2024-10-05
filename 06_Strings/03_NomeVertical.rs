use std::io::{self, Write};

fn input(texto: &str) -> String{ 
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn main(){
    let nome = input("Digite seu nome: ").to_uppercase();
    for letra in nome.trim().chars(){
        println!("{}", letra);
    }
}