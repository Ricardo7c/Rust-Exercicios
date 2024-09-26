// Faça um Programa que peça um valor e mostre na tela se o valor é positivo ou negativo.

use std::io::{self, Write};

fn input(texto:&str) -> i32{
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<i32>().unwrap();
    x
}

fn main(){
    let num = input("Digite um numero: ");
    if num > 0 {
        println!("{} é POSITIVO!", num);
    }else if num < 0 {
        println!("{} é NEGATIVO!", num);
    }else{
        println!("Zeró é um numero neutro!");
    }
}