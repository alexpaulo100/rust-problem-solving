/*

Escreva um programa que leia o número de um funcionário, seu número de horas trabalhadas,
o valor que recebe por hora e calcula o salário desse funcionário.
A seguir, mostre o número e o salário do funcionário, com duas casas decimais.
Entrada

O arquivo de entrada contém 2 números inteiros e 1 número com duas casas decimais,
representando o número, quantidade de horas trabalhadas e o valor que o funcionário recebe por hora trabalhada, respectivamente.
*/

use std::io;

fn main() {
    //variaveis
    let mut numero = String::new();
    let mut horas = String::new();
    let mut valor = String::new();

    // lendo as variaveis

    io::stdin()
        .read_line(&mut numero)
        .expect("Falha ao ler o valor");
    io::stdin()
        .read_line(&mut horas)
        .expect("Falha ao ler o valor");
    io::stdin()
        .read_line(&mut valor)
        .expect("Falha ao ler o valor");

    // convertendo as variaveis para i32 e f64

    let numero: i32 = numero.trim().parse().expect("Digite um numero ->> ");
    let horas: i32 = horas.trim().parse().expect("Digite um numero ->> ");
    let valor: f64 = valor.trim().parse().expect("Digite um numero ->> ");

    // calculando o salario

    let salario = horas as f64 * valor;

    // imprimindo o resultado

    println!("NUMBER = {}", numero);

    println!("SALARY = U$ {:.2}", salario);
}
