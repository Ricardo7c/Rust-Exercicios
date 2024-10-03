fn main(){
    let numeros = vec![2, 3, 8, 9, 4, 12, 3, 4, 7, 5];
    let letras = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    let mut misto = Vec::new();

    for indice in 0..numeros.len(){
        let tupla = (numeros[indice], letras[indice]);
        misto.push(tupla);
    }
    
    for tupla in misto{
        print!(" {} {}", tupla.0, tupla.1);
    }
}