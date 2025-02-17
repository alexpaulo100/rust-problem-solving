use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    //println!("Entre com um numero:");
    io::stdin().read_line(&mut a).expect("Falha ao ler o valor");

    //println!("Entre com um numero:");
    io::stdin().read_line(&mut b).expect("Falha ao ler o valor");

    let a: i32 = a.trim().parse().expect("Digite um numero ->> ");
    let b: i32 = b.trim().parse().expect("Digite um numero ->> ");

    let x = a + b;

    println!("X = {}", x);
}
