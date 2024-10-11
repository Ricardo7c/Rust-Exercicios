use std::io::{self, Write};

fn float(texto:&str) -> f32{
    print!("{texto}");
    io::stdout().flush().expect("Erro ao limpar!");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Valor invalido!");
    let x = x.trim().parse::<f32>().unwrap();
    x
}

fn main() {
    let nota1 = float("Digite a primeira nota: ");
    let nota2 = float("Digite a segunda nota: ");
    let nota3 = float("Digite a terceira nota: ");
    let nota4 = float("Digite a quarta nota: ");
    let media = (nota1+nota2+nota3+nota4)/4.0;
    println!("A media foi: {:.1}", media);
    
}