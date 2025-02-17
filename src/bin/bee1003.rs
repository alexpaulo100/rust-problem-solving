//Soma simples//

use std::io;

fn main() {
    // variaveis//
    let mut a = String::new();
    let mut b = String::new();

    // lendo os valores//
    io::stdin().read_line(&mut a).expect("Falha ao ler o valor");
    io::stdin().read_line(&mut b).expect("Falha ao ler o valor");

    // convertendo as variaveis para i32//
    let a: i32 = a.trim().parse().expect("Digite um numero ->> ");
    let b: i32 = b.trim().parse().expect("Digite um numero ->> ");

    // somando as variaveis//

    let soma = a + b;

    // imprimindo o resultado//

    println!("SOMA = {}", soma);
}
