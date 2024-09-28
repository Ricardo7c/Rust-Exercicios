use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_uppercase()
}

fn float(valor: String) -> f32{
    let x :f32 = valor.trim().parse().unwrap();
    x
}

fn main(){
    println!("Qual o tipo de carne: ");
    println!("A) File Duplo");
    println!("B) Alcatra");
    println!("C) Picanha");

    let mut carne:String;
    loop{
        carne = input("Digite a letra correspondente: ");
        if carne == "A".to_string() || carne == "B".to_string() || carne == "C".to_string(){
            break;
        }else {
            println!("Valor invalido, tente novamente!");
        }
    };
    
    let kg = float(input("Quantos Kg Deseja? "));
    let mut tipo = String::from(" ");
    let mut valor = 0.0;
    if carne == "A".to_string(){
        tipo = String::from("Filé Duplo");
        if kg <= 5.0{
            valor = kg*4.90;
        }else{
            valor = kg*5.80;
        }
    }else if carne == "B".to_string(){
        tipo = String::from("Alcatra");
        if kg <= 5.0{
            valor = kg*5.90;
        }else{
            valor = kg*6.80;
        }
    }else if carne == "C".to_string(){
        tipo = String::from("Picanha");
        if kg <= 5.0{
            valor = kg*6.90;
        }else{
            valor = kg*7.80;
        }
    };

    println!("Qual a forma de pagamento: ");
    println!("A) Dinheiro");
    println!("B) Cartão Tabajara");

    let mut pag:String;
    let tipo_pag:String;
    let desconto:f32;
    let valorfinal:f32;
    loop{
        pag = input("Digire a letra correspondente: ");
        if pag == "A".to_string(){
            tipo_pag = "Dinheiro".to_string();
            desconto = 0.0;
            valorfinal = valor;
            break;
        }else if pag == "B".to_string(){
            tipo_pag = "Cartão Tabajara".to_string();
            desconto = valor*0.05;
            valorfinal = valor - desconto;
            break;
        }else{
            println!("Valor invalido, tente novamente!");
        }
    }

    println!("============ Nota Fiscal ==============");
    println!("{} kg de {}", kg, tipo);
    println!("Preço total: R$ {:.2}", valor);
    println!("Forma de pagamento: {}", tipo_pag);
    println!("Desconto: R$ {:.2}", desconto);
    println!("Total a pagar: R$ {:.2}", valorfinal);

}