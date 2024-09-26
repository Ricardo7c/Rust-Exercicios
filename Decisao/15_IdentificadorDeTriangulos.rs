// Faça um Programa que peça os 3 lados de um triângulo. O programa deverá informar se os valores podem ser um triângulo. Indique, caso os lados formem um triângulo, se o mesmo é: equilátero, isósceles ou escaleno.
// Dicas:
// Três lados formam um triângulo quando a soma de quaisquer dois lados for maior que o terceiro;
// Triângulo Equilátero: três lados iguais;
// Triângulo Isósceles: quaisquer dois lados iguais;
// Triângulo Escaleno: três lados diferentes;

use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{}", texto);
    let mut x = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut x).expect("Erro na entrada de dados");
    let x = x.trim().parse::<f32>().expect("Erro ao converter");
    x
}

fn main(){
    let a = input_float("Digite o primeiro lado: ");
    let b = input_float("Digite o segundo lado: ");
    let c = input_float("Digite o terceiro lado: ");

    if a+b>c && a+c>b && b+c>a{
        if a == b && a == c && b == c{
            println!("O triangulo é Equilátero");
        }else if a != b && a != c && b!=c{
            println!("O triangulo é Escaleno");
        }else{
            println!("O triangulo é Isósceles");
        }
    }else{
        println!("Os lados não formam um triangulo");
    }

}