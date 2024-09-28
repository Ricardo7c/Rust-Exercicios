use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().to_string();
    x
}

fn float(valor: String) -> f32{
    let x :f32 = valor.trim().parse().unwrap();
    x
}

fn main(){
    let num1 = float(input("Digite um número: "));
    let num2 = float(input("Digite outro número: "));
    let resultado:f32;
    loop{
        let operador = input("Digite o operador: ");
        if operador == String::from("+"){
            resultado = num1+num2;
            break;
        }else if operador == String::from("-"){
            resultado = num1-num2;
            break;
        }else if operador == String::from("*"){
            resultado = num1*num2;
            break;
        }else if operador == String::from("/"){
            if num2 > 0.0{
                resultado = num1/num2;
                break;
            }else{
                println!("Divisão por zero não é permitido!");
            }
        }else{
            println!("Operador invalido, tente novamente!");
        }
    };

    let par_inp:&str;
    let pos_neg:&str;
    let dec_int:&str;

    if &resultado % 2.0 == 0.0{
        par_inp = "Par";
    }else{
        par_inp = "Inpar";
    };

    if resultado < 0.0{
        pos_neg = "Negativo";
    }else{
        pos_neg = "Positivo";
    };

    let result = resultado.trunc();
    if result == resultado{
        dec_int = "Inteiro";
    }else{
        dec_int = "Decimal";
    };

    println!("O resultado da operação é {:.1} um numero {}, {} e {}", resultado, par_inp, pos_neg, dec_int);

}