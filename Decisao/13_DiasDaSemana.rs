// Faça um Programa que leia um número e exiba o dia correspondente da semana.
// (1-Domingo, 2- Segunda, etc.), se digitar outro valor deve aparecer valor inválido.

use std::io::{self, Write};

fn input_int(texto: &str) -> char{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.chars().next().expect("");
    x
}

fn dia_semana(texto: &str) -> String{
    loop {
        let num = input_int(&texto);
        match num {
            '1' => return "Domingo".to_string(),
            '2' => return "Segunda".to_string(),
            '3' => return "Terça".to_string(),
            '4' => return "Quarta".to_string(),
            '5' => return "Quinta".to_string(),
            '6' => return "Sexta".to_string(),
            '7' => return "Sabado".to_string(),
            _ => println!("Valor invalido!"),
        }
    }
}

fn main(){
    let dia = dia_semana("Digite um numero: ");
    println!("{}",dia);
}