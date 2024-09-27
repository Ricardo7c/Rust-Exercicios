use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{texto}");
    io::stdout()
        .flush()
        .expect("falha ao limpar");

    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Valor invalido!");
    x
}

fn main (){
    let horas = input("Quanto você ganha por hora? ")
        .trim()
        .parse::<f32>()
        .expect("Falha ao converter!");

    let tempo = input("Quanta horas você trabalhou no mês? ")
        .trim()
        .parse::<f32>()
        .expect("Falha ao converter");

    let salario:f32 = horas*tempo;
    println!("O seu salario no final do mês é de R${:.2}", salario)
}