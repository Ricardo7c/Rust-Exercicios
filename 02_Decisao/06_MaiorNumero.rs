// Faça um Programa que leia três números e mostre o maior deles.

use std::io::{self, Write};       // Biblioteca nescessaria para entradas do usuario no terminal
use std::collections::HashSet;    // Biblioteca nescessaria para usar o hashset


fn input(texto: &str) -> i32 {
    loop {                              // Loop infinito que só para quando a função retorna um valor valido.
        print!("{texto}");
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() { //Tenta converter o que foi digitado para inteiro
            Ok(valor) => return valor,  // Retorna o valor e quebra o loop
            Err(_) => println!("Entrada inválida, tente novamente!"), //Mostra msg e continua o loop
        }
    }
}

fn main(){
    let mut add = HashSet::new();  //Cria um hashset para armazenar os valores digitados
    let num1 = input("Digite um numero: ");
    add.insert(num1);              // Adiciona o valor digitado no hash
    let num2 = input("Digite o segundo numero: ");
    add.insert(num2);
    let num3 = input("Digite o terceiro numero: ");
    add.insert(num3);
    if let Some(valor) = add.iter().max(){  //Itera sobre os valores dentro do hashset e retorna o maior
        println!("O maior valor digititado foi: {}", valor);
    }
}