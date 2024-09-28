use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_uppercase()
}

fn int(valor: String) -> f32{
    let x :f32 = valor.trim().parse().unwrap();
    x
}

fn main(){

    let alcool = String::from("A");
    let gasolina = String::from("G");
    let mut combustivel:String;

    loop{
        combustivel = input("Qual o combustivel desejado: (G)asolina ou (A)lcool? ");
        if combustivel == alcool || combustivel == gasolina{
            break;
        }
    }
    let litros = int(input("Qual a quantidade em litros? "));

    let mut valor:f32 = 0.0;
    if combustivel == alcool{
        valor = litros*1.90;
        if litros <= 20.0{
            valor -= valor*0.03;
        }else {
            valor -= valor*0.05;
        }
    }else if combustivel == combustivel{
        valor = litros*2.50;
        if litros <= 20.0{
            valor -= valor*0.04;
        }else {
            valor -= valor*0.06;
        }
    }

    println!("O valor a ser pago é de R${:.2}", valor);
}