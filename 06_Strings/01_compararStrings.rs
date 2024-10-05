use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn main(){
    let texto1 = input("Digite o primeiro texto: ");
    let texto2 = input("Digite o segundo texto: ");

    println!("Comparando duas Strings");

    println!("String 1: {} - Tamanho: {}", texto1.trim(), texto1.trim().len());
    println!("String 2: {} - Tamanho: {}", texto2.trim(), texto2.trim().len());

    if texto1.trim().len() == texto2.trim().len(){
        println!("Os dois textos tem tamanhos iguais");
    }else{
        println!("Os textos tem tamanho diferentes");
    };

    if texto1 == texto2{
        println!("O Conteudo é o mesmo!");
    }else{
        println!("Os conteudos são diferentes!");
    }

}