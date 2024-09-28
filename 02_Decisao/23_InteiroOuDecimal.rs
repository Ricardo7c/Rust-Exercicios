use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x :f32 = x.trim().parse().unwrap();
    x
}

fn main(){
    let num = input_float("Digite um numero: ");
    let numr = &num.trunc();

    if num == *numr{
        println!("O número é inteiro");
    }else{
        println!("O número é real")
    }
}