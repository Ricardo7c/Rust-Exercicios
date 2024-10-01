use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn float(texto: &str) -> f32{
    loop{
        let x = input(texto);
        match x.trim().parse::<f32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }

    }
}

fn main(){
    let quant = float("Quantas notas vai digitar? ");
    let mut soma:f32 = 0.0;
    for _ in 0..quant as i32{
        let nota = float("Digite a nota: ");
        soma += nota;
    }

    let media = soma/quant;

    println!("A media entre as {} notas digitadas foi: {:.1}", quant, media);

}