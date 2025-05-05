fn main() {
    //podemos declarar uma tupla dividindo os valores com vírgulas entre parênteses.
    let tup = (500, 6.4, 1);
    
    let (x, y, z) = tup; 

    println!("O valor de x é: {x}");
    println!("O valor de y é: {y}");
    println!("O valor de z é: {z}");

    // Também podemos setar a tipagem estática para cada valor dentro da tupla, eles não necessariamente precisam ter o mesmo tipo de dado.
    let tup2: (i32, f64, u8) = (500, 6.4, 1);

    let quinhentos = tup2.0;
    let seis_ponto_quatro = tup2.1;
    let um = tup2.2;
    println!("O valor de quinhentos é: {quinhentos}");
    println!("O valor de seis_ponto_quatro é: {seis_ponto_quatro}");
    println!("O valor de um é: {um}");
}