use std::io::{self, Write};


fn int(texto: &str) -> f32{
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x :f32 = x.trim().parse().unwrap();
    x
}

fn main(){
    let mut num:f32;
    let mut soma:f32 = 0.0;
    for _ in 0..5{
        num = int("Digite uma nota: ");
        soma += num;
    }

    let media :f32 = soma/5.0;

    println!("A soma dos numeros foi: {}", soma);
    println!("A media dos numeros somados foi: {:.1}", media);
}