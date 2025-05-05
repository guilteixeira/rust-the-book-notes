fn main() {
    // arrays precisam ter o mesmo tipo de dado, portanto é importante tiparmos a variável
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // eles também possuem tamanho fixo, após sua definição, então não é possível printar valores fora do limite.
    println!("Valor do primeiro indíce do array: {}", a[0]);
    println!("Valor do último indíce do array: {}", a[4]);
    // Se você tentasse algo como:
    // println!("Valor do último indíce do array: {}", a[7]);
    // receberia um erro com:  index out of bounds: the length is 5 but the index is 7
    // o erro informa que o array possui 5 indíces e estamos tentando exibir o index 7, que não existe.

    let months = ["Janeiro", "Fevereiro", "Março", "Abril", "May", "June", "July",
              "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

              println!("O primeiro mês do ano é: {}", months[0]);
              println!("O primeiro mês do ano é: {}", months[11]);
}
