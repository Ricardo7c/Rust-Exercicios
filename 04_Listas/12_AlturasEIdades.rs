fn main(){
    let idades = [12, 14, 11, 15, 13, 10, 14, 12, 13, 11, 15, 10, 14, 12, 13, 15, 11, 10, 14, 13, 12, 15, 11, 14, 13, 10, 15, 12, 11, 14];
    let alturas = [160.5, 172.3, 168.7, 155.2, 180.0, 170.4, 164.6, 177.8, 169.0, 158.3, 165.2, 180.1, 161.7, 174.5, 167.4, 156.8, 162.9, 175.3, 163.4, 179.0, 166.1, 157.7, 161.5, 173.6, 160.2, 172.9, 168.1, 159.4, 164.8, 176.7];

    let mut soma = 0.0;

    for altura in alturas{
        soma += altura;
    }

    let media = soma/30.0;

    let mut alunos = 0;

    for indice in 0..alturas.len(){
        if idades[indice] > 13 && alturas[indice] < media{
            alunos +=1
        }
    }

    println!("{} alunos com mais de 13 anos tem altura inferior a media", alunos);

}