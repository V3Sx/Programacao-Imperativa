fn main() {
    let mut x = 10;
    let mut y = 3;
    println!("x={x} ({:p}), y={y} ({:p})", &x, &y);
    troca(&mut x, &mut y);
    println!("x={x} ({:p}), y={y} ({:p})", &x, &y);
}

fn troca(a: &mut i32, b: &mut i32) {
    let temp = *a; // o * pega o valor dentro do endereco
    *a = *b;
    *b = temp;
}
