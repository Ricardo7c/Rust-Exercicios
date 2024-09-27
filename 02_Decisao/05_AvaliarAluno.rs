// Faça um programa para a leitura de duas notas parciais de um aluno.
// O programa deve calcular a média alcançada por aluno e apresentar:
// A mensagem "Aprovado", se a média alcançada for maior ou igual a sete;
// A mensagem "Reprovado", se a média for menor do que sete;
// A mensagem "Aprovado com Distinção", se a média for igual a dez.

use std::io::{self, Write};

fn input(texto:&str) -> f32{
    loop{
        print!("{texto}");
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        let resultado = float(x);

        match resultado{
            Ok(valor) => return valor,
            Err(_) => {
                println!("Entrada invalida, tente novamente!");
            } 
        }
    }
}

fn float(num:String) -> Result<f32, std::num::ParseFloatError>{
    let x = num.trim().parse::<f32>();
    x
}   

fn main(){
    let nota1 = input("Digite a primeira nota: ");
    let nota2 = input("Digite a segunda nota: ");
    let media:f32 = (nota1+nota2)/2.0;
    if media == 10.0 {
        println!("Aprovado com Distinção");
    } else if media >= 7.0 {
        println!("Média: {:.1} - Aluno Aprovado", media);
    } else if media < 7.0 {
        println!("Média: {:.1} - Aluno Reprovado", media);
    } else {
        println!("Nota inválida");
    }
}