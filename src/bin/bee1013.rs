/*
beecrowd | 1013
O Maior
Faça um programa que leia três valores e apresente o maior dos três valores lidos seguido da mensagem “eh o maior”. Utilize a fórmula:

Obs.: a fórmula apenas calcula o maior entre os dois primeiros (a e b). Um segundo passo, portanto é necessário para chegar no resultado esperado.
Entrada

O arquivo de entrada contém três valores inteiros.

*/

use std::io;    

fn main() {
    // Lendo a entrada
    let mut entrada = String::new();
    // Convertendo a entrada para um vetor de i32
    io::stdin().read_line(&mut entrada).expect("Falha ao ler a entrada");
    // Atribuindo os valores do vetor a variáveis
    let valores: Vec<i32> = entrada.trim().split_whitespace().map(|valor| valor.parse().unwrap()).collect();
    // Atribuindo os valores do vetor a variáveis
    let a = valores[0];
    let b = valores[1];
    let c = valores[2];
    //  Calculando o maior valor
    let maior_ab = (a + b + (a - b).abs()) / 2;
    let maior = (maior_ab + c + (maior_ab - c).abs()) / 2;
    //  Imprimindo o maior valor
    println!("{} eh o maior", maior);

}