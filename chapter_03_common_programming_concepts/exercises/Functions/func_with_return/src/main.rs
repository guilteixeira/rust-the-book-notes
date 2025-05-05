fn main() {
    let x = sete(); // podemos atribuir o retorno de uma função a uma variável

    println!("O valor de X é: {x}");
}

fn sete() -> u32 {
    7 // Em outras linguagens como python seria necessário a instrução return 7 para o mesmo comportamento.
    // como Rust é uma linguagem baseada em expressões, caso uma linha não possua ; é considerada uma expressão
}