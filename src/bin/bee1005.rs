/* Media 1
Leia 2 valores de ponto flutuante de dupla precisão A e B, que correspondem a 2 notas de um aluno.
A seguir, calcule a média do aluno, sabendo que a nota A tem peso 3.5 e a nota B tem peso 7.5 (A soma dos pesos portanto é 11).
Assuma que cada nota pode ir de 0 até 10.0, sempre com uma casa decimal.*/

use std::io;

fn main() {
    // variaveis
    let mut a = String::new();
    let mut b = String::new();
    // lendo os valores
    io::stdin().read_line(&mut a).expect("Falha ao ler o valor");
    io::stdin().read_line(&mut b).expect("Falha ao ler o valor");
    // convertendo as variaveis para f64
    let a: f64 = a.trim().parse().expect("Digite um numero ->> ");
    let b: f64 = b.trim().parse().expect("Digite um numero ->> ");

    // calculando a media

    let media = ((a * 3.5) + (b * 7.5)) / 11.0;

    // imprimindo o resultado

    println!("MEDIA = {:.5}", media);
}
