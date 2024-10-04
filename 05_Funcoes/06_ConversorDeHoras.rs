use std::io::{self, Write};

fn int(texto:&str) -> i32{
    loop {
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido, tente novamente!")
        }
    }
}

fn conversor(h:i32) -> i32{
    if h > 12{
        let hora = h - 12;
        hora
    }else{
        h
    }
}

fn infor(h:i32) -> &'static str{
    if h > 12{
        return "PM"
    }else{
        return "AM"
    }
}

fn main(){
    let hora = int("Digite a hora: ");
    let minutos = int("Digite os minutos: ");

    let horas = conversor(hora);
    let info = infor(hora);

    println!("São {}:{} {}", horas, minutos, info);

}