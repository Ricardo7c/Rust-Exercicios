use std::io::{self, Write};

fn int(texto:&str) -> i32{
    print!("{texto}");
    io::stdout().flush().expect("Erro ao limpar");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Valor invalido");
    let x = x.trim().parse::<i32>().unwrap();
    x
}

fn main(){

    let num1 = int("Digite um numero: ");
    let num2 = int("Digite outro numero: ");
    println!("A soma de {} e {} é {}", num1, num2, (num1+num2));
    
}