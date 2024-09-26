use std::io::{self, Write};

fn input(texto:&str) -> String{
    print!("{texto}");
    io::stdout().flush().expect("Erro ao limpar!");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Valor invalido!");
    x
}
fn float(input:String) -> f32{
    let num = input.trim().parse::<f32>().unwrap();
    num
}

fn main() {
    let nota1 = float(input("Digite a primeira nota: "));
    let nota2 = float(input("Digite a segunda nota: "));
    let nota3 = float(input("Digite a terceira nota: "));
    let nota4 = float(input("Digite a quarta nota: "));
    let media = (nota1+nota2+nota3+nota4)/4.0;
    println!("A media foi: {:.1}", media);
    
}