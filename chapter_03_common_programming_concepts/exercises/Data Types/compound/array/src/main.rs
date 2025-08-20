use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    println!("Insira um índice para um vetor: ");
    io::stdin()
        .read_line(&mut index)
        .expect("Não foi possível ler o valor inserido");

    let index: usize = index
        .trim()
        .parse::<usize>()
        .expect("Por favor, insira um número inteiro.");

    let element = a[index];

    println!("O valor do elemento no index {index} é {element}.");
}
