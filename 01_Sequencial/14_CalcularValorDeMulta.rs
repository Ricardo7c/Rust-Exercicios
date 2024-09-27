use std::io::{self, Write};

fn main(){
    print!("informe o peso total dos peixes: ");
    io::stdout()
        .flush()
        .expect("");
    let mut peso = String::new();
    io::stdin()
        .read_line(&mut peso)
        .expect("");
    let peso:f32 = peso.trim()
        .parse()
        .expect("");
    
    let excedente = peso - 50.0;
    if excedente > 0. {
        println!("O valor da multa será: R${:.2}", excedente*4.0);
    }else{
        println!("O peso pescado não excede o limite");
    }
}