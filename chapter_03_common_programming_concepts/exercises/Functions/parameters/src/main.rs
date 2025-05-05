fn main() {
   let mensagem = "Rust é foda!";
    exibir_mensagem(mensagem); // funções podem ser iniciadas dentro de outras funções
    // normalmente na maioria das linguagens a função main é utilizada como um bootloader, para iniciar o programa
    // algumas funções podem esperar parâmetros para determinadas ações, como uma mensagem para exibir na tela.

    // também podemos fazer o mesmo para números, desde que tenhamos uma função que espere por ele.
    exibir_numero(10);

    // uma função pode esperar mais de um parâmetro, como por exemplo uma função que espera um peso, em diversas unidades de medida.
    exibir_peso(10, "kg");
}

fn exibir_mensagem(msg: &str) {
    println!("A mensagem recebida é: {msg}");
}

fn exibir_numero(num: u32) {
    println!("O valor do número é: {num}");
}

fn exibir_peso(value: u32, unit: &str){
    println!("O peso informado é: {value}{unit}");
}