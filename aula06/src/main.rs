fn iguais (x: u8, y: u8) -> bool {
    return x == y;
}

fn quadrado(x: i32) -> i32 {
    x * x
}


fn main(){
    let x = 10;
    let y = 11;
    println!("{} == {}? {}", x, y, iguais(x, y));

    let z: i32 = 5;
    println!("Quadrado de {} eh {}", z, quadrado(z));
}