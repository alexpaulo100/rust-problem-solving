/*beecrowd | 1012
Área
Escreva um programa que leia três valores com ponto flutuante de dupla precisão: A, B e C. Em seguida, calcule e mostre:
a) a área do triângulo retângulo que tem A por base e C por altura.
b) a área do círculo de raio C. (pi = 3.14159)
c) a área do trapézio que tem A e B por bases e C por altura.
d) a área do quadrado que tem lado B.
e) a área do retângulo que tem lados A e B.
Entrada

O arquivo de entrada contém três valores com um dígito após o ponto decimal.

*/

use std::io;

fn main() {

    // Lendo a entrada
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler a entrada");
    // Convertendo a entrada para um vetor de f64 
    let valores: Vec<f64> = entrada
        .trim()
        .split_whitespace()
        .map(|valor| valor.parse().unwrap())
        .collect();
    // Atribuindo os valores do vetor a variáveis
    let a = valores[0];
    let b = valores[1];
    let c = valores[2];
    //  Calculando as áreas
    let area_triangulo = a * c / 2.0;
    let area_circulo = 3.14159 * c.powi(2);
    let area_trapezio = (a + b) * c / 2.0;
    let area_quadrado = b.powi(2);
    let area_retangulo = a * b;
    //  Imprimindo as áreas
    println!("TRIANGULO: {:.3}", area_triangulo);
    println!("CIRCULO: {:.3}", area_circulo);
    println!("TRAPEZIO: {:.3}", area_trapezio);
    println!("QUADRADO: {:.3}", area_quadrado);
    println!("RETANGULO: {:.3}", area_retangulo);
}

