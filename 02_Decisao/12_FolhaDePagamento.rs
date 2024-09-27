// Faça um programa para o cálculo de uma folha de pagamento, sabendo que os descontos são do
// Imposto de Renda, que depende do salário bruto (conforme tabela abaixo) e 3% para o Sindicato
// e que o FGTS corresponde a 11% do Salário Bruto, mas não é descontado (é a empresa que deposita).
// O Salário Líquido corresponde ao Salário Bruto menos os descontos.
// O programa deverá pedir ao usuário o valor da sua hora e a quantidade de horas trabalhadas no mês.
// Desconto do IR:
// Salário Bruto até 900 (inclusive) - isento
// Salário Bruto até 1500 (inclusive) - desconto de 5%
// Salário Bruto até 2500 (inclusive) - desconto de 10%
// Salário Bruto acima de 2500 - desconto de 20% Imprima na tela.

use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn float(texto: String) -> f32{
    let x = texto.trim().parse::<f32>().unwrap();
    x
}

fn main(){

    let valorhora = float(input("Qual o valor da sua hora de trabalho? "));
    let quanthora = float(input("Quantas horas você trabalhou esse mês? "));

    let salario = valorhora*quanthora;
    let inss = salario*0.10;
    let fgts = salario*0.11;
    let ir:f32;
    let irperct:&str;

    match salario {
        900.1..=1500.0 => {
            ir = salario*0.05;
            irperct = "5%";
        },
        1500.0..=2500.0 => {
            ir = salario*0.10;
            irperct = "10%";
        },
        0.0..900.0 => {
            ir = 0.0;
            irperct = "ISENTO";
        },
        _ => {
            ir = salario*0.20;
            irperct = "20%"
        }

    }

    println!("Salario Bruto: R${:.2}", salario);
    println!("Desconto IR {}: R${:.2}", irperct, ir);
    println!("Desconto INSS 10%: R${:.2}", inss);
    println!("Deposito de FGTS 11%: R${:.2}", fgts);
    println!("Total de Descontos: R${:.2}", ir+inss);
    println!("Salario liquido: R${:.2}", salario-(ir+inss));
    
}