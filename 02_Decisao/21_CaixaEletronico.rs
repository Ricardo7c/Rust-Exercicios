use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x:f32 = x.trim().parse().unwrap();
    x
}

fn main(){
    let mut saque = input_float("Digite o valor do saque: ");
    let mut n100 = 0;
    let mut n50 = 0;
    let mut n10 = 0;
    let mut n5 = 0;
    let mut n1 = 0;

    while saque >= 100.0 {
        n100 += 1;
        saque -= 100.0;
    }

    while saque >= 50.0 {
        n50 += 1;
        saque -= 50.0;
    }
    
    while saque >= 10.0 {
        n10 += 1;
        saque -= 10.0;
    }
    
    while saque >= 5.0 {
        n5 += 1;
        saque -= 5.0;
    }
    
    while saque >= 1.0 {
        n1 += 1;
        saque -= 1.0;
    }


    if n100 > 0 {
        if n100 > 1{
            println!("{} Notas de R$100,00", n100);
        }else{
            println!("{} Nota de R$100,00", n100);
        }
    }
    if n50 > 0 {
        if n50 > 1{
            println!("{} Notas de R$50,00", n50);
        }else{
            println!("{} Nota de R$50,00", n50);
        }
    }
    if n10 > 0 {
        if n10 > 1{
            println!("{} Notas de R$10,00", n10);
        }else{
            println!("{} Nota de R$10,00", n10);
        }
    }
    if n5 > 0 {
        if n5 > 1{
            println!("{} Notas de R$5,00", n5);
        }else{
            println!("{} Nota de R$5,00", n5);
        }
    }
    if n1 > 0 {
        if n1 > 1{
            println!("{} Notas de R$1,00", n1);
        }else{
            println!("{} Nota de R$1,00", n1);
        }
    }
}