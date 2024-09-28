use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x :f32 = x.trim().parse().unwrap();
    x
}

fn main(){
    let morangos = input_float("Quantos kg de morangos deseja? ");
    let macas = input_float("Quantos kg de maças deseja? ");

    let kg = morangos+macas;
    let valormorangos:f32;
    let valormacas:f32;

    if morangos <= 5.0{
        valormorangos = morangos*2.50;
    }else{
        valormorangos = morangos*2.20;
    }

    if macas <= 5.0{
        valormacas = macas*1.80;
    }else{
        valormacas = macas*1.50;
    }

    let mut valor = valormacas+valormorangos;

    if kg > 8.0 || valor > 25.0{
        valor -= valor*0.10;
    }

    println!("O total a pagar é R$ {:.2}", valor)
}