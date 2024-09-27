use std::io::{self, Write};

fn main(){

    print!("Digite um numero: ");
    io::stdout().flush().expect("Erro ao limpar");
    let mut numero = String::new();
    io::stdin().read_line(&mut numero).expect("Valor invalido");

    print!("O numero digitado foi: {numero}");

}