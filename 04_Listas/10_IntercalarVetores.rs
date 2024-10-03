fn main(){
    let numeros = [2, 3, 8, 9, 4, 12, 3, 4, 7, 5];
    let letras = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

    let misto : Vec<_>= numeros.iter().zip(letras.iter()).collect();
    print!("{:?}", misto);
}