//Area do Circulo
use std::io;
// inicio do programa
fn main() {
    // variavel para armazenar o valor do raio mut significa que a variavel é mutavel
    let mut raio = String::new();
    // lendo o valor do raio
    io::stdin()
        .read_line(&mut raio)
        .expect("Falha ao ler o valor");
    // convertendo o valor do raio para f64
    let raio: f64 = raio.trim().parse().expect("Digite um numero ->> ");
    // calculando a area do circulo
    let area = 3.14159 * raio * raio;
    // imprimindo o valor da area
    println!("A={:.4}", area);
}
