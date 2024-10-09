use std::fs::{File, OpenOptions};
use std::io::{self, Write};

fn escrever(arquivo: &str, conteudo: &str) -> io::Result<()>{
    File::create(arquivo)?.write_all(conteudo.as_bytes())?;
    Ok(())
}

fn proxima_linha(arquivo: &str, conteudo: &str) -> io::Result<()>{
    OpenOptions::new().append(true).open(arquivo)?.write_all(conteudo.as_bytes())?;
    Ok(())
}


fn main(){
    let arquivo = "novo.txt";
    let conteudo = "Testando arquivo de leitura!\n";
    let _ = escrever(arquivo, conteudo);
    for cada in 0..=10{
        let linha = "Texto da linha numero: ".to_owned() + &cada.to_string() + "\n";
        let _ = proxima_linha(arquivo, &linha);
    }

}
