use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_string()
}

fn int(texto: &str) -> i32{
    loop{
        let x = input(texto);
        match x.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn float(texto: &str) -> f32{
    loop{
        let x = input(texto);
        match x.parse::<f32>() {
            Ok(num) => return num,
            Err(_) => println!("Valor invalido!")
        }
    }
}

fn main(){
    let mut nome:String;
    let mut idade:i32;
    let mut salario:f32;
    let mut sexo:String;
    let mut estadocivil:String;

    loop {
        nome = input("Digite seu nome: ");
        if nome.len() > 3{
            break;
        }else{
            println!("O nome deve ter mais de 3 caracteres.");
            println!("-------------------------------------");
        }
    };

    loop {
        idade = int("Digite sua idade: ");
        if idade >= 0 && idade <= 150{
            break;
        }else{
            println!("Valor invalido, tente novamente!");
            println!("--------------------------------");
        }
    };

    loop {
        salario = float("Digite seu salario: ");
        if salario > 0.0{
            break;
        }else{
            println!("Valor invalido, tente novamente!");
            println!("--------------------------------");
        }
    };

    loop {
        sexo = input("Qual o seu sexo M/F? ");
        match sexo.to_uppercase().as_str() {
            "M" => {sexo = String::from("Masculino"); break;},
            "F" => {sexo = String::from("Feminino"); break;},
            _ => println!("Valor invalido!")
        }
    };

    loop {
        estadocivil = input("Qual o seu estado civil S/C/V/D? ");
        match estadocivil.to_uppercase().as_str(){
            "S" => {estadocivil = String::from("Solteiro");break;},
            "C" => {estadocivil = String::from("Casado");break;},
            "V" => {estadocivil = String::from("Viuvo");break;},
            "D" => {estadocivil = String::from("Divorciado");break;},
            _ => println!("Valor invalido!")
        }
    };
    
    println!("-------------------------------");
    println!("Cadastro realizado com sucesso!");
    println!("Nome: {}", nome);
    println!("Idade: {}", idade);
    println!("Salário: R$ {:.2}", salario);
    println!("Sexo: {}",sexo);
    println!("Estado Civil: {}", estadocivil);
    println!("-------------------------------");

}