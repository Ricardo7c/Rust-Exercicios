use std::io::{self, Write};

fn main(){
    print!("Qual o tamanho da area a ser pintada: ");
    io::stdout()
        .flush()
        .expect("");
    let mut area = String::new();
    io::stdin()
        .read_line(&mut area)
        .expect("");
    let area:f32 = area.trim()
        .parse()
        .expect("");
    let mut litros = area/3.0;
    let mut latas = 1;
    while litros > 18.0{
        latas = latas+1;
        litros = litros - 18.0;
    }
    println!("Você vai precisar de {} latas que vai custar R${:.2}", latas, latas*80)   
}