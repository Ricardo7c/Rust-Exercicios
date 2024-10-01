use std::io::{self, Write};

fn int(texto:&str) -> i32{
    loop{
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    let num = int("Digite um numero: ");
    let mut primo = true;
    let mut lista = Vec::new();
    let mut cont = 0;
    
    for cada in 1..=num{
        if num % cada == 0{
            cont += 1;
            lista.push(cada);
        }
    }
    if cont > 2 || num < 2{
        primo = false;
    }

    if primo{
        println!("{} é primo", num);
    }else{
        print!("O numero {} não é primo, ele é divisivel por: ", num);
        for cada in lista{
            print!("{} ", cada)
        }
    }


}