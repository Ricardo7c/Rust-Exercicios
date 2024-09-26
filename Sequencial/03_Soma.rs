use std::io::{self, Write};

fn input(texto:&str) -> String{
    print!("{texto}");
    io::stdout().flush().expect("Erro ao limpar");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Valor invalido");
    x
}
fn int(input:String) -> i32{
    let num = input.trim().parse::<i32>().unwrap();
    num
}

fn main(){

    let num1 = int(input("Digite um numero: "));
    let num2 = int(input("Digite outro numero: "));
    println!("A soma de {} e {} é {}", num1, num2, (num1+num2));
    
}