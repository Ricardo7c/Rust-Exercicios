use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{texto}");
    io::stdout()
        .flush()
        .expect("");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("");
    let x:f32 = x.trim()
        .parse()
        .expect("");
    x
}

fn main(){
    let hora = input_float("Quanto você ganha por hora? ");
    let tempo = input_float("Quantas horas você trabalha no mês? ");
    let salario = hora*tempo;
    let ir = salario*0.11;
    let inss = salario*0.08;
    let sindicato = salario*0.05;

    println!("Salario bruto é R${:.2}",salario);
    println!("IR(11%) é: R${:.2}", ir);
    println!("INSS (8%) é: R${:.2}",inss);
    println!("Sindicato (5%) é: R${:.2}", sindicato);
    println!("Salario liquido é: R${:.2}", salario-(ir+inss+sindicato));
    
}