/*Leia 3 valores, no caso, variáveis A, B e C, que são as três notas de um aluno.
A seguir, calcule a média do aluno, sabendo que a nota A tem peso 2, a nota B tem peso 3 e a nota C tem peso 5.
Considere que cada nota pode ir de 0 até 10.0, sempre com uma casa decimal.*/

use std::io;

fn main() {
    // Variaveis
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    // Lendo os valores

    io::stdin().read_line(&mut a).expect("Falha ao ler o valor");
    io::stdin().read_line(&mut b).expect("Falha ao ler o valor");
    io::stdin().read_line(&mut c).expect("Falha ao ler o valor");

    // Convertendo as variaveis para f64

    let a: f64 = a.trim().parse().expect("Digite um numero ->> ");
    let b: f64 = b.trim().parse().expect("Digite um numero ->> ");
    let c: f64 = c.trim().parse().expect("Digite um numero ->> ");

    // Calculando a média

    let media = ((a * 2.0) + (b * 3.0) + (c * 5.0)) / 10.0;

    // Imprimindo o resultado

    println!("MEDIA = {:.1}", media);
}
