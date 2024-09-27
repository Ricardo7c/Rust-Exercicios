// Faça um programa que calcule as raízes de uma equação do segundo grau, na forma ax2 + bx + c.
// O programa deverá pedir os valores de a, b e c e fazer as consistências,
// informando ao usuário nas seguintes situações:

// Se o usuário informar o valor de A igual a zero, a equação não é do segundo grau e o programa
// não deve fazer pedir os demais valores, sendo encerrado;
// Se o delta calculado for negativo, a equação não possui raizes reais.
// Informe ao usuário e encerre o programa;
// Se o delta calculado for igual a zero a equação possui apenas uma raiz real;
// informe-a ao usuário;
// Se o delta for positivo, a equação possui duas raiz reais; informe-as ao usuário;

use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<f32>().unwrap();
    x
}

fn main(){
    let a = input_float("Digite o valor de A: ");
    if a == 0.0{
        println!("Esta não é uma equação do segundo grau");
    }else{
        let b = input_float("Digite o valor de B: ");
        let c = input_float("Digite o valor de C: ");

        let delta = (b.powi(2)) - (4.0*a*c);

        if delta < 0.0{
            println!("Essa equação não possui raizes reais");
        }else if delta == 0.0{
            let raiz = -b/(2.0*a);
            println!("A equação possui apenas uma raiz: {}", raiz)
        }else{
            let raiz1 = (-b + delta.sqrt()) / (2.0 * a);
            let raiz2 = (-b - delta.sqrt()) / (2.0 * a);
            println!("Duas Raizes reais: {:.1}, {:.1}", raiz1, raiz2);
        }


    }
}