use std::io::{self, Write};

fn main(){

    print!("Digite o raio: ");
    io::stdout().flush().expect("Erro ao limpar.");
    let mut raio = String::new();
    io::stdin().read_line(&mut raio).expect("Valor invalido!");


    let raio = raio.trim().parse::<f32>().expect("Falha ao converter!");
    let result = 3.14*raio.powf(2.0);
    println!("A area do circulo é: {:.2}cm²", result);
    
}