fn main(){
    let lista = vec![2, 6, 8, 15, 5];
    let mut soma = 0;
    let mut multi = 1;
    for cada in &lista{
        soma += cada;
        multi *= cada;
    }

    println!("Os números: {:?}", lista);
    println!("A soma: {}", soma);
    println!("A multiplicação: {}", multi);
}