fn main() {
    let vetor: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let media: f64
}

fn media_valores(valores: &Vec<i32>) -> f64 {
    let mut soma_valores: f64 = 0.0;
    let mut quantidade_valores: u32 = 0;

    for valor in valores {
        soma_valores += *valor;
        quantidade_valores += 1;
    }
    soma_valores / soma_valores as f64

}