use std::net::{Ipv4Addr, Ipv6Addr};
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};

fn validar_ip(ip: String) -> Result<(), String> {
    if ip.parse::<Ipv4Addr>().is_ok() {
        Ok(())
    } else if ip.parse::<Ipv6Addr>().is_ok() {
        Ok(())
    } else {
        Err(format!("Endereço IP inválido: {}", ip))
    }
}

fn ler_arquivo(arquivo: &str) -> Vec<String>{
    let mut x = String::new();
    File::open(arquivo).unwrap().read_to_string(&mut x).unwrap();
    let mut ips = vec![];
    let mut temp = String::new();

    for cada in x.chars(){
        if cada != '\n'{
            if cada != '\r'{
                temp.push(cada);
            }
        }else{
            ips.push(temp.clone());
            temp.clear();
            }
    }
    ips
}


fn proxima_linha(arquivo: &str, conteudo: String){
    let mut x = OpenOptions::new().append(true).open(arquivo).unwrap();
    x.write_all(conteudo.as_bytes()).unwrap();
}

fn escrevendo(conteudo:Vec<String>, arquivo: &str, valido: bool){
    if valido{
        let _ = proxima_linha(arquivo, "----- Ips validos -----".to_string().to_owned()+"\n");
        for cada in conteudo{
            let _ = proxima_linha(arquivo, cada);
        }
        let _ = proxima_linha(arquivo, "\n".to_string());
    }else{
        let _ = proxima_linha(arquivo, "  ".to_string());
        let _ = proxima_linha(arquivo, "----- Ips invalidos -----".to_string().to_owned()+"\n");
        for cada in conteudo{
            let _ = proxima_linha(arquivo, cada);
        }
        let _ = proxima_linha(arquivo, "\n".to_string());
    }
}


fn main() {
    let ips = ler_arquivo("IPsParaValidar.txt");
    let arquivo = "IpsValidados.txt";
    let mut validos = Vec::new();
    let mut invalidos = Vec::new();
    for ip in ips{
        let _ = match validar_ip(ip.clone()) {
            Ok(_) => validos.push(ip.to_owned()+"\n"),
            Err(_) => invalidos.push(ip.to_owned()+"\n"),
        };
    };

    escrevendo(validos, &arquivo, true);
    escrevendo(invalidos, &arquivo, false);
}
