use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}", texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn int(texto: &str) -> i32{
    loop{
        let x = input(texto);
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    let mut total = int("Informe o numero total de eleitores: ");
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    while total > 0 {
        println!("Escolha o seu candidato: ");
        println!("A) Pedro");
        println!("B) João");
        println!("C) Vitor");
        loop {
            let voto = input("Digite a letra correspondente ao candidato: ");
            match voto.to_uppercase().trim() {
                "A" => {
                    println!("Voto computado!");
                    a+=1;
                    total -= 1;
                    break;
                },
                "B" => {
                    println!("Voto computado!");
                    b+=1;
                    total -= 1;
                    break;
                },
                "C" => {
                    println!("Voto computado!");
                    c+=1;
                    total -= 1;
                    break;
                },
                _ => println!("Vato invalido!")
            }
        }
    }

    println!("---------------------------");
    println!("-----VOTAÇÃO ENCERRADA-----");
    println!(" ");
    println!("Pedro {} votos", a);
    println!("João {} votos", b);
    println!("Vitor {} votos", c);
}