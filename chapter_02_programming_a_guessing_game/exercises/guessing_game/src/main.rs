use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe um número!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Por favor, insira sua aposta entre 1 e 10:");

        let mut guess = String::new();

        io::stdin()
            // O livro indica que caso não tenhamos o `use std::io;` aqui,
            // teríamos que usar `std::io::stdin()`
            .read_line(&mut guess)
            // A próxima linha pode ser ignorada,
            // podem irá lançar dois warnings por não tratar um possível erro de input.
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você apostou: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
