use std::io::{self, Write};

fn input_float(texto: &str) -> f32{
    print!("{texto}");
    io::stdout()
        .flush()
        .expect("");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("");
    let x:f32 = x.trim()
        .parse()
        .expect("");
    x
}

fn main (){

    let arquivo = input_float("Digite o tamamnho do arquivo em MBs: ");
    let velocidade = input_float("Digite a velocidade da internet em MBs: ");
    let internet = velocidade/8.0;
    let tempo_segundos = arquivo/internet;
    let tempo_minutos = tempo_segundos/60.0;
    println!("Tempo aproximado de download: {:.2} minutos", tempo_minutos);
    
}