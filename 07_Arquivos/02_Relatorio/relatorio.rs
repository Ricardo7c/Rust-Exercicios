use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn conveter_mb(tamanho: String) -> f64{
    let x = tamanho.trim().parse::<f64>().unwrap();
    let resultado = (x / 1024.0) / 1024.0;
    resultado
}

fn ler_arquivo(arquivo: &str) -> Vec<(String, f64)>{
    let mut nome = String::new();
    let mut tamanho = String::new();
    let mut usuarios: Vec<(String, f64)> = vec![];
    let mut x = String::new();
    File::open(arquivo).unwrap().read_to_string(&mut x).unwrap();

    for cada in x.chars(){
        if cada.is_alphabetic(){
            nome.push(cada);
        }else if cada.is_alphanumeric(){
            tamanho.push(cada);
        }else if cada == '\n' {
            let tupla = (nome.clone(), conveter_mb(tamanho.clone()));
            usuarios.push(tupla);
            nome.clear();
            tamanho.clear();
        }

    }
    usuarios
}


fn proxima_linha(arquivo: &str, conteudo: String){
    let mut x = OpenOptions::new().append(true).open(arquivo).unwrap();
    x.write_all(conteudo.as_bytes()).unwrap();
}

fn main() {
    let usuarios = ler_arquivo("usuarios.txt");
    let mut total = 0.0;
    
    proxima_linha("Relatorio.txt", "ACME Inc.          Uso do espaço em disco pelos usuários \n".to_string());
    proxima_linha("Relatorio.txt", "----------------------------------------------------------------- \n".to_string());
    for (id, cada) in usuarios.iter().enumerate(){
        total += cada.1;
        let conteudo = format!("{} - {:<10} {:>20.2} \n", id+1, cada.0, cada.1);
        proxima_linha("Relatorio.txt", conteudo);
    }

    let media = total/usuarios.len() as f64;

    proxima_linha("Relatorio.txt",format!("\rEspaço total ocupado: {:.2} \n", total));
    proxima_linha("Relatorio.txt",format!("Espaço médio ocupado: {:.2}", media));
    println!("Aquivo escrito com sucesso!");
}
