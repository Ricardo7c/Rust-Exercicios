use std::fs::File;
use std::io::{self, BufRead};

fn buscar_linha(valor: &str) -> io::Result<()>{
    // Abre o arquivo para leitura
    let arquivo = File::open("novo.txt")?;

    // Crar o buffer de leitura
    let leitor = io::BufReader::new(arquivo);


    // itera e imprime linha por linha!
    for linha in leitor.lines(){
        let linha = linha?;
        if linha.contains(valor) {
        println!("{}", linha);
        }
    }

    Ok(())

}

fn main(){
    let _ = buscar_linha("5");
}