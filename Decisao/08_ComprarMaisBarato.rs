// Faça um programa que pergunte o preço de três produtos e informe qual produto você deve comprar,
// sabendo que a decisão é sempre pelo mais barato.

use std::io::{self, Write};
use std::collections::HashMap;

fn input(texto:&str) -> f32{
    loop{
        print!("{texto}");
        io::stdout().flush().unwrap();
        
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<f32>() {
            Ok(valor) => return valor,
            Err(_) => println!("Valor invalido, Tente novamente!"),
            }
        }
    }

fn main(){
    let mut produtos = HashMap::new();
    let produto1 = input("Digite o preço do produto 1: ");
    produtos.insert(String::from("Produto 1"), produto1);
    let produto2 = input("Digite o preço do produto 2: ");
    produtos.insert(String::from("Produto 2"), produto2);
    let produto3 = input("Digite o preço do produto 3: ");
    produtos.insert(String::from("Produto 3"), produto3);
    let mut preco = f32::MAX;
    let mut nome = String::new();
    for (chave, valor) in &produtos{
        if valor < &preco{
            preco = *valor;
            nome = chave.clone();
        }
    }
    println!("Você deve comprar o {} que custa: R${:.2}",nome, preco)
}