fn main() {
    // Rust é uma linguagem baseada em expresões, portanto diferimos o que é uma expressão e o que é uma declaração
    // Exemplo:

    // Statements (declarações): são instruções para uma ação que não retornam um valor
    let x = 6; // é uma declaração de uma variável, e não retorna um valor, ou seja, um Statement.
    println!("O valor de X é: {x}");
   
    // Expressions (expressão): São valores resultantes de uma expressão, podendo ser o resultado de uma ação
    // ou operação aritmética como por exemplo:
    let y = (x * 2); // isso é uma expressão aritmética que resulta numa multiplicação de X por 2.
    // resultando em um novo valor:
    println!("O valor de Y é: {y}");

    // uma outra forma de entender isso é que a declaração abaixo por exemplo é impossível:
    // let x = (let y = 6)
    // como uma declaração não retorna um valor, ela não pode ser armazenada em uma variável.
}