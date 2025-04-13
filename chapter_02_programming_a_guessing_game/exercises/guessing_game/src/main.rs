use std::io;

fn main(){
    println!("Advinhe um número!");
    println!("Por favor, insira sua aposta entre 1 e 100:");

    let mut guess = String::new();
    io::stdin() 
    // O livro indica que caso não tenhamos o `use std::io;` aqui, 
    // teríamos que usar `std::io::stdin()`
        .read_line(&mut guess)
        // A próxima linha pode ser ignorada, 
        // podem irá lançar dois warnings por não tratar um possível erro de input.
        .expect("Falha ao ler a linha"); 
    println!("Você apostou: {}", guess);
}