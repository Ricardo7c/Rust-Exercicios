use std::io::{self, Write};

fn input(texto: &str) -> String{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_uppercase()

}

fn main(){
    let q1 = input("Telefonou para a vitima? ");
    let q2 = input("Esteve no local do crime? ");
    let q3 = input("Mora perto da vitima? ");
    let q4 = input("Devia para a vitima? ");
    let q5 = input("Já trabalhou com a vitima? ");
    let mut pos = 0;
    let resultado:&str;

    if q1 == "SIM".to_string() {
        pos += 1;
    };

    if q2 == "SIM".to_string() {
        pos += 1;
    };

    if q3 == "SIM".to_string() {
        pos += 1;
    };

    if q4 == "SIM".to_string() {
        pos += 1;
    };

    if q5 == "SIM".to_string() {
        pos += 1;
    };

    if pos <= 1{
        resultado = "INOCENTE";
    }else if pos == 2{
        resultado = "SUSPEITO";
    }else if (pos == 3) || (pos == 4){
        resultado = "CÚMPLICE";
    }else{
        resultado = "CULPADO";
    };
    
    println!("O interrogado foi considerado {}", resultado)


}
