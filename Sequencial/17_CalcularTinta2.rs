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
    let lt = (area/6.0)+((area/6.0)*0.10);
    let mut litros:f32 = lt;
    let mut latas:f32 = 0.0;
    let mut latinha:f32 = 0.0;

    while litros > 0.0 {
        latas = latas+1.0;
        litros = litros-18.0;
    }

    println!("Você precisa de {} latas de 18lt que vai custar R${:.2}", latas, latas*80.0);
    println!("ou");

    litros = lt;

    while litros > 0.0 {
        latinha = latinha+1.0;
        litros = litros-3.6;
    
    }

    println!("Você precisa de {} latinhas de 3.6lt que vai custar R${:.2}", latinha, latinha*25.0);
    println!("ou");

    litros = lt;
    latas = 0.0;
    latinha = 0.0;

    while litros > 18.0 {
        latas = latas+1.0;
        litros = litros-18.0;
    }
    while litros > 0.0 {
        latinha = latinha+1.0;
        litros = litros - 3.6;
    }

    println!("Você precisa de {} latas e {} latinhas. que vai custar R${:.2}", latas, latinha, (latas*80.0)+(latinha*25.0));
}