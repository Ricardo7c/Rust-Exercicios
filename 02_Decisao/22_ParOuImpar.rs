use std::io::{self, Write};

fn int_input(texto: &str) -> i32{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<i32>().unwrap();
    x
}

fn main(){
    let num = int_input("Digite um numero: ");

    if num % 2 == 0{
        println!("{} é par", num);
    }else{
        println!("{} é impar", num)
    }
}