fn main(){
    let lista = [2, 3, 8, 9, 4, 12, 3, 4, 7, 5];

    let mut soma = 0;
    for num in lista{
        let quad = num*num;
        soma += quad;
    }

    println!("A soma dos quadrados é: {}", soma);

}