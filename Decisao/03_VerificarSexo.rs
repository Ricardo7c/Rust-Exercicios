// Faça um Programa que verifique se uma letra digitada é "F" ou "M". 
// Conforme a letra escrever: F - Feminino, M - Masculino, Sexo Inválido.

use std::io::{self, Write};

fn input(texto:&str) -> String{
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().to_uppercase();
    x
}

fn main(){
    let sexo = input("Qual o seu sexo (M/F)? ");
    match sexo.as_str(){
        "M" => println!("Seu sexo é Masculino."),
        "F" => println!("Seu sexo é Feminino."),
        _ => println!("Valor invalido, digite M ou F"),
    }
}
