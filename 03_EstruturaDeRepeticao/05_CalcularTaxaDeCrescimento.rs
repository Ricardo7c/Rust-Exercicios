// Altere o programa anterior permitindo ao usuário informar as populações e as taxas de crescimento
// iniciais. Valide a entrada e permita repetir a operação.
use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn float(texto: &str) -> f32{
    loop {
        let x = input(texto);
        match x.trim().parse::<f32>(){
            Ok(num) => return num,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    loop {
        let mut a = float("Digite a população do país A: ");
        let mut tax_a = float("Digite a taxa de crescimento do país A: ");
        tax_a /= 100.0;

        let mut b = float("Digite a população do país B: ");
        let mut tax_b = float("Digite a taxa de crescimento do país B: ");
        tax_b /= 100.0;

        let mut anos = 0;
        while a < b {
            a += a*tax_a;
            b += b*tax_b;
            anos += 1;
        }

        println!("Serão nescessarios {} anos", anos);
        println!("--------------------------------");

        let fim = input("Deseja fazer um novo calculo S/N: ");
        if fim.trim().to_uppercase().as_str() == "N"{
            break;
        }

    }
}
