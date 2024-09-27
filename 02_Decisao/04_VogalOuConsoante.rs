// Faça um Programa que verifique se uma letra digitada é vogal ou consoante.

use std::io::{self, Write};

fn input(texto:&str) -> char{
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().to_uppercase().chars().next().unwrap();
    x
}
fn main(){
    let letra = input("Digite uma letra: ");
    let vogais = ['A','E','I','O','U'];

    if letra.is_alphabetic(){
        match vogais.contains(&letra){
            true => println!("{letra} é uma vogal!"),
            false => println!("{letra} é uma consoante!"),
        }
    }else{
        println!("{letra} não é uma letra.");
    }
}

