use std::io::{self, Write};

fn main(){
    print!("Qual a temperatura em Fahrenheit? ");
    io::stdout()
        .flush()
        .expect("Falha ao limpar");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Valor invalido!");

    let temp = temp.trim()
        .parse::<f32>()
        .expect("Falha ao converter!");

    let celcio = 5.0*((temp-32.0)/9.0);
    println!("{}° Fahrenheit equivalem a {:.2}° Celcios", temp, celcio);
}