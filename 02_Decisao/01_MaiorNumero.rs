// Faça um Programa que peça dois números e imprima o maior deles.

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
    let num1 = input("Digite o primeiro numero: ");
    let num2 = input("Digite o segundo numero: ");
    if num1 > num2 {
        println!("{} é maior que {}", num1, num2);
    }else if num2 > num1 {
        println!("{} é maior que {}", num2, num1);
    }else{
        println!("Os números são iguais!");
    }
}
