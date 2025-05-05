fn main() {
    let a = [2, 4, 6, 8, 10];
    let mut index = 0;

    println!("Printando em um loop while passando o index explícito 5:");
    while index < 5 {
        println!("O valor do index é: {}", a[index]);
        index += 1;
    }

    println!("\nPrintando em um loop while como inicializador, até que chegue a zero:");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("Acabou!!!\n");

    println!("Usando FOR para fazer o loop sobre um array de forma mais eficiente.");
    for value in a {
        println!("O valor do index é: {value}");
    }

    println!("\nUsando FOR para contar de 3 a 1 usando um range.");
    for number in (1..4).rev(){
        println!("{number}");
    }
    println!("Acabou!!!")
}