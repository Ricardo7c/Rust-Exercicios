use std::io::{self, Write};

fn main(){
    print!("informe o tamanho do lado do quadrado: ");
    io::stdout().flush().expect("Erro ao limpar");
    let mut lado = String::new();
    io::stdin().read_line(&mut lado).expect("Valor invalido!");
    let area = 4.0*(lado.trim().parse::<f32>().expect("Falha na conversão!"));
    println!("O dobro da area do quadrado é: {:.2}",area*2.0)
}