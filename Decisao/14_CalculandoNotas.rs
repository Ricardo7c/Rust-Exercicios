// Faça um programa que lê as duas notas parciais obtidas por um aluno numa disciplina ao longo
// de um semestre, e calcule a sua média. A atribuição de conceitos obedece à tabela abaixo:

//   Média de Aproveitamento  Conceito
//   Entre 9.0 e 10.0        A
//   Entre 7.5 e 9.0         B
//   Entre 6.0 e 7.5         C
//   Entre 4.0 e 6.0         D
//   Entre 4.0 e zero        E

// O algoritmo deve mostrar na tela as notas, a média, o conceito correspondente
// e a mensagem “APROVADO” se o conceito for A, B ou C ou “REPROVADO” se o conceito for D ou E.

use std::io::{self, Write};
fn input_float(texto: &str) -> f32{
    loop{
        print!("{}", texto);
        let mut x = String::new();
        io::stdout().flush().unwrap();
        // Recebe os dados digitados.
        io::stdin().read_line(&mut x).expect("Erro na entrada de dados");
        // Converte o valor digitado para float.
        let x = x.trim().parse::<f32>().expect("Erro ao converter");
        // Verifica se o valor da nota é valido!
        if x > 10.0{
            println!("Valor invalido!");
        }else{
            return x;
        }
    }
}
fn main(){
    let nota1 = input_float("Digite a primeira nota: ");
    let nota2 = input_float("Digite a segunda nota: ");
    let media = (nota1+nota2)/2.0;
    let conceito: String;
    match media {
        0.0..=3.9 => conceito = "E".to_string(),
        4.0..=6.1 => conceito = "D".to_string(),  
        6.0..=7.4 => conceito = "C".to_string(),  
        7.5..=8.9 => conceito = "B".to_string(),  
        0.0..=10.0 => conceito = "A".to_string(),
        _ => conceito = "Valor invalido".to_string(),       
    }
    let mut msg = conceito.clone();
    let comparacao1 = String::from("ABC");
    let comparacao2 = String::from("DE");
    if comparacao1.contains(&conceito){
        msg = "APROVADO".to_string();
    }else if comparacao2.contains(&conceito){
        msg = "REPROVADO".to_string();
    }
    println!();
    println!("Nota 1 = {:.1}", nota1);
    println!("Nota 2 = {:.1}", nota2);
    println!("Conceito = {}", conceito);
    println!("O Aluno foi {}", msg);
}