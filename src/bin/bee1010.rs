/*

Neste problema, deve-se ler o código de uma peça 1, o número de peças 1,
o valor unitário de cada peça 1, o código de uma peça 2, o número de peças 2 e o valor unitário de cada peça 2.
Após, calcule e mostre o valor a ser pago.
Entrada
O arquivo de entrada contém duas linhas de dados. Em cada linha haverá 3 valores, respectivamente dois inteiros e um valor com 2 casas decimais.
*/

use std::io;

fn main() {
    // Variáveis para armazenar as entradas
    let mut linha1 = String::new();
    let mut linha2 = String::new();

    // Lendo as duas linhas de entrada
    io::stdin()
        .read_line(&mut linha1)
        .expect("Falha ao ler a linha 1");
    io::stdin()
        .read_line(&mut linha2)
        .expect("Falha ao ler a linha 2");

    // Dividindo as entradas e convertendo para os tipos corretos
    let dados1: Vec<&str> = linha1.trim().split_whitespace().collect();
    let dados2: Vec<&str> = linha2.trim().split_whitespace().collect();

    let qtd1: i32 = dados1[1]
        .parse()
        .expect("Digite um número válido para a quantidade 1");
    let valor1: f64 = dados1[2]
        .parse()
        .expect("Digite um número válido para o valor 1");

    let qtd2: i32 = dados2[1]
        .parse()
        .expect("Digite um número válido para a quantidade 2");
    let valor2: f64 = dados2[2]
        .parse()
        .expect("Digite um número válido para o valor 2");

    // Calculando o total a pagar
    let total = (qtd1 as f64 * valor1) + (qtd2 as f64 * valor2);

    // Imprimindo o resultado
    println!("VALOR A PAGAR: R$ {:.2}", total);
}
