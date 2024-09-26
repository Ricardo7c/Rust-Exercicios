// As Organizações Tabajara resolveram dar um aumento de salário aos seus colaboradores
// e lhe contraram para desenvolver o programa que calculará os reajustes.
// Faça um programa que recebe o salário de um colaborador e o reajuste segundo
// o seguinte critério, baseado no salário atual:
// salários até R$ 280,00 (incluindo) : aumento de 20%
// salários entre R$ 280,00 e R$ 700,00 : aumento de 15%
// salários entre R$ 700,00 e R$ 1500,00 : aumento de 10%
// salários de R$ 1500,00 em diante : aumento de 5% Após o aumento ser realizado, informe na tela:
// o salário antes do reajuste;
// o percentual de aumento aplicado;
// o valor do aumento;
// o novo salário, após o aumento.

use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{}", texto);
    io::stdout().flush().expect("Falha ao limpar memoria");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Falha na entrada de dados!");
    let x = x.trim().parse::<f32>().expect("Falha na conversão!");
    x
}


fn main(){
    let salario = input_float("Digite o salario: ");
    let aumento: f32;
    let percet: &str;
    if salario <= 280.0{
        aumento = salario*0.20;
        percet = "20%";
    } else if salario > 280.0 && salario <= 700.0{
        aumento = salario*0.15;
        percet = "15%";
    }else if salario > 700.0 && salario <= 1500.0{
        aumento = salario*0.10;
        percet = "10%";
    }else {
        aumento = salario*0.05;
        percet = "5%";
    }

    println!("Salario Original: R${:.2}", salario);
    println!("Percentual de aumento: {}", percet);
    println!("Valor do aumento: R${:.2}", aumento);
    println!("Novo salario: R${:.2}", salario+aumento);

}

