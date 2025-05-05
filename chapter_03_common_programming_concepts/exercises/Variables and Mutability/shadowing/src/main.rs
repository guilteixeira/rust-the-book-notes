fn main() {
    let x = 5;

    let x = x + 1;
    // Ao re-declarar a variável incluindo, OU NÃO, ela mesma como parte da atribuição, fazemos o que o Rust chama de shadowing.
    // Nada mais é do que modificar uma variável imutável, reatribuindo o valor a ela, isso é interessante,
    // para variáveis que temporariamente irão armazenar um valor e alterar, como conversões de INT para STR.
    // A principal diferença de shadowing pra mutability é que quando iniciamos uma variável mutável, ainda precisamos obedecer a tua tipagem inicial,
    // em shadowing como eu disse acima, podemos converter valores facilmente, declarando uma nova variável com o novo valor de desejado
    // ou seja, um novo espaço de memória é linkado a essa etiqueta, com novos valores independente de quais forem, isso não irá gerar um erro de compilação por typeError.
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // aqui fazemos outro shadowing dentro de um escopo fechado,
        //  ou seja a variável X do lado de fora, permanece inalterada.
    }

    println!("The value of x is: {x}");
}