use std::io::{self, Write};

fn input(texto: &str) -> Vec<i32>{ 
    loop {
        print!("{}", texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        let mut cpftratado = vec![];
        for cada in x.trim().chars(){
            if cada.is_alphanumeric(){
                cpftratado.push(cada.to_string().parse::<i32>().unwrap());
            }
        }
        if cpftratado.len() == 11{
            return cpftratado
        }else{
            println!("CPF invalido!")
        }
    }
}

fn main(){
    let cpf = input("Digite o CPF: ");
    let mut soma1  = 0;
    let mut cont = 10;
    for indice  in 0..9{
        soma1 += cpf[indice]*cont;
        cont -= 1;
    }

    let mut digito1 = (soma1*10)%11;
    if digito1 == 10 {
        digito1 = 0;
    }

    let mut soma2 = 0;
    let mut cont = 11;
    for indice  in 0..10{
        soma2 += cpf[indice]*cont;
        cont -= 1;
    }

    let mut digito2 = (soma2*10)%11;
    if digito2 == 10{
        digito2 = 0;
    }

    if digito1 == cpf[9] && digito2 == cpf[10]{
        println!("CPF Valido!");
    }else{
        println!("CPF Invalido!");
    }



}