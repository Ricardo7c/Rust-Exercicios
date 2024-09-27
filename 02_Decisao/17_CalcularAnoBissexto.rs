// Faça um Programa que peça um número correspondente a um determinado ano
// e em seguida informe se este ano é ou não bissexto.

use std::io::{self, Write};

fn input_int(texto: &str) -> i32{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<i32>().unwrap();
    x
}

fn main(){
    let ano = input_int("Digite o ano: ");
    if ano % 400 == 0 || ano % 4 == 0{
        println!("O ano é bissexto!");
    }else{
        println!("O ano não é bissexto!");
    }
}