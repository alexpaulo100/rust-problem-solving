/*Faça um programa que leia o nome de um vendedor, o seu salário fixo e o total de vendas efetuadas por ele no mês (em dinheiro).
Sabendo que este vendedor ganha 15% de comissão sobre suas vendas efetuadas,
informar o total a receber no final do mês, com duas casas decimais.*/

use std::io;

fn main() {
    //variaveis
    let mut nome = String::new();
    let mut salario = String::new();
    let mut vendas = String::new();

    // lendo os valores

    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler o valor");
    io::stdin()
        .read_line(&mut salario)
        .expect("Falha ao ler o valor");
    io::stdin()
        .read_line(&mut vendas)
        .expect("Falha ao ler o valor");

    // convertendo as variaveis para f64

    let salario: f64 = salario.trim().parse().expect("Digite um numero ->> ");
    let vendas: f64 = vendas.trim().parse().expect("Digite um numero ->> ");

    // calculando o total a receber

    let total = salario + (vendas * 0.15);

    // imprimindo o resultado

    println!("TOTAL = R$ {:.2}", total);
}
