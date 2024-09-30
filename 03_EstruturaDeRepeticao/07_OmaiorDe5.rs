// Faça um programa que leia 5 números e informe o maior número.
use std::io::{self, Write};


fn int(texto: &str) -> i32{
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x :i32 = x.trim().parse().unwrap();
    x
}

fn main(){
    let mut num:i32;
    let mut maior = 0;
    for _ in 0..5{
        num = int("Digite um numero: ");
        if num > maior{
            maior = num
        }
    }
    println!("O maior numero digitado foi: {}", maior)
}