use std::io::{self, Write};

fn float(texto: &str) -> f64{
    loop{   
        print!("{}", texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<f64>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn valor_pagamento(valor: f64, dias: f64) -> f64{
    let pag:f64;
    if dias > 0.0{
        pag = valor + (((valor*0.03)+(valor*0.001))*dias);
    }else{
        pag = valor;
    }
    pag
}


fn main(){
    let mut quant = 0;
    let mut soma = 0.0;
    loop {
        let valor = float("Qual o valor da prestação? ");
        if valor == 0.0{
            break;
        }
        let dias = float("Esta atrasada a quantos dias? ");
        let pagamento = valor_pagamento(valor, dias);
        quant += 1;
        soma += pagamento;
        println!("O valor a ser pago é: R$ {:.2}", pagamento)
    }

    println!(" ");
    println!("Foram efetuados {} pagamentos.",quant);
    println!("Um total de R$ {:.2} foi pago", soma)
}