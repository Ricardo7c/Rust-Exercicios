fn soma(a: i32, b:i32, c:i32) -> i32{
    let x = a+b+c;
    x
}

fn main(){
    let soma = soma(2, 3, 5);
    println!("{}", soma);
}