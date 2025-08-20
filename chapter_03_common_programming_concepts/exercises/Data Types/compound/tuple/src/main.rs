fn main() {
    let tup: (i8, f64, u8) = (-127, 4.5, 40);
    let x: i8 = tup.0;
    let y: f64 = tup.1;
    let z: u8 = tup.2;


    println!("O valor do primeiro item da tupla é: {}", x);
    println!("O valor do segundo item da tupla é: {}", y);
    println!("O valor do terceiro item da tupla é: {}", z);

}
