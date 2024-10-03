fn main(){
    let letras = vec!["A","B","C","D","E","F","G","H","I","J"];
    let mut consoantes = Vec::new();

    for letra in letras{
        let consoante:bool;
        match letra {
            "A" | "E" | "I" | "O" | "U" => consoante = false,
            _ => consoante = true
        }
        if consoante {
            consoantes.push(letra);
        }
    };

    println!("{:?}", consoantes)
}