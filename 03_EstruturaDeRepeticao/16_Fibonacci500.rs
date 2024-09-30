fn main(){
    let mut a = 0;
    let mut b = 1;
    print!("{} {} ", a, b);

    while b < 500 {
        (a, b) = (b, a+b);
        print!("{} ", b);
    }
}