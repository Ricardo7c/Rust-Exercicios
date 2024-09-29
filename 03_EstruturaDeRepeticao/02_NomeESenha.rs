use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x
}

fn main(){
    let nome = input("Digite o seu nome de usuário: ");
    loop{
        let senha = input("Digite sua senha: ");
        if nome == senha{
            println!("A senha não pode ser igual ao nome de usuário, tente novamente!");
            println!("_______________________________________________________________");

        }else{
            println!("Usuário criado com sucesso!");
            break;
        }
    }
} 