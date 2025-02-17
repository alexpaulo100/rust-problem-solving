/*

Leia quatro valores inteiros A, B, C e D. A seguir, calcule e mostre a diferença do
produto de A e B pelo produto de C e D segundo a fórmula:
DIFERENCA = (A * B - C * D).
Entrada

O arquivo de entrada contém 4 valores inteiros.
*/
use std::io;

fn main() {
    // variaveis
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut d = String::new();

    // lendo os valores

    io::stdin().read_line(&mut a).expect("Falha ao ler o valor");
    io::stdin().read_line(&mut b).expect("Falha ao ler o valor");
    io::stdin().read_line(&mut c).expect("Falha ao ler o valor");
    io::stdin().read_line(&mut d).expect("Falha ao ler o valor");

    // convertendo as variaveis para i32

    let a: i32 = a.trim().parse().expect("Digite um numero ->> ");
    let b: i32 = b.trim().parse().expect("Digite um numero ->> ");
    let c: i32 = c.trim().parse().expect("Digite um numero ->> ");
    let d: i32 = d.trim().parse().expect("Digite um numero ->> ");

    // calculando a diferença

    let diferenca = (a * b) - (c * d);

    // imprimindo o resultado

    println!("DIFERENCA = {}", diferenca);
}
