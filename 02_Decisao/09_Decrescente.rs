// Faça um Programa que leia três números e mostre-os em ordem decrescente.

use std::io::{self, Write};

fn input(texto:&str) -> i32{
    loop{
        print!("{texto}");
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() {
            Ok(valor) => return valor,
            Err(_) => println!("Valor inválido, Tente novamente!"),
            }
        }
    }

fn main(){
    let mut numeros: Vec<i32> = vec![];
    for n in 1..=3{
        let texto = format!("Digite o {}º número: ", n);
        let num = input(&texto);
        numeros.push(num);
    }
    numeros.sort();
    let reverso: Vec<_> = numeros.iter().rev().collect();
    println!("{:?}", reverso);
}