// Faça um Programa que leia um número inteiro menor que 1000 e imprima a quantidade de centenas,
// dezenas e unidades do mesmo.
// Observando os termos no plural a colocação do "e", da vírgula entre outros. Exemplo:
// 326 = 3 centenas, 2 dezenas e 6 unidades
// 12 = 1 dezena e 2 unidades Testar com: 326, 300, 100, 320, 310,305, 301, 101, 311,
// 111, 25, 20, 10, 21, 11, 1, 7 e 16

use std::io::{self, Write};

fn input_int(texto: &str) -> i32{
    print!("{}",texto);
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<i32>().unwrap();
    x
}

fn main(){
    let mut num = input_int("Digite um numero inteiro até mil: ");
    let mut centena: i32 = 0;
    let mut dezena: i32 = 0;
    let mut unidade: i32 = 0;

    while num >= 100 {
        centena += 1;
        num -= 100;
    }
    while num >= 10 {
        dezena += 1;
        num -= 10;
    }
    while num >= 1 {
        unidade += 1;
        num -= 1;
    }

let mut resultado = String::new();

if centena > 0{
    let nomeclatura = if centena > 1 { "centenas" } else { "centena" };
    let separador = if dezena > 0{ if unidade > 0 {", "} else {" e "}} else {if unidade > 0 {" e "} else {" "}};
    resultado.push_str(&format!("{} {}{}", centena, nomeclatura, separador));
}
if dezena > 0{
    let nomeclatura = if dezena > 1 { "dezenas" } else { "dezena" };
    let separador = if unidade > 0 {" e "} else {" "};
    resultado.push_str(&format!("{} {}{}", dezena, nomeclatura, separador));
}
if unidade > 0 {
    let nomeclatura = if unidade > 1 { "unidades" } else { "unidade" };
    resultado.push_str(&format!("{} {}", unidade, nomeclatura));
}

print!("{}", resultado)
}