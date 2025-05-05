fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 
    // constantes, são como o nome diz, constantes e não podem ser alteradas.
    // por padrão existe uma convenção de que nome de constantes devem ser em letras maiúsculas.
    // caso você utilize letras minúsculas o compilador irá emitir um alerta dizendo para usar maiúsculas, mesmo que isso não impeça o código de rodar.
    println!("Três horas em segundo são: {three_hours_in_seconds} segundos");
}
