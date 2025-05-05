fn main() {
    let mut counter = 0;
    println!("Vamos entrar dentro de um loop normal.");
    loop{
        if counter  < 10 {
            println!("dentro do loop!");
            counter += 1;
        } else {
            break;
        }
    }
    println!("O contador girou {counter} vezes.\n");

    println!("Vamos entrar em um loop usando uma label.");
    
    let mut count = 0;
    'contagem: loop { // inicia o loop principal com a contagem, que aqui é um label para o loop
        // labels são declaras com uma aspas simples.
        println!("Iniciando contagem...");
        println!("contagem = {count}"); // iniciamos o loop printando onde está
        let mut remaining = 10; // declara quantos ciclos faltam para o loop
        loop { // inicia outro loop interno para subtrair a variável remaining
            println!("faltam = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'contagem;
            }
            remaining -= 1; // subtrai remaining em 1 resultando em 9
        }

        count += 1; // soma um para a contagem, o que vai finalizar o loop pela label.
    }
    println!("End count = {count}");
}
