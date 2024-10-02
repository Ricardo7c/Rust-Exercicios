use std::io::{self, Write};

fn float(texto:&str) -> f32{
    loop {
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<f32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido, tente novamente!")
        }
    }
}

fn main(){
    let cds = float("Quantos CDs você tem? ");

    let mut quant = cds;
    let mut soma = 0.0;
    while quant > 0.0 {
        let preco = float("Quanto custou o CD: ");
        soma += preco;
        quant -= 1.0;
    }

    let media = soma/cds;
    println!("Você gastou em media R${:.2} por CD", media);
}