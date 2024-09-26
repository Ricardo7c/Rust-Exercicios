// Faça um Programa que pergunte em que turno você estuda.
// Peça para digitar M-matutino ou V-Vespertino ou N- Noturno.
// Imprima a mensagem "Bom Dia!", "Boa Tarde!", "Boa Noite!" ou "Valor Inválido!"

use std::io::{self, Write};

fn input(texto: &str) -> char {
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_uppercase().chars().next().unwrap_or(' ') // Garante que sempre haja um caractere padrão
}

fn main() {
    println!("Em que turno você estuda?");
    let turno = input("(M)atutino, (V)espertino, (N)oturno:");

    let saudacao = match turno {
        'M' => "Bom dia!",
        'V' => "Boa tarde!",
        'N' => "Boa noite!",
        _ => "Valor inválido!",
    };

    println!("{}", saudacao);
}
